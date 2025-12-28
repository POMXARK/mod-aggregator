// Import/Export service
// Handles serialization and deserialization of files, collections, and builds

use crate::models::dependency::FileDependency;
use crate::models::file::File;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Версия схемы импорта/экспорта
pub const SCHEMA_VERSION: &str = "1.0";

/// Тип экспортируемых данных
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportType {
    File,
    Collection,
    Build,
}

/// Экспортируемый файл
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedFile {
    pub file: File,
    pub dependencies: Vec<ExportedDependency>,
}

/// Экспортируемая зависимость
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedDependency {
    pub target_file_name: String,
    pub target_file_version: Option<String>,
    pub dependency_type: String, // "required" | "optional" | "peer"
}

/// Структура экспорта файла
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileExport {
    pub version: String,
    #[serde(rename = "type")]
    pub export_type: ExportType,
    pub data: ExportedFile,
}

/// Экспортируемый файл в коллекции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedCollectionFile {
    pub file_id: i64,
    pub name: String,
    pub version: String,
    pub order_index: i64,
    pub logic_rule: Option<ExportedLogicRule>,
}

/// Экспортируемое правило логики
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedLogicRule {
    pub name: String,
    pub condition_type: String,
    pub condition_params: Value,
    pub action: String,
}

/// Данные экспорта коллекции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionExportData {
    pub collection: crate::models::collection::Collection,
    pub files: Vec<ExportedCollectionFile>,
    pub logic_rules: Vec<ExportedLogicRule>,
}

/// Структура экспорта коллекции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionExport {
    pub version: String,
    #[serde(rename = "type")]
    pub export_type: ExportType,
    pub data: CollectionExportData,
}

/// Экспортируемая коллекция в сборке
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedBuildCollection {
    pub collection_id: i64,
    pub collection_name: String,
}

/// Экспортируемый файл в сборке
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedBuildFile {
    pub file_id: i64,
    pub name: String,
    pub version: String,
}

/// Данные экспорта сборки
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildExportData {
    pub build_name: String,
    pub collections: Vec<ExportedBuildCollection>,
    pub files: Vec<ExportedBuildFile>,
}

/// Структура экспорта сборки
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildExport {
    pub version: String,
    #[serde(rename = "type")]
    pub export_type: ExportType,
    pub data: BuildExportData,
}

/// Результат импорта
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub file_id: Option<i64>,
    pub collection_id: Option<i64>,
    pub created: bool,
    pub missing_dependencies: Vec<MissingDependency>,
    pub missing_files: Vec<MissingFile>,
    pub warnings: Vec<String>,
}

/// Отсутствующая зависимость
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingDependency {
    pub target_file_name: String,
    pub target_file_version: Option<String>,
    pub available_versions: Vec<String>,
}

/// Отсутствующий файл
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingFile {
    pub file_name: String,
    pub file_version: String,
}

/// Результат валидации импорта
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportValidationResult {
    pub valid: bool,
    #[serde(rename = "type")]
    pub import_type: Option<String>,
    pub version: Option<String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub preview: Option<ImportPreview>,
}

/// Превью импорта
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportPreview {
    pub files_count: usize,
    pub dependencies_count: usize,
    pub collections_count: usize,
    pub missing_dependencies: Vec<MissingDependency>,
}

/// Сервис импорта/экспорта
pub struct ImportExportService;

impl ImportExportService {
    /// Экспортировать файл в JSON
    pub fn export_file(file: &File, dependencies: Vec<FileDependency>) -> Result<String, String> {
        let exported_deps: Vec<ExportedDependency> = dependencies
            .iter()
            .map(|dep| ExportedDependency {
                target_file_name: dep.target_file_name.clone(),
                target_file_version: dep.target_file_version.clone(),
                dependency_type: dep.dependency_type.as_str().to_string(),
            })
            .collect();

        let export = FileExport {
            version: SCHEMA_VERSION.to_string(),
            export_type: ExportType::File,
            data: ExportedFile {
                file: file.clone(),
                dependencies: exported_deps,
            },
        };

        serde_json::to_string_pretty(&export)
            .map_err(|e| format!("Failed to serialize file export: {}", e))
    }

    /// Импортировать файл из JSON
    pub fn import_file(json_data: &str) -> Result<FileImportData, String> {
        let export: FileExport =
            serde_json::from_str(json_data).map_err(|e| format!("Invalid JSON format: {}", e))?;

        // Проверяем версию схемы
        if export.version != SCHEMA_VERSION {
            return Err(format!(
                "Invalid schema version: expected {}, got {}",
                SCHEMA_VERSION, export.version
            ));
        }

        // Проверяем тип
        if !matches!(export.export_type, ExportType::File) {
            return Err(format!(
                "Invalid type: expected file, got {:?}",
                export.export_type
            ));
        }

        // Валидация данных
        if export.data.file.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if export.data.file.version.trim().is_empty() {
            return Err("Version cannot be empty".to_string());
        }

        Ok(FileImportData {
            file: export.data.file,
            dependencies: export.data.dependencies,
        })
    }

    /// Валидировать JSON перед импортом
    pub fn validate_import_json(json_data: &str) -> ImportValidationResult {
        let mut result = ImportValidationResult {
            valid: false,
            import_type: None,
            version: None,
            errors: Vec::new(),
            warnings: Vec::new(),
            preview: None,
        };

        // Парсим JSON
        let json: Value = match serde_json::from_str(json_data) {
            Ok(v) => v,
            Err(e) => {
                result.errors.push(format!("Invalid JSON format: {}", e));
                return result;
            }
        };

        // Проверяем обязательные поля
        if !json.is_object() {
            result.errors.push("JSON must be an object".to_string());
            return result;
        }

        let version = json
            .get("version")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        result.version = version.clone();

        let export_type = json
            .get("type")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        result.import_type = export_type.clone();

        // Проверяем версию схемы
        if let Some(ref v) = version {
            if v != SCHEMA_VERSION {
                result.warnings.push(format!(
                    "Schema version mismatch: expected {}, got {}",
                    SCHEMA_VERSION, v
                ));
            }
        } else {
            result
                .errors
                .push("Missing required field: version".to_string());
        }

        // Проверяем тип
        if export_type.is_none() {
            result
                .errors
                .push("Missing required field: type".to_string());
        }

        // Проверяем data
        if !json.get("data").is_some() {
            result
                .errors
                .push("Missing required field: data".to_string());
        } else if let Some(data) = json.get("data") {
            // Создаем превью в зависимости от типа
            if let Some(ref t) = export_type {
                match t.as_str() {
                    "file" => {
                        if let Some(_file_data) = data.get("file") {
                            let files_count = 1;
                            let deps = data
                                .get("dependencies")
                                .and_then(|d| d.as_array())
                                .map(|a| a.len())
                                .unwrap_or(0);

                            result.preview = Some(ImportPreview {
                                files_count,
                                dependencies_count: deps,
                                collections_count: 0,
                                missing_dependencies: Vec::new(),
                            });
                        }
                    }
                    "collection" | "build" => {
                        result
                            .warnings
                            .push(format!("Import type '{}' is not yet fully supported", t));
                    }
                    _ => {
                        result.errors.push(format!("Unknown import type: {}", t));
                    }
                }
            }
        }

        // Если нет ошибок, валидация прошла успешно
        result.valid = result.errors.is_empty();

        result
    }

    /// Экспортировать коллекцию в JSON
    pub fn export_collection(
        collection: &crate::models::collection::Collection,
        files: Vec<ExportedCollectionFile>,
        logic_rules: Vec<ExportedLogicRule>,
    ) -> Result<String, String> {
        let export = CollectionExport {
            version: SCHEMA_VERSION.to_string(),
            export_type: ExportType::Collection,
            data: CollectionExportData {
                collection: collection.clone(),
                files,
                logic_rules,
            },
        };

        serde_json::to_string_pretty(&export)
            .map_err(|e| format!("Failed to serialize collection export: {}", e))
    }

    /// Импортировать коллекцию из JSON
    pub fn import_collection(json_data: &str) -> Result<CollectionImportData, String> {
        let export: CollectionExport =
            serde_json::from_str(json_data).map_err(|e| format!("Invalid JSON format: {}", e))?;

        if export.version != SCHEMA_VERSION {
            return Err(format!(
                "Invalid schema version: expected {}, got {}",
                SCHEMA_VERSION, export.version
            ));
        }

        if !matches!(export.export_type, ExportType::Collection) {
            return Err(format!(
                "Invalid type: expected collection, got {:?}",
                export.export_type
            ));
        }

        if export.data.collection.name.trim().is_empty() {
            return Err("Collection name cannot be empty".to_string());
        }

        Ok(CollectionImportData {
            collection: export.data.collection,
            files: export.data.files,
            logic_rules: export.data.logic_rules,
        })
    }

    /// Экспортировать сборку в JSON
    pub fn export_build(
        build_name: String,
        collections: Vec<ExportedBuildCollection>,
        files: Vec<ExportedBuildFile>,
    ) -> Result<String, String> {
        let export = BuildExport {
            version: SCHEMA_VERSION.to_string(),
            export_type: ExportType::Build,
            data: BuildExportData {
                build_name,
                collections,
                files,
            },
        };

        serde_json::to_string_pretty(&export)
            .map_err(|e| format!("Failed to serialize build export: {}", e))
    }

    /// Импортировать сборку из JSON
    pub fn import_build(json_data: &str) -> Result<BuildImportData, String> {
        let export: BuildExport =
            serde_json::from_str(json_data).map_err(|e| format!("Invalid JSON format: {}", e))?;

        if export.version != SCHEMA_VERSION {
            return Err(format!(
                "Invalid schema version: expected {}, got {}",
                SCHEMA_VERSION, export.version
            ));
        }

        if !matches!(export.export_type, ExportType::Build) {
            return Err(format!(
                "Invalid type: expected build, got {:?}",
                export.export_type
            ));
        }

        if export.data.build_name.trim().is_empty() {
            return Err("Build name cannot be empty".to_string());
        }

        Ok(BuildImportData {
            build_name: export.data.build_name,
            collections: export.data.collections,
            files: export.data.files,
        })
    }
}

/// Данные для импорта файла
#[derive(Debug, Clone)]
pub struct FileImportData {
    pub file: File,
    pub dependencies: Vec<ExportedDependency>,
}

/// Данные для импорта коллекции
#[derive(Debug, Clone)]
pub struct CollectionImportData {
    pub collection: crate::models::collection::Collection,
    pub files: Vec<ExportedCollectionFile>,
    #[allow(dead_code)]
    pub logic_rules: Vec<ExportedLogicRule>,
}

/// Данные для импорта сборки
#[derive(Debug, Clone)]
pub struct BuildImportData {
    pub build_name: String,
    pub collections: Vec<ExportedBuildCollection>,
    pub files: Vec<ExportedBuildFile>,
}
