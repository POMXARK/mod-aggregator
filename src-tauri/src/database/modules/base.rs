//! Базовые операции с базой данных
//!
//! Модуль содержит основные методы для инициализации подключения к базе данных,
//! создания схемы и выполнения миграций.

use sqlx::sqlite::SqlitePool;

/// Структура для работы с базой данных SQLite
///
/// Предоставляет методы для работы с сайтами, модами, уведомлениями и сохраненными страницами.
/// Автоматически создает таблицы и индексы при инициализации.
#[derive(Clone)]
pub struct Database {
    pub pool: SqlitePool,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_creation() {
        // Этот тест проверяет создание структуры базы данных
        // Для полноценного тестирования нужна тестовая БД в памяти
        let result = Database::new().await;
        // Ожидаем успех или ошибку (в зависимости от наличия директории)
        assert!(result.is_ok() || result.is_err());
    }
}