//! Модели данных для команд управления коллекциями
//!
//! Этот модуль содержит все структуры данных, используемые в командах коллекций.

use crate::models::collection::Collection;
use crate::models::collection_logic::{CollectionLogicRule, ConditionType, Action};
use crate::models::file::File;

/// Структура файла в коллекции для ответа API
#[derive(serde::Serialize, Debug)]
pub struct CollectionFile {
    pub id: i64,
    pub collection_id: i64,
    pub file_id: i64,
    pub file: File,
    pub logic_rule_id: Option<i64>,
    pub logic_rule: Option<CollectionLogicRule>,
    pub order_index: i64,
    pub created_at: String,
}

/// Структура для просмотра файла из нескольких коллекций
#[derive(serde::Serialize, Debug)]
pub struct CollectionFileView {
    pub file_id: i64,
    pub file: File,
    pub collections: Vec<CollectionFileInfo>,
}

/// Информация о файле в коллекции
#[derive(serde::Serialize, Debug)]
pub struct CollectionFileInfo {
    pub collection_id: i64,
    pub collection_name: String,
    pub order_index: i64,
    pub logic_rule_id: Option<i64>,
}

/// Результат оценки логики коллекции
#[derive(serde::Serialize, Debug)]
pub struct CollectionEvaluationResult {
    pub collection_id: i64,
    pub enabled_files: Vec<i64>,
    pub disabled_files: Vec<i64>,
    pub evaluation_details: Vec<FileEvaluationDetail>,
}

/// Детали оценки файла
#[derive(serde::Serialize, Debug)]
pub struct FileEvaluationDetail {
    pub file_id: i64,
    pub enabled: bool,
    pub applied_rules: Vec<i64>,
}

/// Параметры для создания коллекции
#[derive(serde::Deserialize)]
pub struct CreateCollectionParams {
    pub name: String,
    pub description: Option<String>,
}

/// Параметры для обновления коллекции
#[derive(serde::Deserialize)]
pub struct UpdateCollectionParams {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Параметры для добавления файла в коллекцию
#[derive(serde::Deserialize)]
pub struct AddFileToCollectionParams {
    pub collection_id: i64,
    pub file_id: i64,
    pub logic_rule_id: Option<i64>,
    pub order_index: Option<i64>,
}

/// Параметры для изменения порядка файлов в коллекции
#[derive(serde::Deserialize)]
pub struct ReorderFilesParams {
    pub collection_id: i64,
    pub file_orders: Vec<FileOrder>,
}

/// Структура для порядка файла
#[derive(serde::Deserialize)]
pub struct FileOrder {
    pub file_id: i64,
    pub order_index: i64,
}

/// Параметры для создания правила логики
#[derive(serde::Deserialize)]
pub struct CreateLogicRuleParams {
    pub collection_id: i64,
    pub name: String,
    pub condition_type: String, // "boolean" | "collection_check" | "file_check" | "and" | "or"
    pub condition_params: serde_json::Value,
    pub action: String, // "enable" | "disable"
}

/// Параметры для обновления правила логики
#[derive(serde::Deserialize)]
pub struct UpdateLogicRuleParams {
    pub id: i64,
    pub name: Option<String>,
    pub condition_type: Option<String>,
    pub condition_params: Option<serde_json::Value>,
    pub action: Option<String>,
}

/// Параметры для объединения коллекций
#[derive(serde::Deserialize)]
pub struct CombineCollectionsParams {
    pub name: String,
    pub description: Option<String>,
    pub source_collection_ids: Vec<i64>,
    pub selected_file_ids: Option<Vec<i64>>,
}

/// Вспомогательные функции для работы с типами
pub mod types {
    use crate::models::collection_logic::{ConditionType, Action};

    /// Преобразовать строку в ConditionType
    pub fn parse_condition_type(s: &str) -> Result<ConditionType, String> {
        match s {
            "boolean" => Ok(ConditionType::Boolean),
            "collection_check" => Ok(ConditionType::CollectionCheck),
            "file_check" => Ok(ConditionType::FileCheck),
            "and" => Ok(ConditionType::And),
            "or" => Ok(ConditionType::Or),
            _ => Err(format!("Unknown condition type: {}", s)),
        }
    }

    /// Преобразовать строку в Action
    pub fn parse_action(s: &str) -> Result<Action, String> {
        match s {
            "enable" => Ok(Action::Enable),
            "disable" => Ok(Action::Disable),
            _ => Err(format!("Unknown action: {}", s)),
        }
    }
}