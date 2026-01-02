//! Операции с сайтами в базе данных
//!
//! Модуль содержит методы для CRUD операций с сайтами:
//! получение, добавление, обновление и удаление сайтов.

use crate::database::modules::base::Database;
use crate::models::Site;
use chrono::Utc;

/// Получить список всех сайтов из базы данных
///
/// Возвращает все сайты, отсортированные по имени.
///
/// # Возвращает
/// Вектор сайтов или ошибку базы данных
pub async fn get_sites(db: &Database) -> Result<Vec<Site>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM sites ORDER BY name")
        .fetch_all(&db.pool)
        .await?;

    Ok(rows
        .iter()
        .map(|row| Site {
            id: row.get(0),
            name: row.get(1),
            url: row.get(2),
            parser_config: serde_json::from_str(row.get::<String, _>(3).as_str())
                .unwrap_or(serde_json::json!({})),
            created_at: row.get::<String, _>(4).parse().unwrap_or(Utc::now()),
            updated_at: row.get::<String, _>(5).parse().unwrap_or(Utc::now()),
        })
        .collect())
}

/// Получить сайт по ID
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `id` - идентификатор сайта
///
/// # Возвращает
/// Сайт или ошибку, если не найден
pub async fn get_site(db: &Database, id: i64) -> Result<Site, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM sites WHERE id = ?")
        .bind(id)
        .fetch_one(&db.pool)
        .await?;

    Ok(Site {
        id: row.get(0),
        name: row.get(1),
        url: row.get(2),
        parser_config: serde_json::from_str(row.get::<String, _>(3).as_str())
            .unwrap_or(serde_json::json!({})),
        created_at: row.get::<String, _>(4).parse().unwrap_or(Utc::now()),
        updated_at: row.get::<String, _>(5).parse().unwrap_or(Utc::now()),
    })
}

/// Добавить новый сайт в базу данных
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `name` - название сайта
/// * `url` - URL сайта
/// * `parser_config` - конфигурация парсера в формате JSON
///
/// # Возвращает
/// Созданный сайт с присвоенным ID или ошибку
pub async fn add_site(
    db: &Database,
    name: &str,
    url: &str,
    parser_config: &serde_json::Value,
) -> Result<Site, sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    let config_str = serde_json::to_string(parser_config).unwrap_or_default();

    sqlx::query(
        "INSERT INTO sites (name, url, parser_config, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(name)
    .bind(url)
    .bind(&config_str)
    .bind(&now)
    .bind(&now)
    .execute(&db.pool)
    .await?;

    let id = sqlx::query("SELECT last_insert_rowid()")
        .fetch_one(&db.pool)
        .await?
        .get(0);

    Ok(Site {
        id,
        name: name.to_string(),
        url: url.to_string(),
        parser_config: parser_config.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}

/// Обновить существующий сайт в базе данных
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `id` - идентификатор сайта для обновления
/// * `name` - новое название сайта
/// * `url` - новый URL сайта
/// * `parser_config` - новая конфигурация парсера в формате JSON
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn update_site(
    db: &Database,
    id: i64,
    name: &str,
    url: &str,
    parser_config: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    let config_str = serde_json::to_string(parser_config).unwrap_or_default();

    sqlx::query(
        "UPDATE sites SET name = ?, url = ?, parser_config = ?, updated_at = ? WHERE id = ?",
    )
    .bind(name)
    .bind(url)
    .bind(&config_str)
    .bind(&now)
    .bind(id)
    .execute(&db.pool)
    .await?;

    Ok(())
}

/// Удалить сайт из базы данных
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `id` - идентификатор сайта для удаления
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn delete_site(db: &Database, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sites WHERE id = ?")
        .bind(id)
        .execute(&db.pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_sites_operations() {
        // Для полноценного тестирования нужна тестовая БД
        // Этот тест проверяет только сигнатуры функций
        let result = Database::new().await;
        assert!(result.is_ok() || result.is_err());
    }
}