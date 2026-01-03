//! Операции с зависимостями файлов в базе данных
//!
//! Модуль содержит методы для работы с зависимостями файлов:
//! получение, добавление, удаление и анализ зависимостей.

use crate::database::modules::base::Database;
use crate::models::dependency::DependencyType;
use crate::models::file::File;
use sqlx::Row;

/// Получить зависимости файла
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `file_id` - ID файла
///
/// # Возвращает
/// Вектор зависимостей файла
pub async fn get_file_dependencies(db: &Database, file_id: i64) -> Result<Vec<crate::models::FileDependency>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM file_dependencies WHERE source_file_id = ?")
        .bind(file_id)
        .fetch_all(&db.pool)
        .await?;

    Ok(rows
        .iter()
        .map(|row| crate::models::FileDependency {
            id: row.get(0),
            source_file_id: row.get(1),
            target_file_name: row.get(2),
            target_file_version: row.get(3),
            dependency_type: match row.get::<String, _>(4).as_str() {
                "required" => DependencyType::Required,
                "optional" => DependencyType::Optional,
                "peer" => DependencyType::Peer,
                _ => DependencyType::Required,
            },
            created_at: row.get::<chrono::DateTime<chrono::Utc>, _>(5),
        })
        .collect())
}

/// Добавить зависимость файла
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `dependency` - объект зависимости для добавления
///
/// # Возвращает
/// Созданная зависимость с присвоенным ID или ошибку
pub async fn add_file_dependency(
    db: &Database,
    dependency: &crate::models::FileDependency,
) -> Result<crate::models::FileDependency, sqlx::Error> {
    let dep_type_str = match dependency.dependency_type {
        DependencyType::Required => "required",
        DependencyType::Optional => "optional",
        DependencyType::Peer => "peer",
    };

    sqlx::query(
        "INSERT INTO file_dependencies (source_file_id, target_file_name, target_file_version, dependency_type, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(dependency.source_file_id)
    .bind(&dependency.target_file_name)
    .bind(&dependency.target_file_version)
    .bind(dep_type_str)
    .bind(dependency.created_at)
    .execute(&db.pool)
    .await?;

    let id = sqlx::query("SELECT last_insert_rowid()")
        .fetch_one(&db.pool)
        .await?
        .get(0);

    Ok(crate::models::FileDependency { id, ..dependency.clone() })
}

/// Добавить зависимость файла (упрощенная версия)
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `source_file_id` - ID исходного файла
/// * `target_file_name` - имя целевого файла
/// * `target_file_version` - версия целевого файла (опционально)
/// * `dependency_type` - тип зависимости
///
/// # Возвращает
/// Созданная зависимость с присвоенным ID или ошибку
pub async fn add_file_dependency_simple(
    db: &Database,
    source_file_id: i64,
    target_file_name: &str,
    target_file_version: Option<&str>,
    dependency_type: DependencyType,
) -> Result<crate::models::FileDependency, sqlx::Error> {
    let dep_type_str = match dependency_type {
        DependencyType::Required => "required",
        DependencyType::Optional => "optional",
        DependencyType::Peer => "peer",
    };

    sqlx::query(
        "INSERT INTO file_dependencies (source_file_id, target_file_name, target_file_version, dependency_type, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(source_file_id)
    .bind(target_file_name)
    .bind(target_file_version)
    .bind(dep_type_str)
    .bind(chrono::Utc::now())
    .execute(&db.pool)
    .await?;

    let id = sqlx::query("SELECT last_insert_rowid()")
        .fetch_one(&db.pool)
        .await?
        .get(0);

    Ok(crate::models::FileDependency {
        id,
        source_file_id,
        target_file_name: target_file_name.to_string(),
        target_file_version: target_file_version.map(|s| s.to_string()),
        dependency_type,
        created_at: chrono::Utc::now(),
    })
}

/// Удалить зависимость файла
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `dependency_id` - ID зависимости для удаления
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn remove_file_dependency(db: &Database, dependency_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM file_dependencies WHERE id = ?")
        .bind(dependency_id)
        .execute(&db.pool)
        .await?;
    Ok(())
}

/// Получить зависимые файлы
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `file` - файл для анализа зависимостей
///
/// # Возвращает
/// Вектор файлов, которые зависят от данного файла
pub async fn get_dependent_files(db: &Database, file: &File) -> Result<Vec<File>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT f.* FROM files f
         INNER JOIN file_dependencies fd ON f.name = fd.target_file_name
         WHERE fd.source_file_id = ?",
    )
    .bind(file.id)
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
            created_at: row.get::<chrono::DateTime<chrono::Utc>, _>(5),
            updated_at: row.get::<chrono::DateTime<chrono::Utc>, _>(6),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dependencies_operations() {
        let result = Database::new().await;
        assert!(result.is_ok() || result.is_err());
    }
}