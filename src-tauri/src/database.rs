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
        .bind(if notification.read { 1 } else { 0 })
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Отметить уведомление как прочитанное
    ///
    /// # Параметры
    /// * `id` - идентификатор уведомления для отметки
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

    /// Сохранить ссылку на сохраненную страницу для сайта
    ///
    /// Создает запись в базе данных, связывающую сохраненную страницу с сайтом.
    ///
    /// # Параметры
    /// * `site_id` - ID сайта для привязки
    /// * `url` - URL страницы
    /// * `folder_path` - путь к папке с сохраненной страницей
    /// * `version_timestamp` - временная метка версии страницы
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

    /// Получить последнюю сохраненную страницу для сайта по URL
    ///
    /// Сначала ищет точное совпадение URL, затем ищет частичное совпадение
    /// (если запрашиваемый URL начинается с сохраненного URL или наоборот)
    pub async fn get_saved_page(
        &self,
        site_id: i64,
        url: &str,
    ) -> Result<Option<String>, sqlx::Error> {
        // Сначала пытаемся найти точное совпадение
        let row = sqlx::query(
            "SELECT folder_path FROM saved_pages WHERE site_id = ? AND url = ? ORDER BY version_timestamp DESC LIMIT 1"
        )
        .bind(site_id)
        .bind(url)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(folder_path) = row.map(|r| r.get::<String, _>(0)) {
            return Ok(Some(folder_path));
        }

        // Если точного совпадения нет, ищем частичное совпадение
        // Ищем сохраненные URL, которые начинаются с запрашиваемого URL или наоборот
        let rows = sqlx::query(
            "SELECT folder_path, url FROM saved_pages WHERE site_id = ? ORDER BY version_timestamp DESC"
        )
        .bind(site_id)
        .fetch_all(&self.pool)
        .await?;

        for row in rows {
            let saved_url: String = row.get(1);
            let folder_path: String = row.get(0);

            // Проверяем, является ли один URL префиксом другого
            if url.starts_with(&saved_url) || saved_url.starts_with(url) {
                return Ok(Some(folder_path));
            }
        }

        Ok(None)
    }

    /// Получить все версии сохраненной страницы для сайта
    ///
    /// Возвращает список всех версий сохраненной страницы с их метаданными.
    ///
    /// # Параметры
    /// * `site_id` - ID сайта для поиска
    /// * `url` - URL страницы для поиска
    ///
    /// # Возвращает
    /// Вектор кортежей (id, folder_path, timestamp) или ошибку
    pub async fn get_saved_page_versions(
        &self,
        site_id: i64,
        url: &str,
    ) -> Result<Vec<(i64, String, String)>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, folder_path, version_timestamp FROM saved_pages WHERE site_id = ? AND url = ? ORDER BY version_timestamp DESC"
        )
        .bind(site_id)
        .bind(url)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|row| (row.get(0), row.get(1), row.get(2)))
            .collect())
    }

    /// Удалить конкретную версию сохраненной страницы
    ///
    /// Удаляет запись о версии страницы из базы данных.
    ///
    /// # Параметры
    /// * `page_id` - ID версии страницы для удаления
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

    // ============================================
    // Методы для работы с файлами (files table)
    // ============================================

    /// Получить файл по ID
    pub async fn get_file(&self, id: i64) -> Result<Option<File>, sqlx::Error> {
        let row = sqlx::query("SELECT id, name, version, path, metadata, created_at, updated_at FROM files WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| File {
            id: r.get(0),
            name: r.get(1),
            version: r.get(2),
            path: r.get(3),
            metadata: r
                .get::<String, _>(4)
                .parse::<serde_json::Value>()
                .unwrap_or(serde_json::json!({})),
            created_at: r.get::<String, _>(5).parse().unwrap_or(Utc::now()),
            updated_at: r.get::<String, _>(6).parse().unwrap_or(Utc::now()),
        }))
    }

    /// Получить файл по name@version
    pub async fn get_file_by_name_version(
        &self,
        name: &str,
        version: &str,
    ) -> Result<Option<File>, sqlx::Error> {
        let row = sqlx::query("SELECT id, name, version, path, metadata, created_at, updated_at FROM files WHERE name = ? AND version = ?")
            .bind(name)
            .bind(version)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| File {
            id: r.get(0),
            name: r.get(1),
            version: r.get(2),
            path: r.get(3),
            metadata: r
                .get::<String, _>(4)
                .parse::<serde_json::Value>()
                .unwrap_or(serde_json::json!({})),
            created_at: r.get::<String, _>(5).parse().unwrap_or(Utc::now()),
            updated_at: r.get::<String, _>(6).parse().unwrap_or(Utc::now()),
        }))
    }

    /// Получить все файлы (если name пустой) или версии файла с указанным именем
    pub async fn get_file_versions(&self, name: &str) -> Result<Vec<File>, sqlx::Error> {
        if name.is_empty() {
            // Возвращаем все файлы
            let rows = sqlx::query("SELECT id, name, version, path, metadata, created_at, updated_at FROM files ORDER BY name, version")
                .fetch_all(&self.pool)
                .await?;

            return Ok(rows
                .iter()
                .map(|r| File {
                    id: r.get(0),
                    name: r.get(1),
                    version: r.get(2),
                    path: r.get(3),
                    metadata: r
                        .get::<String, _>(4)
                        .parse::<serde_json::Value>()
                        .unwrap_or(serde_json::json!({})),
                    created_at: r.get::<String, _>(5).parse().unwrap_or(Utc::now()),
                    updated_at: r.get::<String, _>(6).parse().unwrap_or(Utc::now()),
                })
                .collect());
        }

        // Возвращаем только версии указанного файла
        let rows = sqlx::query("SELECT id, name, version, path, metadata, created_at, updated_at FROM files WHERE name = ? ORDER BY version")
            .bind(name)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|r| File {
                id: r.get(0),
                name: r.get(1),
                version: r.get(2),
                path: r.get(3),
                metadata: r
                    .get::<String, _>(4)
                    .parse::<serde_json::Value>()
                    .unwrap_or(serde_json::json!({})),
                created_at: r.get::<String, _>(5).parse().unwrap_or(Utc::now()),
                updated_at: r.get::<String, _>(6).parse().unwrap_or(Utc::now()),
            })
            .collect())
    }

    /// Создать новый файл
    pub async fn create_file(
        &self,
        name: &str,
        version: &str,
        path: Option<&str>,
        metadata: Option<&serde_json::Value>,
    ) -> Result<File, sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        let metadata_str =
            serde_json::to_string(metadata.unwrap_or(&serde_json::json!({}))).unwrap_or_default();

        sqlx::query(
            "INSERT INTO files (name, version, path, metadata, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(name)
        .bind(version)
        .bind(path)
        .bind(&metadata_str)
        .bind(&now)
        .bind(&now)
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

    /// Обновить файл
    pub async fn update_file(
        &self,
        id: i64,
        name: Option<&str>,
        version: Option<&str>,
        path: Option<&str>,
        metadata: Option<&serde_json::Value>,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();

        // Строим динамический UPDATE запрос
        let mut updates = Vec::new();

        if name.is_some() {
            updates.push("name = ?");
        }
        if version.is_some() {
            updates.push("version = ?");
        }
        if path.is_some() {
            updates.push("path = ?");
        }
        if metadata.is_some() {
            updates.push("metadata = ?");
        }

        if updates.is_empty() {
            // Обновляем только updated_at
            sqlx::query("UPDATE files SET updated_at = ? WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(&self.pool)
                .await?;
            return Ok(());
        }

        updates.push("updated_at = ?");
        let query_str = format!("UPDATE files SET {} WHERE id = ?", updates.join(", "));
        let mut query = sqlx::query(&query_str);

        // Биндим параметры в правильном порядке
        if let Some(n) = name {
            query = query.bind(n);
        }
        if let Some(v) = version {
            query = query.bind(v);
        }
        if let Some(p) = path {
            query = query.bind(p);
        }
        if let Some(m) = metadata {
            query = query.bind(serde_json::to_string(m).unwrap_or_default());
        }
        query = query.bind(&now);
        query = query.bind(id);

        query.execute(&self.pool).await?;
        Ok(())
    }

    /// Удалить файл
    pub async fn delete_file(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM files WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // ============================================
    // Методы для работы с зависимостями
    // ============================================

    /// Получить все зависимости файла
    pub async fn get_file_dependencies(
        &self,
        file_id: i64,
    ) -> Result<Vec<FileDependency>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, source_file_id, target_file_name, target_file_version, dependency_type, created_at FROM file_dependencies WHERE source_file_id = ?"
        )
        .bind(file_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| FileDependency {
                id: r.get(0),
                source_file_id: r.get(1),
                target_file_name: r.get(2),
                target_file_version: r.get(3),
                dependency_type: DependencyType::from_str(r.get::<String, _>(4).as_str())
                    .unwrap_or(DependencyType::Required),
                created_at: r.get::<String, _>(5).parse().unwrap_or(Utc::now()),
            })
            .collect())
    }

    /// Добавить зависимость к файлу
    pub async fn add_file_dependency(
        &self,
        source_file_id: i64,
        target_file_name: &str,
        target_file_version: Option<&str>,
        dependency_type: DependencyType,
    ) -> Result<FileDependency, sqlx::Error> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO file_dependencies (source_file_id, target_file_name, target_file_version, dependency_type, created_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(source_file_id)
        .bind(target_file_name)
        .bind(target_file_version)
        .bind(dependency_type.as_str())
        .bind(&now)
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

    /// Удалить зависимость
    pub async fn remove_file_dependency(&self, dependency_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM file_dependencies WHERE id = ?")
            .bind(dependency_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Получить все файлы, зависящие от указанного файла
    pub async fn get_dependent_files(&self, file: &File) -> Result<Vec<File>, sqlx::Error> {
        // Ищем зависимости по имени и версии
        let rows = sqlx::query(
            "SELECT DISTINCT f.id, f.name, f.version, f.path, f.metadata, f.created_at, f.updated_at 
             FROM files f
             INNER JOIN file_dependencies fd ON f.id = fd.source_file_id
             WHERE fd.target_file_name = ? AND (fd.target_file_version IS NULL OR fd.target_file_version = ?)"
        )
        .bind(&file.name)
        .bind(&file.version)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| File {
                id: r.get(0),
                name: r.get(1),
                version: r.get(2),
                path: r.get(3),
                metadata: r
                    .get::<String, _>(4)
                    .parse::<serde_json::Value>()
                    .unwrap_or(serde_json::json!({})),
                created_at: r.get::<String, _>(5).parse().unwrap_or(Utc::now()),
                updated_at: r.get::<String, _>(6).parse().unwrap_or(Utc::now()),
            })
            .collect())
    }

    // ============================================
    // Методы для работы с коллекциями
    // ============================================

    /// Получить все коллекции
    pub async fn get_collections(&self) -> Result<Vec<Collection>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, name, description, created_at, updated_at FROM collections ORDER BY name",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| Collection {
                id: r.get(0),
                name: r.get(1),
                description: r.get(2),
                created_at: r.get::<String, _>(3).parse().unwrap_or(Utc::now()),
                updated_at: r.get::<String, _>(4).parse().unwrap_or(Utc::now()),
            })
            .collect())
    }

    /// Получить коллекцию по ID
    pub async fn get_collection(&self, id: i64) -> Result<Option<Collection>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, name, description, created_at, updated_at FROM collections WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Collection {
            id: r.get(0),
            name: r.get(1),
            description: r.get(2),
            created_at: r.get::<String, _>(3).parse().unwrap_or(Utc::now()),
            updated_at: r.get::<String, _>(4).parse().unwrap_or(Utc::now()),
        }))
    }

    /// Получить коллекцию по имени
    pub async fn get_collection_by_name(
        &self,
        name: &str,
    ) -> Result<Option<Collection>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, name, description, created_at, updated_at FROM collections WHERE name = ?",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Collection {
            id: r.get(0),
            name: r.get(1),
            description: r.get(2),
            created_at: r.get::<String, _>(3).parse().unwrap_or(Utc::now()),
            updated_at: r.get::<String, _>(4).parse().unwrap_or(Utc::now()),
        }))
    }

    /// Создать коллекцию
    pub async fn create_collection(
        &self,
        name: &str,
        description: Option<&str>,
    ) -> Result<Collection, sqlx::Error> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO collections (name, description, created_at, updated_at) VALUES (?, ?, ?, ?)"
        )
        .bind(name)
        .bind(description)
        .bind(&now)
        .bind(&now)
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
    pub async fn update_collection(
        &self,
        id: i64,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();

        let mut updates = Vec::new();
        if name.is_some() {
            updates.push("name = ?");
        }
        if description.is_some() {
            updates.push("description = ?");
        }

        if updates.is_empty() {
            sqlx::query("UPDATE collections SET updated_at = ? WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(&self.pool)
                .await?;
            return Ok(());
        }

        updates.push("updated_at = ?");
        let query_str = format!("UPDATE collections SET {} WHERE id = ?", updates.join(", "));
        let mut query = sqlx::query(&query_str);

        if let Some(n) = name {
            query = query.bind(n);
        }
        if let Some(d) = description {
            query = query.bind(d);
        }
        query = query.bind(&now);
        query = query.bind(id);

        query.execute(&self.pool).await?;
        Ok(())
    }

    /// Удалить коллекцию
    pub async fn delete_collection(&self, id: i64) -> Result<(), sqlx::Error> {
        // Каскадное удаление обрабатывается внешними ключами
        sqlx::query("DELETE FROM collections WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Получить файлы коллекции
    pub async fn get_collection_files(
        &self,
        collection_id: i64,
    ) -> Result<Vec<CollectionFileData>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT 
                cf.id, cf.collection_id, cf.file_id, cf.logic_rule_id, cf.order_index, cf.created_at,
                f.name, f.version, f.path, f.metadata, f.created_at as file_created_at, f.updated_at as file_updated_at
            FROM collection_files cf
            INNER JOIN files f ON cf.file_id = f.id
            WHERE cf.collection_id = ?
            ORDER BY cf.order_index, cf.created_at
            "#
        )
        .bind(collection_id)
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::new();
        for r in rows {
            let file = File {
                id: r.get(6),
                name: r.get(7),
                version: r.get(8),
                path: r.get(9),
                metadata: r
                    .get::<String, _>(10)
                    .parse::<serde_json::Value>()
                    .unwrap_or(serde_json::json!({})),
                created_at: r.get::<String, _>(11).parse().unwrap_or(Utc::now()),
                updated_at: r.get::<String, _>(12).parse().unwrap_or(Utc::now()),
            };

            let logic_rule_id: Option<i64> = r.get(3);
            let logic_rule = if let Some(rule_id) = logic_rule_id {
                self.get_collection_logic_rule(rule_id).await.ok().flatten()
            } else {
                None
            };

            result.push(CollectionFileData {
                id: r.get(0),
                collection_id: r.get(1),
                file_id: r.get(2),
                file,
                logic_rule_id,
                logic_rule,
                order_index: r.get(4),
                created_at: r.get::<String, _>(5).parse().unwrap_or(Utc::now()),
            });
        }

        Ok(result)
    }

    /// Добавить файл в коллекцию
    pub async fn add_file_to_collection(
        &self,
        collection_id: i64,
        file_id: i64,
        logic_rule_id: Option<i64>,
        order_index: Option<i64>,
    ) -> Result<i64, sqlx::Error> {
        let order = order_index.unwrap_or(0);
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO collection_files (collection_id, file_id, logic_rule_id, order_index, created_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(collection_id)
        .bind(file_id)
        .bind(logic_rule_id)
        .bind(order)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        let id = sqlx::query("SELECT last_insert_rowid()")
            .fetch_one(&self.pool)
            .await?
            .get(0);

        Ok(id)
    }

    /// Удалить файл из коллекции
    pub async fn remove_file_from_collection(
        &self,
        collection_id: i64,
        file_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM collection_files WHERE collection_id = ? AND file_id = ?")
            .bind(collection_id)
            .bind(file_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Изменить порядок файлов в коллекции
    pub async fn reorder_collection_files(
        &self,
        collection_id: i64,
        file_orders: &[(i64, i64)], // (file_id, order_index)
    ) -> Result<(), sqlx::Error> {
        for (file_id, order_index) in file_orders {
            sqlx::query(
                "UPDATE collection_files SET order_index = ? WHERE collection_id = ? AND file_id = ?"
            )
            .bind(order_index)
            .bind(collection_id)
            .bind(file_id)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    /// Получить правила логики коллекции
    pub async fn get_collection_logic_rules(
        &self,
        collection_id: i64,
    ) -> Result<Vec<CollectionLogicRule>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, collection_id, name, condition_type, condition_params, action, created_at FROM collection_logic_rules WHERE collection_id = ? ORDER BY created_at"
        )
        .bind(collection_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| CollectionLogicRule {
                id: r.get(0),
                collection_id: r.get(1),
                name: r.get(2),
                condition_type: ConditionType::from_str(r.get::<String, _>(3).as_str())
                    .unwrap_or(ConditionType::Boolean),
                condition_params: r
                    .get::<String, _>(4)
                    .parse::<serde_json::Value>()
                    .unwrap_or(serde_json::json!({})),
                action: Action::from_str(r.get::<String, _>(5).as_str()).unwrap_or(Action::Enable),
                created_at: r.get::<String, _>(6).parse().unwrap_or(Utc::now()),
            })
            .collect())
    }

    /// Получить правило логики по ID
    pub async fn get_collection_logic_rule(
        &self,
        rule_id: i64,
    ) -> Result<Option<CollectionLogicRule>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, collection_id, name, condition_type, condition_params, action, created_at FROM collection_logic_rules WHERE id = ?"
        )
        .bind(rule_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| CollectionLogicRule {
            id: r.get(0),
            collection_id: r.get(1),
            name: r.get(2),
            condition_type: ConditionType::from_str(r.get::<String, _>(3).as_str())
                .unwrap_or(ConditionType::Boolean),
            condition_params: r
                .get::<String, _>(4)
                .parse::<serde_json::Value>()
                .unwrap_or(serde_json::json!({})),
            action: Action::from_str(r.get::<String, _>(5).as_str()).unwrap_or(Action::Enable),
            created_at: r.get::<String, _>(6).parse().unwrap_or(Utc::now()),
        }))
    }

    /// Создать правило логики
    pub async fn create_collection_logic_rule(
        &self,
        collection_id: i64,
        name: &str,
        condition_type: ConditionType,
        condition_params: &serde_json::Value,
        action: Action,
    ) -> Result<CollectionLogicRule, sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        let params_str = serde_json::to_string(condition_params).unwrap_or_default();

        sqlx::query(
            "INSERT INTO collection_logic_rules (collection_id, name, condition_type, condition_params, action, created_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(collection_id)
        .bind(name)
        .bind(condition_type.as_str())
        .bind(&params_str)
        .bind(action.as_str())
        .bind(&now)
        .execute(&self.pool)
        .await?;

        let id = sqlx::query("SELECT last_insert_rowid()")
            .fetch_one(&self.pool)
            .await?
            .get(0);

        Ok(CollectionLogicRule {
            id,
            collection_id,
            name: name.to_string(),
            condition_type,
            condition_params: condition_params.clone(),
            action,
            created_at: Utc::now(),
        })
    }

    /// Обновить правило логики
    pub async fn update_collection_logic_rule(
        &self,
        rule_id: i64,
        name: Option<&str>,
        condition_type: Option<ConditionType>,
        condition_params: Option<&serde_json::Value>,
        action: Option<Action>,
    ) -> Result<(), sqlx::Error> {
        let mut updates = Vec::new();

        if name.is_some() {
            updates.push("name = ?");
        }
        if condition_type.is_some() {
            updates.push("condition_type = ?");
        }
        if condition_params.is_some() {
            updates.push("condition_params = ?");
        }
        if action.is_some() {
            updates.push("action = ?");
        }

        if updates.is_empty() {
            return Ok(());
        }

        let query_str = format!(
            "UPDATE collection_logic_rules SET {} WHERE id = ?",
            updates.join(", ")
        );
        let mut query = sqlx::query(&query_str);

        if let Some(n) = name {
            query = query.bind(n);
        }
        if let Some(ct) = condition_type {
            query = query.bind(ct.as_str());
        }
        if let Some(cp) = condition_params {
            query = query.bind(serde_json::to_string(cp).unwrap_or_default());
        }
        if let Some(a) = action {
            query = query.bind(a.as_str());
        }
        query = query.bind(rule_id);

        query.execute(&self.pool).await?;
        Ok(())
    }

    /// Удалить правило логики
    pub async fn delete_collection_logic_rule(&self, rule_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM collection_logic_rules WHERE id = ?")
            .bind(rule_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Проверить, есть ли файл в коллекции
    pub async fn file_in_collection(
        &self,
        collection_id: i64,
        file_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let row = sqlx::query(
            "SELECT 1 FROM collection_files WHERE collection_id = ? AND file_id = ? LIMIT 1",
        )
        .bind(collection_id)
        .bind(file_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.is_some())
    }

    // ========== Session State Methods ==========

    /// Получить состояние сессии
    pub async fn get_session_state(
        &self,
    ) -> Result<crate::models::session_state::SessionState, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, file_order, ui_preferences, open_collections, selected_files, recent_actions, last_updated FROM session_state WHERE id = 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = row {
            let file_order_str: String = r.get(1);
            let ui_prefs_str: String = r.get(2);
            let open_colls_str: String = r.get(3);
            let selected_files_str: String = r.get(4);
            let recent_actions_str: String = r.get(5);

            let file_order: Vec<i64> = serde_json::from_str(&file_order_str).unwrap_or_default();
            let ui_preferences: crate::models::session_state::UiPreferences =
                serde_json::from_str(&ui_prefs_str).unwrap_or_default();
            let open_collections: Vec<i64> =
                serde_json::from_str(&open_colls_str).unwrap_or_default();
            let selected_files: Vec<i64> =
                serde_json::from_str(&selected_files_str).unwrap_or_default();
            let recent_actions: Vec<crate::models::session_state::RecentAction> =
                serde_json::from_str(&recent_actions_str).unwrap_or_default();

            Ok(crate::models::session_state::SessionState {
                id: r.get(0),
                file_order,
                ui_preferences,
                open_collections,
                selected_files,
                recent_actions,
                last_updated: r.get::<String, _>(6).parse().unwrap_or(Utc::now()),
            })
        } else {
            // Возвращаем состояние по умолчанию
            Ok(crate::models::session_state::SessionState::default())
        }
    }

    /// Обновить порядок файлов в сессии
    pub async fn update_session_file_order(&self, file_order: &[i64]) -> Result<(), sqlx::Error> {
        let json_str = serde_json::to_string(file_order).unwrap_or_default();
        let now = Utc::now().to_rfc3339();

        sqlx::query("UPDATE session_state SET file_order = ?, last_updated = ? WHERE id = 1")
            .bind(&json_str)
            .bind(&now)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Обновить настройки UI в сессии
    pub async fn update_session_ui_preferences(
        &self,
        preferences: &crate::models::session_state::UiPreferences,
    ) -> Result<(), sqlx::Error> {
        let json_str = serde_json::to_string(preferences).unwrap_or_default();
        let now = Utc::now().to_rfc3339();

        sqlx::query("UPDATE session_state SET ui_preferences = ?, last_updated = ? WHERE id = 1")
            .bind(&json_str)
            .bind(&now)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Обновить открытые коллекции в сессии
    pub async fn update_session_open_collections(
        &self,
        collection_ids: &[i64],
    ) -> Result<(), sqlx::Error> {
        let json_str = serde_json::to_string(collection_ids).unwrap_or_default();
        let now = Utc::now().to_rfc3339();

        sqlx::query("UPDATE session_state SET open_collections = ?, last_updated = ? WHERE id = 1")
            .bind(&json_str)
            .bind(&now)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Обновить выбранные файлы в сессии
    pub async fn update_session_selected_files(&self, file_ids: &[i64]) -> Result<(), sqlx::Error> {
        let json_str = serde_json::to_string(file_ids).unwrap_or_default();
        let now = Utc::now().to_rfc3339();

        sqlx::query("UPDATE session_state SET selected_files = ?, last_updated = ? WHERE id = 1")
            .bind(&json_str)
            .bind(&now)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Добавить действие в историю
    pub async fn add_recent_action(
        &self,
        action: &crate::models::session_state::RecentAction,
    ) -> Result<(), sqlx::Error> {
        let mut state = self.get_session_state().await?;
        state.add_recent_action(action.clone());

        let recent_actions_str = serde_json::to_string(&state.recent_actions).unwrap_or_default();
        let now = Utc::now().to_rfc3339();

        sqlx::query("UPDATE session_state SET recent_actions = ?, last_updated = ? WHERE id = 1")
            .bind(&recent_actions_str)
            .bind(&now)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Получить последние действия
    pub async fn get_recent_actions(
        &self,
        limit: usize,
    ) -> Result<Vec<crate::models::session_state::RecentAction>, sqlx::Error> {
        let state = self.get_session_state().await?;
        Ok(state.recent_actions.into_iter().take(limit).collect())
    }

    /// Очистить историю действий
    pub async fn clear_recent_actions(&self) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE session_state SET recent_actions = '[]', last_updated = ? WHERE id = 1",
        )
        .bind(&now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Сбросить состояние сессии
    pub async fn reset_session_state(&self) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE session_state SET file_order = '[]', ui_preferences = '{}', open_collections = '[]', selected_files = '[]', recent_actions = '[]', last_updated = ? WHERE id = 1"
        )
        .bind(&now)
        .execute(&self.pool)
        .await?;

        Ok(())
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
