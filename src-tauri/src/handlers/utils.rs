//! Утилиты для работы с URL и HTML
//!
//! Модуль содержит вспомогательные функции для обработки URL и извлечения
//! информации из HTML документов.

/// Извлекает data-base-url из HTML
///
/// Ищет атрибут data-base-url в тегах html или body для определения
/// базового URL страницы. Использует два регулярных выражения:
/// одно для атрибутов с кавычками, другое для атрибутов без кавычек.
///
/// # Параметры
/// * `html` - HTML содержимое страницы для анализа
///
/// # Возвращает
/// Найденный базовый URL или None, если атрибут не найден
pub fn extract_data_base_url(html: &str) -> Option<String> {
    // Ищем data-base-url в атрибутах html или body
    let patterns = [
        r#"data-base-url\s*=\s*["']([^"']+)["']"#,
        r#"data-base-url\s*=\s*([^\s>]+)"#,
    ];

    for pattern in &patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(captures) = re.captures(html) {
                if let Some(url_match) = captures.get(1) {
                    return Some(url_match.as_str().to_string());
                }
            }
        }
    }

    None
}

/// Нормализует URL для сравнения (убирает trailing slash, нормализует)
///
/// Приводит URL к единому формату для корректного сравнения. Парсит URL,
/// убирает trailing slash (если путь не корневой и содержит более 4 сегментов),
/// возвращает строку в нормализованном формате или исходную строку при ошибке парсинга.
///
/// # Параметры
/// * `url` - URL для нормализации
///
/// # Возвращает
/// Нормализованную строку URL для сравнения
pub fn normalize_url_for_comparison(url: &str) -> String {
    match url::Url::parse(url) {
        Ok(url_obj) => {
            let mut normalized = url_obj.to_string();
            // Убираем trailing slash для консистентности (если это не корневой путь)
            if normalized.ends_with('/') && normalized.split('/').count() > 4 {
                normalized.pop();
            }
            normalized
        }
        Err(_) => url.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_data_base_url_with_quotes() {
        let html = r#"<html data-base-url="https://example.com">"#;
        assert_eq!(extract_data_base_url(html), Some("https://example.com".to_string()));
    }

    #[test]
    fn test_extract_data_base_url_without_quotes() {
        let html = r#"<html data-base-url=https://example.com>"#;
        assert_eq!(extract_data_base_url(html), Some("https://example.com".to_string()));
    }

    #[test]
    fn test_extract_data_base_url_not_found() {
        let html = r#"<html><body></body></html>"#;
        assert_eq!(extract_data_base_url(html), None);
    }

    #[test]
    fn test_normalize_url_for_comparison_with_trailing_slash() {
        let url = "https://example.com/path/";
        assert_eq!(normalize_url_for_comparison(url), "https://example.com/path");
    }

    #[test]
    fn test_normalize_url_for_comparison_without_trailing_slash() {
        let url = "https://example.com/path";
        assert_eq!(normalize_url_for_comparison(url), "https://example.com/path");
    }

    #[test]
    fn test_normalize_url_for_comparison_root_path() {
        let url = "https://example.com/";
        assert_eq!(normalize_url_for_comparison(url), "https://example.com/"); // root path сохраняется
    }

    #[test]
    fn test_normalize_url_for_comparison_invalid_url() {
        let url = "not-a-url";
        assert_eq!(normalize_url_for_comparison(url), "not-a-url");
    }
}