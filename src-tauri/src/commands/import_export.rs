// Import/Export commands
// Implementation for User Story 3: File Import and Export

use crate::database::Database;
use crate::services::import_export_service::{
    ImportExportService, ImportResult, ImportValidationResult, MissingDependency, MissingFile,
};
use log::{error, info, warn};

/// Экспортировать файл в JSON формат
#[tauri::command]
pub async fn export_file(file_id: i64) -> Result<String, String> {
    info!("Exporting file id: {}", file_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Получаем файл
    let file = db
        .get_file(file_id)
        .await
        .map_err(|e| {
            error!("Failed to get file: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("File not found: {}", file_id);
            "File not found".to_string()
        })?;

    // Получаем зависимости
    let dependencies = db.get_file_dependencies(file_id).await.map_err(|e| {
        error!("Failed to get dependencies: {}", e);
        format!("Database error: {}", e)
    })?;

    // Экспортируем
    let json = ImportExportService::export_file(&file, dependencies).map_err(|e| {
        error!("Failed to export file: {}", e);
        e
    })?;

    info!(
        "File exported successfully: {}@{} (id: {})",
        file.name, file.version, file.id
    );
    Ok(json)
}

/// Экспортировать коллекцию в JSON формат
#[tauri::command]
pub async fn export_collection(collection_id: i64) -> Result<String, String> {
    info!("Exporting collection id: {}", collection_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Получаем коллекцию
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

    // Получаем файлы коллекции
    let collection_files = db.get_collection_files_detailed(collection_id).await.map_err(|e| {
        error!("Failed to get collection files: {}", e);
        format!("Database error: {}", e)
    })?;

    // Получаем правила логики
    let logic_rules = db
        .get_collection_logic_rules(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get logic rules: {}", e);
            format!("Database error: {}", e)
        })?;

    // Формируем экспортируемые данные
    use crate::services::import_export_service::{ExportedCollectionFile, ExportedLogicRule};
    let exported_files: Vec<ExportedCollectionFile> = collection_files
        .iter()
        .map(|cf| {
            let logic_rule = cf.logic_rule.as_ref().map(|lr| ExportedLogicRule {
                name: lr.name.clone(),
                condition_type: lr.condition_type.as_str().to_string(),
                condition_params: lr.condition_params.clone(),
                action: lr.action.as_str().to_string(),
            });

            ExportedCollectionFile {
                file_id: cf.file.id,
                name: cf.file.name.clone(),
                version: cf.file.version.clone(),
                order_index: cf.order_index,
                logic_rule,
            }
        })
        .collect();

    let exported_rules: Vec<ExportedLogicRule> = logic_rules
        .iter()
        .map(|lr| ExportedLogicRule {
            name: lr.name.clone(),
            condition_type: lr.condition_type.as_str().to_string(),
            condition_params: lr.condition_params.clone(),
            action: lr.action.as_str().to_string(),
        })
        .collect();

    // Экспортируем
    let json = ImportExportService::export_collection(&collection, exported_files, exported_rules)
        .map_err(|e| {
            error!("Failed to export collection: {}", e);
            e
        })?;

    info!(
        "Collection exported successfully: {} (id: {})",
        collection.name, collection.id
    );
    Ok(json)
}

/// Экспортировать сборку в JSON формат
#[tauri::command]
pub async fn export_build(
    build_name: String,
    collection_ids: Vec<i64>,
    file_ids: Option<Vec<i64>>,
) -> Result<String, String> {
    info!(
        "Exporting build: {} (collections: {:?}, files: {:?})",
        build_name, collection_ids, file_ids
    );

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    use crate::services::import_export_service::{ExportedBuildCollection, ExportedBuildFile};

    // Получаем коллекции
    let mut exported_collections = Vec::new();
    for collection_id in &collection_ids {
        let collection = db
            .get_collection(*collection_id)
            .await
            .map_err(|e| {
                error!("Failed to get collection {}: {}", collection_id, e);
                format!("Database error: {}", e)
            })?
            .ok_or_else(|| {
                error!("Collection not found: {}", collection_id);
                format!("Collection {} not found", collection_id)
            })?;

        exported_collections.push(ExportedBuildCollection {
            collection_id: collection.id,
            collection_name: collection.name.clone(),
        });
    }

    // Получаем файлы
    let mut exported_files = Vec::new();
    if let Some(file_ids) = file_ids {
        for file_id in &file_ids {
            let file = db
                .get_file(*file_id)
                .await
                .map_err(|e| {
                    error!("Failed to get file {}: {}", file_id, e);
                    format!("Database error: {}", e)
                })?
                .ok_or_else(|| {
                    error!("File not found: {}", file_id);
                    format!("File {} not found", file_id)
                })?;

            exported_files.push(ExportedBuildFile {
                file_id: file.id,
                name: file.name.clone(),
                version: file.version.clone(),
            });
        }
    }

    // Экспортируем
    let json = ImportExportService::export_build(build_name, exported_collections, exported_files)
        .map_err(|e| {
            error!("Failed to export build: {}", e);
            e
        })?;

    info!("Build exported successfully");
    Ok(json)
}

/// Импортировать файл из JSON
#[tauri::command]
pub async fn import_file(json_data: String) -> Result<ImportResult, String> {
    info!("Importing file from JSON");

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Парсим и валидируем JSON
    let import_data = ImportExportService::import_file(&json_data).map_err(|e| {
        error!("Failed to parse import JSON: {}", e);
        e
    })?;

    let file = &import_data.file;
    let mut result = ImportResult {
        file_id: None,
        collection_id: None,
        created: false,
        missing_dependencies: Vec::new(),
        missing_files: Vec::new(),
        warnings: Vec::new(),
    };

    // Проверяем, существует ли файл
    let existing = db
        .get_file_by_name_version(&file.name, &file.version)
        .await
        .map_err(|e| {
            error!("Failed to check file existence: {}", e);
            format!("Database error: {}", e)
        })?;

    let file_id = if let Some(existing_file) = existing {
        // Обновляем существующий файл
        info!("Updating existing file: {}@{}", file.name, file.version);
        db.update_file(
            existing_file.id,
            None, // name не меняем
            None, // version не меняем
            file.path.as_deref(),
            Some(&file.metadata),
        )
        .await
        .map_err(|e| {
            error!("Failed to update file: {}", e);
            format!("Database error: {}", e)
        })?;

        result.created = false;
        existing_file.id
    } else {
        // Создаем новый файл
        info!("Creating new file: {}@{}", file.name, file.version);
        let new_file = db
            .create_file(
                &file.name,
                &file.version,
                file.path.as_deref(),
                Some(&file.metadata),
            )
            .await
            .map_err(|e| {
                error!("Failed to create file: {}", e);
                format!("Database error: {}", e)
            })?;

        result.created = true;
        new_file.id
    };

    result.file_id = Some(file_id);

    // Импортируем зависимости
    for dep in &import_data.dependencies {
        // Проверяем наличие целевого файла
        let target_exists = if let Some(ref version) = dep.target_file_version {
            db.get_file_by_name_version(&dep.target_file_name, version)
                .await
                .map_err(|e| {
                    error!("Failed to check target file: {}", e);
                    format!("Database error: {}", e)
                })?
                .is_some()
        } else {
            // Если версия не указана, проверяем наличие любой версии
            let versions = db
                .get_file_versions(&dep.target_file_name)
                .await
                .map_err(|e| {
                    error!("Failed to get file versions: {}", e);
                    format!("Database error: {}", e)
                })?;
            !versions.is_empty()
        };

        if !target_exists {
            // Получаем доступные версии
            let available_versions = db
                .get_file_versions(&dep.target_file_name)
                .await
                .unwrap_or_default()
                .iter()
                .map(|f| f.version.clone())
                .collect();

            result.missing_dependencies.push(MissingDependency {
                target_file_name: dep.target_file_name.clone(),
                target_file_version: dep.target_file_version.clone(),
                available_versions,
            });
        } else {
            // Добавляем зависимость
            use crate::models::dependency::DependencyType;
            let dep_type = match dep.dependency_type.as_str() {
                "required" => DependencyType::Required,
                "optional" => DependencyType::Optional,
                "peer" => DependencyType::Peer,
                _ => {
                    warn!(
                        "Unknown dependency type: {}, defaulting to required",
                        dep.dependency_type
                    );
                    DependencyType::Required
                }
            };

            db.add_file_dependency(
                file_id,
                &dep.target_file_name,
                dep.target_file_version.as_deref(),
                dep_type,
            )
            .await
            .map_err(|e| {
                error!("Failed to add dependency: {}", e);
                format!("Database error: {}", e)
            })?;
        }
    }

    if !result.missing_dependencies.is_empty() {
        result.warnings.push(format!(
            "{} dependency(ies) not found",
            result.missing_dependencies.len()
        ));
    }

    info!(
        "File imported successfully: {}@{} (id: {}, created: {})",
        file.name, file.version, file_id, result.created
    );
    Ok(result)
}

/// Импортировать коллекцию из JSON
#[tauri::command]
pub async fn import_collection(json_data: String) -> Result<ImportResult, String> {
    info!("Importing collection from JSON");

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Парсим и валидируем JSON
    let import_data = ImportExportService::import_collection(&json_data).map_err(|e| {
        error!("Failed to parse import JSON: {}", e);
        e
    })?;

    let collection = &import_data.collection;
    let mut result = ImportResult {
        file_id: None,
        collection_id: None,
        created: false,
        missing_dependencies: Vec::new(),
        missing_files: Vec::new(),
        warnings: Vec::new(),
    };

    // Проверяем, существует ли коллекция с таким именем
    let existing = db
        .get_collection_by_name(&collection.name)
        .await
        .map_err(|e| {
            error!("Failed to check collection existence: {}", e);
            format!("Database error: {}", e)
        })?;

    let collection_id = if let Some(existing_collection) = existing {
        result.warnings.push(format!(
            "Collection '{}' already exists, skipping creation",
            collection.name
        ));
        existing_collection.id
    } else {
        // Создаем новую коллекцию
        info!("Creating new collection: {}", collection.name);
        let new_collection = db
            .create_collection(&collection.name, collection.description.as_deref())
            .await
            .map_err(|e| {
                error!("Failed to create collection: {}", e);
                format!("Database error: {}", e)
            })?;

        result.created = true;
        new_collection.id
    };

    result.collection_id = Some(collection_id);

    // Импортируем файлы
    for exported_file in &import_data.files {
        // Проверяем существование файла
        let file = db
            .get_file_by_name_version(&exported_file.name, &exported_file.version)
            .await
            .map_err(|e| {
                error!("Failed to check file: {}", e);
                format!("Database error: {}", e)
            })?;

        if let Some(file) = file {
            // Добавляем файл в коллекцию
            let logic_rule_id = if let Some(ref rule) = exported_file.logic_rule {
                // Создаем правило логики если оно указано
                use crate::models::collection_logic::{Action, ConditionType};
                let condition_type =
                    ConditionType::from_str(&rule.condition_type).unwrap_or(ConditionType::Boolean);
                let action = Action::from_str(&rule.action).unwrap_or(Action::Enable);

                let rule_to_create = crate::models::collection_logic::CollectionLogicRule {
                    id: 0, // будет присвоено базой данных
                    collection_id,
                    name: rule.name.clone(),
                    condition_type,
                    condition_params: rule.condition_params.clone(),
                    action,
                    created_at: chrono::Utc::now(),
                };

                let created_rule = db
                    .create_collection_logic_rule(&rule_to_create)
                    .await
                    .map_err(|e| {
                        error!("Failed to create logic rule: {}", e);
                        format!("Database error: {}", e)
                    })?;

                Some(created_rule.id)
            } else {
                None
            };

            db.add_file_to_collection(
                collection_id,
                file.id,
                logic_rule_id,
            )
            .await
            .map_err(|e| {
                error!("Failed to add file to collection: {}", e);
                format!("Database error: {}", e)
            })?;
        } else {
            result.missing_files.push(MissingFile {
                file_name: exported_file.name.clone(),
                file_version: exported_file.version.clone(),
            });
        }
    }

    if !result.missing_files.is_empty() {
        result
            .warnings
            .push(format!("{} file(s) not found", result.missing_files.len()));
    }

    info!(
        "Collection imported successfully: {} (id: {}, created: {})",
        collection.name, collection_id, result.created
    );
    Ok(result)
}

/// Импортировать сборку из JSON
#[tauri::command]
pub async fn import_build(json_data: String) -> Result<ImportResult, String> {
    info!("Importing build from JSON");

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Парсим и валидируем JSON
    let import_data = ImportExportService::import_build(&json_data).map_err(|e| {
        error!("Failed to parse import JSON: {}", e);
        e
    })?;

    let mut result = ImportResult {
        file_id: None,
        collection_id: None,
        created: false,
        missing_dependencies: Vec::new(),
        missing_files: Vec::new(),
        warnings: Vec::new(),
    };

    // Проверяем наличие коллекций
    let mut missing_collection_ids = Vec::new();
    for exported_collection in &import_data.collections {
        let collection = db
            .get_collection(exported_collection.collection_id)
            .await
            .ok()
            .flatten();

        if collection.is_none() {
            missing_collection_ids.push(exported_collection.collection_id);
            result.warnings.push(format!(
                "Collection '{}' (id: {}) not found",
                exported_collection.collection_name, exported_collection.collection_id
            ));
        }
    }

    // Проверяем наличие файлов
    for exported_file in &import_data.files {
        let file = db
            .get_file_by_name_version(&exported_file.name, &exported_file.version)
            .await
            .ok()
            .flatten();

        if file.is_none() {
            result.missing_files.push(MissingFile {
                file_name: exported_file.name.clone(),
                file_version: exported_file.version.clone(),
            });
        }
    }

    // Создаем коллекцию для сборки, если нужно
    if !missing_collection_ids.is_empty() || !result.missing_files.is_empty() {
        result
            .warnings
            .push("Some collections or files are missing, build may be incomplete".to_string());
    }

    // Если все коллекции и файлы найдены, можно создать объединенную коллекцию
    if missing_collection_ids.is_empty() && result.missing_files.is_empty() {
        let collection_ids: Vec<i64> = import_data
            .collections
            .iter()
            .map(|c| c.collection_id)
            .collect();
        if !collection_ids.is_empty() {
            // Используем combine_collections для создания объединенной коллекции
            use crate::commands::collections::combine_collections;
            use crate::commands::collections::CombineCollectionsParams;

            match combine_collections(CombineCollectionsParams {
                name: import_data.build_name.clone(),
                description: Some(format!("Imported build: {}", import_data.build_name)),
                source_collection_ids: collection_ids,
                selected_file_ids: None,
            })
            .await
            {
                Ok(combined_collection) => {
                    result.collection_id = Some(combined_collection.id);
                    result.created = true;
                }
                Err(e) => {
                    result
                        .warnings
                        .push(format!("Failed to create combined collection: {}", e));
                }
            }
        }
    }

    info!("Build imported successfully");
    Ok(result)
}

/// Валидировать JSON перед импортом
#[tauri::command]
pub async fn validate_import_json(json_data: String) -> Result<ImportValidationResult, String> {
    info!("Validating import JSON");

    let result = ImportExportService::validate_import_json(&json_data);

    if result.valid {
        info!("Import JSON validation passed");
    } else {
        warn!("Import JSON validation failed: {:?}", result.errors);
    }

    Ok(result)
}
