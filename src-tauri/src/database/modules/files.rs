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

/// Создать новый файл (упрощенная версия)
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `name` - имя файла
/// * `version` - версия файла
/// * `path` - путь к файлу (опционально)
/// * `metadata` - метаданные в формате JSON (опционально)
///
/// # Возвращает
/// Созданный файл с присвоенным ID или ошибку
pub async fn create_file_simple(
    db: &Database,
    name: &str,
    version: &str,
    path: Option<&str>,
    metadata: Option<&serde_json::Value>,
) -> Result<File, sqlx::Error> {
    let metadata_str = metadata.map(|m| serde_json::to_string(m).unwrap_or_default()).unwrap_or_else(|| "{}".to_string());

    sqlx::query(
        "INSERT INTO files (name, version, path, metadata, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(name)
    .bind(version)
    .bind(path)
    .bind(&metadata_str)
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(&db.pool)
    .await?;

    let id = sqlx::query("SELECT last_insert_rowid()")
        .fetch_one(&db.pool)
        .await?
        .get(0);

    Ok(File {
        id,
        name: name.to_string(),
        version: version.to_string(),
        path: path.map(|s| s.to_string()),
        metadata: metadata.cloned().unwrap_or(serde_json::json!({})),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}

/// Обновить существующий файл (упрощенная версия)
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `id` - ID файла
/// * `name` - новое имя файла (опционально)
/// * `version` - новая версия файла (опционально)
/// * `path` - новый путь к файлу (опционально)
/// * `metadata` - новые метаданные в формате JSON (опционально)
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn update_file_simple(
    db: &Database,
    id: i64,
    name: Option<&str>,
    version: Option<&str>,
    path: Option<&str>,
    metadata: Option<&serde_json::Value>,
) -> Result<(), sqlx::Error> {
    let metadata_str = metadata.map(|m| serde_json::to_string(m).unwrap_or_default());

    sqlx::query(
        "UPDATE files SET name = COALESCE(?, name), version = COALESCE(?, version), path = COALESCE(?, path), metadata = COALESCE(?, metadata), updated_at = ? WHERE id = ?",
    )
    .bind(name)
    .bind(version)
    .bind(path)
    .bind(&metadata_str)
    .bind(Utc::now())
    .bind(id)
    .execute(&db.pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{sqlite::SqlitePool, Row};
    use std::env;

    // Helper function to create an in-memory database for testing
    async fn create_test_db() -> Result<Database, sqlx::Error> {
        // Use in-memory SQLite database for tests
        let pool = SqlitePool::connect("sqlite::memory:").await?;

        let db = Database { pool };

        // Initialize schema
        db.init().await?;
        db.run_migrations().await?;

        Ok(db)
    }

    #[tokio::test]
    async fn test_create_file_simple() {
        let db = create_test_db().await.unwrap();

        let metadata = serde_json::json!({"author": "test", "description": "test file"});
        let file = create_file_simple(&db, "test-mod", "1.0.0", Some("/path/to/file"), Some(&metadata)).await.unwrap();

        assert_eq!(file.name, "test-mod");
        assert_eq!(file.version, "1.0.0");
        assert_eq!(file.path, Some("/path/to/file".to_string()));
        assert_eq!(file.metadata["author"], "test");
        assert_eq!(file.metadata["description"], "test file");
    }

    #[tokio::test]
    async fn test_get_file_by_name_version() {
        let db = create_test_db().await.unwrap();

        // Create a file first
        let file = create_file_simple(&db, "test-mod", "1.0.0", None, None).await.unwrap();

        // Retrieve it
        let retrieved = get_file_by_name_version(&db, "test-mod", "1.0.0").await.unwrap().unwrap();

        assert_eq!(retrieved.id, file.id);
        assert_eq!(retrieved.name, "test-mod");
        assert_eq!(retrieved.version, "1.0.0");
    }

    #[tokio::test]
    async fn test_get_file_versions() {
        let db = create_test_db().await.unwrap();

        // Create multiple versions of the same file
        let _file1 = create_file_simple(&db, "test-mod", "1.0.0", None, None).await.unwrap();
        let _file2 = create_file_simple(&db, "test-mod", "1.1.0", None, None).await.unwrap();
        let _file3 = create_file_simple(&db, "test-mod", "2.0.0", None, None).await.unwrap();

        let versions = get_file_versions(&db, "test-mod").await.unwrap();

        assert_eq!(versions.len(), 3);
        // Should be ordered by version DESC
        assert_eq!(versions[0].version, "2.0.0");
        assert_eq!(versions[1].version, "1.1.0");
        assert_eq!(versions[2].version, "1.0.0");
    }

    #[tokio::test]
    async fn test_update_file_simple() {
        let db = create_test_db().await.unwrap();

        // Create a file
        let mut file = create_file_simple(&db, "test-mod", "1.0.0", Some("/old/path"), None).await.unwrap();

        // Update it
        let new_metadata = serde_json::json!({"updated": true});
        update_file_simple(&db, file.id, Some("updated-mod"), Some("2.0.0"), Some("/new/path"), Some(&new_metadata)).await.unwrap();

        // Retrieve and verify
        let updated = get_file(&db, file.id).await.unwrap().unwrap();
        assert_eq!(updated.name, "updated-mod");
        assert_eq!(updated.version, "2.0.0");
        assert_eq!(updated.path, Some("/new/path".to_string()));
        assert_eq!(updated.metadata["updated"], true);
    }

    #[tokio::test]
    async fn test_delete_file() {
        let db = create_test_db().await.unwrap();

        // Create a file
        let file = create_file_simple(&db, "test-mod", "1.0.0", None, None).await.unwrap();

        // Delete it
        delete_file(&db, file.id).await.unwrap();

        // Verify it's gone
        let retrieved = get_file(&db, file.id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_get_all_files() {
        let db = create_test_db().await.unwrap();

        // Create multiple files
        let _file1 = create_file_simple(&db, "mod1", "1.0.0", None, None).await.unwrap();
        let _file2 = create_file_simple(&db, "mod2", "1.0.0", None, None).await.unwrap();
        let _file3 = create_file_simple(&db, "mod3", "1.0.0", None, None).await.unwrap();

        let all_files = get_all_files(&db).await.unwrap();
        assert_eq!(all_files.len(), 3);
    }

    #[tokio::test]
    async fn test_get_file_not_found() {
        let db = create_test_db().await.unwrap();

        let result = get_file(&db, 999).await.unwrap();
        assert!(result.is_none());

        let result = get_file_by_name_version(&db, "nonexistent", "1.0.0").await.unwrap();
        assert!(result.is_none());
    }
}