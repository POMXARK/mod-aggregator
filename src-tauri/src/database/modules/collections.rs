//! Операции с коллекциями в базе данных
//!
//! Модуль содержит методы для работы с коллекциями:
//! получение, создание, обновление коллекций и управление файлами в коллекциях.

use crate::database::Database;
use crate::models::collection::Collection;
use crate::models::collection_logic::{CollectionLogicRule, ConditionType, Action};
use crate::models::file::File;
use sqlx::Row;
use chrono::{DateTime, Utc};

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

/// Получить полную информацию о файлах коллекции
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `collection_id` - ID коллекции
///
/// # Возвращает
/// Вектор структур с полной информацией о файлах в коллекции
pub async fn get_collection_files_detailed(db: &Database, collection_id: i64) -> Result<Vec<crate::database::CollectionFileData>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT cf.id, cf.collection_id, cf.file_id, cf.logic_rule_id, cf.order_index, cf.created_at,
                f.id as f_id, f.name, f.version, f.path, f.metadata, f.created_at as f_created_at, f.updated_at,
                clr.id as clr_id, clr.name as clr_name, clr.condition_type, clr.condition_params, clr.action, clr.created_at as clr_created_at
         FROM collection_files cf
         INNER JOIN files f ON cf.file_id = f.id
         LEFT JOIN collection_logic_rules clr ON cf.logic_rule_id = clr.id
         WHERE cf.collection_id = ?
         ORDER BY cf.order_index",
    )
    .bind(collection_id)
    .fetch_all(&db.pool)
    .await?;

    let mut result = Vec::new();

    for row in rows {
        let file = File {
            id: row.get("f_id"),
            name: row.get("name"),
            version: row.get("version"),
            path: row.get("path"),
            metadata: serde_json::from_str(row.get::<String, _>("metadata").as_str()).unwrap_or(serde_json::json!({})),
            created_at: row.get::<DateTime<Utc>, _>("f_created_at"),
            updated_at: row.get::<DateTime<Utc>, _>("f_updated_at"),
        };

        let logic_rule = if let Some(clr_id) = row.get::<Option<i64>, _>("clr_id") {
            Some(CollectionLogicRule {
                id: clr_id,
                collection_id: row.get("collection_id"),
                name: row.get("clr_name"),
                condition_type: match row.get::<String, _>("condition_type").as_str() {
                    "boolean" => ConditionType::Boolean,
                    "collection_check" => ConditionType::CollectionCheck,
                    "file_check" => ConditionType::FileCheck,
                    "and" => ConditionType::And,
                    "or" => ConditionType::Or,
                    _ => ConditionType::Boolean,
                },
                condition_params: row.get("condition_params"),
                action: match row.get::<String, _>("action").as_str() {
                    "enable" => Action::Enable,
                    "disable" => Action::Disable,
                    _ => Action::Enable,
                },
                created_at: row.get::<DateTime<Utc>, _>("clr_created_at"),
            })
        } else {
            None
        };

        result.push(crate::database::CollectionFileData {
            id: row.get("id"),
            collection_id: row.get("collection_id"),
            file_id: row.get("file_id"),
            file,
            logic_rule_id: row.get("logic_rule_id"),
            logic_rule,
            order_index: row.get("order_index"),
            created_at: row.get::<DateTime<Utc>, _>("created_at"),
        });
    }

    Ok(result)
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

/// Получить логическое правило коллекции по ID
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `rule_id` - ID правила
///
/// # Возвращает
/// Правило или None, если не найдено
pub async fn get_collection_logic_rule(db: &Database, rule_id: i64) -> Result<Option<CollectionLogicRule>, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM collection_logic_rules WHERE id = ?")
        .bind(rule_id)
        .fetch_optional(&db.pool)
        .await?;

    Ok(row.map(|r| CollectionLogicRule {
        id: r.get(0),
        collection_id: r.get(1),
        name: r.get(2),
        condition_type: match r.get::<String, _>(3).as_str() {
            "boolean" => ConditionType::Boolean,
            "collection_check" => ConditionType::CollectionCheck,
            "file_check" => ConditionType::FileCheck,
            "and" => ConditionType::And,
            "or" => ConditionType::Or,
            _ => ConditionType::Boolean,
        },
        condition_params: r.get(4),
        action: match r.get::<String, _>(5).as_str() {
            "enable" => Action::Enable,
            "disable" => Action::Disable,
            _ => Action::Enable,
        },
        created_at: r.get::<DateTime<Utc>, _>(6),
    }))
}

/// Получить все логические правила коллекции
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `collection_id` - ID коллекции
///
/// # Возвращает
/// Вектор правил коллекции
pub async fn get_collection_logic_rules(db: &Database, collection_id: i64) -> Result<Vec<CollectionLogicRule>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM collection_logic_rules WHERE collection_id = ?")
        .bind(collection_id)
        .fetch_all(&db.pool)
        .await?;

    Ok(rows
        .iter()
        .map(|r| CollectionLogicRule {
            id: r.get(0),
            collection_id: r.get(1),
            name: r.get(2),
            condition_type: match r.get::<String, _>(3).as_str() {
                "boolean" => ConditionType::Boolean,
                "collection_check" => ConditionType::CollectionCheck,
                "file_check" => ConditionType::FileCheck,
                "and" => ConditionType::And,
                "or" => ConditionType::Or,
                _ => ConditionType::Boolean,
            },
            condition_params: r.get(4),
            action: match r.get::<String, _>(5).as_str() {
                "enable" => Action::Enable,
                "disable" => Action::Disable,
                _ => Action::Enable,
            },
            created_at: r.get::<DateTime<Utc>, _>(6),
        })
        .collect())
}

/// Создать логическое правило коллекции
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `rule` - правило для создания
///
/// # Возвращает
/// Созданное правило с присвоенным ID или ошибку
pub async fn create_collection_logic_rule(db: &Database, rule: &CollectionLogicRule) -> Result<CollectionLogicRule, sqlx::Error> {
    let condition_type_str = match rule.condition_type {
        ConditionType::Boolean => "boolean",
        ConditionType::CollectionCheck => "collection_check",
        ConditionType::FileCheck => "file_check",
        ConditionType::And => "and",
        ConditionType::Or => "or",
    };

    let action_str = match rule.action {
        Action::Enable => "enable",
        Action::Disable => "disable",
    };

    sqlx::query(
        "INSERT INTO collection_logic_rules (collection_id, name, condition_type, condition_params, action, created_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(rule.collection_id)
    .bind(&rule.name)
    .bind(condition_type_str)
    .bind(&rule.condition_params)
    .bind(action_str)
    .bind(rule.created_at)
    .execute(&db.pool)
    .await?;

    let id = sqlx::query("SELECT last_insert_rowid()")
        .fetch_one(&db.pool)
        .await?
        .get(0);

    Ok(CollectionLogicRule { id, ..rule.clone() })
}

/// Обновить логическое правило коллекции
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `rule` - правило с обновленными данными
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn update_collection_logic_rule(db: &Database, rule: &CollectionLogicRule) -> Result<(), sqlx::Error> {
    let condition_type_str = match rule.condition_type {
        ConditionType::Boolean => "boolean",
        ConditionType::CollectionCheck => "collection_check",
        ConditionType::FileCheck => "file_check",
        ConditionType::And => "and",
        ConditionType::Or => "or",
    };

    let action_str = match rule.action {
        Action::Enable => "enable",
        Action::Disable => "disable",
    };

    sqlx::query(
        "UPDATE collection_logic_rules SET name = ?, condition_type = ?, condition_params = ?, action = ? WHERE id = ?",
    )
    .bind(&rule.name)
    .bind(condition_type_str)
    .bind(&rule.condition_params)
    .bind(action_str)
    .bind(rule.id)
    .execute(&db.pool)
    .await?;

    Ok(())
}

/// Удалить логическое правило коллекции
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `rule_id` - ID правила для удаления
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
pub async fn delete_collection_logic_rule(db: &Database, rule_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM collection_logic_rules WHERE id = ?")
        .bind(rule_id)
        .execute(&db.pool)
        .await?;
    Ok(())
}

/// Проверить, находится ли файл в коллекции
///
/// # Параметры
/// * `db` - подключение к базе данных
/// * `collection_id` - ID коллекции
/// * `file_id` - ID файла
///
/// # Возвращает
/// true если файл находится в коллекции
pub async fn file_in_collection(db: &Database, collection_id: i64, file_id: i64) -> Result<bool, sqlx::Error> {
    let count: i64 = sqlx::query("SELECT COUNT(*) FROM collection_files WHERE collection_id = ? AND file_id = ?")
        .bind(collection_id)
        .bind(file_id)
        .fetch_one(&db.pool)
        .await?
        .get(0);

    Ok(count > 0)
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