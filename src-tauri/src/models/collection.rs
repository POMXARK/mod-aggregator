use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Структура данных для коллекции файлов
///
/// Коллекция представляет группу файлов с опциональной логикой включения/выключения.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}












