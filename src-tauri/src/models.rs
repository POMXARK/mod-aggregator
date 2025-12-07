use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Структура данных для сайта
/// 
/// Представляет сайт с его конфигурацией парсера и метаданными
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub parser_config: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Структура данных для мода
/// 
/// Представляет мод с его метаданными (название, версия, автор и т.д.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mod {
    pub id: i64,
    pub site_id: i64,
    pub title: String,
    pub url: String,
    pub version: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub changes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Структура данных для обновления мода
/// 
/// Используется для отслеживания изменений версий модов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModUpdate {
    pub mod_id: i64,
    pub site_id: i64,
    pub old_version: Option<String>,
    pub new_version: Option<String>,
    pub changes: Option<String>,
}

/// Структура данных для уведомления
/// 
/// Представляет уведомление о обновлении мода или других событиях
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: i64,
    pub mod_id: i64,
    pub site_id: i64,
    pub title: String,
    pub message: String,
    pub read: bool,
    pub created_at: DateTime<Utc>,
}

/// Структура данных для узла парсера
/// 
/// Представляет узел в графе парсера (selector, extract, filter, transform, output)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserNode {
    /// Уникальный идентификатор узла
    pub id: String,
    /// Тип узла: "selector", "extract", "filter", "transform", "output"
    pub node_type: String,
    /// Конфигурация узла в формате JSON
    pub config: serde_json::Value,
    /// Позиция узла на графе (x, y)
    pub position: (f64, f64),
}

/// Структура данных для конфигурации парсера
/// 
/// Представляет полную конфигурацию парсера с узлами и связями между ними
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    /// Список узлов парсера
    pub nodes: Vec<ParserNode>,
    /// Список связей между узлами (from_id, to_id)
    pub connections: Vec<(String, String)>,
}

