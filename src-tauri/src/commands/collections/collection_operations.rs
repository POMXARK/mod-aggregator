//! Сложные операции с коллекциями
//!
//! Этот модуль содержит функции для выполнения сложных операций,
//! таких как объединение коллекций и получение файлов из нескольких коллекций.

use crate::database::Database;
use crate::models::collection::Collection;
use super::models::{CollectionFileView, CollectionFileInfo, CombineCollectionsParams};
use log::{error, info};

/// Объединить несколько коллекций в новую
#[tauri::command]
pub async fn combine_collections(params: CombineCollectionsParams) -> Result<Collection, String> {
    info!("Combining collections into: {}", params.name);

    if params.name.trim().is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    if params.source_collection_ids.is_empty() {
        return Err("At least one source collection is required".to_string());
    }

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем уникальность имени
    let existing = db.get_collection_by_name(&params.name).await.map_err(|e| {
        error!("Failed to check collection existence: {}", e);
        format!("Database error: {}", e)
    })?;

    if existing.is_some() {
        return Err(format!("Collection name already exists: {}", params.name));
    }

    // Проверяем существование всех исходных коллекций
    for collection_id in &params.source_collection_ids {
        let collection = db
            .get_collection(*collection_id)
            .await
            .map_err(|e| {
                error!("Failed to get collection: {}", e);
                format!("Database error: {}", e)
            })?
            .ok_or_else(|| {
                error!("Source collection not found: {}", collection_id);
                format!("Source collection not found: {}", collection_id)
            })?;

        let _ = collection;
    }

    // Создаем новую коллекцию
    let new_collection = db
        .create_collection(&params.name, params.description.as_deref())
        .await
        .map_err(|e| {
            error!("Failed to create collection: {}", e);
            format!("Database error: {}", e)
        })?;

    // Собираем все файлы из исходных коллекций
    let mut all_file_ids = std::collections::HashSet::new();
    let mut file_orders = Vec::new();

    for collection_id in &params.source_collection_ids {
        let collection_files = db.get_collection_files_detailed(*collection_id).await.map_err(|e| {
            error!("Failed to get collection files: {}", e);
            format!("Database error: {}", e)
        })?;

        for cf in collection_files {
            if !all_file_ids.contains(&cf.file_id) {
                all_file_ids.insert(cf.file_id);
                file_orders.push((cf.file_id, cf.order_index));
            }
        }
    }

    // Фильтруем по selected_file_ids, если указаны
    let file_ids_to_add: Vec<i64> = if let Some(ref selected) = params.selected_file_ids {
        if selected.is_empty() {
            return Err("No files selected".to_string());
        }

        // Проверяем, что все выбранные файлы существуют
        for file_id in selected {
            let file = db
                .get_file(*file_id)
                .await
                .map_err(|e| {
                    error!("Failed to get file: {}", e);
                    format!("Database error: {}", e)
                })?
                .ok_or_else(|| {
                    error!("File not found: {}", file_id);
                    format!("File not found: {}", file_id)
                })?;

            let _ = file;
        }

        selected.clone()
    } else {
        all_file_ids.into_iter().collect()
    };

    // Добавляем файлы в новую коллекцию
    for (index, file_id) in file_ids_to_add.iter().enumerate() {
        db.add_file_to_collection(
            new_collection.id,
            *file_id,
            None, // Логика не копируется
        )
        .await
        .map_err(|e| {
            error!("Failed to add file to collection: {}", e);
            format!("Database error: {}", e)
        })?;
    }

    info!(
        "Collections combined successfully: {} files in new collection {}",
        file_ids_to_add.len(),
        new_collection.name
    );
    Ok(new_collection)
}

/// Получить файлы из нескольких коллекций
#[tauri::command]
pub async fn get_files_from_multiple_collections(
    collection_ids: Vec<i64>,
) -> Result<Vec<CollectionFileView>, String> {
    info!("Getting files from {} collection(s)", collection_ids.len());

    if collection_ids.is_empty() {
        return Ok(Vec::new());
    }

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование всех коллекций
    for collection_id in &collection_ids {
        let collection = db
            .get_collection(*collection_id)
            .await
            .map_err(|e| {
                error!("Failed to get collection: {}", e);
                format!("Database error: {}", e)
            })?
            .ok_or_else(|| {
                error!("Collection not found: {}", collection_id);
                format!("Collection not found: {}", collection_id)
            })?;

        let _ = collection;
    }

    // Собираем файлы из всех коллекций
    let mut files_map: std::collections::HashMap<i64, CollectionFileView> =
        std::collections::HashMap::new();

    for collection_id in &collection_ids {
        let collection = db
            .get_collection(*collection_id)
            .await
            .map_err(|e| {
                error!("Failed to get collection: {}", e);
                format!("Database error: {}", e)
            })?
            .ok_or_else(|| {
                error!("Collection not found: {}", collection_id);
                "Collection not found".to_string()
            })?;

        let collection_files = db.get_collection_files_detailed(*collection_id).await.map_err(|e| {
            error!("Failed to get collection files: {}", e);
            format!("Database error: {}", e)
        })?;

        for cf in collection_files {
            let file_id = cf.file_id;

            if let Some(file_view) = files_map.get_mut(&file_id) {
                // Файл уже есть, добавляем информацию о коллекции
                file_view.collections.push(CollectionFileInfo {
                    collection_id: *collection_id,
                    collection_name: collection.name.clone(),
                    order_index: cf.order_index,
                    logic_rule_id: cf.logic_rule_id,
                });
            } else {
                // Новый файл
                files_map.insert(
                    file_id,
                    CollectionFileView {
                        file_id,
                        file: cf.file.clone(),
                        collections: vec![CollectionFileInfo {
                            collection_id: *collection_id,
                            collection_name: collection.name.clone(),
                            order_index: cf.order_index,
                            logic_rule_id: cf.logic_rule_id,
                        }],
                    },
                );
            }
        }
    }

    let result: Vec<CollectionFileView> = files_map.into_values().collect();
    info!(
        "Found {} unique file(s) across {} collection(s)",
        result.len(),
        collection_ids.len()
    );
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    #[tokio::test]
    async fn test_combine_collections_empty_name() {
        let params = CombineCollectionsParams {
            name: "".to_string(),
            description: None,
            source_collection_ids: vec![1],
            selected_file_ids: None,
        };

        let result = combine_collections(params).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name cannot be empty");
    }

    #[tokio::test]
    async fn test_combine_collections_no_sources() {
        let params = CombineCollectionsParams {
            name: "Test Collection".to_string(),
            description: None,
            source_collection_ids: vec![],
            selected_file_ids: None,
        };

        let result = combine_collections(params).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "At least one source collection is required");
    }

    #[tokio::test]
    async fn test_combine_collections_empty_selection() {
        let params = CombineCollectionsParams {
            name: "Test Collection".to_string(),
            description: None,
            source_collection_ids: vec![1],
            selected_file_ids: Some(vec![]),
        };

        let result = combine_collections(params).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No files selected");
    }

    #[tokio::test]
    async fn test_get_files_from_multiple_collections_empty() {
        let collection_ids: Vec<i64> = vec![];

        let result = get_files_from_multiple_collections(collection_ids).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_get_files_from_multiple_collections_invalid_collection() {
        let collection_ids = vec![99999];

        let result = get_files_from_multiple_collections(collection_ids).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Collection not found"));
    }
}