use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Действие пользователя для истории
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RecentActionType {
    FileAdded,
    FileDeleted,
    CollectionCreated,
    DependencyAdded,
    DependencyRemoved,
    CollectionUpdated,
}

/// Запись о последнем действии пользователя
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentAction {
    pub r#type: RecentActionType,
    pub target_id: i64,
    pub target_name: String,
    pub timestamp: DateTime<Utc>,
}

/// Настройки UI пользователя
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiPreferences {
    pub view_mode: Option<String>, // "list" | "tiles"
    pub window_width: Option<i32>,
    pub window_height: Option<i32>,
    pub sidebar_collapsed: Option<bool>,
    pub theme: Option<String>, // "dark" | "light"
    pub current_page: Option<String>, // текущая страница ("mods", "sites", etc.)
    pub selected_site_id: Option<i64>, // ID выбранного сайта
}

/// Структура данных для состояния сессии
///
/// Сохраняет состояние приложения для восстановления при следующем запуске.
/// Использует singleton pattern (только одна запись с id=1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub id: i64,              // Всегда 1
    pub file_order: Vec<i64>, // file_ids в порядке отображения
    pub ui_preferences: UiPreferences,
    pub open_collections: Vec<i64>,        // collection_ids
    pub selected_files: Vec<i64>,          // file_ids
    pub recent_actions: Vec<RecentAction>, // Максимум 50 записей
    pub current_page: Option<String>,
    pub selected_site_id: Option<i64>,
    pub last_updated: DateTime<Utc>,
}

impl SessionState {
    /// Создать состояние по умолчанию
    pub fn default() -> Self {
        Self {
            id: 1,
            file_order: Vec::new(),
            ui_preferences: UiPreferences::default(),
            open_collections: Vec::new(),
            selected_files: Vec::new(),
            recent_actions: Vec::new(),
            current_page: None,
            selected_site_id: None,
            last_updated: Utc::now(),
        }
    }

    /// Добавить действие в историю с ограничением до 50 записей
    pub fn add_recent_action(&mut self, action: RecentAction) {
        self.recent_actions.insert(0, action);
        // Ограничиваем до 50 записей
        if self.recent_actions.len() > 50 {
            self.recent_actions.truncate(50);
        }
        self.last_updated = Utc::now();
    }
}












