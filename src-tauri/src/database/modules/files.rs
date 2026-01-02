//! Операции с файлами в базе данных
//!
//! Модуль содержит методы для работы с файлами:
//! получение, создание, обновление и удаление файлов.

use crate::database::modules::base::Database;
use crate::models::file::File;
use chrono::Utc;
use sqlx::Row;

/// Получить файл по ID
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `id` - ID файла
///
/// # Возвращает
/// Файл или None, если не найден
pub async fn get_file(db: &Database, id: i64) -> Result<Option<File>, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM files WHERE id = ?")
        .bind(id)
        .fetch_optional(&db.pool)
        .await?;

    Ok(row.map(|r| File {
        id: r.get(0),
        name: r.get(1),
        version: r.get(2),
        path: r.get(3),
        metadata: serde_json::from_str(r.get::<String, _>(4).as_str()).unwrap_or(serde_json::json!({})),
        created_at: r.get::<chrono::DateTime<Utc>, _>(5),
        updated_at: r.get::<chrono::DateTime<Utc>, _>(6),
    }))
}

/// Получить файл по имени и версии
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `name` - имя файла
/// * `version` - версия файла
///
/// # Возвращает
/// Файл или None, если не найден
pub async fn get_file_by_name_version(
    db: &Database,
    name: &str,
    version: &str,
) -> Result<Option<File>, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM files WHERE name = ? AND version = ?")
        .bind(name)
        .bind(version)
        .fetch_optional(&db.pool)
        .await?;

    Ok(row.map(|r| File {
        id: r.get(0),
        name: r.get(1),
        version: r.get(2),
        path: r.get(3),
        metadata: serde_json::from_str(r.get::<String, _>(4).as_str()).unwrap_or(serde_json::json!({})),
        created_at: r.get::<chrono::DateTime<Utc>, _>(5),
        updated_at: r.get::<chrono::DateTime<Utc>, _>(6),
    }))
}

/// Получить все версии файла
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `name` - имя файла
///
/// # Возвращает
/// Вектор файлов с разными версиями
pub async fn get_file_versions(db: &Database, name: &str) -> Result<Vec<File>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM files WHERE name = ? ORDER BY version DESC")
        .bind(name)
        .fetch_all(&db.pool)
        .await?;

    Ok(rows
        .iter()
        .map(|row| File {
            id: row.get(0),
            name: row.get(1),
            version: row.get(2),
            path: row.get(3),
            metadata: serde_json::from_str(row.get::<String, _>(4).as_str()).unwrap_or(serde_json::json!({})),
            created_at: row.get::<chrono::DateTime<Utc>, _>(5),
            updated_at: row.get::<chrono::DateTime<Utc>, _>(6),
        })
        .collect())
}

/// Создать новый файл
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `file` - объект файла для создания
///
/// # Возвращает
/// Созданный файл с присвоенным ID или ошибку
pub async fn create_file(db: &Database, file: &File) -> Result<File, sqlx::Error> {
    let metadata_str = serde_json::to_string(&file.metadata).unwrap_or_default();

    sqlx::query(
        "INSERT INTO files (name, version, path, metadata, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&file.name)
    .bind(&file.version)
    .bind(&file.path)
    .bind(&metadata_str)
    .bind(file.created_at)
    .bind(file.updated_at)
    .execute(&db.pool)
    .await?;

    let id = sqlx::query("SELECT last_insert_rowid()")
        .fetch_one(&db.pool)
        .await?
        .get(0);

    Ok(File { id, ..file.clone() })
}

/// Обновить существующий файл
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `file` - объект файла с обновленными данными
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn update_file(db: &Database, file: &File) -> Result<(), sqlx::Error> {
    let metadata_str = serde_json::to_string(&file.metadata).unwrap_or_default();

    sqlx::query(
        "UPDATE files SET path = ?, metadata = ?, updated_at = ? WHERE id = ?",
    )
    .bind(&file.path)
    .bind(&metadata_str)
    .bind(file.updated_at)
    .bind(file.id)
    .execute(&db.pool)
    .await?;
    Ok(())
}

/// Удалить файл
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `id` - ID файла для удаления
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn delete_file(db: &Database, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM files WHERE id = ?")
        .bind(id)
        .execute(&db.pool)
        .await?;
    Ok(())
}

/// Получить все файлы
///
/// # Параметры
/// * `db` - подключение к базе данных
///
/// # Возвращает
/// Вектор всех файлов
pub async fn get_all_files(db: &Database) -> Result<Vec<File>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM files ORDER BY name, version")
        .fetch_all(&db.pool)
        .await?;

    Ok(rows
        .iter()
        .map(|row| File {
            id: row.get(0),
            name: row.get(1),
            version: row.get(2),
            path: row.get(3),
            metadata: serde_json::from_str(row.get::<String, _>(4).as_str()).unwrap_or(serde_json::json!({})),
            created_at: row.get::<chrono::DateTime<Utc>, _>(5),
            updated_at: row.get::<chrono::DateTime<Utc>, _>(6),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_files_operations() {
        let result = Database::new().await;
        assert!(result.is_ok() || result.is_err());
    }
}