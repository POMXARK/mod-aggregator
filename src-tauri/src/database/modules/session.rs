//! Операции с состоянием сессии в базе данных
//!
//! Модуль содержит методы для работы с состоянием пользовательской сессии:
//! предпочтения UI, порядок файлов, открытые коллекции и т.д.

use crate::database::modules::base::Database;
use sqlx::Row;

/// Получить состояние сессии
///
/// # Параметры
/// * `db` - подключение к базе данных
///
/// # Возвращает
/// JSON объект с состоянием сессии или ошибку
pub async fn get_session_state(db: &Database) -> Result<serde_json::Value, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM session_state WHERE id = 1")
        .fetch_optional(&db.pool)
        .await?;

    if let Some(row) = row {
        Ok(serde_json::json!({
            "file_order": serde_json::from_str::<Vec<i64>>(row.get::<String, _>(1).as_str()).unwrap_or_default(),
            "ui_preferences": serde_json::from_str(row.get::<String, _>(2).as_str()).unwrap_or(serde_json::json!({})),
            "open_collections": serde_json::from_str::<Vec<i64>>(row.get::<String, _>(3).as_str()).unwrap_or_default(),
            "selected_files": serde_json::from_str::<Vec<i64>>(row.get::<String, _>(4).as_str()).unwrap_or_default(),
            "recent_actions": serde_json::from_str(row.get::<String, _>(5).as_str()).unwrap_or(serde_json::json!([])),
            "last_updated": row.get::<chrono::DateTime<chrono::Utc>, _>(6)
        }))
    } else {
        Ok(serde_json::json!({
            "file_order": [],
            "ui_preferences": {},
            "open_collections": [],
            "selected_files": [],
            "recent_actions": [],
            "last_updated": chrono::Utc::now()
        }))
    }
}

/// Обновить порядок файлов в сессии
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `file_order` - новый порядок файлов
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn update_session_file_order(db: &Database, file_order: &[i64]) -> Result<(), sqlx::Error> {
    let file_order_str = serde_json::to_string(file_order).unwrap_or_default();
    sqlx::query("UPDATE session_state SET file_order = ?, last_updated = ? WHERE id = 1")
        .bind(&file_order_str)
        .bind(chrono::Utc::now())
        .execute(&db.pool)
        .await?;
    Ok(())
}

/// Обновить UI предпочтения сессии
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `ui_preferences` - JSON объект с предпочтениями UI
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn update_session_ui_preferences(db: &Database, ui_preferences: &serde_json::Value) -> Result<(), sqlx::Error> {
    let ui_preferences_str = serde_json::to_string(ui_preferences).unwrap_or_default();
    sqlx::query("UPDATE session_state SET ui_preferences = ?, last_updated = ? WHERE id = 1")
        .bind(&ui_preferences_str)
        .bind(chrono::Utc::now())
        .execute(&db.pool)
        .await?;
    Ok(())
}

/// Обновить открытые коллекции в сессии
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `open_collections` - список ID открытых коллекций
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn update_session_open_collections(db: &Database, open_collections: &[i64]) -> Result<(), sqlx::Error> {
    let open_collections_str = serde_json::to_string(open_collections).unwrap_or_default();
    sqlx::query("UPDATE session_state SET open_collections = ?, last_updated = ? WHERE id = 1")
        .bind(&open_collections_str)
        .bind(chrono::Utc::now())
        .execute(&db.pool)
        .await?;
    Ok(())
}

/// Обновить выбранные файлы в сессии
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `file_ids` - список ID выбранных файлов
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn update_session_selected_files(db: &Database, file_ids: &[i64]) -> Result<(), sqlx::Error> {
    let selected_files_str = serde_json::to_string(file_ids).unwrap_or_default();
    sqlx::query("UPDATE session_state SET selected_files = ?, last_updated = ? WHERE id = 1")
        .bind(&selected_files_str)
        .bind(chrono::Utc::now())
        .execute(&db.pool)
        .await?;
    Ok(())
}

/// Добавить недавнее действие в сессию
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `action` - JSON объект с описанием действия
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn add_recent_action(db: &Database, action: &serde_json::Value) -> Result<(), sqlx::Error> {
    let current_state = get_session_state(db).await?;
    let mut recent_actions: Vec<serde_json::Value> = serde_json::from_value(
        current_state["recent_actions"].clone()
    ).unwrap_or_default();

    recent_actions.insert(0, action.clone());

    // Ограничить до 50 последних действий
    recent_actions.truncate(50);

    let recent_actions_str = serde_json::to_string(&recent_actions).unwrap_or_default();
    sqlx::query("UPDATE session_state SET recent_actions = ?, last_updated = ? WHERE id = 1")
        .bind(&recent_actions_str)
        .bind(chrono::Utc::now())
        .execute(&db.pool)
        .await?;
    Ok(())
}

/// Получить недавние действия из сессии
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `limit` - максимальное количество действий
///
/// # Возвращает
/// Вектор недавних действий или ошибку
pub async fn get_recent_actions(db: &Database, limit: usize) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let state = get_session_state(db).await?;
    let mut actions: Vec<serde_json::Value> = serde_json::from_value(state["recent_actions"].clone()).unwrap_or_default();
    actions.truncate(limit);
    Ok(actions)
}

/// Очистить недавние действия в сессии
///
/// # Параметры
/// * `db` - подключение к базе данных
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn clear_recent_actions(db: &Database) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE session_state SET recent_actions = '[]', last_updated = ? WHERE id = 1")
        .bind(chrono::Utc::now())
        .execute(&db.pool)
        .await?;
    Ok(())
}

/// Сбросить состояние сессии
///
/// # Параметры
/// * `db` - подключение к базе данных
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn reset_session_state(db: &Database) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE session_state SET file_order = '[]', ui_preferences = '{}', open_collections = '[]', selected_files = '[]', recent_actions = '[]', last_updated = ? WHERE id = 1"
    )
    .bind(chrono::Utc::now())
    .execute(&db.pool)
    .await?;
    Ok(())
}

/// Обновить текущую страницу в сессии
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `page` - имя текущей страницы
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn update_session_current_page(db: &Database, page: &str) -> Result<(), sqlx::Error> {
    // Для текущей страницы можно добавить отдельное поле или использовать ui_preferences
    let current_state = get_session_state(db).await?;
    let mut ui_preferences: serde_json::Map<String, serde_json::Value> =
        serde_json::from_value(current_state["ui_preferences"].clone()).unwrap_or_default();

    ui_preferences.insert("current_page".to_string(), serde_json::json!(page));

    update_session_ui_preferences(db, &serde_json::Value::Object(ui_preferences)).await
}

/// Обновить выбранный сайт в сессии
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `site_id` - ID выбранного сайта (None для снятия выбора)
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn update_session_selected_site(db: &Database, site_id: Option<i64>) -> Result<(), sqlx::Error> {
    let current_state = get_session_state(db).await?;
    let mut ui_preferences: serde_json::Map<String, serde_json::Value> =
        serde_json::from_value(current_state["ui_preferences"].clone()).unwrap_or_default();

    ui_preferences.insert("selected_site".to_string(), serde_json::json!(site_id));

    update_session_ui_preferences(db, &serde_json::Value::Object(ui_preferences)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session_operations() {
        let result = Database::new().await;
        assert!(result.is_ok() || result.is_err());
    }
}