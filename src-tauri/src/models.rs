use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub parser_config: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModUpdate {
    pub mod_id: i64,
    pub site_id: i64,
    pub old_version: Option<String>,
    pub new_version: Option<String>,
    pub changes: Option<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserNode {
    pub id: String,
    pub node_type: String, // "selector", "extract", "filter", "transform"
    pub config: serde_json::Value,
    pub position: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    pub nodes: Vec<ParserNode>,
    pub connections: Vec<(String, String)>, // (from_id, to_id)
}

