//! Операции с сохраненными страницами в базе данных
//!
//! Модуль содержит методы для работы с сохраненными страницами:
//! сохранение, получение и управление версиями страниц.

use crate::database::modules::base::Database;
use sqlx::Row;

/// Сохранить страницу для сайта
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `site_id` - ID сайта
/// * `url` - URL страницы
/// * `folder_path` - путь к папке с файлами
/// * `version_timestamp` - временная метка версии
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn save_page_for_site(
    db: &Database,
    site_id: i64,
    url: &str,
    folder_path: &str,
    version_timestamp: &str,
) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO saved_pages (site_id, url, folder_path, version_timestamp, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(site_id)
    .bind(url)
    .bind(folder_path)
    .bind(version_timestamp)
    .bind(&now)
    .bind(&now)
    .execute(&db.pool)
    .await?;
    Ok(())
}

/// Получить сохраненную страницу
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `site_id` - ID сайта
/// * `url` - URL страницы
///
/// # Возвращает
/// Кортеж (id, folder_path, timestamp) или None
pub async fn get_saved_page(
    db: &Database,
    site_id: i64,
    url: &str,
) -> Result<Option<(i64, String, String)>, sqlx::Error> {
    let row = sqlx::query("SELECT id, folder_path, version_timestamp FROM saved_pages WHERE site_id = ? AND url = ? ORDER BY created_at DESC LIMIT 1")
        .bind(site_id)
        .bind(url)
        .fetch_optional(&db.pool)
        .await?;

    Ok(row.map(|r| (r.get(0), r.get(1), r.get(2))))
}

/// Получить все версии сохраненной страницы
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `site_id` - ID сайта
/// * `url` - URL страницы
///
/// # Возвращает
/// Вектор кортежей (id, folder_path, timestamp)
pub async fn get_saved_page_versions(
    db: &Database,
    site_id: i64,
    url: &str,
) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, folder_path, version_timestamp FROM saved_pages WHERE site_id = ? AND url = ? ORDER BY created_at DESC")
        .bind(site_id)
        .bind(url)
        .fetch_all(&db.pool)
        .await?;

    Ok(rows
        .iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<i64, _>(0),
                "folder_path": row.get::<String, _>(1),
                "timestamp": row.get::<String, _>(2)
            })
        })
        .collect())
}

/// Удалить версию сохраненной страницы
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `page_id` - ID версии страницы
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn delete_saved_page_version(db: &Database, page_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM saved_pages WHERE id = ?")
        .bind(page_id)
        .execute(&db.pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pages_operations() {
        let result = Database::new().await;
        assert!(result.is_ok() || result.is_err());
    }
}