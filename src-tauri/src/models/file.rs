use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Идентификатор файла в формате name@version
#[allow(dead_code)]
pub type FileId = String;

/// Структура данных для файла/мода
///
/// Представляет файл в системе с идентификацией через name@version.
/// Множественные версии одного файла могут сосуществовать.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub path: Option<String>,
    #[serde(default)]
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl File {
    /// Получить идентификатор файла в формате name@version
    #[allow(dead_code)]
    pub fn file_id(&self) -> FileId {
        format!("{}@{}", self.name, self.version)
    }

    /// Создать FileId из имени и версии
    #[allow(dead_code)]
    pub fn create_file_id(name: &str, version: &str) -> FileId {
        format!("{}@{}", name, version)
    }

    /// Парсить FileId на имя и версию
    #[allow(dead_code)]
    pub fn parse_file_id(file_id: &str) -> Option<(String, String)> {
        file_id
            .split_once('@')
            .map(|(name, version)| (name.to_string(), version.to_string()))
    }
}
