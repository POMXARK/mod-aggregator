use scraper::{Html, Selector};
use crate::models::ParserConfig;

/// Построитель парсеров из CSS селекторов
/// 
/// Предоставляет методы для анализа HTML и построения конфигурации парсера
pub struct ParserBuilder;

impl ParserBuilder {
    /// Создать новый экземпляр построителя парсеров
    /// 
    /// # Возвращает
    /// Новый экземпляр ParserBuilder
    pub fn new() -> Self {
        ParserBuilder
    }

    /// Построить конфигурацию парсера из HTML и CSS селектора
    /// 
    /// Анализирует HTML и извлекает все элементы, соответствующие селектору,
    /// вместе с их текстом, HTML содержимым и атрибутами.
    /// 
    /// # Параметры
    /// * `html` - HTML содержимое страницы для анализа
    /// * `selector` - CSS селектор для поиска элементов
    /// 
    /// # Возвращает
    /// JSON объект с результатами анализа или ошибку
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

    /// Извлечь все атрибуты из HTML элемента
    /// 
    /// # Параметры
    /// * `element` - HTML элемент для извлечения атрибутов
    /// 
    /// # Возвращает
    /// JSON объект с атрибутами элемента
    fn extract_attributes(&self, element: &scraper::element_ref::ElementRef) -> serde_json::Value {
        let mut attrs = serde_json::Map::new();
        for (key, value) in element.value().attrs() {
            attrs.insert(key.to_string(), serde_json::Value::String(value.to_string()));
        }
        serde_json::Value::Object(attrs)
    }

    /// Построить конфигурацию парсера из списка узлов
    /// 
    /// # Параметры
    /// * `nodes` - вектор узлов парсера
    /// 
    /// # Возвращает
    /// Конфигурацию парсера с узлами и пустым списком связей
    pub fn build_parser_config(nodes: Vec<crate::models::ParserNode>) -> ParserConfig {
        ParserConfig {
            nodes,
            connections: Vec::new(),
        }
    }
}

