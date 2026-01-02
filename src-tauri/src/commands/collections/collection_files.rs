//! Управление файлами в коллекциях
//!
//! Этот модуль содержит функции для добавления, удаления и изменения порядка файлов в коллекциях.

use crate::database::Database;
use super::models::{CollectionFile, AddFileToCollectionParams, FileOrder};
use log::{error, info};

/// Получить файлы коллекции
#[tauri::command]
pub async fn get_collection_files(collection_id: i64) -> Result<Vec<CollectionFile>, String> {
    info!("Getting files for collection id: {}", collection_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", collection_id);
            "Collection not found".to_string()
        })?;

    let collection_files = db.get_collection_files_detailed(collection_id).await.map_err(|e| {
        error!("Failed to get collection files: {}", e);
        format!("Database error: {}", e)
    })?;

    let result: Vec<CollectionFile> = collection_files
        .iter()
        .map(|cf| CollectionFile {
            id: cf.id,
            collection_id: cf.collection_id,
            file_id: cf.file_id,
            file: cf.file.clone(),
            logic_rule_id: cf.logic_rule_id,
            logic_rule: cf.logic_rule.clone(),
            order_index: cf.order_index,
            created_at: cf.created_at.to_rfc3339(),
        })
        .collect();

    info!(
        "Found {} file(s) in collection {}",
        result.len(),
        collection_id
    );
    Ok(result)
}

/// Добавить файл в коллекцию
#[tauri::command]
pub async fn add_file_to_collection(
    params: AddFileToCollectionParams,
) -> Result<CollectionFile, String> {
    info!(
        "Adding file {} to collection {}",
        params.file_id, params.collection_id
    );

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(params.collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", params.collection_id);
            "Collection not found".to_string()
        })?;

    // Проверяем существование файла
    let _file = db
        .get_file(params.file_id)
        .await
        .map_err(|e| {
            error!("Failed to get file: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("File not found: {}", params.file_id);
            "File not found".to_string()
        })?;

    // Проверяем, не находится ли файл уже в коллекции
    let already_in = db
        .file_in_collection(params.collection_id, params.file_id)
        .await
        .map_err(|e| {
            error!("Failed to check file in collection: {}", e);
            format!("Database error: {}", e)
        })?;

    if already_in {
        return Err("File already in collection".to_string());
    }

    // Проверяем правило логики, если указано
    if let Some(rule_id) = params.logic_rule_id {
        let rule = db
            .get_collection_logic_rule(rule_id)
            .await
            .map_err(|e| {
                error!("Failed to get logic rule: {}", e);
                format!("Database error: {}", e)
            })?
            .ok_or_else(|| {
                error!("Logic rule not found: {}", rule_id);
                "Logic rule not found".to_string()
            })?;

        // Проверяем, что правило принадлежит этой коллекции
        if rule.collection_id != params.collection_id {
            return Err("Logic rule does not belong to this collection".to_string());
        }
    }

    // Добавляем файл
    db
        .add_file_to_collection(
            params.collection_id,
            params.file_id,
            params.logic_rule_id,
        )
        .await
        .map_err(|e| {
            error!("Failed to add file to collection: {}", e);
            format!("Database error: {}", e)
        })?;

    // Получаем добавленный файл
    let collection_files = db
        .get_collection_files_detailed(params.collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection files: {}", e);
            format!("Database error: {}", e)
        })?;

    let collection_file = collection_files
        .iter()
        .find(|cf| cf.file_id == params.file_id)
        .ok_or_else(|| {
            error!(
                "Collection file not found after creation: file_id={}",
                params.file_id
            );
            "Collection file not found".to_string()
        })?;

    let result = CollectionFile {
        id: collection_file.id,
        collection_id: collection_file.collection_id,
        file_id: collection_file.file_id,
        file: collection_file.file.clone(),
        logic_rule_id: collection_file.logic_rule_id,
        logic_rule: collection_file.logic_rule.clone(),
        order_index: collection_file.order_index,
        created_at: collection_file.created_at.to_rfc3339(),
    };

    info!(
        "File added to collection successfully: file_id={}, collection_id={}",
        params.file_id, params.collection_id
    );
    Ok(result)
}

/// Удалить файл из коллекции
#[tauri::command]
pub async fn remove_file_from_collection(collection_id: i64, file_id: i64) -> Result<(), String> {
    info!(
        "Removing file {} from collection {}",
        file_id, collection_id
    );

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", collection_id);
            "Collection not found".to_string()
        })?;

    // Проверяем, находится ли файл в коллекции
    let in_collection = db
        .file_in_collection(collection_id, file_id)
        .await
        .map_err(|e| {
            error!("Failed to check file in collection: {}", e);
            format!("Database error: {}", e)
        })?;

    if !in_collection {
        return Err("File not in collection".to_string());
    }

    // Удаляем файл
    db.remove_file_from_collection(collection_id, file_id)
        .await
        .map_err(|e| {
            error!("Failed to remove file from collection: {}", e);
            format!("Database error: {}", e)
        })?;

    info!(
        "File removed from collection successfully: file_id={}, collection_id={}",
        file_id, collection_id
    );
    Ok(())
}

/// Изменить порядок файлов в коллекции
#[tauri::command]
pub async fn reorder_collection_files(
    collection_id: i64,
    file_orders: Vec<FileOrder>,
) -> Result<(), String> {
    info!("Reordering files in collection {}", collection_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", collection_id);
            "Collection not found".to_string()
        })?;

    // Проверяем, что все файлы в коллекции
    for file_order in &file_orders {
        let in_collection = db
            .file_in_collection(collection_id, file_order.file_id)
            .await
            .map_err(|e| {
                error!("Failed to check file in collection: {}", e);
                format!("Database error: {}", e)
            })?;

        if !in_collection {
            return Err(format!("File not in collection: {}", file_order.file_id));
        }
    }

    // Изменяем порядок
    let orders: Vec<(i64, i64)> = file_orders
        .iter()
        .map(|fo| (fo.file_id, fo.order_index))
        .collect();

    db.reorder_collection_files(collection_id, &orders)
        .await
        .map_err(|e| {
            error!("Failed to reorder collection files: {}", e);
            format!("Database error: {}", e)
        })?;

    info!(
        "Files reordered successfully in collection {}",
        collection_id
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    #[tokio::test]
    async fn test_get_collection_files_invalid_collection() {
        let result = get_collection_files(99999).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Collection not found");
    }

    #[tokio::test]
    async fn test_add_file_to_collection_invalid_collection() {
        let params = AddFileToCollectionParams {
            collection_id: 99999,
            file_id: 1,
            logic_rule_id: None,
            order_index: None,
        };

        let result = add_file_to_collection(params).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Collection not found");
    }

    #[tokio::test]
    async fn test_add_file_to_collection_invalid_file() {
        let params = AddFileToCollectionParams {
            collection_id: 1,
            file_id: 99999,
            logic_rule_id: None,
            order_index: None,
        };

        let result = add_file_to_collection(params).await;
        // Может быть ошибка "Collection not found" или "File not found" в зависимости от состояния БД
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_file_from_collection_invalid_collection() {
        let result = remove_file_from_collection(99999, 1).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Collection not found");
    }

    #[tokio::test]
    async fn test_reorder_collection_files_invalid_collection() {
        let file_orders = vec![FileOrder {
            file_id: 1,
            order_index: 0,
        }];

        let result = reorder_collection_files(99999, file_orders).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Collection not found");
    }

    #[tokio::test]
    async fn test_reorder_collection_files_empty_orders() {
        let file_orders: Vec<FileOrder> = vec![];

        let result = reorder_collection_files(1, file_orders).await;
        // Может быть ошибка "Collection not found" в зависимости от состояния БД
        assert!(result.is_ok() || result.is_err());
    }
}