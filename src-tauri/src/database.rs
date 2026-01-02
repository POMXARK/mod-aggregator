//! Модуль базы данных с разделением на подмодули
//!
//! Этот файл содержит основную структуру Database и делегирует
//! вызовы методам в специализированных модулях для лучшей организации.

// Модули базы данных
pub mod modules;

use crate::database::modules::*;
use crate::models::collection::Collection;
use crate::models::collection_logic::{Action, CollectionLogicRule, ConditionType};
use crate::models::dependency::{DependencyType, FileDependency};
use crate::models::file::File;
use crate::models::{Mod, Notification, Site};
use chrono::Utc;
use sqlx::{sqlite::SqlitePool, Row};

/// Структура для работы с базой данных SQLite
///
/// Предоставляет методы для работы с сайтами, модами, уведомлениями и сохраненными страницами.
/// Автоматически создает таблицы и индексы при инициализации.
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Создать новое подключение к базе данных
    ///
    /// Инициализирует подключение к SQLite базе данных, создает необходимые таблицы
    /// и индексы, если они не существуют.
    ///
    /// # Возвращает
    /// Экземпляр Database или ошибку подключения
    pub async fn new() -> Result<Self, sqlx::Error> {
        // Use current directory for database (will be in app directory when running)
        let db_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        std::fs::create_dir_all(&db_dir).map_err(|e| {
            sqlx::Error::Configuration(format!("Failed to create data directory: {}", e).into())
        })?;

        let db_path = db_dir.join("mod_aggregator.db");
        let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
        let pool = SqlitePool::connect(&db_url).await?;

        let db = Database { pool };
        db.init().await?;
        Ok(db)
    }

    /// Инициализировать схему базы данных
    ///
    /// Создает все необходимые таблицы (sites, mods, notifications, saved_pages)
    /// и индексы, если они не существуют.
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    async fn init(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sites (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                url TEXT NOT NULL UNIQUE,
                parser_config TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS mods (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                site_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                url TEXT NOT NULL UNIQUE,
                version TEXT,
                author TEXT,
                description TEXT,
                image_url TEXT,
                changes TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (site_id) REFERENCES sites(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS notifications (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                mod_id INTEGER NOT NULL,
                site_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                message TEXT NOT NULL,
                read INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                FOREIGN KEY (mod_id) REFERENCES mods(id),
                FOREIGN KEY (site_id) REFERENCES sites(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS saved_pages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                site_id INTEGER NOT NULL,
                url TEXT NOT NULL,
                folder_path TEXT NOT NULL,
                version_timestamp TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (site_id) REFERENCES sites(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_mods_site_id ON mods(site_id);
            CREATE INDEX IF NOT EXISTS idx_mods_url ON mods(url);
            CREATE INDEX IF NOT EXISTS idx_notifications_read ON notifications(read);
            CREATE INDEX IF NOT EXISTS idx_saved_pages_site_id ON saved_pages(site_id);
            CREATE INDEX IF NOT EXISTS idx_saved_pages_url ON saved_pages(url);
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Выполнить миграции для новой функциональности зависимостей
        self.run_migrations().await?;

        Ok(())
    }

    // ===== ДЕЛЕГИРОВАНИЕ К СПЕЦИАЛИЗИРОВАННЫМ МОДУЛЯМ =====
    // Методы делегируют вызовы в соответствующие модули
    // TODO: После успешного тестирования заменить тела методов на делегирование

    /// Получить список всех сайтов из базы данных
    ///
    /// Возвращает все сайты, отсортированные по имени.
    ///
    /// # Возвращает
    /// Вектор сайтов или ошибку базы данных
    pub async fn get_sites(&self) -> Result<Vec<Site>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM sites ORDER BY name")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|row| Site {
                id: row.get(0),
                name: row.get(1),
                url: row.get(2),
                parser_config: serde_json::from_str(row.get::<String, _>(3).as_str())
                    .unwrap_or(serde_json::json!({})),
                created_at: row.get::<String, _>(4).parse().unwrap_or(Utc::now()),
                updated_at: row.get::<String, _>(5).parse().unwrap_or(Utc::now()),
            })
            .collect())
    }

    // ===== ДОПОЛНИТЕЛЬНЫЕ МЕТОДЫ ДЛЯ СОВМЕСТИМОСТИ =====

    // Методы для совместимости с существующим кодом
    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        self.inner().run_migrations().await
    }

    pub async fn get_sites(&self) -> Result<Vec<crate::models::Site>, sqlx::Error> {
        sites::get_sites(self.inner()).await
    }

    pub async fn get_site(&self, id: i64) -> Result<crate::models::Site, sqlx::Error> {
        sites::get_site(self.inner(), id).await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn add_site(
        &self,
        name: &str,
        url: &str,
        parser_config: &serde_json::Value,
    ) -> Result<crate::models::Site, sqlx::Error> {
        sites::add_site(self.inner(), name, url, parser_config).await
    }

    pub async fn update_site(
        &self,
        id: i64,
        name: &str,
        url: &str,
        parser_config: &serde_json::Value,
    ) -> Result<(), sqlx::Error> {
        sites::update_site(self.inner(), id, name, url, parser_config).await
    }

    pub async fn delete_site(&self, id: i64) -> Result<(), sqlx::Error> {
        sites::delete_site(self.inner(), id).await
    }

    pub async fn get_mods(&self, site_id: Option<i64>) -> Result<Vec<crate::models::Mod>, sqlx::Error> {
        mods::get_mods(self.inner(), site_id).await
    }

    pub async fn get_mod_by_url(&self, url: &str) -> Result<Option<crate::models::Mod>, sqlx::Error> {
        mods::get_mod_by_url(self.inner(), url).await
    }

    pub async fn add_mod(&self, mod_item: &crate::models::Mod) -> Result<crate::models::Mod, sqlx::Error> {
        mods::add_mod(self.inner(), mod_item).await
    }

    pub async fn update_mod(&self, id: i64, mod_item: &crate::models::Mod) -> Result<(), sqlx::Error> {
        mods::update_mod(self.inner(), id, mod_item).await
    }

    pub async fn get_notifications(&self) -> Result<Vec<crate::models::Notification>, sqlx::Error> {
        notifications::get_notifications(self.inner()).await
    }

    pub async fn add_notification(&self, notification: &crate::models::Notification) -> Result<(), sqlx::Error> {
        notifications::add_notification(self.inner(), notification).await
    }

    pub async fn mark_notification_read(&self, id: i64) -> Result<(), sqlx::Error> {
        notifications::mark_notification_read(self.inner(), id).await
    }

    pub async fn save_page_for_site(
        &self,
        site_id: i64,
        url: &str,
        folder_path: &str,
        version_timestamp: &str,
    ) -> Result<(), sqlx::Error> {
        pages::save_page_for_site(self.inner(), site_id, url, folder_path, version_timestamp).await
    }

    pub async fn get_saved_page(
        &self,
        site_id: i64,
        url: &str,
    ) -> Result<Option<(i64, String, String)>, sqlx::Error> {
        pages::get_saved_page(self.inner(), site_id, url).await
    }

    pub async fn get_saved_page_versions(
        &self,
        site_id: i64,
        url: &str,
    ) -> Result<Vec<serde_json::Value>, sqlx::Error> {
        pages::get_saved_page_versions(self.inner(), site_id, url).await
    }

    pub async fn delete_saved_page_version(&self, page_id: i64) -> Result<(), sqlx::Error> {
        pages::delete_saved_page_version(self.inner(), page_id).await
    }

    pub async fn get_file(&self, id: i64) -> Result<Option<crate::models::file::File>, sqlx::Error> {
        files::get_file(self.inner(), id).await
    }

    pub async fn get_file_by_name_version(
        &self,
        name: &str,
        version: &str,
    ) -> Result<Option<crate::models::file::File>, sqlx::Error> {
        files::get_file_by_name_version(self.inner(), name, version).await
    }

    pub async fn get_file_versions(&self, name: &str) -> Result<Vec<crate::models::file::File>, sqlx::Error> {
        files::get_file_versions(self.inner(), name).await
    }

    pub async fn create_file(&self, file: &crate::models::file::File) -> Result<crate::models::file::File, sqlx::Error> {
        files::create_file(self.inner(), file).await
    }

    pub async fn update_file(&self, file: &crate::models::file::File) -> Result<(), sqlx::Error> {
        files::update_file(self.inner(), file).await
    }

    pub async fn delete_file(&self, id: i64) -> Result<(), sqlx::Error> {
        files::delete_file(self.inner(), id).await
    }

    pub async fn get_all_files(&self) -> Result<Vec<crate::models::file::File>, sqlx::Error> {
        files::get_all_files(self.inner()).await
    }

    pub async fn get_file_dependencies(&self, file_id: i64) -> Result<Vec<crate::models::FileDependency>, sqlx::Error> {
        dependencies::get_file_dependencies(self.inner(), file_id).await
    }

    pub async fn add_file_dependency(&self, dependency: &crate::models::FileDependency) -> Result<crate::models::FileDependency, sqlx::Error> {
        dependencies::add_file_dependency(self.inner(), dependency).await
    }

    pub async fn remove_file_dependency(&self, dependency_id: i64) -> Result<(), sqlx::Error> {
        dependencies::remove_file_dependency(self.inner(), dependency_id).await
    }

    pub async fn get_dependent_files(&self, file: &crate::models::file::File) -> Result<Vec<crate::models::file::File>, sqlx::Error> {
        dependencies::get_dependent_files(self.inner(), file).await
    }

    pub async fn get_collections(&self) -> Result<Vec<crate::models::collection::Collection>, sqlx::Error> {
        collections::get_collections(self.inner()).await
    }

    pub async fn get_collection(&self, id: i64) -> Result<Option<crate::models::collection::Collection>, sqlx::Error> {
        collections::get_collection(self.inner(), id).await
    }

    pub async fn get_collection_by_name(&self, name: &str) -> Result<Option<crate::models::collection::Collection>, sqlx::Error> {
        collections::get_collection_by_name(self.inner(), name).await
    }

    pub async fn create_collection(&self, collection: &crate::models::collection::Collection) -> Result<crate::models::collection::Collection, sqlx::Error> {
        collections::create_collection(self.inner(), collection).await
    }

    pub async fn update_collection(&self, collection: &crate::models::collection::Collection) -> Result<(), sqlx::Error> {
        collections::update_collection(self.inner(), collection).await
    }

    pub async fn delete_collection(&self, id: i64) -> Result<(), sqlx::Error> {
        collections::delete_collection(self.inner(), id).await
    }

    pub async fn get_collection_files(&self, collection_id: i64) -> Result<Vec<(crate::models::file::File, i64)>, sqlx::Error> {
        collections::get_collection_files(self.inner(), collection_id).await
    }

    pub async fn add_file_to_collection(
        &self,
        collection_id: i64,
        file_id: i64,
        logic_rule_id: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        collections::add_file_to_collection(self.inner(), collection_id, file_id, logic_rule_id).await
    }

    pub async fn remove_file_from_collection(&self, collection_id: i64, file_id: i64) -> Result<(), sqlx::Error> {
        collections::remove_file_from_collection(self.inner(), collection_id, file_id).await
    }

    pub async fn reorder_collection_files(
        &self,
        collection_id: i64,
        file_orders: &[(i64, i64)],
    ) -> Result<(), sqlx::Error> {
        collections::reorder_collection_files(self.inner(), collection_id, file_orders).await
    }

    pub async fn get_session_state(&self) -> Result<serde_json::Value, sqlx::Error> {
        session::get_session_state(self.inner()).await
    }

    pub async fn update_session_file_order(&self, file_order: &[i64]) -> Result<(), sqlx::Error> {
        session::update_session_file_order(self.inner(), file_order).await
    }

    pub async fn update_session_ui_preferences(&self, ui_preferences: &serde_json::Value) -> Result<(), sqlx::Error> {
        session::update_session_ui_preferences(self.inner(), ui_preferences).await
    }

    pub async fn update_session_open_collections(&self, open_collections: &[i64]) -> Result<(), sqlx::Error> {
        session::update_session_open_collections(self.inner(), open_collections).await
    }

    pub async fn update_session_selected_files(&self, file_ids: &[i64]) -> Result<(), sqlx::Error> {
        session::update_session_selected_files(self.inner(), file_ids).await
    }

    pub async fn add_recent_action(&self, action: &serde_json::Value) -> Result<(), sqlx::Error> {
        session::add_recent_action(self.inner(), action).await
    }

    pub async fn get_recent_actions(&self) -> Result<Vec<serde_json::Value>, sqlx::Error> {
        session::get_recent_actions(self.inner()).await
    }

    pub async fn clear_recent_actions(&self) -> Result<(), sqlx::Error> {
        session::clear_recent_actions(self.inner()).await
    }

    pub async fn reset_session_state(&self) -> Result<(), sqlx::Error> {
        session::reset_session_state(self.inner()).await
    }

    pub async fn update_session_current_page(&self, page: &str) -> Result<(), sqlx::Error> {
        session::update_session_current_page(self.inner(), page).await
    }

    pub async fn update_session_selected_site(&self, site_id: Option<i64>) -> Result<(), sqlx::Error> {
        session::update_session_selected_site(self.inner(), site_id).await
    }
}

// TODO: Добавить остальные методы делегирования
// Пока оставляем старые методы для совместимости

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
    pub created_at: chrono::DateTime<Utc>,
}