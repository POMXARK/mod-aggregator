// File management commands
// Implementation for User Story 2: Manual File Addition and Editing

use crate::database::Database;
use crate::models::dependency::DependencyType;
use crate::models::file::File;
use crate::services::dependency_service::{DependencyGraph, DependencyValidator};
use log::{error, info, warn};

/// Параметры для создания файла
#[derive(serde::Deserialize)]
pub struct CreateFileParams {
    pub name: String,
    pub version: String,
    pub path: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub dependencies: Option<Vec<CreateDependencyParams>>,
}

/// Параметры для создания зависимости при создании файла
#[derive(serde::Deserialize)]
pub struct CreateDependencyParams {
    pub target_file_name: String,
    pub target_file_version: Option<String>,
    #[serde(default = "default_dependency_type")]
    pub dependency_type: String, // "required" | "optional" | "peer"
}

fn default_dependency_type() -> String {
    "required".to_string()
}

/// Создать файл вручную через форму
///
/// Создает новый файл с указанными параметрами и опциональными зависимостями.
/// Проверяет уникальность name@version и наличие циклических зависимостей.
#[tauri::command]
pub async fn create_file(params: CreateFileParams) -> Result<File, String> {
    info!("Creating file: {}@{}", params.name, params.version);

    // Валидация входных данных
    if params.name.trim().is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    if params.version.trim().is_empty() {
        return Err("Version cannot be empty".to_string());
    }

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем уникальность name@version
    let existing = db
        .get_file_by_name_version(&params.name, &params.version)
        .await
        .map_err(|e| {
            error!("Failed to check file existence: {}", e);
            format!("Database error: {}", e)
        })?;

    if existing.is_some() {
        return Err(format!(
            "File with name@version already exists: {}@{}",
            params.name, params.version
        ));
    }

    // Создаем файл
    let file = db
        .create_file(
            &params.name,
            &params.version,
            params.path.as_deref(),
            params.metadata.as_ref(),
        )
        .await
        .map_err(|e| {
            error!("Failed to create file: {}", e);
            format!("Database error: {}", e)
        })?;

    // Если указаны зависимости, добавляем их
    if let Some(deps) = params.dependencies {
        // Строим граф зависимостей для валидации
        let mut graph = DependencyGraph::new();
        let all_files = db.get_file_versions("").await.unwrap_or_default();

        for f in &all_files {
            graph.add_file(f.clone());
        }
        graph.add_file(file.clone());

        // Загружаем существующие зависимости
        for f in &all_files {
            let existing_deps = db.get_file_dependencies(f.id).await.unwrap_or_default();
            for dep in existing_deps {
                graph.add_dependency(dep);
            }
        }

        let validator = DependencyValidator::new(graph);

        // Добавляем зависимости с валидацией
        for dep_params in deps {
            // Парсим тип зависимости
            let dependency_type = match dep_params.dependency_type.as_str() {
                "required" => DependencyType::Required,
                "optional" => DependencyType::Optional,
                "peer" => DependencyType::Peer,
                _ => {
                    warn!(
                        "Invalid dependency type: {}, defaulting to required",
                        dep_params.dependency_type
                    );
                    DependencyType::Required
                }
            };

            // Если указана версия, проверяем существование целевого файла
            if let Some(ref version) = dep_params.target_file_version {
                let target_file = db
                    .get_file_by_name_version(&dep_params.target_file_name, version)
                    .await
                    .map_err(|e| {
                        error!("Failed to check target file: {}", e);
                        format!("Database error: {}", e)
                    })?;

                if target_file.is_none() {
                    return Err(format!(
                        "Dependency not found: {}@{}",
                        dep_params.target_file_name, version
                    ));
                }
            }

            // Проверяем на циклические зависимости
            if validator.check_circular_dependency(
                file.id,
                &dep_params.target_file_name,
                dep_params.target_file_version.as_deref(),
            ) {
                return Err("Circular dependency detected".to_string());
            }

            // Проверяем на самозависимость
            if validator.check_self_dependency(
                &file,
                &dep_params.target_file_name,
                dep_params.target_file_version.as_deref(),
            ) {
                return Err("File cannot depend on itself".to_string());
            }

            // Добавляем зависимость
            db.add_file_dependency(
                file.id,
                &dep_params.target_file_name,
                dep_params.target_file_version.as_deref(),
                dependency_type,
            )
            .await
            .map_err(|e| {
                error!("Failed to add dependency: {}", e);
                format!("Database error: {}", e)
            })?;
        }
    }

    info!(
        "File created successfully: {}@{} (id: {})",
        file.name, file.version, file.id
    );
    Ok(file)
}

/// Параметры для обновления файла
#[derive(serde::Deserialize)]
pub struct UpdateFileParams {
    pub id: i64,
    pub name: Option<String>,
    pub version: Option<String>,
    pub path: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Обновить параметры файла
///
/// Обновляет указанные поля файла. При изменении name или version
/// проверяется уникальность новой комбинации.
#[tauri::command]
pub async fn update_file(params: UpdateFileParams) -> Result<File, String> {
    info!("Updating file id: {}", params.id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Получаем текущий файл
    let current_file = db
        .get_file(params.id)
        .await
        .map_err(|e| {
            error!("Failed to get file: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("File not found: {}", params.id);
            "File not found".to_string()
        })?;

    // Если изменяется name или version, проверяем уникальность
    if let (Some(ref new_name), Some(ref new_version)) = (&params.name, &params.version) {
        if new_name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if new_version.trim().is_empty() {
            return Err("Version cannot be empty".to_string());
        }

        // Проверяем, не занята ли новая комбинация другим файлом
        let existing = db
            .get_file_by_name_version(new_name, new_version)
            .await
            .map_err(|e| {
                error!("Failed to check file existence: {}", e);
                format!("Database error: {}", e)
            })?;

        if let Some(existing_file) = existing {
            if existing_file.id != params.id {
                return Err(format!(
                    "File with name@version already exists: {}@{}",
                    new_name, new_version
                ));
            }
        }
    } else if let Some(ref new_name) = params.name {
        if new_name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        // Проверяем с текущей версией
        let existing = db
            .get_file_by_name_version(new_name, &current_file.version)
            .await
            .map_err(|e| {
                error!("Failed to check file existence: {}", e);
                format!("Database error: {}", e)
            })?;

        if let Some(existing_file) = existing {
            if existing_file.id != params.id {
                return Err(format!(
                    "File with name@version already exists: {}@{}",
                    new_name, current_file.version
                ));
            }
        }
    } else if let Some(ref new_version) = params.version {
        if new_version.trim().is_empty() {
            return Err("Version cannot be empty".to_string());
        }
        // Проверяем с текущим именем
        let existing = db
            .get_file_by_name_version(&current_file.name, new_version)
            .await
            .map_err(|e| {
                error!("Failed to check file existence: {}", e);
                format!("Database error: {}", e)
            })?;

        if let Some(existing_file) = existing {
            if existing_file.id != params.id {
                return Err(format!(
                    "File with name@version already exists: {}@{}",
                    current_file.name, new_version
                ));
            }
        }
    }

    // Обновляем файл
    db.update_file(
        params.id,
        params.name.as_deref(),
        params.version.as_deref(),
        params.path.as_deref(),
        params.metadata.as_ref(),
    )
    .await
    .map_err(|e| {
        error!("Failed to update file: {}", e);
        format!("Database error: {}", e)
    })?;

    // Получаем обновленный файл
    let updated_file = db
        .get_file(params.id)
        .await
        .map_err(|e| {
            error!("Failed to get updated file: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("File not found after update: {}", params.id);
            "File not found".to_string()
        })?;

    info!(
        "File updated successfully: {}@{} (id: {})",
        updated_file.name, updated_file.version, updated_file.id
    );
    Ok(updated_file)
}

/// Результат удаления файла
#[derive(serde::Serialize)]
pub struct DeleteFileResult {
    pub deleted: bool,
    pub dependent_files: Option<Vec<File>>,
    pub message: String,
}

/// Удалить файл с проверкой зависимостей
///
/// Удаляет файл, если нет зависимых файлов или если force=true.
/// При force=false возвращает список зависимых файлов без удаления.
#[tauri::command]
pub async fn delete_file(file_id: i64, force: Option<bool>) -> Result<DeleteFileResult, String> {
    info!("Deleting file id: {} (force: {:?})", file_id, force);

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

    // Проверяем зависимые файлы
    let dependent_files = db.get_dependent_files(&file).await.map_err(|e| {
        error!("Failed to get dependent files: {}", e);
        format!("Database error: {}", e)
    })?;

    let force_delete = force.unwrap_or(false);

    if !dependent_files.is_empty() && !force_delete {
        let message = format!(
            "Cannot delete file {}@{}: {} file(s) depend on it",
            file.name,
            file.version,
            dependent_files.len()
        );
        info!("{}", message);
        return Ok(DeleteFileResult {
            deleted: false,
            dependent_files: Some(dependent_files),
            message,
        });
    }

    // Удаляем зависимости файла
    let dependencies = db.get_file_dependencies(file_id).await.unwrap_or_default();
    for dep in dependencies {
        db.remove_file_dependency(dep.id).await.map_err(|e| {
            error!("Failed to remove dependency: {}", e);
            format!("Database error: {}", e)
        })?;
    }

    // Удаляем файл
    db.delete_file(file_id).await.map_err(|e| {
        error!("Failed to delete file: {}", e);
        format!("Database error: {}", e)
    })?;

    let message = if !dependent_files.is_empty() {
        format!(
            "File {}@{} deleted (force mode). {} dependent file(s) may be affected.",
            file.name,
            file.version,
            dependent_files.len()
        )
    } else {
        format!("File {}@{} deleted successfully", file.name, file.version)
    };

    info!("{}", message);
    Ok(DeleteFileResult {
        deleted: true,
        dependent_files: if force_delete && !dependent_files.is_empty() {
            Some(dependent_files)
        } else {
            None
        },
        message,
    })
}

/// Параметры для загрузки новой версии файла
#[derive(serde::Deserialize)]
pub struct UploadFileVersionParams {
    pub name: String,
    pub version: String,
    pub path: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Загрузить новую версию файла вручную
///
/// Создает новый файл с тем же именем, но другой версией.
/// Зависимости не копируются автоматически.
#[tauri::command]
pub async fn upload_file_version(params: UploadFileVersionParams) -> Result<File, String> {
    info!("Uploading new version: {}@{}", params.name, params.version);

    // Валидация
    if params.name.trim().is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    if params.version.trim().is_empty() {
        return Err("Version cannot be empty".to_string());
    }

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем уникальность name@version
    let existing = db
        .get_file_by_name_version(&params.name, &params.version)
        .await
        .map_err(|e| {
            error!("Failed to check file existence: {}", e);
            format!("Database error: {}", e)
        })?;

    if existing.is_some() {
        return Err(format!(
            "File with name@version already exists: {}@{}",
            params.name, params.version
        ));
    }

    // Создаем новую версию
    let file = db
        .create_file(
            &params.name,
            &params.version,
            params.path.as_deref(),
            params.metadata.as_ref(),
        )
        .await
        .map_err(|e| {
            error!("Failed to create file version: {}", e);
            format!("Database error: {}", e)
        })?;

    info!(
        "File version uploaded successfully: {}@{} (id: {})",
        file.name, file.version, file.id
    );
    Ok(file)
}

/// Получить файл по name@version
#[tauri::command]
pub async fn get_file_by_name_version(name: String, version: String) -> Result<File, String> {
    info!("Getting file: {}@{}", name, version);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let file = db
        .get_file_by_name_version(&name, &version)
        .await
        .map_err(|e| {
            error!("Failed to get file: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("File not found: {}@{}", name, version);
            format!("File not found: {}@{}", name, version)
        })?;

    Ok(file)
}

/// Получить все версии файла с указанным именем
///
/// Если name пустой, возвращает все файлы.
#[tauri::command]
pub async fn get_file_versions(name: String) -> Result<Vec<File>, String> {
    info!(
        "Getting file versions for: {}",
        if name.is_empty() { "all files" } else { &name }
    );

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let files = db.get_file_versions(&name).await.map_err(|e| {
        error!("Failed to get file versions: {}", e);
        format!("Database error: {}", e)
    })?;

    info!("Found {} file(s)", files.len());
    Ok(files)
}

/// Получить все файлы
#[tauri::command]
pub async fn get_all_files() -> Result<Vec<File>, String> {
    info!("Getting all files");

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Получаем все файлы через get_file_versions с пустой строкой
    // или создаем отдельный метод в Database
    let all_files = db.get_file_versions("").await.map_err(|e| {
        error!("Failed to get files: {}", e);
        format!("Database error: {}", e)
    })?;

    Ok(all_files)
}
