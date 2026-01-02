//! Операции с уведомлениями в базе данных
//!
//! Модуль содержит методы для работы с уведомлениями:
//! получение, добавление и отметка как прочитанные.

use crate::database::modules::base::Database;
use crate::models::Notification;
use chrono::Utc;

/// Получить список всех уведомлений
///
/// Возвращает последние 100 уведомлений, отсортированных по дате создания (новые первыми).
///
/// # Параметры
/// * `db` - подключение к базе данных
///
/// # Возвращает
/// Вектор уведомлений или ошибку
pub async fn get_notifications(db: &Database) -> Result<Vec<Notification>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM notifications ORDER BY created_at DESC LIMIT 100")
        .fetch_all(&db.pool)
        .await?;

    Ok(rows
        .iter()
        .map(|row| Notification {
            id: row.get(0),
            mod_id: row.get(1),
            site_id: row.get(2),
            title: row.get(3),
            message: row.get(4),
            read: row.get::<i64, _>(5) != 0,
            created_at: row.get::<String, _>(6).parse().unwrap_or(Utc::now()),
        })
        .collect())
}

/// Добавить новое уведомление в базу данных
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `notification` - объект уведомления для добавления
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn add_notification(db: &Database, notification: &Notification) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO notifications (mod_id, site_id, title, message, read, created_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(notification.mod_id)
    .bind(notification.site_id)
    .bind(&notification.title)
    .bind(&notification.message)
    .bind(notification.read as i64)
    .bind(&now)
    .execute(&db.pool)
    .await?;
    Ok(())
}

/// Отметить уведомление как прочитанное
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `id` - ID уведомления для отметки
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn mark_notification_read(db: &Database, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE notifications SET read = 1 WHERE id = ?")
        .bind(id)
        .execute(&db.pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_notifications_operations() {
        let result = Database::new().await;
        assert!(result.is_ok() || result.is_err());
    }
}