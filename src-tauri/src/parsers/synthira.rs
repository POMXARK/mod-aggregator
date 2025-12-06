use crate::models::Mod;
use reqwest;
use scraper::{Html, Selector};
use chrono::Utc;

pub struct SynthiraParser;

impl SynthiraParser {
    pub fn new() -> Self {
        SynthiraParser
    }

    pub async fn parse(&self, url: &str) -> Result<Vec<Mod>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()?;
        
        let html = client.get(url).send().await?.text().await?;
        let document = Html::parse_document(&html);

        let mut mods = Vec::new();
        
        // Parse mod items - adjust selectors based on actual synthira.ru structure
        // This is a template that should be adjusted
        let list_selector = Selector::parse(".mod-item, .article-item, .post-item")?;
        
        for element in document.select(&list_selector) {
            if let Some(mod_item) = self.parse_mod_element(&element, url) {
                mods.push(mod_item);
            }
        }

        Ok(mods)
    }

    fn parse_mod_element(
        &self,
        element: &scraper::element_ref::ElementRef,
        base_url: &str,
    ) -> Option<Mod> {
        // Try to find title
        let title_selector = Selector::parse("h2, h3, .title, .post-title, a").ok();
        let title = title_selector
            .and_then(|sel| element.select(&sel).next())
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        // Try to find URL
        let url_selector = Selector::parse("a").ok();
        let url = url_selector
            .and_then(|sel| element.select(&sel).next())
            .and_then(|e| e.value().attr("href"))
            .map(|href| {
                if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("{}{}", base_url.trim_end_matches('/'), href)
                }
            })
            .unwrap_or_default();

        if url.is_empty() || title == "Unknown" {
            return None;
        }

        // Try to find version
        let version_selector = Selector::parse(".version, .ver").ok();
        let version = version_selector
            .and_then(|sel| element.select(&sel).next())
            .map(|e| e.text().collect::<String>().trim().to_string());

        // Try to find author
        let author_selector = Selector::parse(".author, .by-author").ok();
        let author = author_selector
            .and_then(|sel| element.select(&sel).next())
            .map(|e| e.text().collect::<String>().trim().to_string());

        // Try to find image
        let image_selector = Selector::parse("img").ok();
        let image_url = image_selector
            .and_then(|sel| element.select(&sel).next())
            .and_then(|e| e.value().attr("src"))
            .map(|src| {
                if src.starts_with("http") {
                    src.to_string()
                } else {
                    format!("{}{}", base_url.trim_end_matches('/'), src)
                }
            });

        Some(Mod {
            id: 0,
            site_id: 0,
            title,
            url,
            version,
            author,
            description: None,
            image_url,
            changes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

