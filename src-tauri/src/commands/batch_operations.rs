// Batch operations commands
// Implementation for User Story 5: Multi-Selection and Batch Operations

use crate::database::Database;
use crate::models::dependency::DependencyType;
use log::{error, info};
use serde::{Deserialize, Serialize};

/// Результат операции для одного файла
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationResult {
    pub file_id: i64,
    pub success: bool,
    pub error: Option<String>,
}

/// Заблокированный файл (не может быть удален из-за зависимостей)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedFile {
    pub file_id: i64,
    pub file_name: String,
    pub dependent_files: Vec<i64>,
}

/// Результат пакетной операции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationResult {
    pub success_count: usize,
    pub failed_count: usize,
    pub results: Vec<FileOperationResult>,
    pub warnings: Vec<String>,
    pub blocked_files: Vec<BlockedFile>,
}

/// Результат проверки зависимостей для нескольких файлов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingDependency {
    pub target_file_name: String,
    pub target_file_version: Option<String>,
    pub dependency_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionConflict {
    pub target_file_name: String,
    pub required_version: Option<String>,
    pub found_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDependencyCheck {
    pub file_id: i64,
    pub file_name: String,
    pub missing_dependencies: Vec<MissingDependency>,
    pub satisfied_dependencies_count: usize,
    pub version_conflicts: Vec<VersionConflict>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDependencyCheckResult {
    pub results: Vec<FileDependencyCheck>,
    pub summary: DependencyCheckSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyCheckSummary {
    pub total_files: usize,
    pub files_with_missing_deps: usize,
    pub files_with_conflicts: usize,
    pub total_missing_deps: usize,
}

/// Результат валидации пакетной операции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedDependency {
    pub file_id: i64,
    pub dependent_files: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchValidationResult {
    pub valid: bool,
    pub can_proceed: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub affected_dependencies: Vec<AffectedDependency>,
}

/// Удалить несколько файлов одновременно
///
/// Проверяет зависимости для каждого файла и удаляет только те, которые не имеют зависимых файлов.
/// Если force=true, удаляет все файлы без проверки зависимостей.
#[tauri::command]
pub async fn batch_delete_files(
    file_ids: Vec<i64>,
    force: Option<bool>,
) -> Result<BatchOperationResult, String> {
    info!(
        "Batch deleting {} files (force: {:?})",
        file_ids.len(),
        force
    );

    if file_ids.is_empty() {
        return Err("No files specified".to_string());
    }

    let force_delete = force.unwrap_or(false);
    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let mut results = Vec::new();
    #[allow(unused_mut)]
    #[allow(unused_mut)]
    let mut warnings = Vec::new();
    let mut blocked_files = Vec::new();
    let mut success_count = 0;
    let mut failed_count = 0;

    for file_id in file_ids {
        // Получаем информацию о файле
        let file = match db.get_file(file_id).await {
            Ok(Some(f)) => f,
            Ok(None) => {
                results.push(FileOperationResult {
                    file_id,
                    success: false,
                    error: Some("File not found".to_string()),
                });
                failed_count += 1;
                continue;
            }
            Err(e) => {
                error!("Failed to get file {}: {}", file_id, e);
                results.push(FileOperationResult {
                    file_id,
                    success: false,
                    error: Some(format!("Database error: {}", e)),
                });
                failed_count += 1;
                continue;
            }
        };

        // Проверяем зависимости, если не force
        if !force_delete {
            let dependent_files = db.get_dependent_files(&file).await.unwrap_or_default();

            if !dependent_files.is_empty() {
                blocked_files.push(BlockedFile {
                    file_id,
                    file_name: format!("{}@{}", file.name, file.version),
                    dependent_files: dependent_files.iter().map(|f| f.id).collect(),
                });
                results.push(FileOperationResult {
                    file_id,
                    success: false,
                    error: Some(format!(
                        "File has {} dependent file(s)",
                        dependent_files.len()
                    )),
                });
                failed_count += 1;
                continue;
            }
        }

        // Удаляем файл
        match db.delete_file(file_id).await {
            Ok(_) => {
                results.push(FileOperationResult {
                    file_id,
                    success: true,
                    error: None,
                });
                success_count += 1;
            }
            Err(e) => {
                error!("Failed to delete file {}: {}", file_id, e);
                results.push(FileOperationResult {
                    file_id,
                    success: false,
                    error: Some(format!("Database error: {}", e)),
                });
                failed_count += 1;
            }
        }
    }

    Ok(BatchOperationResult {
        success_count,
        failed_count,
        results,
        warnings,
        blocked_files,
    })
}

/// Переместить несколько файлов в коллекцию
#[tauri::command]
pub async fn batch_move_files_to_collection(
    file_ids: Vec<i64>,
    collection_id: i64,
) -> Result<BatchOperationResult, String> {
    info!(
        "Moving {} files to collection {}",
        file_ids.len(),
        collection_id
    );

    if file_ids.is_empty() {
        return Err("No files specified".to_string());
    }

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let collection = db.get_collection(collection_id).await.map_err(|e| {
        error!("Failed to get collection: {}", e);
        format!("Database error: {}", e)
    })?;

    if collection.is_none() {
        return Err("Collection not found".to_string());
    }

    let mut results = Vec::new();
    #[allow(unused_mut)]
    let mut warnings = Vec::new();
    let mut success_count = 0;
    let mut failed_count = 0;

    for file_id in file_ids {
        // Проверяем, не находится ли файл уже в коллекции
        let collection_files = db
            .get_collection_files(collection_id)
            .await
            .unwrap_or_default();
        let already_in_collection = collection_files.iter().any(|cf| cf.file_id == file_id);

        if already_in_collection {
            warnings.push(format!("File {} is already in collection", file_id));
            results.push(FileOperationResult {
                file_id,
                success: true,
                error: None,
            });
            success_count += 1;
            continue;
        }

        // Добавляем файл в коллекцию
        match db
            .add_file_to_collection(collection_id, file_id, None, None)
            .await
        {
            Ok(_) => {
                results.push(FileOperationResult {
                    file_id,
                    success: true,
                    error: None,
                });
                success_count += 1;
            }
            Err(e) => {
                error!("Failed to add file {} to collection: {}", file_id, e);
                results.push(FileOperationResult {
                    file_id,
                    success: false,
                    error: Some(format!("Database error: {}", e)),
                });
                failed_count += 1;
            }
        }
    }

    Ok(BatchOperationResult {
        success_count,
        failed_count,
        results,
        warnings,
        blocked_files: Vec::new(),
    })
}

/// Обновить свойства нескольких файлов
#[tauri::command]
pub async fn batch_update_file_properties(
    file_ids: Vec<i64>,
    properties: serde_json::Value,
) -> Result<BatchOperationResult, String> {
    info!("Updating properties for {} files", file_ids.len());

    if file_ids.is_empty() {
        return Err("No files specified".to_string());
    }

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let metadata = properties
        .get("metadata")
        .and_then(|v| v.as_object())
        .cloned();
    let path = properties
        .get("path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let mut results = Vec::new();
    #[allow(unused_mut)]
    let mut warnings = Vec::new();
    let mut success_count = 0;
    let mut failed_count = 0;

    for file_id in file_ids {
        // Получаем текущий файл
        let _file = match db.get_file(file_id).await {
            Ok(Some(f)) => f,
            Ok(None) => {
                results.push(FileOperationResult {
                    file_id,
                    success: false,
                    error: Some("File not found".to_string()),
                });
                failed_count += 1;
                continue;
            }
            Err(e) => {
                error!("Failed to get file {}: {}", file_id, e);
                results.push(FileOperationResult {
                    file_id,
                    success: false,
                    error: Some(format!("Database error: {}", e)),
                });
                failed_count += 1;
                continue;
            }
        };

        // Обновляем файл
        let new_metadata = if let Some(m) = metadata.as_ref() {
            Some(serde_json::Value::Object(m.clone()))
        } else {
            None
        };

        match db
            .update_file(
                file_id,
                None, // name не обновляется пакетно
                None, // version не обновляется пакетно
                path.as_deref(),
                new_metadata.as_ref(),
            )
            .await
        {
            Ok(_) => {
                results.push(FileOperationResult {
                    file_id,
                    success: true,
                    error: None,
                });
                success_count += 1;
            }
            Err(e) => {
                error!("Failed to update file {}: {}", file_id, e);
                results.push(FileOperationResult {
                    file_id,
                    success: false,
                    error: Some(format!("Database error: {}", e)),
                });
                failed_count += 1;
            }
        }
    }

    Ok(BatchOperationResult {
        success_count,
        failed_count,
        results,
        warnings,
        blocked_files: Vec::new(),
    })
}

/// Добавить несколько зависимостей к файлу
#[tauri::command]
pub async fn batch_add_dependencies(
    file_id: i64,
    dependencies: Vec<serde_json::Value>,
) -> Result<BatchOperationResult, String> {
    info!(
        "Adding {} dependencies to file {}",
        dependencies.len(),
        file_id
    );

    if dependencies.is_empty() {
        return Err("No dependencies specified".to_string());
    }

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование файла
    let file = db.get_file(file_id).await.map_err(|e| {
        error!("Failed to get file: {}", e);
        format!("Database error: {}", e)
    })?;

    if file.is_none() {
        return Err("File not found".to_string());
    }

    let mut results = Vec::new();
    #[allow(unused_mut)]
    let mut warnings = Vec::new();
    let mut success_count = 0;
    let mut failed_count = 0;

    for dep_json in dependencies {
        let target_file_name = dep_json
            .get("target_file_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Missing target_file_name".to_string())?
            .to_string();

        let target_file_version = dep_json
            .get("target_file_version")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let dependency_type_str = dep_json
            .get("dependency_type")
            .and_then(|v| v.as_str())
            .unwrap_or("required");

        let dependency_type = match dependency_type_str {
            "required" => DependencyType::Required,
            "optional" => DependencyType::Optional,
            "peer" => DependencyType::Peer,
            _ => {
                results.push(FileOperationResult {
                    file_id,
                    success: false,
                    error: Some(format!("Invalid dependency_type: {}", dependency_type_str)),
                });
                failed_count += 1;
                continue;
            }
        };

        // Проверяем, не существует ли уже такая зависимость
        let existing_deps = db.get_file_dependencies(file_id).await.unwrap_or_default();
        let already_exists = existing_deps.iter().any(|d| {
            d.target_file_name == target_file_name
                && d.target_file_version.as_deref() == target_file_version.as_deref()
        });

        if already_exists {
            warnings.push(format!(
                "Dependency {}@{} already exists",
                target_file_name,
                target_file_version.as_deref().unwrap_or("any")
            ));
            results.push(FileOperationResult {
                file_id,
                success: true,
                error: None,
            });
            success_count += 1;
            continue;
        }

        // Добавляем зависимость
        match db
            .add_file_dependency(
                file_id,
                &target_file_name,
                target_file_version.as_deref(),
                dependency_type,
            )
            .await
        {
            Ok(_) => {
                results.push(FileOperationResult {
                    file_id,
                    success: true,
                    error: None,
                });
                success_count += 1;
            }
            Err(e) => {
                error!("Failed to add dependency: {}", e);
                results.push(FileOperationResult {
                    file_id,
                    success: false,
                    error: Some(format!("Database error: {}", e)),
                });
                failed_count += 1;
            }
        }
    }

    Ok(BatchOperationResult {
        success_count,
        failed_count,
        results,
        warnings,
        blocked_files: Vec::new(),
    })
}

/// Проверить зависимости для нескольких файлов
#[tauri::command]
pub async fn batch_check_dependencies(
    file_ids: Vec<i64>,
) -> Result<BatchDependencyCheckResult, String> {
    info!("Checking dependencies for {} files", file_ids.len());

    if file_ids.is_empty() {
        return Err("No files specified".to_string());
    }

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let total_files = file_ids.len();
    let mut results = Vec::new();
    let mut files_with_missing = 0;
    let mut files_with_conflicts = 0;
    let mut total_missing = 0;

    for file_id in file_ids {
        let file = match db.get_file(file_id).await {
            Ok(Some(f)) => f,
            Ok(None) => continue,
            Err(_) => continue,
        };

        let dependencies = db.get_file_dependencies(file_id).await.unwrap_or_default();
        let mut missing_deps = Vec::new();
        let mut version_conflicts = Vec::new();
        let mut satisfied_count = 0;

        for dep in &dependencies {
            let target_file = if let Some(version) = &dep.target_file_version {
                db.get_file_by_name_version(&dep.target_file_name, version)
                    .await
                    .ok()
                    .flatten()
            } else {
                // Если версия не указана, берем любую версию
                db.get_file_versions(&dep.target_file_name)
                    .await
                    .ok()
                    .and_then(|files| files.first().cloned())
            };

            if let Some(_target_file) = target_file {
                satisfied_count += 1;
            } else {
                // Проверяем, есть ли файл с другим именем/версией
                let any_version = db
                    .get_file_versions(&dep.target_file_name)
                    .await
                    .ok()
                    .and_then(|files| files.first().cloned());

                if let Some(found_file) = any_version {
                    version_conflicts.push(VersionConflict {
                        target_file_name: dep.target_file_name.clone(),
                        required_version: dep.target_file_version.clone(),
                        found_version: found_file.version,
                    });
                } else {
                    missing_deps.push(MissingDependency {
                        target_file_name: dep.target_file_name.clone(),
                        target_file_version: dep.target_file_version.clone(),
                        dependency_type: dep.dependency_type.as_str().to_string(),
                    });
                }
            }
        }

        if !missing_deps.is_empty() {
            files_with_missing += 1;
            total_missing += missing_deps.len();
        }
        if !version_conflicts.is_empty() {
            files_with_conflicts += 1;
        }

        results.push(FileDependencyCheck {
            file_id,
            file_name: format!("{}@{}", file.name, file.version),
            missing_dependencies: missing_deps,
            satisfied_dependencies_count: satisfied_count,
            version_conflicts,
        });
    }

    Ok(BatchDependencyCheckResult {
        results,
        summary: DependencyCheckSummary {
            total_files,
            files_with_missing_deps: files_with_missing,
            files_with_conflicts,
            total_missing_deps: total_missing,
        },
    })
}

/// Экспортировать несколько файлов в один JSON
#[tauri::command]
pub async fn batch_export_files(file_ids: Vec<i64>) -> Result<String, String> {
    info!("Exporting {} files", file_ids.len());

    if file_ids.is_empty() {
        return Err("No files specified".to_string());
    }

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let mut exported_files = Vec::new();

    for file_id in file_ids {
        let file = db.get_file(file_id).await.map_err(|e| {
            error!("Failed to get file {}: {}", file_id, e);
            format!("Database error: {}", e)
        })?;

        let file = file.ok_or_else(|| format!("File {} not found", file_id))?;

        let dependencies = db.get_file_dependencies(file_id).await.unwrap_or_default();

        exported_files.push(serde_json::json!({
            "file": {
                "id": file.id,
                "name": file.name,
                "version": file.version,
                "path": file.path,
                "metadata": file.metadata,
                "created_at": file.created_at.to_rfc3339(),
                "updated_at": file.updated_at.to_rfc3339(),
            },
            "dependencies": dependencies.iter().map(|d| serde_json::json!({
                "target_file_name": d.target_file_name,
                "target_file_version": d.target_file_version,
                "dependency_type": d.dependency_type.as_str().to_string(),
            })).collect::<Vec<_>>(),
        }));
    }

    let export_data = serde_json::json!({
        "version": "1.0",
        "type": "files",
        "data": {
            "files": exported_files,
        },
    });

    serde_json::to_string_pretty(&export_data).map_err(|e| {
        error!("Failed to serialize export data: {}", e);
        format!("Serialization error: {}", e)
    })
}

/// Валидировать пакетную операцию перед выполнением
#[tauri::command]
pub async fn validate_batch_operation(
    operation_type: String,
    file_ids: Vec<i64>,
) -> Result<BatchValidationResult, String> {
    info!(
        "Validating batch operation: {} for {} files",
        operation_type,
        file_ids.len()
    );

    if file_ids.is_empty() {
        return Ok(BatchValidationResult {
            valid: false,
            can_proceed: false,
            warnings: Vec::new(),
            errors: vec!["No files specified".to_string()],
            affected_dependencies: Vec::new(),
        });
    }

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    #[allow(unused_mut)]
    let mut warnings = Vec::new();
    let mut errors = Vec::new();
    let mut affected_dependencies = Vec::new();

    // Проверяем существование всех файлов
    for file_id in &file_ids {
        let file = db.get_file(*file_id).await.map_err(|e| {
            error!("Failed to get file {}: {}", file_id, e);
            format!("Database error: {}", e)
        })?;

        if file.is_none() {
            errors.push(format!("File {} not found", file_id));
        }
    }

    // Для операций удаления проверяем зависимости
    if operation_type == "delete" {
        for file_id in &file_ids {
            let file = match db.get_file(*file_id).await {
                Ok(Some(f)) => f,
                _ => continue,
            };
            let dependent_files = db.get_dependent_files(&file).await.unwrap_or_default();

            if !dependent_files.is_empty() {
                affected_dependencies.push(AffectedDependency {
                    file_id: *file_id,
                    dependent_files: dependent_files.iter().map(|f| f.id).collect(),
                });
                warnings.push(format!(
                    "File {} has {} dependent file(s)",
                    file_id,
                    dependent_files.len()
                ));
            }
        }
    }

    let valid = errors.is_empty();
    let can_proceed = valid && (operation_type != "delete" || affected_dependencies.is_empty());

    Ok(BatchValidationResult {
        valid,
        can_proceed,
        warnings,
        errors,
        affected_dependencies,
    })
}
