use reqwest;
use scraper::{Html, Selector};
use crate::models::{Site, Mod};
use chrono::Utc;

/// Движок для парсинга сайтов
/// 
/// Предоставляет методы для парсинга HTML страниц и извлечения данных о модах
/// на основе конфигурации парсера сайта
pub struct ParserEngine;

impl ParserEngine {
    /// Создать новый экземпляр парсера
    /// 
    /// # Возвращает
    /// Новый экземпляр ParserEngine
    pub fn new() -> Self {
        ParserEngine
    }

    /// Распарсить сайт и извлечь список модов
    /// 
    /// Загружает страницу сайта, применяет конфигурацию парсера и извлекает
    /// данные о модах из HTML.
    /// 
    /// # Параметры
    /// * `site` - объект сайта с конфигурацией парсера
    /// 
    /// # Возвращает
    /// Вектор найденных модов или ошибку
    pub async fn parse_site(&self, site: &Site) -> Result<Vec<Mod>, Box<dyn std::error::Error + Send + Sync>> {
        let config = &site.parser_config;
        
        // Get base URL and list selector from config
        let list_url = config.get("list_url")
            .and_then(|v| v.as_str())
            .unwrap_or(&site.url);
        
        let list_selector = config.get("list_selector")
            .and_then(|v| v.as_str())
            .ok_or("Missing list_selector in parser config")?;

        // Fetch the page
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()?;
        
        let html = client.get(list_url).send().await?.text().await?;
        let document = Html::parse_document(&html);

        // Parse mod items - clone selector string to avoid lifetime issues
        let selector_str = list_selector.to_string();
        let selector = Selector::parse(&selector_str)
            .map_err(|e| format!("Invalid CSS selector: {}", e))?;
        let mut mods = Vec::new();

        for element in document.select(&selector) {
            if let Some(mod_item) = self.parse_mod_element(&element, &document, config) {
                mods.push(Mod {
                    id: 0,
                    site_id: site.id,
                    ..mod_item
                });
            }
        }

        Ok(mods)
    }

    /// Распарсить элемент мода из HTML
    /// 
    /// Извлекает данные о моде из HTML элемента на основе конфигурации парсера.
    /// 
    /// # Параметры
    /// * `element` - HTML элемент, содержащий данные о моде
    /// * `_document` - полный HTML документ (не используется, но нужен для совместимости)
    /// * `config` - конфигурация парсера с селекторами
    /// 
    /// # Возвращает
    /// Объект мода, если удалось извлечь данные, или None
    fn parse_mod_element(
        &self,
        element: &scraper::element_ref::ElementRef,
        _document: &Html,
        config: &serde_json::Value,
    ) -> Option<Mod> {
        // Extract title
        let title_selector = config.get("title_selector")
            .and_then(|v| v.as_str())
            .and_then(|s| Selector::parse(s).ok());
        
        let title = title_selector
            .and_then(|sel| element.select(&sel).next())
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        // Extract URL
        let url_selector = config.get("url_selector")
            .and_then(|v| v.as_str())
            .and_then(|s| Selector::parse(s).ok());
        
        let url = url_selector
            .and_then(|sel| element.select(&sel).next())
            .and_then(|e| e.value().attr("href"))
            .map(|href| {
                if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("{}{}", config.get("base_url").and_then(|v| v.as_str()).unwrap_or(""), href)
                }
            })
            .unwrap_or_default();

        if url.is_empty() {
            return None;
        }

        // Extract version
        let version_selector = config.get("version_selector")
            .and_then(|v| v.as_str())
            .and_then(|s| Selector::parse(s).ok());
        
        let version = version_selector
            .and_then(|sel| element.select(&sel).next())
            .map(|e| e.text().collect::<String>().trim().to_string());

        // Extract author
        let author_selector = config.get("author_selector")
            .and_then(|v| v.as_str())
            .and_then(|s| Selector::parse(s).ok());
        
        let author = author_selector
            .and_then(|sel| element.select(&sel).next())
            .map(|e| e.text().collect::<String>().trim().to_string());

        // Extract image
        let image_selector = config.get("image_selector")
            .and_then(|v| v.as_str())
            .and_then(|s| Selector::parse(s).ok());
        
        let image_url = image_selector
            .and_then(|sel| element.select(&sel).next())
            .and_then(|e| e.value().attr("src"))
            .map(|src| {
                if src.starts_with("http") {
                    src.to_string()
                } else {
                    format!("{}{}", config.get("base_url").and_then(|v| v.as_str()).unwrap_or(""), src)
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

