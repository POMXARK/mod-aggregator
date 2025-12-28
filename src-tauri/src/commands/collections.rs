// Collection management commands
// Implementation for User Story 4: Advanced Collection Logic and Composition

use crate::database::Database;
use crate::models::collection::Collection;
use crate::models::collection_logic::{Action, CollectionLogicRule, ConditionType};
use crate::models::file::File;
use log::{error, info};

/// Структура файла в коллекции для ответа API
#[derive(serde::Serialize)]
pub struct CollectionFile {
    pub id: i64,
    pub collection_id: i64,
    pub file_id: i64,
    pub file: File,
    pub logic_rule_id: Option<i64>,
    pub logic_rule: Option<CollectionLogicRule>,
    pub order_index: i64,
    pub created_at: String,
}

/// Структура для просмотра файла из нескольких коллекций
#[derive(serde::Serialize)]
pub struct CollectionFileView {
    pub file_id: i64,
    pub file: File,
    pub collections: Vec<CollectionFileInfo>,
}

/// Информация о файле в коллекции
#[derive(serde::Serialize)]
pub struct CollectionFileInfo {
    pub collection_id: i64,
    pub collection_name: String,
    pub order_index: i64,
    pub logic_rule_id: Option<i64>,
}

/// Результат оценки логики коллекции
#[derive(serde::Serialize)]
pub struct CollectionEvaluationResult {
    pub collection_id: i64,
    pub enabled_files: Vec<i64>,
    pub disabled_files: Vec<i64>,
    pub evaluation_details: Vec<FileEvaluationDetail>,
}

/// Детали оценки файла
#[derive(serde::Serialize)]
pub struct FileEvaluationDetail {
    pub file_id: i64,
    pub enabled: bool,
    pub applied_rules: Vec<i64>,
}

/// Получить все коллекции
#[tauri::command]
pub async fn get_collections() -> Result<Vec<Collection>, String> {
    info!("Getting all collections");

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let collections = db.get_collections().await.map_err(|e| {
        error!("Failed to get collections: {}", e);
        format!("Database error: {}", e)
    })?;

    info!("Found {} collection(s)", collections.len());
    Ok(collections)
}

/// Параметры для создания коллекции
#[derive(serde::Deserialize)]
pub struct CreateCollectionParams {
    pub name: String,
    pub description: Option<String>,
}

/// Создать новую коллекцию
#[tauri::command]
pub async fn create_collection(params: CreateCollectionParams) -> Result<Collection, String> {
    info!("Creating collection: {}", params.name);

    if params.name.trim().is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем уникальность имени
    let existing = db.get_collection_by_name(&params.name).await.map_err(|e| {
        error!("Failed to check collection existence: {}", e);
        format!("Database error: {}", e)
    })?;

    if existing.is_some() {
        return Err(format!("Collection name already exists: {}", params.name));
    }

    let collection = db
        .create_collection(&params.name, params.description.as_deref())
        .await
        .map_err(|e| {
            error!("Failed to create collection: {}", e);
            format!("Database error: {}", e)
        })?;

    info!(
        "Collection created successfully: {} (id: {})",
        collection.name, collection.id
    );
    Ok(collection)
}

/// Параметры для обновления коллекции
#[derive(serde::Deserialize)]
pub struct UpdateCollectionParams {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Обновить коллекцию
#[tauri::command]
pub async fn update_collection(params: UpdateCollectionParams) -> Result<Collection, String> {
    info!("Updating collection id: {}", params.id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(params.id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", params.id);
            "Collection not found".to_string()
        })?;

    // Если изменяется имя, проверяем уникальность
    if let Some(ref new_name) = params.name {
        if new_name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        let existing = db.get_collection_by_name(new_name).await.map_err(|e| {
            error!("Failed to check collection existence: {}", e);
            format!("Database error: {}", e)
        })?;

        if let Some(existing_collection) = existing {
            if existing_collection.id != params.id {
                return Err(format!("Collection name already exists: {}", new_name));
            }
        }
    }

    // Обновляем коллекцию
    db.update_collection(
        params.id,
        params.name.as_deref(),
        params.description.as_deref(),
    )
    .await
    .map_err(|e| {
        error!("Failed to update collection: {}", e);
        format!("Database error: {}", e)
    })?;

    // Получаем обновленную коллекцию
    let updated = db
        .get_collection(params.id)
        .await
        .map_err(|e| {
            error!("Failed to get updated collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found after update: {}", params.id);
            "Collection not found".to_string()
        })?;

    info!(
        "Collection updated successfully: {} (id: {})",
        updated.name, updated.id
    );
    Ok(updated)
}

/// Удалить коллекцию
#[tauri::command]
pub async fn delete_collection(collection_id: i64) -> Result<(), String> {
    info!("Deleting collection id: {}", collection_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let collection = db
        .get_collection(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", collection_id);
            "Collection not found".to_string()
        })?;

    // Удаляем коллекцию (каскадное удаление обрабатывается БД)
    db.delete_collection(collection_id).await.map_err(|e| {
        error!("Failed to delete collection: {}", e);
        format!("Database error: {}", e)
    })?;

    info!(
        "Collection deleted successfully: {} (id: {})",
        collection.name, collection_id
    );
    Ok(())
}

/// Получить файлы коллекции
#[tauri::command]
pub async fn get_collection_files(collection_id: i64) -> Result<Vec<CollectionFile>, String> {
    info!("Getting files for collection id: {}", collection_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", collection_id);
            "Collection not found".to_string()
        })?;

    let collection_files = db.get_collection_files(collection_id).await.map_err(|e| {
        error!("Failed to get collection files: {}", e);
        format!("Database error: {}", e)
    })?;

    let result: Vec<CollectionFile> = collection_files
        .iter()
        .map(|cf| CollectionFile {
            id: cf.id,
            collection_id: cf.collection_id,
            file_id: cf.file_id,
            file: cf.file.clone(),
            logic_rule_id: cf.logic_rule_id,
            logic_rule: cf.logic_rule.clone(),
            order_index: cf.order_index,
            created_at: cf.created_at.to_rfc3339(),
        })
        .collect();

    info!(
        "Found {} file(s) in collection {}",
        result.len(),
        collection_id
    );
    Ok(result)
}

/// Параметры для добавления файла в коллекцию
#[derive(serde::Deserialize)]
pub struct AddFileToCollectionParams {
    pub collection_id: i64,
    pub file_id: i64,
    pub logic_rule_id: Option<i64>,
    pub order_index: Option<i64>,
}

/// Добавить файл в коллекцию
#[tauri::command]
pub async fn add_file_to_collection(
    params: AddFileToCollectionParams,
) -> Result<CollectionFile, String> {
    info!(
        "Adding file {} to collection {}",
        params.file_id, params.collection_id
    );

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(params.collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", params.collection_id);
            "Collection not found".to_string()
        })?;

    // Проверяем существование файла
    let _file = db
        .get_file(params.file_id)
        .await
        .map_err(|e| {
            error!("Failed to get file: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("File not found: {}", params.file_id);
            "File not found".to_string()
        })?;

    // Проверяем, не находится ли файл уже в коллекции
    let already_in = db
        .file_in_collection(params.collection_id, params.file_id)
        .await
        .map_err(|e| {
            error!("Failed to check file in collection: {}", e);
            format!("Database error: {}", e)
        })?;

    if already_in {
        return Err("File already in collection".to_string());
    }

    // Проверяем правило логики, если указано
    if let Some(rule_id) = params.logic_rule_id {
        let rule = db
            .get_collection_logic_rule(rule_id)
            .await
            .map_err(|e| {
                error!("Failed to get logic rule: {}", e);
                format!("Database error: {}", e)
            })?
            .ok_or_else(|| {
                error!("Logic rule not found: {}", rule_id);
                "Logic rule not found".to_string()
            })?;

        // Проверяем, что правило принадлежит этой коллекции
        if rule.collection_id != params.collection_id {
            return Err("Logic rule does not belong to this collection".to_string());
        }
    }

    // Добавляем файл
    let collection_file_id = db
        .add_file_to_collection(
            params.collection_id,
            params.file_id,
            params.logic_rule_id,
            params.order_index,
        )
        .await
        .map_err(|e| {
            error!("Failed to add file to collection: {}", e);
            format!("Database error: {}", e)
        })?;

    // Получаем добавленный файл
    let collection_files = db
        .get_collection_files(params.collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection files: {}", e);
            format!("Database error: {}", e)
        })?;

    let collection_file = collection_files
        .iter()
        .find(|cf| cf.id == collection_file_id)
        .ok_or_else(|| {
            error!(
                "Collection file not found after creation: {}",
                collection_file_id
            );
            "Collection file not found".to_string()
        })?;

    let result = CollectionFile {
        id: collection_file.id,
        collection_id: collection_file.collection_id,
        file_id: collection_file.file_id,
        file: collection_file.file.clone(),
        logic_rule_id: collection_file.logic_rule_id,
        logic_rule: collection_file.logic_rule.clone(),
        order_index: collection_file.order_index,
        created_at: collection_file.created_at.to_rfc3339(),
    };

    info!(
        "File added to collection successfully: file_id={}, collection_id={}",
        params.file_id, params.collection_id
    );
    Ok(result)
}

/// Удалить файл из коллекции
#[tauri::command]
pub async fn remove_file_from_collection(collection_id: i64, file_id: i64) -> Result<(), String> {
    info!(
        "Removing file {} from collection {}",
        file_id, collection_id
    );

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", collection_id);
            "Collection not found".to_string()
        })?;

    // Проверяем, находится ли файл в коллекции
    let in_collection = db
        .file_in_collection(collection_id, file_id)
        .await
        .map_err(|e| {
            error!("Failed to check file in collection: {}", e);
            format!("Database error: {}", e)
        })?;

    if !in_collection {
        return Err("File not in collection".to_string());
    }

    // Удаляем файл
    db.remove_file_from_collection(collection_id, file_id)
        .await
        .map_err(|e| {
            error!("Failed to remove file from collection: {}", e);
            format!("Database error: {}", e)
        })?;

    info!(
        "File removed from collection successfully: file_id={}, collection_id={}",
        file_id, collection_id
    );
    Ok(())
}

/// Параметры для изменения порядка файлов
#[derive(serde::Deserialize)]
pub struct FileOrder {
    pub file_id: i64,
    pub order_index: i64,
}

/// Изменить порядок файлов в коллекции
#[tauri::command]
pub async fn reorder_collection_files(
    collection_id: i64,
    file_orders: Vec<FileOrder>,
) -> Result<(), String> {
    info!("Reordering files in collection {}", collection_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", collection_id);
            "Collection not found".to_string()
        })?;

    // Проверяем, что все файлы в коллекции
    for file_order in &file_orders {
        let in_collection = db
            .file_in_collection(collection_id, file_order.file_id)
            .await
            .map_err(|e| {
                error!("Failed to check file in collection: {}", e);
                format!("Database error: {}", e)
            })?;

        if !in_collection {
            return Err(format!("File not in collection: {}", file_order.file_id));
        }
    }

    // Изменяем порядок
    let orders: Vec<(i64, i64)> = file_orders
        .iter()
        .map(|fo| (fo.file_id, fo.order_index))
        .collect();

    db.reorder_collection_files(collection_id, &orders)
        .await
        .map_err(|e| {
            error!("Failed to reorder collection files: {}", e);
            format!("Database error: {}", e)
        })?;

    info!(
        "Files reordered successfully in collection {}",
        collection_id
    );
    Ok(())
}

/// Получить правила логики коллекции
#[tauri::command]
pub async fn get_collection_logic_rules(
    collection_id: i64,
) -> Result<Vec<CollectionLogicRule>, String> {
    info!("Getting logic rules for collection id: {}", collection_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", collection_id);
            "Collection not found".to_string()
        })?;

    let rules = db
        .get_collection_logic_rules(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get logic rules: {}", e);
            format!("Database error: {}", e)
        })?;

    info!(
        "Found {} logic rule(s) for collection {}",
        rules.len(),
        collection_id
    );
    Ok(rules)
}

/// Параметры для создания правила логики
#[derive(serde::Deserialize)]
pub struct CreateLogicRuleParams {
    pub collection_id: i64,
    pub name: String,
    pub condition_type: String, // "boolean" | "collection_check" | "file_check" | "and" | "or"
    pub condition_params: serde_json::Value,
    pub action: String, // "enable" | "disable"
}

/// Создать правило логики для коллекции
#[tauri::command]
pub async fn create_collection_logic_rule(
    params: CreateLogicRuleParams,
) -> Result<CollectionLogicRule, String> {
    info!(
        "Creating logic rule for collection {}: {}",
        params.collection_id, params.name
    );

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(params.collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", params.collection_id);
            "Collection not found".to_string()
        })?;

    // Парсим тип условия
    let condition_type = ConditionType::from_str(&params.condition_type).ok_or_else(|| {
        error!("Invalid condition type: {}", params.condition_type);
        format!("Invalid condition type: {}", params.condition_type)
    })?;

    // Парсим действие
    let action = Action::from_str(&params.action).ok_or_else(|| {
        error!("Invalid action: {}", params.action);
        format!("Invalid action: {}", params.action)
    })?;

    // Валидация параметров условия
    validate_condition_params(&condition_type, &params.condition_params, &db).await?;

    // Создаем правило
    let rule = db
        .create_collection_logic_rule(
            params.collection_id,
            &params.name,
            condition_type,
            &params.condition_params,
            action,
        )
        .await
        .map_err(|e| {
            error!("Failed to create logic rule: {}", e);
            format!("Database error: {}", e)
        })?;

    info!(
        "Logic rule created successfully: {} (id: {})",
        rule.name, rule.id
    );
    Ok(rule)
}

/// Валидация параметров условия
async fn validate_condition_params(
    condition_type: &ConditionType,
    params: &serde_json::Value,
    db: &Database,
) -> Result<(), String> {
    match condition_type {
        ConditionType::Boolean => {
            if !params.is_object() || !params.get("value").and_then(|v| v.as_bool()).is_some() {
                return Err(
                    "Invalid condition params: boolean requires {\"value\": true/false}"
                        .to_string(),
                );
            }
        }
        ConditionType::CollectionCheck => {
            let collection_id = params.get("collection_id")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| "Invalid condition params: collection_check requires {\"collection_id\": number}".to_string())?;

            let collection = db
                .get_collection(collection_id)
                .await
                .map_err(|e| {
                    error!("Failed to check collection: {}", e);
                    format!("Database error: {}", e)
                })?
                .ok_or_else(|| format!("Referenced collection not found: {}", collection_id))?;

            // Проверяем, что коллекция существует (уже проверили выше)
            let _ = collection;
        }
        ConditionType::FileCheck => {
            let file_name = params
                .get("file_name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    "Invalid condition params: file_check requires {\"file_name\": string}"
                        .to_string()
                })?;

            let file_version = params
                .get("file_version")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    "Invalid condition params: file_check requires {\"file_version\": string}"
                        .to_string()
                })?;

            let file = db
                .get_file_by_name_version(file_name, file_version)
                .await
                .map_err(|e| {
                    error!("Failed to check file: {}", e);
                    format!("Database error: {}", e)
                })?
                .ok_or_else(|| {
                    format!("Referenced file not found: {}@{}", file_name, file_version)
                })?;

            // Проверяем, что файл существует (уже проверили выше)
            let _ = file;
        }
        ConditionType::And | ConditionType::Or => {
            let rules = params.get("rules")
                .and_then(|v| v.as_array())
                .ok_or_else(|| "Invalid condition params: and/or requires {\"rules\": [rule_id1, rule_id2, ...]}".to_string())?;

            for rule_id_val in rules {
                let rule_id = rule_id_val
                    .as_i64()
                    .ok_or_else(|| "Invalid rule ID in rules array".to_string())?;

                let rule = db
                    .get_collection_logic_rule(rule_id)
                    .await
                    .map_err(|e| {
                        error!("Failed to check rule: {}", e);
                        format!("Database error: {}", e)
                    })?
                    .ok_or_else(|| format!("Referenced rule not found: {}", rule_id))?;

                // Проверяем, что правило существует (уже проверили выше)
                let _ = rule;
            }
        }
    }
    Ok(())
}

/// Параметры для обновления правила логики
#[derive(serde::Deserialize)]
pub struct UpdateLogicRuleParams {
    pub id: i64,
    pub name: Option<String>,
    pub condition_type: Option<String>,
    pub condition_params: Option<serde_json::Value>,
    pub action: Option<String>,
}

/// Обновить правило логики
#[tauri::command]
pub async fn update_collection_logic_rule(
    params: UpdateLogicRuleParams,
) -> Result<CollectionLogicRule, String> {
    info!("Updating logic rule id: {}", params.id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Получаем текущее правило
    let current_rule = db
        .get_collection_logic_rule(params.id)
        .await
        .map_err(|e| {
            error!("Failed to get logic rule: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Rule not found: {}", params.id);
            "Rule not found".to_string()
        })?;

    // Определяем новые значения
    let condition_type = if let Some(ref ct_str) = params.condition_type {
        Some(ConditionType::from_str(ct_str).ok_or_else(|| {
            error!("Invalid condition type: {}", ct_str);
            format!("Invalid condition type: {}", ct_str)
        })?)
    } else {
        None
    };

    let action = if let Some(ref a_str) = params.action {
        Some(Action::from_str(a_str).ok_or_else(|| {
            error!("Invalid action: {}", a_str);
            format!("Invalid action: {}", a_str)
        })?)
    } else {
        None
    };

    // Валидация параметров условия, если они изменяются
    if let Some(ref condition_params) = params.condition_params {
        let ct = condition_type
            .as_ref()
            .unwrap_or(&current_rule.condition_type);
        validate_condition_params(ct, condition_params, &db).await?;
    } else if condition_type.is_some() {
        // Если тип меняется, но параметры не указаны, используем текущие
        let ct = condition_type.as_ref().unwrap();
        validate_condition_params(ct, &current_rule.condition_params, &db).await?;
    }

    // Обновляем правило
    db.update_collection_logic_rule(
        params.id,
        params.name.as_deref(),
        condition_type,
        params.condition_params.as_ref(),
        action,
    )
    .await
    .map_err(|e| {
        error!("Failed to update logic rule: {}", e);
        format!("Database error: {}", e)
    })?;

    // Получаем обновленное правило
    let updated = db
        .get_collection_logic_rule(params.id)
        .await
        .map_err(|e| {
            error!("Failed to get updated rule: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Rule not found after update: {}", params.id);
            "Rule not found".to_string()
        })?;

    info!(
        "Logic rule updated successfully: {} (id: {})",
        updated.name, updated.id
    );
    Ok(updated)
}

/// Удалить правило логики
#[tauri::command]
pub async fn delete_collection_logic_rule(rule_id: i64) -> Result<(), String> {
    info!("Deleting logic rule id: {}", rule_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование правила
    let _rule = db
        .get_collection_logic_rule(rule_id)
        .await
        .map_err(|e| {
            error!("Failed to get logic rule: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Rule not found: {}", rule_id);
            "Rule not found".to_string()
        })?;

    // Удаляем правило
    db.delete_collection_logic_rule(rule_id)
        .await
        .map_err(|e| {
            error!("Failed to delete logic rule: {}", e);
            format!("Database error: {}", e)
        })?;

    info!("Logic rule deleted successfully: id={}", rule_id);
    Ok(())
}

use crate::services::collection_service::CollectionLogicEvaluator;

/// Оценить логику коллекции
#[tauri::command]
pub async fn evaluate_collection_logic(
    collection_id: i64,
) -> Result<CollectionEvaluationResult, String> {
    info!(
        "Evaluating collection logic for collection id: {}",
        collection_id
    );

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", collection_id);
            "Collection not found".to_string()
        })?;

    // Оцениваем логику
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator
        .evaluate_collection(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to evaluate collection logic: {}", e);
            e
        })?;

    // Преобразуем в формат для API
    let api_result = CollectionEvaluationResult {
        collection_id: result.collection_id,
        enabled_files: result.enabled_files,
        disabled_files: result.disabled_files,
        evaluation_details: result
            .evaluation_details
            .iter()
            .map(|d| FileEvaluationDetail {
                file_id: d.file_id,
                enabled: d.enabled,
                applied_rules: d.applied_rules.clone(),
            })
            .collect(),
    };

    info!(
        "Collection evaluation completed: {} enabled, {} disabled",
        api_result.enabled_files.len(),
        api_result.disabled_files.len()
    );
    Ok(api_result)
}

/// Параметры для объединения коллекций
#[derive(serde::Deserialize)]
pub struct CombineCollectionsParams {
    pub name: String,
    pub description: Option<String>,
    pub source_collection_ids: Vec<i64>,
    pub selected_file_ids: Option<Vec<i64>>,
}

/// Объединить несколько коллекций в новую
#[tauri::command]
pub async fn combine_collections(params: CombineCollectionsParams) -> Result<Collection, String> {
    info!("Combining collections into: {}", params.name);

    if params.name.trim().is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    if params.source_collection_ids.is_empty() {
        return Err("At least one source collection is required".to_string());
    }

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем уникальность имени
    let existing = db.get_collection_by_name(&params.name).await.map_err(|e| {
        error!("Failed to check collection existence: {}", e);
        format!("Database error: {}", e)
    })?;

    if existing.is_some() {
        return Err(format!("Collection name already exists: {}", params.name));
    }

    // Проверяем существование всех исходных коллекций
    for collection_id in &params.source_collection_ids {
        let collection = db
            .get_collection(*collection_id)
            .await
            .map_err(|e| {
                error!("Failed to get collection: {}", e);
                format!("Database error: {}", e)
            })?
            .ok_or_else(|| {
                error!("Source collection not found: {}", collection_id);
                format!("Source collection not found: {}", collection_id)
            })?;

        let _ = collection;
    }

    // Создаем новую коллекцию
    let new_collection = db
        .create_collection(&params.name, params.description.as_deref())
        .await
        .map_err(|e| {
            error!("Failed to create collection: {}", e);
            format!("Database error: {}", e)
        })?;

    // Собираем все файлы из исходных коллекций
    let mut all_file_ids = std::collections::HashSet::new();
    let mut file_orders = Vec::new();

    for collection_id in &params.source_collection_ids {
        let collection_files = db.get_collection_files(*collection_id).await.map_err(|e| {
            error!("Failed to get collection files: {}", e);
            format!("Database error: {}", e)
        })?;

        for cf in collection_files {
            if !all_file_ids.contains(&cf.file_id) {
                all_file_ids.insert(cf.file_id);
                file_orders.push((cf.file_id, cf.order_index));
            }
        }
    }

    // Фильтруем по selected_file_ids, если указаны
    let file_ids_to_add: Vec<i64> = if let Some(ref selected) = params.selected_file_ids {
        if selected.is_empty() {
            return Err("No files selected".to_string());
        }

        // Проверяем, что все выбранные файлы существуют
        for file_id in selected {
            let file = db
                .get_file(*file_id)
                .await
                .map_err(|e| {
                    error!("Failed to get file: {}", e);
                    format!("Database error: {}", e)
                })?
                .ok_or_else(|| {
                    error!("File not found: {}", file_id);
                    format!("File not found: {}", file_id)
                })?;

            let _ = file;
        }

        selected.clone()
    } else {
        all_file_ids.into_iter().collect()
    };

    // Добавляем файлы в новую коллекцию
    for (index, file_id) in file_ids_to_add.iter().enumerate() {
        db.add_file_to_collection(
            new_collection.id,
            *file_id,
            None, // Логика не копируется
            Some(index as i64),
        )
        .await
        .map_err(|e| {
            error!("Failed to add file to collection: {}", e);
            format!("Database error: {}", e)
        })?;
    }

    info!(
        "Collections combined successfully: {} files in new collection {}",
        file_ids_to_add.len(),
        new_collection.name
    );
    Ok(new_collection)
}

/// Получить файлы из нескольких коллекций
#[tauri::command]
pub async fn get_files_from_multiple_collections(
    collection_ids: Vec<i64>,
) -> Result<Vec<CollectionFileView>, String> {
    info!("Getting files from {} collection(s)", collection_ids.len());

    if collection_ids.is_empty() {
        return Ok(Vec::new());
    }

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование всех коллекций
    for collection_id in &collection_ids {
        let collection = db
            .get_collection(*collection_id)
            .await
            .map_err(|e| {
                error!("Failed to get collection: {}", e);
                format!("Database error: {}", e)
            })?
            .ok_or_else(|| {
                error!("Collection not found: {}", collection_id);
                format!("Collection not found: {}", collection_id)
            })?;

        let _ = collection;
    }

    // Собираем файлы из всех коллекций
    let mut files_map: std::collections::HashMap<i64, CollectionFileView> =
        std::collections::HashMap::new();

    for collection_id in &collection_ids {
        let collection = db
            .get_collection(*collection_id)
            .await
            .map_err(|e| {
                error!("Failed to get collection: {}", e);
                format!("Database error: {}", e)
            })?
            .ok_or_else(|| {
                error!("Collection not found: {}", collection_id);
                "Collection not found".to_string()
            })?;

        let collection_files = db.get_collection_files(*collection_id).await.map_err(|e| {
            error!("Failed to get collection files: {}", e);
            format!("Database error: {}", e)
        })?;

        for cf in collection_files {
            let file_id = cf.file_id;

            if let Some(file_view) = files_map.get_mut(&file_id) {
                // Файл уже есть, добавляем информацию о коллекции
                file_view.collections.push(CollectionFileInfo {
                    collection_id: *collection_id,
                    collection_name: collection.name.clone(),
                    order_index: cf.order_index,
                    logic_rule_id: cf.logic_rule_id,
                });
            } else {
                // Новый файл
                files_map.insert(
                    file_id,
                    CollectionFileView {
                        file_id,
                        file: cf.file.clone(),
                        collections: vec![CollectionFileInfo {
                            collection_id: *collection_id,
                            collection_name: collection.name.clone(),
                            order_index: cf.order_index,
                            logic_rule_id: cf.logic_rule_id,
                        }],
                    },
                );
            }
        }
    }

    let result: Vec<CollectionFileView> = files_map.into_values().collect();
    info!(
        "Found {} unique file(s) across {} collection(s)",
        result.len(),
        collection_ids.len()
    );
    Ok(result)
}
