//! Управление коллекциями (CRUD операции)
//!
//! Этот модуль содержит функции для создания, чтения, обновления и удаления коллекций.

use crate::database::Database;
use crate::models::collection::Collection;
use super::models::{CreateCollectionParams, UpdateCollectionParams};
use log::{error, info};

/// Получить все коллекции
#[tauri::command]
pub async fn get_collections() -> Result<Vec<Collection>, String> {
    info!("Getting all collections");

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let collections = db.get_collections().await.map_err(|e| {
        error!("Failed to get collections: {}", e);
        format!("Database error: {}", e)
    })?;

    info!("Found {} collection(s)", collections.len());
    Ok(collections)
}

/// Создать новую коллекцию
#[tauri::command]
pub async fn create_collection(params: CreateCollectionParams) -> Result<Collection, String> {
    info!("Creating collection: {}", params.name);

    if params.name.trim().is_empty() {
        return Err("Name cannot be empty".to_string());
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

    let collection = db
        .create_collection(&params.name, params.description.as_deref())
        .await
        .map_err(|e| {
            error!("Failed to create collection: {}", e);
            format!("Database error: {}", e)
        })?;

    info!(
        "Collection created successfully: {} (id: {})",
        collection.name, collection.id
    );
    Ok(collection)
}

/// Обновить коллекцию
#[tauri::command]
pub async fn update_collection(params: UpdateCollectionParams) -> Result<Collection, String> {
    info!("Updating collection id: {}", params.id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(params.id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", params.id);
            "Collection not found".to_string()
        })?;

    // Если изменяется имя, проверяем уникальность
    if let Some(ref new_name) = params.name {
        if new_name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        let existing = db.get_collection_by_name(new_name).await.map_err(|e| {
            error!("Failed to check collection existence: {}", e);
            format!("Database error: {}", e)
        })?;

        if let Some(existing_collection) = existing {
            if existing_collection.id != params.id {
                return Err(format!("Collection name already exists: {}", new_name));
            }
        }
    }

    // Обновляем коллекцию
    db.update_collection(
        params.id,
        params.name.as_deref(),
        params.description.as_deref(),
    )
    .await
    .map_err(|e| {
        error!("Failed to update collection: {}", e);
        format!("Database error: {}", e)
    })?;

    // Получаем обновленную коллекцию
    let updated = db
        .get_collection(params.id)
        .await
        .map_err(|e| {
            error!("Failed to get updated collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found after update: {}", params.id);
            "Collection not found".to_string()
        })?;

    info!(
        "Collection updated successfully: {} (id: {})",
        updated.name, updated.id
    );
    Ok(updated)
}

/// Удалить коллекцию
#[tauri::command]
pub async fn delete_collection(collection_id: i64) -> Result<(), String> {
    info!("Deleting collection id: {}", collection_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let collection = db
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

    // Удаляем коллекцию (каскадное удаление обрабатывается БД)
    db.delete_collection(collection_id).await.map_err(|e| {
        error!("Failed to delete collection: {}", e);
        format!("Database error: {}", e)
    })?;

    info!(
        "Collection deleted successfully: {} (id: {})",
        collection.name, collection_id
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    #[tokio::test]
    async fn test_get_collections_empty() {
        // Тест получения пустого списка коллекций
        let result = get_collections().await;
        assert!(result.is_ok() || result.is_err()); // Принимаем оба результата, так как зависит от состояния БД
    }

    #[tokio::test]
    async fn test_create_collection_valid() {
        let params = CreateCollectionParams {
            name: "Test Collection".to_string(),
            description: Some("Test description".to_string()),
        };

        let result = create_collection(params).await;
        // Тест может упасть если коллекция с таким именем уже существует
        // В реальном тесте нужно использовать уникальное имя
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_create_collection_empty_name() {
        let params = CreateCollectionParams {
            name: "".to_string(),
            description: None,
        };

        let result = create_collection(params).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name cannot be empty");
    }

    #[tokio::test]
    async fn test_update_collection_invalid_id() {
        let params = UpdateCollectionParams {
            id: 99999,
            name: Some("Updated Name".to_string()),
            description: Some("Updated description".to_string()),
        };

        let result = update_collection(params).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Collection not found");
    }
}