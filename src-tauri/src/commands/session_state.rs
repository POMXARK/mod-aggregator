// Session state commands
// Implementation for User Story 6: State Persistence and Session Recovery

use crate::database::Database;
use crate::models::session_state::{RecentAction, SessionState, UiPreferences};
use chrono::Utc;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

/// Результат восстановления сессии
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRestoreResult {
    pub restored: bool,
    pub file_order: Vec<i64>,
    pub ui_preferences: UiPreferences,
    pub open_collections: Vec<i64>,
    pub selected_files: Vec<i64>,
    pub warnings: Vec<String>,
    pub current_page: Option<String>,
    pub selected_site_id: Option<i64>,
}

/// Получить сохраненное состояние сессии
#[tauri::command]
pub async fn get_session_state() -> Result<SessionState, String> {
    info!("Getting session state");

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    match db.get_session_state().await {
        Ok(state_json) => {
            let state: SessionState = serde_json::from_value(state_json).map_err(|e| {
                error!("Failed to parse session state from database: {}", e);
                format!("Database state parsing error: {}", e)
            })?;
            Ok(state)
        },
        Err(e) => {
            error!("Failed to get session state: {}", e);
            // Возвращаем состояние по умолчанию
            Ok(SessionState::default())
        }
    }
}

/// Обновить порядок файлов
#[tauri::command]
pub async fn update_file_order(file_order: Vec<i64>) -> Result<(), String> {
    info!("Updating file order: {} files", file_order.len());

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование всех файлов
    for file_id in &file_order {
        let file = db.get_file(*file_id).await.map_err(|e| {
            error!("Failed to check file {}: {}", file_id, e);
            format!("Database error: {}", e)
        })?;

        if file.is_none() {
            return Err(format!("File {} not found", file_id));
        }
    }

    db.update_session_file_order(&file_order)
        .await
        .map_err(|e| {
            error!("Failed to update file order: {}", e);
            format!("Database error: {}", e)
        })?;

    Ok(())
}

/// Обновить настройки UI
#[tauri::command]
pub async fn update_ui_preferences(preferences: serde_json::Value) -> Result<(), String> {
    info!("Updating UI preferences");

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let ui_prefs: UiPreferences = serde_json::from_value(preferences).map_err(|e| {
        error!("Failed to parse UI preferences: {}", e);
        format!("Invalid preferences format: {}", e)
    })?;

    let ui_prefs_json = serde_json::to_value(&ui_prefs).map_err(|e| format!("Serialization error: {}", e))?;
    db.update_session_ui_preferences(&ui_prefs_json)
        .await
        .map_err(|e| {
            error!("Failed to update UI preferences: {}", e);
            format!("Database error: {}", e)
        })?;

    Ok(())
}

/// Обновить список открытых коллекций
#[tauri::command]
pub async fn update_open_collections(collection_ids: Vec<i64>) -> Result<(), String> {
    info!(
        "Updating open collections: {} collections",
        collection_ids.len()
    );

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование всех коллекций
    for collection_id in &collection_ids {
        let collection = db.get_collection(*collection_id).await.map_err(|e| {
            error!("Failed to check collection {}: {}", collection_id, e);
            format!("Database error: {}", e)
        })?;

        if collection.is_none() {
            return Err(format!("Collection {} not found", collection_id));
        }
    }

    db.update_session_open_collections(&collection_ids)
        .await
        .map_err(|e| {
            error!("Failed to update open collections: {}", e);
            format!("Database error: {}", e)
        })?;

    Ok(())
}

/// Обновить список выбранных файлов
#[tauri::command]
pub async fn update_selected_files(file_ids: Vec<i64>) -> Result<(), String> {
    info!("Updating selected files: {} files", file_ids.len());

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование всех файлов
    for file_id in &file_ids {
        let file = db.get_file(*file_id).await.map_err(|e| {
            error!("Failed to check file {}: {}", file_id, e);
            format!("Database error: {}", e)
        })?;

        if file.is_none() {
            return Err(format!("File {} not found", file_id));
        }
    }

    db.update_session_selected_files(&file_ids)
        .await
        .map_err(|e| {
            error!("Failed to update selected files: {}", e);
            format!("Database error: {}", e)
        })?;

    Ok(())
}

/// Добавить действие в историю
#[tauri::command]
pub async fn add_recent_action(
    action_type: String,
    target_id: i64,
    target_name: String,
) -> Result<(), String> {
    info!(
        "Adding recent action: {} - {} ({})",
        action_type, target_name, target_id
    );

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    use crate::models::session_state::RecentActionType;

    let action_type_enum = match action_type.as_str() {
        "file_added" => RecentActionType::FileAdded,
        "file_deleted" => RecentActionType::FileDeleted,
        "collection_created" => RecentActionType::CollectionCreated,
        "dependency_added" => RecentActionType::DependencyAdded,
        "dependency_removed" => RecentActionType::DependencyRemoved,
        "collection_updated" => RecentActionType::CollectionUpdated,
        _ => {
            warn!(
                "Unknown action type: {}, defaulting to FileAdded",
                action_type
            );
            RecentActionType::FileAdded
        }
    };

    let action = RecentAction {
        r#type: action_type_enum,
        target_id,
        target_name,
        timestamp: Utc::now(),
    };

    let action_json = serde_json::to_value(&action).map_err(|e| format!("Serialization error: {}", e))?;
    db.add_recent_action(&action_json).await.map_err(|e| {
        error!("Failed to add recent action: {}", e);
        format!("Database error: {}", e)
    })?;

    Ok(())
}

/// Получить последние действия
#[tauri::command]
pub async fn get_recent_actions(limit: Option<usize>) -> Result<Vec<RecentAction>, String> {
    let limit = limit.unwrap_or(50);
    info!("Getting recent actions (limit: {})", limit);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let actions_json = db.get_recent_actions(limit).await.map_err(|e| {
        error!("Failed to get recent actions: {}", e);
        format!("Database error: {}", e)
    })?;

    let actions: Vec<RecentAction> = actions_json
        .iter()
        .filter_map(|action_json| {
            serde_json::from_value(action_json.clone()).ok()
        })
        .collect();

    Ok(actions)
}

/// Очистить историю действий
#[tauri::command]
pub async fn clear_recent_actions() -> Result<(), String> {
    info!("Clearing recent actions");

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    db.clear_recent_actions().await.map_err(|e| {
        error!("Failed to clear recent actions: {}", e);
        format!("Database error: {}", e)
    })?;

    Ok(())
}

/// Восстановить сессию при запуске приложения
#[tauri::command]
pub async fn restore_session() -> Result<SessionRestoreResult, String> {
    info!("Restoring session");

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let state = db.get_session_state().await.map_err(|e| {
        error!("Failed to get session state: {}", e);
        format!("Database error: {}", e)
    })?;

    let mut warnings = Vec::new();

    // Проверяем существование файлов в file_order
    let mut valid_file_order = Vec::new();
    if let Some(file_order) = state["file_order"].as_array() {
        for file_id_value in file_order {
            if let Some(file_id) = file_id_value.as_i64() {
                let file = db.get_file(file_id).await.ok().flatten();
                if file.is_some() {
                    valid_file_order.push(file_id);
                } else {
                    warnings.push(format!("File {} not found, removed from order", file_id));
                }
            }
        }
    }

    // Проверяем существование коллекций в open_collections
    let mut valid_open_collections = Vec::new();
    if let Some(open_collections) = state["open_collections"].as_array() {
        for collection_id_value in open_collections {
            if let Some(collection_id) = collection_id_value.as_i64() {
                let collection = db.get_collection(collection_id).await.ok().flatten();
                if collection.is_some() {
                    valid_open_collections.push(collection_id);
                } else {
                    warnings.push(format!(
                        "Collection {} not found, removed from open collections",
                        collection_id
                    ));
                }
            }
        }
    }

    // Проверяем существование файлов в selected_files
    let mut valid_selected_files = Vec::new();
    if let Some(selected_files) = state["selected_files"].as_array() {
        for file_id_value in selected_files {
            if let Some(file_id) = file_id_value.as_i64() {
                let file = db.get_file(file_id).await.ok().flatten();
                if file.is_some() {
                    valid_selected_files.push(file_id);
                } else {
                    warnings.push(format!(
                        "File {} not found, removed from selected files",
                        file_id
                    ));
                }
            }
        }
    }

    // Обновляем состояние, если были удалены несуществующие элементы
    let original_file_order_len = state["file_order"].as_array().map(|a| a.len()).unwrap_or(0);
    let original_open_collections_len = state["open_collections"].as_array().map(|a| a.len()).unwrap_or(0);
    let original_selected_files_len = state["selected_files"].as_array().map(|a| a.len()).unwrap_or(0);

    if valid_file_order.len() != original_file_order_len
        || valid_open_collections.len() != original_open_collections_len
        || valid_selected_files.len() != original_selected_files_len
    {
        if !valid_file_order.is_empty() {
            db.update_session_file_order(&valid_file_order).await.ok();
        }
        if !valid_open_collections.is_empty() {
            db.update_session_open_collections(&valid_open_collections)
                .await
                .ok();
        }
        if !valid_selected_files.is_empty() {
            db.update_session_selected_files(&valid_selected_files)
                .await
                .ok();
        }
    }

    let ui_preferences: UiPreferences = serde_json::from_value(state["ui_preferences"].clone()).unwrap_or_default();

    Ok(SessionRestoreResult {
        restored: true,
        file_order: valid_file_order,
        ui_preferences: ui_preferences.clone(),
        open_collections: valid_open_collections,
        selected_files: valid_selected_files,
        warnings,
        current_page: ui_preferences.current_page.clone(),
        selected_site_id: ui_preferences.selected_site_id,
    })
}

/// Обновить текущую страницу
#[tauri::command]
pub async fn update_current_page(page: String) -> Result<(), String> {
    info!("Updating current page: {}", page);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    db.update_session_current_page(&page).await.map_err(|e| {
        error!("Failed to update current page: {}", e);
        format!("Database error: {}", e)
    })?;

    Ok(())
}

/// Обновить выбранный сайт
#[tauri::command]
pub async fn update_selected_site(site_id: Option<i64>) -> Result<(), String> {
    info!("Updating selected site: {:?}", site_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    db.update_session_selected_site(site_id).await.map_err(|e| {
        error!("Failed to update selected site: {}", e);
        format!("Database error: {}", e)
    })?;

    Ok(())
}

/// Сбросить состояние сессии к значениям по умолчанию
#[tauri::command]
pub async fn reset_session_state() -> Result<(), String> {
    info!("Resetting session state");

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    db.reset_session_state().await.map_err(|e| {
        error!("Failed to reset session state: {}", e);
        format!("Database error: {}", e)
    })?;

    Ok(())
}
