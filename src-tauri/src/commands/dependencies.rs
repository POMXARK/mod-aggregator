use crate::database::Database;
use crate::models::{DependencyType, File, FileDependency};
use crate::services::dependency_service::{DependencyGraph, DependencyValidator, VersionResolver};
use log::{error, info, warn};

/// Получить все зависимости файла
#[tauri::command]
pub async fn get_file_dependencies(file_id: i64) -> Result<Vec<FileDependency>, String> {
    info!("Getting dependencies for file_id: {}", file_id);

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

    db.get_file_dependencies(file_id).await.map_err(|e| {
        error!("Failed to get dependencies: {}", e);
        format!("Database error: {}", e)
    })
}

/// Параметры для добавления зависимости
#[derive(serde::Deserialize)]
pub struct AddDependencyParams {
    pub source_file_id: i64,
    pub target_file_name: String,
    pub target_file_version: Option<String>,
    pub dependency_type: String, // "required" | "optional" | "peer"
}

/// Добавить зависимость к файлу
#[tauri::command]
pub async fn add_file_dependency(params: AddDependencyParams) -> Result<FileDependency, String> {
    info!(
        "Adding dependency: source_file_id={}, target={}@{:?}, type={}",
        params.source_file_id,
        params.target_file_name,
        params.target_file_version,
        params.dependency_type
    );

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Получаем исходный файл
    let source_file = db
        .get_file(params.source_file_id)
        .await
        .map_err(|e| {
            error!("Failed to get source file: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Source file not found: {}", params.source_file_id);
            "Source file not found".to_string()
        })?;

    // Парсим тип зависимости
    let dependency_type = match params.dependency_type.as_str() {
        "required" => DependencyType::Required,
        "optional" => DependencyType::Optional,
        "peer" => DependencyType::Peer,
        _ => {
            error!("Invalid dependency type: {}", params.dependency_type);
            return Err(format!(
                "Invalid dependency type: {}",
                params.dependency_type
            ));
        }
    };

    // Если указана версия, проверяем существование целевого файла
    if let Some(ref version) = params.target_file_version {
        let target_file = db
            .get_file_by_name_version(&params.target_file_name, version)
            .await
            .map_err(|e| {
                error!("Failed to check target file: {}", e);
                format!("Database error: {}", e)
            })?;

        if target_file.is_none() {
            return Err(format!(
                "Target version not found: {}@{}",
                params.target_file_name, version
            ));
        }
    }

    // Строим граф зависимостей для валидации
    let mut graph = DependencyGraph::new();

    // Загружаем все файлы
    let all_files = db.get_file_versions("").await.unwrap_or_default();

    for file in &all_files {
        graph.add_file(file.clone());
    }

    // Загружаем существующие зависимости
    for file in &all_files {
        let deps = db.get_file_dependencies(file.id).await.unwrap_or_default();
        for dep in deps {
            graph.add_dependency(dep);
        }
    }

    // Добавляем исходный файл в граф, если его там нет
    if !graph.get_dependencies(source_file.id).iter().any(|_| true) {
        graph.add_file(source_file.clone());
    }

    // Валидация через валидатор
    let validator = DependencyValidator::new(graph);

    match validator.validate_dependency(
        &source_file,
        &params.target_file_name,
        params.target_file_version.as_deref(),
    ) {
        Ok(()) => {
            // Добавляем зависимость
            db.add_file_dependency(
                params.source_file_id,
                &params.target_file_name,
                params.target_file_version.as_deref(),
                dependency_type,
            )
            .await
            .map_err(|e| {
                error!("Failed to add dependency: {}", e);
                format!("Database error: {}", e)
            })
        }
        Err(e) => {
            error!("Dependency validation failed: {}", e);
            Err(e)
        }
    }
}

/// Удалить зависимость
#[tauri::command]
pub async fn remove_file_dependency(dependency_id: i64) -> Result<(), String> {
    info!("Removing dependency: {}", dependency_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    db.remove_file_dependency(dependency_id).await.map_err(|e| {
        error!("Failed to remove dependency: {}", e);
        format!("Database error: {}", e)
    })
}

/// Результат проверки зависимостей
#[derive(serde::Serialize)]
pub struct DependencyCheckResult {
    pub file_id: i64,
    pub missing_dependencies: Vec<MissingDependency>,
    pub satisfied_dependencies: Vec<i64>,
    pub version_conflicts: Vec<VersionConflict>,
}

#[derive(serde::Serialize)]
pub struct MissingDependency {
    pub dependency_id: i64,
    pub target_file_name: String,
    pub target_file_version: Option<String>,
    pub dependency_type: String,
    pub available_versions: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct VersionConflict {
    pub dependency_id: i64,
    pub target_file_name: String,
    pub required_version: Option<String>,
    pub available_version: String,
}

/// Проверить состояние зависимостей файла
#[tauri::command]
pub async fn check_dependencies(file_id: i64) -> Result<DependencyCheckResult, String> {
    info!("Checking dependencies for file_id: {}", file_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let _file = db
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

    let dependencies = db.get_file_dependencies(file_id).await.map_err(|e| {
        error!("Failed to get dependencies: {}", e);
        format!("Database error: {}", e)
    })?;

    let all_files = db.get_file_versions("").await.unwrap_or_default();
    // VersionResolver uses static methods

    let mut missing = Vec::new();
    let mut satisfied = Vec::new();
    let mut conflicts = Vec::new();

    for dep in &dependencies {
        let satisfying = VersionResolver::find_satisfying_files(&all_files, dep);

        if satisfying.is_empty() {
            // Зависимость отсутствует
            let available =
                VersionResolver::get_available_versions(&all_files, &dep.target_file_name);
            missing.push(MissingDependency {
                dependency_id: dep.id,
                target_file_name: dep.target_file_name.clone(),
                target_file_version: dep.target_file_version.clone(),
                dependency_type: dep.dependency_type.as_str().to_string(),
                available_versions: available,
            });
        } else {
            // Проверяем версию, если указана
            if let Some(ref required_version) = dep.target_file_version {
                let has_exact = satisfying.iter().any(|f| f.version == *required_version);
                if !has_exact {
                    // Конфликт версий
                    if let Some(available_file) = satisfying.first() {
                        conflicts.push(VersionConflict {
                            dependency_id: dep.id,
                            target_file_name: dep.target_file_name.clone(),
                            required_version: Some(required_version.clone()),
                            available_version: available_file.version.clone(),
                        });
                    }
                } else {
                    satisfied.push(dep.id);
                }
            } else {
                satisfied.push(dep.id);
            }
        }
    }

    Ok(DependencyCheckResult {
        file_id,
        missing_dependencies: missing,
        satisfied_dependencies: satisfied,
        version_conflicts: conflicts,
    })
}

/// Проверить наличие циклических зависимостей
#[tauri::command]
pub async fn check_circular_dependencies() -> Result<Vec<String>, String> {
    info!("Checking for circular dependencies");

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Загружаем все файлы и зависимости
    let all_files = db.get_file_versions("").await.unwrap_or_default();
    let mut graph = DependencyGraph::new();

    for file in &all_files {
        graph.add_file(file.clone());
        let deps = db.get_file_dependencies(file.id).await.unwrap_or_default();
        for dep in deps {
            graph.add_dependency(dep);
        }
    }

    // Проверяем каждый файл на наличие циклов
    let mut cycles = Vec::new();
    for file in &all_files {
        // Проверяем циклы через проверку циклических зависимостей
        // Используем упрощенную проверку - если файл имеет зависимости, проверяем их
        let deps = graph.get_dependencies(file.id);
        let mut has_cycle = false;
        for dep in deps {
            // Находим целевой файл
            if let Some(target_file) = all_files.iter().find(|f| {
                f.name == dep.target_file_name
                    && dep
                        .target_file_version
                        .as_ref()
                        .map_or(true, |v| f.version == *v)
            }) {
                // Проверяем, есть ли обратный путь
                if graph.check_circular(target_file.id, &file.name, Some(&file.version)) {
                    has_cycle = true;
                    break;
                }
            }
        }
        if has_cycle {
            cycles.push(format!("{}@{}", file.name, file.version));
        }
    }

    if cycles.is_empty() {
        Ok(Vec::new())
    } else {
        warn!("Found {} circular dependencies", cycles.len());
        Ok(cycles)
    }
}

/// Получить все файлы, зависящие от указанного файла
#[tauri::command]
pub async fn get_dependent_files(file_id: i64) -> Result<Vec<File>, String> {
    info!("Getting dependent files for file_id: {}", file_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

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

    db.get_dependent_files(&file).await.map_err(|e| {
        error!("Failed to get dependent files: {}", e);
        format!("Database error: {}", e)
    })
}

/// Разрешить версию зависимости
#[tauri::command]
pub async fn resolve_dependency_version(
    file_name: String,
    version: Option<String>,
) -> Result<Vec<String>, String> {
    info!("Resolving dependency version: {}@{:?}", file_name, version);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let versions = db.get_file_versions(&file_name).await.map_err(|e| {
        error!("Failed to get file versions: {}", e);
        format!("Database error: {}", e)
    })?;

    Ok(versions.into_iter().map(|f| f.version).collect())
}

/// Узел графа зависимостей
#[derive(serde::Serialize)]
pub struct GraphNode {
    pub file_id: i64,
    pub name: String,
    pub version: String,
    pub has_missing_dependencies: bool,
}

/// Ребро графа зависимостей
#[derive(serde::Serialize)]
pub struct GraphEdge {
    pub from_file_id: i64,
    pub to_file_id: i64,
    pub dependency_id: i64,
    pub dependency_type: String,
    pub satisfied: bool,
}

/// Граф зависимостей
#[derive(serde::Serialize)]
pub struct DependencyGraphResult {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// Получить граф зависимостей
#[tauri::command]
pub async fn get_dependency_graph() -> Result<DependencyGraphResult, String> {
    info!("Getting dependency graph");

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    let all_files = db.get_file_versions("").await.unwrap_or_default();
    // VersionResolver uses static methods

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for file in &all_files {
        let deps = db.get_file_dependencies(file.id).await.unwrap_or_default();
        let has_missing = deps
            .iter()
            .any(|dep| !VersionResolver::is_dependency_satisfied(&all_files, dep));

        nodes.push(GraphNode {
            file_id: file.id,
            name: file.name.clone(),
            version: file.version.clone(),
            has_missing_dependencies: has_missing,
        });

        for dep in deps {
            // Находим целевой файл
            if let Some(target_file) =
                VersionResolver::find_satisfying_files(&all_files, &dep).first()
            {
                let satisfied = VersionResolver::satisfies_dependency(target_file, &dep);
                edges.push(GraphEdge {
                    from_file_id: file.id,
                    to_file_id: target_file.id,
                    dependency_id: dep.id,
                    dependency_type: dep.dependency_type.as_str().to_string(),
                    satisfied,
                });
            }
        }
    }

    Ok(DependencyGraphResult { nodes, edges })
}

/// Получить порядок установки файлов
#[tauri::command]
pub async fn get_installation_order(file_ids: Vec<i64>) -> Result<Vec<i64>, String> {
    info!("Getting installation order for {} files", file_ids.len());

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Загружаем все файлы и зависимости
    let all_files = db.get_file_versions("").await.unwrap_or_default();
    let mut graph = DependencyGraph::new();

    for file in &all_files {
        graph.add_file(file.clone());
        let deps = db.get_file_dependencies(file.id).await.unwrap_or_default();
        for dep in deps {
            graph.add_dependency(dep);
        }
    }

    graph.get_installation_order(&file_ids).map_err(|e| {
        error!("Failed to get installation order: {}", e);
        e
    })
}
