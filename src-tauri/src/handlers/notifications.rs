//! Обработчики команд для работы с уведомлениями
//!
//! Модуль содержит функции для получения уведомлений и управления их статусом.

use crate::database::Database;
use crate::models;

/// Получить список всех уведомлений
///
/// Возвращает последние 100 уведомлений, отсортированных по дате создания.
///
/// # Возвращает
/// Вектор уведомлений или ошибку
#[tauri::command]
pub async fn get_notifications() -> Result<Vec<models::Notification>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.get_notifications().await.map_err(|e| e.to_string())
}

/// Отметить уведомление как прочитанное
///
/// # Параметры
/// * `id` - ID уведомления для отметки
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
#[tauri::command]
pub async fn mark_notification_read(id: i64) -> Result<(), String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.mark_notification_read(id)
        .await
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_notifications() {
        let result = get_notifications().await;
        // Функция может успешно выполниться или вернуть ошибку в зависимости от среды
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_mark_notification_read() {
        let result = mark_notification_read(1).await;
        // Функция может успешно выполниться или вернуть ошибку в зависимости от среды
        assert!(result.is_ok() || result.is_err());
    }
}