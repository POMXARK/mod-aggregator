    /// Update current page
    pub async fn update_session_current_page(&self, page: &str) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();

        sqlx::query("UPDATE session_state SET current_page = ?, last_updated = ? WHERE id = 1")
            .bind(page)
            .bind(&now)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Update selected site
    pub async fn update_session_selected_site(&self, site_id: Option<i64>) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();

        sqlx::query("UPDATE session_state SET selected_site_id = ?, last_updated = ? WHERE id = 1")
            .bind(site_id)
            .bind(&now)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
