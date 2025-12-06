use sqlx::{sqlite::SqlitePool, Row};
use chrono::Utc;
use crate::models::{Site, Mod, Notification};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        // Use current directory for database (will be in app directory when running)
        let db_dir = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."));
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
            CREATE INDEX IF NOT EXISTS idx_mods_site_id ON mods(site_id);
            CREATE INDEX IF NOT EXISTS idx_mods_url ON mods(url);
            CREATE INDEX IF NOT EXISTS idx_notifications_read ON notifications(read);
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

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

    pub async fn delete_site(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM sites WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

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

    pub async fn mark_notification_read(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE notifications SET read = 1 WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

