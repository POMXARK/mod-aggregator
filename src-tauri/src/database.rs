//! Основной модуль базы данных
//!
//! Этот файл содержит основную структуру Database с методами для работы с БД.
//! Дополнительные модули с разделением функциональности находятся в modules/.
//!
//! Модули позволяют:
//! - sites: CRUD операции с сайтами
//! - mods: Управление модами
//! - notifications: Работа с уведомлениями
//! - pages: Кеширование страниц
//! - files: Управление файлами
//! - dependencies: Зависимости файлов
//! - collections: Коллекции модов
//! - session: Состояние пользовательской сессии
//!
//! Модули базы данных (новая модульная структура)
pub mod modules;

// Импорты для совместимости с существующими модулями
pub use modules::*;

// Re-export types for backward compatibility
pub use crate::database::modules::base::Database;
pub use crate::models::collection::Collection;
pub use crate::models::collection_logic::{Action, CollectionLogicRule, ConditionType};
pub use crate::models::dependency::{DependencyType, FileDependency};
pub use crate::models::file::File;
pub use crate::models::{Mod, Notification, Site};

// Re-export the Database struct from modules for backward compatibility
pub use crate::database::modules::base::Database as DatabaseStruct;

impl Database {
    // Sites methods
    pub async fn get_sites(&self) -> Result<Vec<Site>, sqlx::Error> {
        sites::get_sites(self).await
    }

    pub async fn get_site(&self, id: i64) -> Result<Site, sqlx::Error> {
        sites::get_site(self, id).await
    }

    pub async fn add_site(
        &self,
        name: &str,
        url: &str,
        parser_config: &serde_json::Value,
    ) -> Result<Site, sqlx::Error> {
        sites::add_site(self, name, url, parser_config).await
    }

    pub async fn update_site(
        &self,
        id: i64,
        name: &str,
        url: &str,
        parser_config: &serde_json::Value,
    ) -> Result<(), sqlx::Error> {
        sites::update_site(self, id, name, url, parser_config).await
    }

    pub async fn delete_site(&self, id: i64) -> Result<(), sqlx::Error> {
        sites::delete_site(self, id).await
    }

    // Mods methods
    pub async fn get_mods(&self, site_id: Option<i64>) -> Result<Vec<Mod>, sqlx::Error> {
        mods::get_mods(self, site_id).await
    }

    pub async fn get_mod_by_url(&self, url: &str) -> Result<Option<Mod>, sqlx::Error> {
        mods::get_mod_by_url(self, url).await
    }

    pub async fn add_mod(&self, mod_item: &Mod) -> Result<Mod, sqlx::Error> {
        mods::add_mod(self, mod_item).await
    }

    pub async fn update_mod(&self, id: i64, mod_item: &Mod) -> Result<(), sqlx::Error> {
        mods::update_mod(self, id, mod_item).await
    }

    // Notifications methods
    pub async fn get_notifications(&self) -> Result<Vec<Notification>, sqlx::Error> {
        notifications::get_notifications(self).await
    }

    pub async fn add_notification(&self, notification: &Notification) -> Result<(), sqlx::Error> {
        notifications::add_notification(self, notification).await
    }

    pub async fn mark_notification_read(&self, id: i64) -> Result<(), sqlx::Error> {
        notifications::mark_notification_read(self, id).await
    }

    // Pages methods
    pub async fn save_page_for_site(
        &self,
        site_id: i64,
        url: &str,
        folder_path: &str,
        version_timestamp: &str,
    ) -> Result<(), sqlx::Error> {
        pages::save_page_for_site(self, site_id, url, folder_path, version_timestamp).await
    }

    pub async fn get_saved_page(
        &self,
        site_id: i64,
        url: &str,
    ) -> Result<Option<(i64, String, String)>, sqlx::Error> {
        pages::get_saved_page(self, site_id, url).await
    }

    pub async fn get_saved_page_versions(
        &self,
        site_id: i64,
        url: &str,
    ) -> Result<Vec<serde_json::Value>, sqlx::Error> {
        pages::get_saved_page_versions(self, site_id, url).await
    }

    pub async fn delete_saved_page_version(&self, page_id: i64) -> Result<(), sqlx::Error> {
        pages::delete_saved_page_version(self, page_id).await
    }

    // Files methods
    pub async fn get_file(&self, id: i64) -> Result<Option<File>, sqlx::Error> {
        files::get_file(self, id).await
    }

    pub async fn get_file_by_name_version(
        &self,
        name: &str,
        version: &str,
    ) -> Result<Option<File>, sqlx::Error> {
        files::get_file_by_name_version(self, name, version).await
    }

    pub async fn get_file_versions(&self, name: &str) -> Result<Vec<File>, sqlx::Error> {
        files::get_file_versions(self, name).await
    }

    pub async fn create_file(
        &self,
        name: &str,
        version: &str,
        path: Option<&str>,
        metadata: Option<&serde_json::Value>,
    ) -> Result<File, sqlx::Error> {
        files::create_file_simple(self, name, version, path, metadata).await
    }

    pub async fn update_file(
        &self,
        id: i64,
        name: Option<&str>,
        version: Option<&str>,
        path: Option<&str>,
        metadata: Option<&serde_json::Value>,
    ) -> Result<(), sqlx::Error> {
        files::update_file_simple(self, id, name, version, path, metadata).await
    }

    pub async fn delete_file(&self, id: i64) -> Result<(), sqlx::Error> {
        files::delete_file(self, id).await
    }

    pub async fn get_all_files(&self) -> Result<Vec<File>, sqlx::Error> {
        files::get_all_files(self).await
    }

    // Dependencies methods
    pub async fn get_file_dependencies(&self, file_id: i64) -> Result<Vec<FileDependency>, sqlx::Error> {
        dependencies::get_file_dependencies(self, file_id).await
    }

    pub async fn add_file_dependency(
        &self,
        source_file_id: i64,
        target_file_name: &str,
        target_file_version: Option<&str>,
        dependency_type: DependencyType,
    ) -> Result<FileDependency, sqlx::Error> {
        dependencies::add_file_dependency_simple(self, source_file_id, target_file_name, target_file_version, dependency_type).await
    }

    pub async fn remove_file_dependency(&self, dependency_id: i64) -> Result<(), sqlx::Error> {
        dependencies::remove_file_dependency(self, dependency_id).await
    }

    pub async fn get_dependent_files(&self, file: &File) -> Result<Vec<File>, sqlx::Error> {
        dependencies::get_dependent_files(self, file).await
    }

    // Collections methods
    pub async fn get_collections(&self) -> Result<Vec<Collection>, sqlx::Error> {
        collections::get_collections(self).await
    }

    pub async fn get_collection(&self, id: i64) -> Result<Option<Collection>, sqlx::Error> {
        collections::get_collection(self, id).await
    }

    pub async fn get_collection_by_name(&self, name: &str) -> Result<Option<Collection>, sqlx::Error> {
        collections::get_collection_by_name(self, name).await
    }

    pub async fn create_collection(&self, name: &str, description: Option<&str>) -> Result<Collection, sqlx::Error> {
        collections::create_collection_simple(self, name, description).await
    }

    pub async fn update_collection(
        &self,
        id: i64,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        collections::update_collection_simple(self, id, name, description).await
    }

    pub async fn delete_collection(&self, id: i64) -> Result<(), sqlx::Error> {
        collections::delete_collection(self, id).await
    }

    pub async fn get_collection_files(&self, collection_id: i64) -> Result<Vec<(File, i64)>, sqlx::Error> {
        collections::get_collection_files(self, collection_id).await
    }

    pub async fn get_collection_files_detailed(&self, collection_id: i64) -> Result<Vec<CollectionFileData>, sqlx::Error> {
        collections::get_collection_files_detailed(self, collection_id).await
    }

    pub async fn add_file_to_collection(
        &self,
        collection_id: i64,
        file_id: i64,
        logic_rule_id: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        collections::add_file_to_collection(self, collection_id, file_id, logic_rule_id).await
    }

    pub async fn remove_file_from_collection(&self, collection_id: i64, file_id: i64) -> Result<(), sqlx::Error> {
        collections::remove_file_from_collection(self, collection_id, file_id).await
    }

    pub async fn reorder_collection_files(
        &self,
        collection_id: i64,
        file_orders: &[(i64, i64)],
    ) -> Result<(), sqlx::Error> {
        collections::reorder_collection_files(self, collection_id, file_orders).await
    }

    pub async fn get_collection_logic_rule(&self, rule_id: i64) -> Result<Option<CollectionLogicRule>, sqlx::Error> {
        collections::get_collection_logic_rule(self, rule_id).await
    }

    pub async fn get_collection_logic_rules(&self, collection_id: i64) -> Result<Vec<CollectionLogicRule>, sqlx::Error> {
        collections::get_collection_logic_rules(self, collection_id).await
    }

    pub async fn create_collection_logic_rule(&self, rule: &CollectionLogicRule) -> Result<CollectionLogicRule, sqlx::Error> {
        collections::create_collection_logic_rule(self, rule).await
    }

    pub async fn update_collection_logic_rule(&self, rule: &CollectionLogicRule) -> Result<(), sqlx::Error> {
        collections::update_collection_logic_rule(self, rule).await
    }

    pub async fn delete_collection_logic_rule(&self, rule_id: i64) -> Result<(), sqlx::Error> {
        collections::delete_collection_logic_rule(self, rule_id).await
    }

    pub async fn file_in_collection(&self, collection_id: i64, file_id: i64) -> Result<bool, sqlx::Error> {
        collections::file_in_collection(self, collection_id, file_id).await
    }

    // Session methods
    pub async fn get_session_state(&self) -> Result<serde_json::Value, sqlx::Error> {
        session::get_session_state(self).await
    }

    pub async fn update_session_file_order(&self, file_order: &[i64]) -> Result<(), sqlx::Error> {
        session::update_session_file_order(self, file_order).await
    }

    pub async fn update_session_ui_preferences(&self, ui_preferences: &serde_json::Value) -> Result<(), sqlx::Error> {
        session::update_session_ui_preferences(self, ui_preferences).await
    }

    pub async fn update_session_open_collections(&self, open_collections: &[i64]) -> Result<(), sqlx::Error> {
        session::update_session_open_collections(self, open_collections).await
    }

    pub async fn update_session_selected_files(&self, file_ids: &[i64]) -> Result<(), sqlx::Error> {
        session::update_session_selected_files(self, file_ids).await
    }

    pub async fn add_recent_action(&self, action: &serde_json::Value) -> Result<(), sqlx::Error> {
        session::add_recent_action(self, action).await
    }

    pub async fn get_recent_actions(&self, limit: usize) -> Result<Vec<serde_json::Value>, sqlx::Error> {
        session::get_recent_actions(self, limit).await
    }

    pub async fn clear_recent_actions(&self) -> Result<(), sqlx::Error> {
        session::clear_recent_actions(self).await
    }

    pub async fn reset_session_state(&self) -> Result<(), sqlx::Error> {
        session::reset_session_state(self).await
    }

    pub async fn update_session_current_page(&self, page: &str) -> Result<(), sqlx::Error> {
        session::update_session_current_page(self, page).await
    }

    pub async fn update_session_selected_site(&self, site_id: Option<i64>) -> Result<(), sqlx::Error> {
        session::update_session_selected_site(self, site_id).await
    }
}

/// Данные файла в коллекции
#[derive(Debug, Clone)]
pub struct CollectionFileData {
    pub id: i64,
    pub collection_id: i64,
    pub file_id: i64,
    pub file: File,
    pub logic_rule_id: Option<i64>,
    pub logic_rule: Option<CollectionLogicRule>,
    pub order_index: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}