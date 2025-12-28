use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Тип зависимости между файлами
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DependencyType {
    /// Обязательная зависимость - файл не может работать без неё
    Required,
    /// Опциональная зависимость - файл может работать без неё
    Optional,
    /// Peer зависимость - зависимость на уровне (как в npm peer dependencies)
    Peer,
}

impl DependencyType {
    /// Преобразовать в строку для БД
    pub fn as_str(&self) -> &'static str {
        match self {
            DependencyType::Required => "required",
            DependencyType::Optional => "optional",
            DependencyType::Peer => "peer",
        }
    }

    /// Создать из строки БД
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "required" => Some(DependencyType::Required),
            "optional" => Some(DependencyType::Optional),
            "peer" => Some(DependencyType::Peer),
            _ => None,
        }
    }
}

/// Структура данных для зависимости файла
///
/// Представляет зависимость одного файла от другого.
/// Зависимость идентифицируется по имени и опционально версии.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDependency {
    pub id: i64,
    pub source_file_id: i64,
    pub target_file_name: String,
    pub target_file_version: Option<String>,
    pub dependency_type: DependencyType,
    pub created_at: DateTime<Utc>,
}

impl FileDependency {
    /// Получить идентификатор целевого файла (если версия указана)
    #[allow(dead_code)]
    pub fn target_file_id(&self) -> Option<String> {
        self.target_file_version
            .as_ref()
            .map(|v| format!("{}@{}", self.target_file_name, v))
    }

    /// Проверить, соответствует ли файл этой зависимости
    #[allow(dead_code)]
    pub fn matches_file(&self, name: &str, version: &str) -> bool {
        if self.target_file_name != name {
            return false;
        }

        match &self.target_file_version {
            Some(required_version) => required_version == version,
            None => true, // Любая версия подходит
        }
    }
}
