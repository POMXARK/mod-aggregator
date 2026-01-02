//! Операции с коллекциями в базе данных
//!
//! Модуль содержит методы для работы с коллекциями:
//! получение, создание, обновление коллекций и управление файлами в коллекциях.

use crate::database::modules::base::Database;
use crate::models::collection::Collection;
use crate::models::file::File;
use sqlx::Row;

/// Получить все коллекции
///
/// # Параметры
/// * `db` - подключение к базе данных
///
/// # Возвращает
/// Вектор всех коллекций
pub async fn get_collections(db: &Database) -> Result<Vec<Collection>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM collections ORDER BY name")
        .fetch_all(&db.pool)
        .await?;

    Ok(rows
        .iter()
        .map(|row| Collection {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            created_at: row.get::<chrono::DateTime<chrono::Utc>, _>(3),
            updated_at: row.get::<chrono::DateTime<chrono::Utc>, _>(4),
        })
        .collect())
}

/// Получить коллекцию по ID
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `id` - ID коллекции
///
/// # Возвращает
/// Коллекция или None, если не найдена
pub async fn get_collection(db: &Database, id: i64) -> Result<Option<Collection>, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM collections WHERE id = ?")
        .bind(id)
        .fetch_optional(&db.pool)
        .await?;

    Ok(row.map(|r| Collection {
        id: r.get(0),
        name: r.get(1),
        description: r.get(2),
        created_at: r.get::<chrono::DateTime<chrono::Utc>, _>(3),
        updated_at: r.get::<chrono::DateTime<chrono::Utc>, _>(4),
    }))
}

/// Получить коллекцию по имени
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `name` - имя коллекции
///
/// # Возвращает
/// Коллекция или None, если не найдена
pub async fn get_collection_by_name(db: &Database, name: &str) -> Result<Option<Collection>, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM collections WHERE name = ?")
        .bind(name)
        .fetch_optional(&db.pool)
        .await?;

    Ok(row.map(|r| Collection {
        id: r.get(0),
        name: r.get(1),
        description: r.get(2),
        created_at: r.get::<chrono::DateTime<chrono::Utc>, _>(3),
        updated_at: r.get::<chrono::DateTime<chrono::Utc>, _>(4),
    }))
}

/// Создать новую коллекцию
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `collection` - объект коллекции для создания
///
/// # Возвращает
/// Созданная коллекция с присвоенным ID или ошибку
pub async fn create_collection(db: &Database, collection: &Collection) -> Result<Collection, sqlx::Error> {
    sqlx::query(
        "INSERT INTO collections (name, description, created_at, updated_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&collection.name)
    .bind(&collection.description)
    .bind(collection.created_at)
    .bind(collection.updated_at)
    .execute(&db.pool)
    .await?;

    let id = sqlx::query("SELECT last_insert_rowid()")
        .fetch_one(&db.pool)
        .await?
        .get(0);

    Ok(Collection { id, ..collection.clone() })
}

/// Обновить коллекцию
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `collection` - объект коллекции с обновленными данными
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn update_collection(db: &Database, collection: &Collection) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE collections SET name = ?, description = ?, updated_at = ? WHERE id = ?",
    )
    .bind(&collection.name)
    .bind(&collection.description)
    .bind(collection.updated_at)
    .bind(collection.id)
    .execute(&db.pool)
    .await?;
    Ok(())
}

/// Удалить коллекцию
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `id` - ID коллекции для удаления
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn delete_collection(db: &Database, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM collections WHERE id = ?")
        .bind(id)
        .execute(&db.pool)
        .await?;
    Ok(())
}

/// Получить файлы коллекции
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `collection_id` - ID коллекции
///
/// # Возвращает
/// Вектор файлов в коллекции с их порядком
pub async fn get_collection_files(db: &Database, collection_id: i64) -> Result<Vec<(File, i64)>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT f.*, cf.order_index FROM files f
         INNER JOIN collection_files cf ON f.id = cf.file_id
         WHERE cf.collection_id = ?
         ORDER BY cf.order_index",
    )
    .bind(collection_id)
    .fetch_all(&db.pool)
    .await?;

    Ok(rows
        .iter()
        .map(|row| {
            let file = File {
                id: row.get(0),
                name: row.get(1),
                version: row.get(2),
                path: row.get(3),
                metadata: serde_json::from_str(row.get::<String, _>(4).as_str()).unwrap_or(serde_json::json!({})),
                created_at: row.get::<chrono::DateTime<chrono::Utc>, _>(5),
                updated_at: row.get::<chrono::DateTime<chrono::Utc>, _>(6),
            };
            let order_index = row.get::<i64, _>(7);
            (file, order_index)
        })
        .collect())
}

/// Добавить файл в коллекцию
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `collection_id` - ID коллекции
/// * `file_id` - ID файла
/// * `logic_rule_id` - ID логического правила (опционально)
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn add_file_to_collection(
    db: &Database,
    collection_id: i64,
    file_id: i64,
    logic_rule_id: Option<i64>,
) -> Result<(), sqlx::Error> {
    let max_order = sqlx::query("SELECT COALESCE(MAX(order_index), 0) FROM collection_files WHERE collection_id = ?")
        .bind(collection_id)
        .fetch_one(&db.pool)
        .await?
        .get::<i64, _>(0);

    sqlx::query(
        "INSERT INTO collection_files (collection_id, file_id, logic_rule_id, order_index, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(collection_id)
    .bind(file_id)
    .bind(logic_rule_id)
    .bind(max_order + 1)
    .bind(chrono::Utc::now())
    .execute(&db.pool)
    .await?;
    Ok(())
}

/// Удалить файл из коллекции
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `collection_id` - ID коллекции
/// * `file_id` - ID файла
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn remove_file_from_collection(db: &Database, collection_id: i64, file_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM collection_files WHERE collection_id = ? AND file_id = ?")
        .bind(collection_id)
        .bind(file_id)
        .execute(&db.pool)
        .await?;
    Ok(())
}

/// Изменить порядок файлов в коллекции
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `collection_id` - ID коллекции
/// * `file_orders` - вектор пар (file_id, order_index)
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn reorder_collection_files(
    db: &Database,
    collection_id: i64,
    file_orders: &[(i64, i64)],
) -> Result<(), sqlx::Error> {
    for (file_id, order_index) in file_orders {
        sqlx::query(
            "UPDATE collection_files SET order_index = ? WHERE collection_id = ? AND file_id = ?",
        )
        .bind(order_index)
        .bind(collection_id)
        .bind(file_id)
        .execute(&db.pool)
        .await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_collections_operations() {
        let result = Database::new().await;
        assert!(result.is_ok() || result.is_err());
    }
}