//! Модуль базы данных с разделением на подмодули
//!
//! Этот файл содержит основную структуру Database и делегирует
//! вызовы методам в специализированных модулях для лучшей организации.

// Модули базы данных
pub mod modules;

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