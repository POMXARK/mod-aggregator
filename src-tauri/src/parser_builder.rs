use scraper::{Html, Selector};
use crate::models::ParserConfig;

pub struct ParserBuilder;

impl ParserBuilder {
    pub fn new() -> Self {
        ParserBuilder
    }

    pub async fn build_from_selector(
        &self,
        html: &str,
        selector: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        let document = Html::parse_document(html);
        let selector_str = selector.to_string();
        let sel = Selector::parse(&selector_str)
            .map_err(|e| format!("Invalid CSS selector: {}", e))?;

        let mut results = Vec::new();
        for element in document.select(&sel) {
            let text = element.text().collect::<String>();
            let html_content = element.html();
            
            results.push(serde_json::json!({
                "text": text.trim(),
                "html": html_content,
                "attributes": self.extract_attributes(&element),
            }));
        }

        Ok(serde_json::json!({
            "selector": selector,
            "matches": results.len(),
            "results": results,
        }))
    }

    fn extract_attributes(&self, element: &scraper::element_ref::ElementRef) -> serde_json::Value {
        let mut attrs = serde_json::Map::new();
        for (key, value) in element.value().attrs() {
            attrs.insert(key.to_string(), serde_json::Value::String(value.to_string()));
        }
        serde_json::Value::Object(attrs)
    }

    pub fn build_parser_config(nodes: Vec<crate::models::ParserNode>) -> ParserConfig {
        ParserConfig {
            nodes,
            connections: Vec::new(),
        }
    }
}

