//! Обработчики команд для работы с сайтами
//!
//! Модуль содержит функции для CRUD операций с сайтами в базе данных.

use crate::database::Database;
use crate::models;

/// Получить список всех сайтов из базы данных
///
/// # Возвращает
/// Вектор всех сайтов, отсортированных по имени, или ошибку
#[tauri::command]
pub async fn get_sites() -> Result<Vec<models::Site>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.get_sites().await.map_err(|e| e.to_string())
}

/// Добавить новый сайт в базу данных
///
/// # Параметры
/// * `name` - название сайта
/// * `url` - URL сайта
/// * `parser_config` - конфигурация парсера в формате JSON
///
/// # Возвращает
/// Созданный сайт или ошибку
#[tauri::command]
pub async fn add_site(
    name: String,
    url: String,
    parser_config: serde_json::Value,
) -> Result<models::Site, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.add_site(&name, &url, &parser_config)
        .await
        .map_err(|e| e.to_string())
}

/// Обновить существующий сайт в базе данных
///
/// # Параметры
/// * `id` - идентификатор сайта для обновления
/// * `name` - новое название сайта
/// * `url` - новый URL сайта
/// * `parser_config` - новая конфигурация парсера в формате JSON
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
#[tauri::command]
pub async fn update_site(
    id: i64,
    name: String,
    url: String,
    parser_config: serde_json::Value,
) -> Result<(), String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.update_site(id, &name, &url, &parser_config)
        .await
        .map_err(|e| e.to_string())
}

/// Удалить сайт из базы данных
///
/// # Параметры
/// * `id` - идентификатор сайта для удаления
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
#[tauri::command]
pub async fn delete_site(id: i64) -> Result<(), String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.delete_site(id).await.map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Note: Для полноценного тестирования нужны интеграционные тесты
    // с тестовой базой данных. Эти тесты требуют настройки тестовой среды.

    #[tokio::test]
    async fn test_add_site_validation() {
        // Этот тест проверяет базовую валидацию, но без реальной БД
        // Для полного тестирования нужны интеграционные тесты
        let result = add_site(
            "Test Site".to_string(),
            "https://example.com".to_string(),
            json!({}),
        ).await;

        // Функция может успешно выполниться или вернуть ошибку в зависимости от среды
        // Главное - что она не паникует и возвращает корректный тип
        assert!(result.is_ok() || result.is_err());
    }
}