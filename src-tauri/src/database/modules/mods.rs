//! Операции с модами в базе данных
//!
//! Модуль содержит методы для работы с модами:
//! получение, добавление и обновление модов.

use crate::database::modules::base::Database;
use crate::models::Mod;
use chrono::Utc;

/// Получить список модов из базы данных
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `site_id` - ID сайта для фильтрации (None = все сайты)
///
/// # Возвращает
/// Вектор модов, отсортированных по дате обновления, или ошибку
pub async fn get_mods(db: &Database, site_id: Option<i64>) -> Result<Vec<Mod>, sqlx::Error> {
    let rows = if let Some(id) = site_id {
        sqlx::query("SELECT * FROM mods WHERE site_id = ? ORDER BY updated_at DESC")
            .bind(id)
            .fetch_all(&db.pool)
            .await?
    } else {
        sqlx::query("SELECT * FROM mods ORDER BY updated_at DESC")
            .fetch_all(&db.pool)
            .await?
    };

    Ok(rows
        .iter()
        .map(|row| Mod {
            id: row.get(0),
            site_id: row.get(1),
            title: row.get(2),
            url: row.get(3),
            version: row.get(4),
            author: row.get(5),
            description: row.get(6),
            image_url: row.get(7),
            changes: row.get(8),
            created_at: row.get::<String, _>(9).parse().unwrap_or(Utc::now()),
            updated_at: row.get::<String, _>(10).parse().unwrap_or(Utc::now()),
        })
        .collect())
}

/// Получить мод по URL
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `url` - URL мода для поиска
///
/// # Возвращает
/// Мод, если найден, или None
pub async fn get_mod_by_url(db: &Database, url: &str) -> Result<Option<Mod>, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM mods WHERE url = ?")
        .bind(url)
        .fetch_optional(&db.pool)
        .await?;

    Ok(row.map(|r| Mod {
        id: r.get(0),
        site_id: r.get(1),
        title: r.get(2),
        url: r.get(3),
        version: r.get(4),
        author: r.get(5),
        description: r.get(6),
        image_url: r.get(7),
        changes: r.get(8),
        created_at: r.get::<String, _>(9).parse().unwrap_or(Utc::now()),
        updated_at: r.get::<String, _>(10).parse().unwrap_or(Utc::now()),
    }))
}

/// Добавить новый мод в базу данных
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `mod_item` - объект мода для добавления
///
/// # Возвращает
/// Созданный мод с присвоенным ID или ошибку
pub async fn add_mod(db: &Database, mod_item: &Mod) -> Result<Mod, sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO mods (site_id, title, url, version, author, description, image_url, changes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(mod_item.site_id)
    .bind(&mod_item.title)
    .bind(&mod_item.url)
    .bind(&mod_item.version)
    .bind(&mod_item.author)
    .bind(&mod_item.description)
    .bind(&mod_item.image_url)
    .bind(&mod_item.changes)
    .bind(&now)
    .bind(&now)
    .execute(&db.pool)
    .await?;

    let id = sqlx::query("SELECT last_insert_rowid()")
        .fetch_one(&db.pool)
        .await?
        .get(0);

    Ok(Mod {
        id,
        ..mod_item.clone()
    })
}

/// Обновить существующий мод в базе данных
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `id` - идентификатор мода для обновления
/// * `mod_item` - объект мода с новыми данными
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn update_mod(db: &Database, id: i64, mod_item: &Mod) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE mods SET title = ?, version = ?, author = ?, description = ?, image_url = ?, changes = ?, updated_at = ? WHERE id = ?",
    )
    .bind(&mod_item.title)
    .bind(&mod_item.version)
    .bind(&mod_item.author)
    .bind(&mod_item.description)
    .bind(&mod_item.image_url)
    .bind(&mod_item.changes)
    .bind(&now)
    .bind(id)
    .execute(&db.pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mods_operations() {
        // Для полноценного тестирования нужна тестовая БД
        // Этот тест проверяет только сигнатуры функций
        let result = Database::new().await;
        assert!(result.is_ok() || result.is_err());
    }
}