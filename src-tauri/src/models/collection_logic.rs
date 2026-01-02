use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Тип условия для логики коллекции
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    /// Простое boolean условие (включено/выключено)
    Boolean,
    /// Проверка активности другой коллекции
    CollectionCheck,
    /// Проверка установки файла
    FileCheck,
    /// Логическое И (комбинация условий)
    And,
    /// Логическое ИЛИ (комбинация условий)
    Or,
}

impl ConditionType {
    /// Преобразовать в строку для БД
    pub fn as_str(&self) -> &'static str {
        match self {
            ConditionType::Boolean => "boolean",
            ConditionType::CollectionCheck => "collection_check",
            ConditionType::FileCheck => "file_check",
            ConditionType::And => "and",
            ConditionType::Or => "or",
        }
    }

    /// Создать из строки БД
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "boolean" => Some(ConditionType::Boolean),
            "collection_check" => Some(ConditionType::CollectionCheck),
            "file_check" => Some(ConditionType::FileCheck),
            "and" => Some(ConditionType::And),
            "or" => Some(ConditionType::Or),
            _ => None,
        }
    }
}

/// Действие для правила логики
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    /// Включить файл
    Enable,
    /// Выключить файл
    Disable,
}

impl Action {
    /// Преобразовать в строку для БД
    pub fn as_str(&self) -> &'static str {
        match self {
            Action::Enable => "enable",
            Action::Disable => "disable",
        }
    }

    /// Создать из строки БД
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "enable" => Some(Action::Enable),
            "disable" => Some(Action::Disable),
            _ => None,
        }
    }
}

/// Структура данных для правила логики коллекции
///
/// Представляет условие для включения/выключения файла в коллекции.
/// Правила настраиваются через UI-конструктор с предопределенными условиями.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionLogicRule {
    pub id: i64,
    pub collection_id: i64,
    pub name: String,
    pub condition_type: ConditionType,
    /// Параметры условия в формате JSON
    /// Для boolean: {"value": true/false}
    /// Для collection_check: {"collection_id": 123}
    /// Для file_check: {"file_name": "mod", "file_version": "1.0"}
    /// Для and/or: {"rules": [rule_id1, rule_id2]}
    pub condition_params: serde_json::Value,
    pub action: Action,
    pub created_at: DateTime<Utc>,
}












