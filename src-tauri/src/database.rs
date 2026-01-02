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

// Модули базы данных (новая модульная структура)
pub mod modules;

// Импорты для совместимости с существующими модулями
pub use modules::*;

// Основная структура Database остается без изменений для обратной совместимости
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

    /// Выполнить миграции базы данных
    ///
    /// Применяет миграции из директории migrations/ для добавления новых таблиц
    /// и изменения схемы существующих таблиц.
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        // Миграция 003: Добавление поддержки зависимостей
        // Создаем таблицу files (универсальная таблица для файлов/модов)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                version TEXT NOT NULL DEFAULT '1.0.0',
                path TEXT,
                metadata TEXT DEFAULT '{}',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(name, version)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Проверяем, существует ли колонка version в таблице files
        // Если таблица уже существует без version, добавляем её
        let table_info: Result<Vec<sqlx::sqlite::SqliteRow>, _> =
            sqlx::query("PRAGMA table_info(files)")
                .fetch_all(&self.pool)
                .await;

        if let Ok(rows) = table_info {
            let has_version = rows.iter().any(|row| {
                let name: String = row.get(1);
                name == "version"
            });

            if !has_version {
                // Добавляем колонку version, если её нет
                sqlx::query("ALTER TABLE files ADD COLUMN version TEXT NOT NULL DEFAULT '1.0.0'")
                    .execute(&self.pool)
                    .await?;
            }
        }

        // Создаем уникальный индекс для name@version
        sqlx::query(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_files_name_version ON files(name, version)",
        )
        .execute(&self.pool)
        .await?;

        // Таблица зависимостей файлов
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS file_dependencies (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_file_id INTEGER NOT NULL,
                target_file_name TEXT NOT NULL,
                target_file_version TEXT,
                dependency_type TEXT NOT NULL CHECK(dependency_type IN ('required', 'optional', 'peer')),
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (source_file_id) REFERENCES files(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_file_dependencies_source ON file_dependencies(source_file_id)")
            .execute(&self.pool)
            .await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_file_dependencies_target ON file_dependencies(target_file_name, target_file_version)")
            .execute(&self.pool)
            .await?;

        // Миграция 004: Добавление поддержки коллекций
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS collections (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS collection_files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                collection_id INTEGER NOT NULL,
                file_id INTEGER NOT NULL,
                logic_rule_id INTEGER,
                order_index INTEGER DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE,
                FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
                FOREIGN KEY (logic_rule_id) REFERENCES collection_logic_rules(id) ON DELETE SET NULL,
                UNIQUE(collection_id, file_id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS collection_logic_rules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                collection_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                condition_type TEXT NOT NULL CHECK(condition_type IN ('boolean', 'collection_check', 'file_check', 'and', 'or')),
                condition_params TEXT NOT NULL,
                action TEXT NOT NULL CHECK(action IN ('enable', 'disable')),
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_collection_files_collection ON collection_files(collection_id)")
            .execute(&self.pool)
            .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_collection_files_file ON collection_files(file_id)",
        )
        .execute(&self.pool)
        .await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_collection_logic_rules_collection ON collection_logic_rules(collection_id)")
            .execute(&self.pool)
            .await?;

        // Миграция 005: Добавление поддержки состояния сессии
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS session_state (
                id INTEGER PRIMARY KEY CHECK(id = 1),
                file_order TEXT NOT NULL DEFAULT '[]',
                ui_preferences TEXT NOT NULL DEFAULT '{}',
                open_collections TEXT DEFAULT '[]',
                selected_files TEXT DEFAULT '[]',
                recent_actions TEXT DEFAULT '[]',
                last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query("INSERT OR IGNORE INTO session_state (id) VALUES (1)")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

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

    /// Получить сайт по ID
    ///
    /// # Параметры
    /// * `id` - идентификатор сайта
    ///
    /// # Возвращает
    /// Сайт или ошибку, если не найден
    pub async fn get_site(&self, id: i64) -> Result<Site, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM sites WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(Site {
            id: row.get(0),
            name: row.get(1),
            url: row.get(2),
            parser_config: serde_json::from_str(row.get::<String, _>(3).as_str())
                .unwrap_or(serde_json::json!({})),
            created_at: row.get::<String, _>(4).parse().unwrap_or(Utc::now()),
            updated_at: row.get::<String, _>(5).parse().unwrap_or(Utc::now()),
        })
    }

    /// Добавить новый сайт в базу данных
    ///
    /// # Параметры
    /// * `name` - название сайта
    /// * `url` - URL сайта
    /// * `parser_config` - конфигурация парсера в формате JSON
    ///
    /// # Возвращает
    /// Созданный сайт с присвоенным ID или ошибку
    pub async fn add_site(
        &self,
        name: &str,
        url: &str,
        parser_config: &serde_json::Value,
    ) -> Result<Site, sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        let config_str = serde_json::to_string(parser_config).unwrap_or_default();

        sqlx::query(
            "INSERT INTO sites (name, url, parser_config, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(name)
        .bind(url)
        .bind(&config_str)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        let id = sqlx::query("SELECT last_insert_rowid()")
            .fetch_one(&self.pool)
            .await?
            .get(0);

        Ok(Site {
            id,
            name: name.to_string(),
            url: url.to_string(),
            parser_config: parser_config.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Обновить существующий сайт в базе данных
    ///
    /// # Параметры
    /// * `id` - идентификатор сайта для обновления
    /// * `name` - новое название сайта
    /// * `url` - новый URL сайта
    /// * `parser_config` - новая конфигурация парсера в формате JSON
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn update_site(
        &self,
        id: i64,
        name: &str,
        url: &str,
        parser_config: &serde_json::Value,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        let config_str = serde_json::to_string(parser_config).unwrap_or_default();

        sqlx::query(
            "UPDATE sites SET name = ?, url = ?, parser_config = ?, updated_at = ? WHERE id = ?",
        )
        .bind(name)
        .bind(url)
        .bind(&config_str)
        .bind(&now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Удалить сайт из базы данных
    ///
    /// # Параметры
    /// * `id` - идентификатор сайта для удаления
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn delete_site(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM sites WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Получить список модов из базы данных
    ///
    /// # Параметры
    /// * `site_id` - ID сайта для фильтрации (None = все сайты)
    ///
    /// # Возвращает
    /// Вектор модов, отсортированных по дате обновления, или ошибку
    pub async fn get_mods(&self, site_id: Option<i64>) -> Result<Vec<Mod>, sqlx::Error> {
        let rows = if let Some(id) = site_id {
            sqlx::query("SELECT * FROM mods WHERE site_id = ? ORDER BY updated_at DESC")
                .bind(id)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query("SELECT * FROM mods ORDER BY updated_at DESC")
                .fetch_all(&self.pool)
                .await?
        };

        Ok(rows
            .iter()
            .map(|row| Mod {
                id: row.get(0),
                site_id: row.get(1),
                title: row.get(2),
                url: row.get(3),
                version: row.get(4),
                author: row.get(5),
                description: row.get(6),
                image_url: row.get(7),
                changes: row.get(8),
                created_at: row.get::<String, _>(9).parse().unwrap_or(Utc::now()),
                updated_at: row.get::<String, _>(10).parse().unwrap_or(Utc::now()),
            })
            .collect())
    }

    /// Получить мод по URL
    ///
    /// # Параметры
    /// * `url` - URL мода для поиска
    ///
    /// # Возвращает
    /// Мод, если найден, или None
    pub async fn get_mod_by_url(&self, url: &str) -> Result<Option<Mod>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM mods WHERE url = ?")
            .bind(url)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| Mod {
            id: r.get(0),
            site_id: r.get(1),
            title: r.get(2),
            url: r.get(3),
            version: r.get(4),
            author: r.get(5),
            description: r.get(6),
            image_url: r.get(7),
            changes: r.get(8),
            created_at: r.get::<String, _>(9).parse().unwrap_or(Utc::now()),
            updated_at: r.get::<String, _>(10).parse().unwrap_or(Utc::now()),
        }))
    }

    /// Добавить новый мод в базу данных
    ///
    /// # Параметры
    /// * `mod_item` - объект мода для добавления
    ///
    /// # Возвращает
    /// Созданный мод с присвоенным ID или ошибку
    pub async fn add_mod(&self, mod_item: &Mod) -> Result<Mod, sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO mods (site_id, title, url, version, author, description, image_url, changes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(mod_item.site_id)
        .bind(&mod_item.title)
        .bind(&mod_item.url)
        .bind(&mod_item.version)
        .bind(&mod_item.author)
        .bind(&mod_item.description)
        .bind(&mod_item.image_url)
        .bind(&mod_item.changes)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        let id = sqlx::query("SELECT last_insert_rowid()")
            .fetch_one(&self.pool)
            .await?
            .get(0);

        Ok(Mod {
            id,
            ..mod_item.clone()
        })
    }

    /// Обновить существующий мод в базе данных
    ///
    /// # Параметры
    /// * `id` - идентификатор мода для обновления
    /// * `mod_item` - объект мода с новыми данными
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn update_mod(&self, id: i64, mod_item: &Mod) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE mods SET title = ?, version = ?, author = ?, description = ?, image_url = ?, changes = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&mod_item.title)
        .bind(&mod_item.version)
        .bind(&mod_item.author)
        .bind(&mod_item.description)
        .bind(&mod_item.image_url)
        .bind(&mod_item.changes)
        .bind(&now)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Получить список всех уведомлений
    ///
    /// Возвращает последние 100 уведомлений, отсортированных по дате создания (новые первыми).
    ///
    /// # Возвращает
    /// Вектор уведомлений или ошибку
    pub async fn get_notifications(&self) -> Result<Vec<Notification>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM notifications ORDER BY created_at DESC LIMIT 100")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|row| Notification {
                id: row.get(0),
                mod_id: row.get(1),
                site_id: row.get(2),
                title: row.get(3),
                message: row.get(4),
                read: row.get::<i64, _>(5) != 0,
                created_at: row.get::<String, _>(6).parse().unwrap_or(Utc::now()),
            })
            .collect())
    }

    /// Добавить новое уведомление в базу данных
    ///
    /// # Параметры
    /// * `notification` - объект уведомления для добавления
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn add_notification(&self, notification: &Notification) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO notifications (mod_id, site_id, title, message, read, created_at) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(notification.mod_id)
        .bind(notification.site_id)
        .bind(&notification.title)
        .bind(&notification.message)
        .bind(notification.read as i64)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Отметить уведомление как прочитанное
    ///
    /// # Параметры
    /// * `id` - ID уведомления для отметки
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn mark_notification_read(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE notifications SET read = 1 WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Сохранить страницу для сайта
    ///
    /// # Параметры
    /// * `site_id` - ID сайта
    /// * `url` - URL страницы
    /// * `folder_path` - путь к папке с файлами
    /// * `version_timestamp` - временная метка версии
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn save_page_for_site(
        &self,
        site_id: i64,
        url: &str,
        folder_path: &str,
        version_timestamp: &str,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO saved_pages (site_id, url, folder_path, version_timestamp, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(site_id)
        .bind(url)
        .bind(folder_path)
        .bind(version_timestamp)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Получить сохраненную страницу
    ///
    /// # Параметры
    /// * `site_id` - ID сайта
    /// * `url` - URL страницы
    ///
    /// # Возвращает
    /// Кортеж (id, folder_path, timestamp) или None
    pub async fn get_saved_page(
        &self,
        site_id: i64,
        url: &str,
    ) -> Result<Option<(i64, String, String)>, sqlx::Error> {
        let row = sqlx::query("SELECT id, folder_path, version_timestamp FROM saved_pages WHERE site_id = ? AND url = ? ORDER BY created_at DESC LIMIT 1")
            .bind(site_id)
            .bind(url)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| (r.get(0), r.get(1), r.get(2))))
    }

    /// Получить все версии сохраненной страницы
    ///
    /// # Параметры
    /// * `site_id` - ID сайта
    /// * `url` - URL страницы
    ///
    /// # Возвращает
    /// Вектор кортежей (id, folder_path, timestamp)
    pub async fn get_saved_page_versions(
        &self,
        site_id: i64,
        url: &str,
    ) -> Result<Vec<serde_json::Value>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, folder_path, version_timestamp FROM saved_pages WHERE site_id = ? AND url = ? ORDER BY created_at DESC")
            .bind(site_id)
            .bind(url)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "id": row.get::<i64, _>(0),
                    "folder_path": row.get::<String, _>(1),
                    "timestamp": row.get::<String, _>(2)
                })
            })
            .collect())
    }

    /// Удалить версию сохраненной страницы
    ///
    /// # Параметры
    /// * `page_id` - ID версии страницы
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn delete_saved_page_version(&self, page_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM saved_pages WHERE id = ?")
            .bind(page_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Получить файл по ID
    ///
    /// # Параметры
    /// * `id` - ID файла
    ///
    /// # Возвращает
    /// Файл или None, если не найден
    pub async fn get_file(&self, id: i64) -> Result<Option<File>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM files WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| File {
            id: r.get(0),
            name: r.get(1),
            version: r.get(2),
            path: r.get(3),
            metadata: serde_json::from_str(r.get::<String, _>(4).as_str()).unwrap_or(serde_json::json!({})),
            created_at: r.get::<chrono::DateTime<Utc>, _>(5),
            updated_at: r.get::<chrono::DateTime<Utc>, _>(6),
        }))
    }

    /// Получить файл по имени и версии
    ///
    /// # Параметры
    /// * `name` - имя файла
    /// * `version` - версия файла
    ///
    /// # Возвращает
    /// Файл или None, если не найден
    pub async fn get_file_by_name_version(
        &self,
        name: &str,
        version: &str,
    ) -> Result<Option<File>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM files WHERE name = ? AND version = ?")
            .bind(name)
            .bind(version)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| File {
            id: r.get(0),
            name: r.get(1),
            version: r.get(2),
            path: r.get(3),
            metadata: serde_json::from_str(r.get::<String, _>(4).as_str()).unwrap_or(serde_json::json!({})),
            created_at: r.get::<chrono::DateTime<Utc>, _>(5),
            updated_at: r.get::<chrono::DateTime<Utc>, _>(6),
        }))
    }

    /// Получить все версии файла
    ///
    /// # Параметры
    /// * `name` - имя файла
    ///
    /// # Возвращает
    /// Вектор файлов с разными версиями
    pub async fn get_file_versions(&self, name: &str) -> Result<Vec<File>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM files WHERE name = ? ORDER BY version DESC")
            .bind(name)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|row| File {
                id: row.get(0),
                name: row.get(1),
                version: row.get(2),
                path: row.get(3),
                metadata: serde_json::from_str(row.get::<String, _>(4).as_str()).unwrap_or(serde_json::json!({})),
                created_at: row.get::<chrono::DateTime<Utc>, _>(5),
                updated_at: row.get::<chrono::DateTime<Utc>, _>(6),
            })
            .collect())
    }

    /// Создать новый файл
    ///
    /// # Параметры
    /// * `name` - имя файла
    /// * `version` - версия файла
    /// * `path` - путь к файлу
    /// * `metadata` - метаданные в формате JSON
    ///
    /// # Возвращает
    /// Созданный файл с присвоенным ID или ошибку
    pub async fn create_file(
        &self,
        name: &str,
        version: &str,
        path: Option<&str>,
        metadata: Option<&serde_json::Value>,
    ) -> Result<File, sqlx::Error> {
        let metadata_str = metadata.map(|m| serde_json::to_string(m).unwrap_or_default()).unwrap_or_else(|| "{}".to_string());

        sqlx::query(
            "INSERT INTO files (name, version, path, metadata, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(name)
        .bind(version)
        .bind(path)
        .bind(&metadata_str)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;

        let id = sqlx::query("SELECT last_insert_rowid()")
            .fetch_one(&self.pool)
            .await?
            .get(0);

        Ok(File {
            id,
            name: name.to_string(),
            version: version.to_string(),
            path: path.map(|s| s.to_string()),
            metadata: metadata.cloned().unwrap_or(serde_json::json!({})),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Обновить существующий файл
    ///
    /// # Параметры
    /// * `id` - ID файла
    /// * `name` - новое имя файла
    /// * `version` - новая версия файла
    /// * `path` - новый путь к файлу
    /// * `metadata` - новые метаданные в формате JSON
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn update_file(
        &self,
        id: i64,
        name: Option<&str>,
        version: Option<&str>,
        path: Option<&str>,
        metadata: Option<&serde_json::Value>,
    ) -> Result<(), sqlx::Error> {
        let metadata_str = metadata.map(|m| serde_json::to_string(m).unwrap_or_default());

        sqlx::query(
            "UPDATE files SET name = COALESCE(?, name), version = COALESCE(?, version), path = COALESCE(?, path), metadata = COALESCE(?, metadata), updated_at = ? WHERE id = ?",
        )
        .bind(name)
        .bind(version)
        .bind(path)
        .bind(&metadata_str)
        .bind(Utc::now())
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Удалить файл
    ///
    /// # Параметры
    /// * `id` - ID файла для удаления
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn delete_file(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM files WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Получить все файлы
    ///
    /// # Возвращает
    /// Вектор всех файлов
    pub async fn get_all_files(&self) -> Result<Vec<File>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM files ORDER BY name, version")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|row| File {
                id: row.get(0),
                name: row.get(1),
                version: row.get(2),
                path: row.get(3),
                metadata: serde_json::from_str(row.get::<String, _>(4).as_str()).unwrap_or(serde_json::json!({})),
                created_at: row.get::<chrono::DateTime<Utc>, _>(5),
                updated_at: row.get::<chrono::DateTime<Utc>, _>(6),
            })
            .collect())
    }

    /// Получить зависимости файла
    ///
    /// # Параметры
    /// * `file_id` - ID файла
    ///
    /// # Возвращает
    /// Вектор зависимостей файла
    pub async fn get_file_dependencies(&self, file_id: i64) -> Result<Vec<FileDependency>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM file_dependencies WHERE source_file_id = ?")
            .bind(file_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|row| FileDependency {
                id: row.get(0),
                source_file_id: row.get(1),
                target_file_name: row.get(2),
                target_file_version: row.get(3),
                dependency_type: match row.get::<String, _>(4).as_str() {
                    "required" => DependencyType::Required,
                    "optional" => DependencyType::Optional,
                    "peer" => DependencyType::Peer,
                    _ => DependencyType::Required,
                },
                created_at: row.get::<chrono::DateTime<Utc>, _>(5),
            })
            .collect())
    }

    /// Добавить зависимость файла
    ///
    /// # Параметры
    /// * `source_file_id` - ID исходного файла
    /// * `target_file_name` - имя целевого файла
    /// * `target_file_version` - версия целевого файла
    /// * `dependency_type` - тип зависимости
    ///
    /// # Возвращает
    /// Созданная зависимость с присвоенным ID или ошибку
    pub async fn add_file_dependency(
        &self,
        source_file_id: i64,
        target_file_name: &str,
        target_file_version: Option<&str>,
        dependency_type: DependencyType,
    ) -> Result<FileDependency, sqlx::Error> {
        let dep_type_str = match dependency_type {
            DependencyType::Required => "required",
            DependencyType::Optional => "optional",
            DependencyType::Peer => "peer",
        };

        sqlx::query(
            "INSERT INTO file_dependencies (source_file_id, target_file_name, target_file_version, dependency_type, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(source_file_id)
        .bind(target_file_name)
        .bind(target_file_version)
        .bind(dep_type_str)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;

        let id = sqlx::query("SELECT last_insert_rowid()")
            .fetch_one(&self.pool)
            .await?
            .get(0);

        Ok(FileDependency {
            id,
            source_file_id,
            target_file_name: target_file_name.to_string(),
            target_file_version: target_file_version.map(|s| s.to_string()),
            dependency_type,
            created_at: Utc::now(),
        })
    }

    /// Удалить зависимость файла
    ///
    /// # Параметры
    /// * `dependency_id` - ID зависимости для удаления
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn remove_file_dependency(&self, dependency_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM file_dependencies WHERE id = ?")
            .bind(dependency_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Получить зависимые файлы
    ///
    /// # Параметры
    /// * `file` - файл для анализа зависимостей
    ///
    /// # Возвращает
    /// Вектор файлов, которые зависят от данного файла
    pub async fn get_dependent_files(&self, file: &File) -> Result<Vec<File>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT f.* FROM files f
             INNER JOIN file_dependencies fd ON f.name = fd.target_file_name
             WHERE fd.source_file_id = ?",
        )
        .bind(file.id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|row| File {
                id: row.get(0),
                name: row.get(1),
                version: row.get(2),
                path: row.get(3),
                metadata: serde_json::from_str(row.get::<String, _>(4).as_str()).unwrap_or(serde_json::json!({})),
                created_at: row.get::<chrono::DateTime<Utc>, _>(5),
                updated_at: row.get::<chrono::DateTime<Utc>, _>(6),
            })
            .collect())
    }

    /// Получить все коллекции
    ///
    /// # Возвращает
    /// Вектор всех коллекций
    pub async fn get_collections(&self) -> Result<Vec<Collection>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM collections ORDER BY name")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|row| Collection {
                id: row.get(0),
                name: row.get(1),
                description: row.get(2),
                created_at: row.get::<chrono::DateTime<Utc>, _>(3),
                updated_at: row.get::<chrono::DateTime<Utc>, _>(4),
            })
            .collect())
    }

    /// Получить коллекцию по ID
    ///
    /// # Параметры
    /// * `id` - ID коллекции
    ///
    /// # Возвращает
    /// Коллекция или None, если не найдена
    pub async fn get_collection(&self, id: i64) -> Result<Option<Collection>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM collections WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| Collection {
            id: r.get(0),
            name: r.get(1),
            description: r.get(2),
            created_at: r.get::<chrono::DateTime<Utc>, _>(3),
            updated_at: r.get::<chrono::DateTime<Utc>, _>(4),
        }))
    }

    /// Получить коллекцию по имени
    ///
    /// # Параметры
    /// * `name` - имя коллекции
    ///
    /// # Возвращает
    /// Коллекция или None, если не найдена
    pub async fn get_collection_by_name(&self, name: &str) -> Result<Option<Collection>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM collections WHERE name = ?")
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| Collection {
            id: r.get(0),
            name: r.get(1),
            description: r.get(2),
            created_at: r.get::<chrono::DateTime<Utc>, _>(3),
            updated_at: r.get::<chrono::DateTime<Utc>, _>(4),
        }))
    }

    /// Создать новую коллекцию
    ///
    /// # Параметры
    /// * `name` - имя коллекции
    /// * `description` - описание коллекции
    ///
    /// # Возвращает
    /// Созданная коллекция с присвоенным ID или ошибку
    pub async fn create_collection(&self, name: &str, description: Option<&str>) -> Result<Collection, sqlx::Error> {
        sqlx::query(
            "INSERT INTO collections (name, description, created_at, updated_at) VALUES (?, ?, ?, ?)",
        )
        .bind(name)
        .bind(description)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;

        let id = sqlx::query("SELECT last_insert_rowid()")
            .fetch_one(&self.pool)
            .await?
            .get(0);

        Ok(Collection {
            id,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Обновить коллекцию
    ///
    /// # Параметры
    /// * `id` - ID коллекции
    /// * `name` - новое имя коллекции
    /// * `description` - новое описание коллекции
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn update_collection(
        &self,
        id: i64,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE collections SET name = COALESCE(?, name), description = COALESCE(?, description), updated_at = ? WHERE id = ?",
        )
        .bind(name)
        .bind(description)
        .bind(Utc::now())
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Удалить коллекцию
    ///
    /// # Параметры
    /// * `id` - ID коллекции для удаления
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn delete_collection(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM collections WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Получить файлы коллекции
    ///
    /// # Параметры
    /// * `collection_id` - ID коллекции
    ///
    /// # Возвращает
    /// Вектор кортежей (файл, порядок) для коллекции
    pub async fn get_collection_files(&self, collection_id: i64) -> Result<Vec<(File, i64)>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT f.*, cf.order_index FROM files f
             INNER JOIN collection_files cf ON f.id = cf.file_id
             WHERE cf.collection_id = ?
             ORDER BY cf.order_index",
        )
        .bind(collection_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|row| {
                let file = File {
                    id: row.get(0),
                    name: row.get(1),
                    version: row.get(2),
                    path: row.get(3),
                    metadata: serde_json::from_str(row.get::<String, _>(4).as_str()).unwrap_or(serde_json::json!({})),
                    created_at: row.get::<chrono::DateTime<Utc>, _>(5),
                    updated_at: row.get::<chrono::DateTime<Utc>, _>(6),
                };
                let order_index = row.get::<i64, _>(7);
                (file, order_index)
            })
            .collect())
    }

    /// Добавить файл в коллекцию
    ///
    /// # Параметры
    /// * `collection_id` - ID коллекции
    /// * `file_id` - ID файла
    /// * `logic_rule_id` - ID логического правила (опционально)
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn add_file_to_collection(
        &self,
        collection_id: i64,
        file_id: i64,
        logic_rule_id: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        let max_order = sqlx::query("SELECT COALESCE(MAX(order_index), 0) FROM collection_files WHERE collection_id = ?")
            .bind(collection_id)
            .fetch_one(&self.pool)
            .await?
            .get::<i64, _>(0);

        sqlx::query(
            "INSERT INTO collection_files (collection_id, file_id, logic_rule_id, order_index, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(collection_id)
        .bind(file_id)
        .bind(logic_rule_id)
        .bind(max_order + 1)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Удалить файл из коллекции
    ///
    /// # Параметры
    /// * `collection_id` - ID коллекции
    /// * `file_id` - ID файла
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn remove_file_from_collection(&self, collection_id: i64, file_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM collection_files WHERE collection_id = ? AND file_id = ?")
            .bind(collection_id)
            .bind(file_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Изменить порядок файлов в коллекции
    ///
    /// # Параметры
    /// * `collection_id` - ID коллекции
    /// * `file_orders` - вектор пар (file_id, order_index)
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn reorder_collection_files(
        &self,
        collection_id: i64,
        file_orders: &[(i64, i64)],
    ) -> Result<(), sqlx::Error> {
        for (file_id, order_index) in file_orders {
            sqlx::query(
                "UPDATE collection_files SET order_index = ? WHERE collection_id = ? AND file_id = ?",
            )
            .bind(order_index)
            .bind(collection_id)
            .bind(file_id)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    /// Получить состояние сессии
    ///
    /// # Возвращает
    /// JSON объект с состоянием сессии или ошибку
    pub async fn get_session_state(&self) -> Result<serde_json::Value, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM session_state WHERE id = 1")
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            Ok(serde_json::json!({
                "file_order": serde_json::from_str::<Vec<i64>>(row.get::<String, _>(1).as_str()).unwrap_or_default(),
                "ui_preferences": serde_json::from_str(row.get::<String, _>(2).as_str()).unwrap_or(serde_json::json!({})),
                "open_collections": serde_json::from_str::<Vec<i64>>(row.get::<String, _>(3).as_str()).unwrap_or_default(),
                "selected_files": serde_json::from_str::<Vec<i64>>(row.get::<String, _>(4).as_str()).unwrap_or_default(),
                "recent_actions": serde_json::from_str(row.get::<String, _>(5).as_str()).unwrap_or(serde_json::json!([])),
                "last_updated": row.get::<chrono::DateTime<Utc>, _>(6)
            }))
        } else {
            Ok(serde_json::json!({
                "file_order": [],
                "ui_preferences": {},
                "open_collections": [],
                "selected_files": [],
                "recent_actions": [],
                "last_updated": Utc::now()
            }))
        }
    }

    /// Обновить порядок файлов в сессии
    ///
    /// # Параметры
    /// * `file_order` - новый порядок файлов
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn update_session_file_order(&self, file_order: &[i64]) -> Result<(), sqlx::Error> {
        let file_order_str = serde_json::to_string(file_order).unwrap_or_default();
        sqlx::query("UPDATE session_state SET file_order = ?, last_updated = ? WHERE id = 1")
            .bind(&file_order_str)
            .bind(Utc::now())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Обновить UI предпочтения сессии
    ///
    /// # Параметры
    /// * `ui_preferences` - JSON объект с предпочтениями UI
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn update_session_ui_preferences(&self, ui_preferences: &serde_json::Value) -> Result<(), sqlx::Error> {
        let ui_preferences_str = serde_json::to_string(ui_preferences).unwrap_or_default();
        sqlx::query("UPDATE session_state SET ui_preferences = ?, last_updated = ? WHERE id = 1")
            .bind(&ui_preferences_str)
            .bind(Utc::now())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Обновить открытые коллекции в сессии
    ///
    /// # Параметры
    /// * `open_collections` - список ID открытых коллекций
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn update_session_open_collections(&self, open_collections: &[i64]) -> Result<(), sqlx::Error> {
        let open_collections_str = serde_json::to_string(open_collections).unwrap_or_default();
        sqlx::query("UPDATE session_state SET open_collections = ?, last_updated = ? WHERE id = 1")
            .bind(&open_collections_str)
            .bind(Utc::now())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Обновить выбранные файлы в сессии
    ///
    /// # Параметры
    /// * `file_ids` - список ID выбранных файлов
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn update_session_selected_files(&self, file_ids: &[i64]) -> Result<(), sqlx::Error> {
        let selected_files_str = serde_json::to_string(file_ids).unwrap_or_default();
        sqlx::query("UPDATE session_state SET selected_files = ?, last_updated = ? WHERE id = 1")
            .bind(&selected_files_str)
            .bind(Utc::now())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Добавить недавнее действие в сессию
    ///
    /// # Параметры
    /// * `action` - JSON объект с описанием действия
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn add_recent_action(&self, action: &serde_json::Value) -> Result<(), sqlx::Error> {
        let current_state = self.get_session_state().await?;
        let mut recent_actions: Vec<serde_json::Value> = serde_json::from_value(
            current_state["recent_actions"].clone()
        ).unwrap_or_default();

        recent_actions.insert(0, action.clone());

        // Ограничить до 50 последних действий
        recent_actions.truncate(50);

        let recent_actions_str = serde_json::to_string(&recent_actions).unwrap_or_default();
        sqlx::query("UPDATE session_state SET recent_actions = ?, last_updated = ? WHERE id = 1")
            .bind(&recent_actions_str)
            .bind(Utc::now())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Получить недавние действия из сессии
    ///
    /// # Параметры
    /// * `limit` - максимальное количество действий
    ///
    /// # Возвращает
    /// Вектор недавних действий или ошибку
    pub async fn get_recent_actions(&self, limit: usize) -> Result<Vec<serde_json::Value>, sqlx::Error> {
        let state = self.get_session_state().await?;
        let mut actions: Vec<serde_json::Value> = serde_json::from_value(state["recent_actions"].clone()).unwrap_or_default();
        actions.truncate(limit);
        Ok(actions)
    }

    /// Очистить недавние действия в сессии
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn clear_recent_actions(&self) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE session_state SET recent_actions = '[]', last_updated = ? WHERE id = 1")
            .bind(Utc::now())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Сбросить состояние сессии
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn reset_session_state(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE session_state SET file_order = '[]', ui_preferences = '{}', open_collections = '[]', selected_files = '[]', recent_actions = '[]', last_updated = ? WHERE id = 1"
        )
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Обновить текущую страницу в сессии
    ///
    /// # Параметры
    /// * `page` - имя текущей страницы
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn update_session_current_page(&self, page: &str) -> Result<(), sqlx::Error> {
        let current_state = self.get_session_state().await?;
        let mut ui_preferences: serde_json::Map<String, serde_json::Value> =
            serde_json::from_value(current_state["ui_preferences"].clone()).unwrap_or_default();

        ui_preferences.insert("current_page".to_string(), serde_json::json!(page));

        self.update_session_ui_preferences(&serde_json::Value::Object(ui_preferences)).await
    }

    /// Обновить выбранный сайт в сессии
    ///
    /// # Параметры
    /// * `site_id` - ID выбранного сайта (None для снятия выбора)
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn update_session_selected_site(&self, site_id: Option<i64>) -> Result<(), sqlx::Error> {
        let current_state = self.get_session_state().await?;
        let mut ui_preferences: serde_json::Map<String, serde_json::Value> =
            serde_json::from_value(current_state["ui_preferences"].clone()).unwrap_or_default();

        ui_preferences.insert("selected_site".to_string(), serde_json::json!(site_id));

        self.update_session_ui_preferences(&serde_json::Value::Object(ui_preferences)).await
    }

    /// Получить детальную информацию о файлах коллекции
    ///
    /// # Параметры
    /// * `collection_id` - ID коллекции
    ///
    /// # Возвращает
    /// Вектор структур с полной информацией о файлах в коллекции
    pub async fn get_collection_files_detailed(&self, collection_id: i64) -> Result<Vec<CollectionFileData>, sqlx::Error> {
        collections::get_collection_files_detailed(self, collection_id).await
    }

    /// Получить логическое правило коллекции по ID
    ///
    /// # Параметры
    /// * `rule_id` - ID правила
    ///
    /// # Возвращает
    /// Правило или None, если не найдено
    pub async fn get_collection_logic_rule(&self, rule_id: i64) -> Result<Option<CollectionLogicRule>, sqlx::Error> {
        collections::get_collection_logic_rule(self, rule_id).await
    }

    /// Получить все логические правила коллекции
    ///
    /// # Параметры
    /// * `collection_id` - ID коллекции
    ///
    /// # Возвращает
    /// Вектор правил коллекции
    pub async fn get_collection_logic_rules(&self, collection_id: i64) -> Result<Vec<CollectionLogicRule>, sqlx::Error> {
        collections::get_collection_logic_rules(self, collection_id).await
    }

    /// Создать логическое правило коллекции
    ///
    /// # Параметры
    /// * `rule` - правило для создания
    ///
    /// # Возвращает
    /// Созданное правило с присвоенным ID или ошибку
    pub async fn create_collection_logic_rule(&self, rule: &CollectionLogicRule) -> Result<CollectionLogicRule, sqlx::Error> {
        collections::create_collection_logic_rule(self, rule).await
    }

    /// Обновить логическое правило коллекции
    ///
    /// # Параметры
    /// * `rule` - правило с обновленными данными
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn update_collection_logic_rule(&self, rule: &CollectionLogicRule) -> Result<(), sqlx::Error> {
        collections::update_collection_logic_rule(self, rule).await
    }

    /// Удалить логическое правило коллекции
    ///
    /// # Параметры
    /// * `rule_id` - ID правила для удаления
    ///
    /// # Возвращает
    /// Пустой результат при успехе или ошибку
    pub async fn delete_collection_logic_rule(&self, rule_id: i64) -> Result<(), sqlx::Error> {
        collections::delete_collection_logic_rule(self, rule_id).await
    }

    /// Проверить, находится ли файл в коллекции
    ///
    /// # Параметры
    /// * `collection_id` - ID коллекции
    /// * `file_id` - ID файла
    ///
    /// # Возвращает
    /// true если файл находится в коллекции
    pub async fn file_in_collection(&self, collection_id: i64, file_id: i64) -> Result<bool, sqlx::Error> {
        collections::file_in_collection(self, collection_id, file_id).await
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
    pub created_at: chrono::DateTime<Utc>,
}