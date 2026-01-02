//! Обработчики команд для работы с парсерами
//!
//! Модуль содержит функции для построения, тестирования и использования парсеров.

use crate::database::Database;
use crate::models;
use crate::parser::ParserEngine;
use crate::parser_builder::ParserBuilder;

/// Построить конфигурацию парсера из HTML и CSS селектора
///
/// Анализирует HTML и создает конфигурацию парсера на основе указанного селектора.
///
/// # Параметры
/// * `html` - HTML содержимое страницы для анализа
/// * `selector` - CSS селектор для поиска элементов
///
/// # Возвращает
/// Конфигурацию парсера в формате JSON или ошибку
#[tauri::command]
pub async fn build_parser(html: String, selector: String) -> Result<serde_json::Value, String> {
    let builder = ParserBuilder::new();
    builder
        .build_from_selector(&html, &selector)
        .await
        .map_err(|e| e.to_string())
}

/// Тестировать парсер на HTML странице
///
/// Выполняет парсинг HTML используя конфигурацию из узлов графа.
///
/// # Параметры
/// * `html` - HTML содержимое страницы для парсинга
/// * `nodes` - массив узлов парсера (selector, extract и т.д.)
/// * `edges` - массив связей между узлами
/// * `max_elements` - максимальное количество элементов для обработки
/// * `timeout_millis` - таймаут выполнения в миллисекундах
/// * `slow_mode` - режим замедленного выполнения
/// * `delay_per_element_ms` - задержка между элементами в миллисекундах
///
/// # Возвращает
/// Результаты парсинга в формате JSON или ошибку
#[tauri::command]
pub async fn test_parser_from_nodes(
    html: String,
    nodes: Vec<serde_json::Value>,
    edges: Vec<serde_json::Value>,
    max_elements: Option<usize>,
    timeout_millis: Option<u64>,
    slow_mode: Option<bool>,
    delay_per_element_ms: Option<u64>,
) -> Result<serde_json::Value, String> {
    use scraper::{Html, Selector};

    let mut diagnostics = Vec::new();

    // Проверка входных данных
    if html.is_empty() {
        return Err("HTML страница пуста".to_string());
    }

    if nodes.is_empty() {
        return Err("Нет узлов парсера для тестирования".to_string());
    }

    diagnostics.push(serde_json::json!({
        "type": "info",
        "message": format!("HTML размер: {} байт", html.len()),
        "nodes_count": nodes.len(),
        "edges_count": edges.len(),
    }));

    let document = Html::parse_document(&html);

    // Находим корневой selector node
    let root_selector_node = nodes.iter().find(|n| {
        n.get("type").and_then(|t| t.as_str()) == Some("selector")
            && !edges.iter().any(|e| {
                e.get("target").and_then(|t| t.as_str()) == n.get("id").and_then(|id| id.as_str())
            })
    });

    let list_selector = root_selector_node
        .and_then(|n| n.get("data"))
        .and_then(|d| d.get("selector"))
        .and_then(|s| s.as_str())
        .ok_or_else(|| {
            diagnostics.push(serde_json::json!({
                "type": "error",
                "message": "Не найден корневой selector node или селектор",
            }));
            "Не найден корневой selector node или селектор".to_string()
        })?;

    diagnostics.push(serde_json::json!({
        "type": "info",
        "message": format!("Найден корневой селектор: {}", list_selector),
    }));

    // Проверяем синтаксис селектора
    let selector_str = list_selector.to_string();
    let selector = Selector::parse(&selector_str).map_err(|e| {
        diagnostics.push(serde_json::json!({
            "type": "error",
            "message": format!("Неверный CSS селектор '{}': {}", list_selector, e),
        }));
        format!("Неверный CSS селектор '{}': {}", list_selector, e)
    })?;

    diagnostics.push(serde_json::json!({
        "type": "success",
        "message": "Синтаксис селектора корректен",
    }));

    // Проверяем, есть ли элементы на странице
    let all_elements: Vec<_> = document.select(&selector).collect();
    let elements_count = all_elements.len();

    // Применяем ограничение на количество элементов
    let max_elements_limit = max_elements.unwrap_or(0);
    let test_elements: Vec<_> = if max_elements_limit > 0 && elements_count > max_elements_limit {
        all_elements.into_iter().take(max_elements_limit).collect()
    } else {
        all_elements
    };

    let processed_count = test_elements.len();

    let element_status = if elements_count > 0 {
        "success"
    } else {
        "warning"
    };
    diagnostics.push(serde_json::json!({
        "type": element_status,
        "message": format!("Найдено элементов по селектору: {} (будет обработано: {})",
            elements_count,
            if max_elements_limit > 0 && elements_count > max_elements_limit {
                format!("{} (ограничение: {})", processed_count, max_elements_limit)
            } else {
                processed_count.to_string()
            }),
    }));

    if max_elements_limit > 0 && elements_count > max_elements_limit {
        diagnostics.push(serde_json::json!({
            "type": "info",
            "message": format!("Применено ограничение: будет обработано максимум {} элементов из {}",
                max_elements_limit, elements_count),
        }));
    }

    if elements_count == 0 {
        diagnostics.push(serde_json::json!({
            "type": "warning",
            "message": "Селектор не нашел элементов на странице. Проверьте правильность селектора.",
        }));
    }

    // Находим все extract узлы, связанные с корневым selector
    let root_node_id = root_selector_node
        .and_then(|n| n.get("id"))
        .and_then(|id| id.as_str())
        .ok_or("Не найден ID корневого узла")?;

    let extract_edges: Vec<_> = edges
        .iter()
        .filter(|e| e.get("source").and_then(|s| s.as_str()) == Some(root_node_id))
        .collect();

    diagnostics.push(serde_json::json!({
        "type": "info",
        "message": format!("Найдено extract узлов: {}", extract_edges.len()),
    }));

    if extract_edges.is_empty() {
        diagnostics.push(serde_json::json!({
            "type": "warning",
            "message": "Нет extract узлов. Парсер будет извлекать только текст из элементов списка.",
        }));
    }

    let mut results = Vec::new();
    let mut extraction_stats = serde_json::Map::new();

    // Начало времени выполнения для проверки таймаута
    let start_time = std::time::Instant::now();
    let timeout_duration = timeout_millis.map(|ms| std::time::Duration::from_millis(ms));
    let timeout_millis_value = timeout_millis.unwrap_or(0);

    // Настройки медленного режима
    let is_slow_mode = slow_mode.unwrap_or(false);
    let delay_per_element = delay_per_element_ms.unwrap_or(0);

    // Парсим каждый элемент списка
    for (index, element) in test_elements.iter().enumerate() {
        // Проверяем таймаут перед обработкой каждого элемента
        if let Some(timeout) = timeout_duration {
            if start_time.elapsed() > timeout {
                diagnostics.push(serde_json::json!({
                    "type": "warning",
                    "message": format!("Достигнут таймаут выполнения ({} мс). Обработано элементов: {}/{}",
                        timeout_millis_value, index, processed_count),
                }));
                break;
            }
        }
        let mut item = serde_json::Map::new();
        let mut item_diagnostics = Vec::new();

        // Обрабатываем каждый extract узел
        for edge in &extract_edges {
            if let Some(extract_node_id) = edge.get("target").and_then(|t| t.as_str()) {
                if let Some(extract_node) = nodes
                    .iter()
                    .find(|n| n.get("id").and_then(|id| id.as_str()) == Some(extract_node_id))
                {
                    if let Some(data) = extract_node.get("data") {
                        let extract_selector =
                            data.get("selector").and_then(|s| s.as_str()).unwrap_or("");
                        let attribute = data
                            .get("attribute")
                            .and_then(|a| a.as_str())
                            .unwrap_or("text");
                        let label = data.get("label").and_then(|l| l.as_str()).unwrap_or("");

                        // Если селектор относительный, ищем внутри текущего элемента
                        if !extract_selector.is_empty() {
                            match Selector::parse(extract_selector) {
                                Ok(extract_sel) => {
                                    if let Some(extract_element) =
                                        element.select(&extract_sel).next()
                                    {
                                        match attribute {
                                            "text" => {
                                                let text = extract_element
                                                    .text()
                                                    .collect::<String>()
                                                    .trim()
                                                    .to_string();
                                                if !text.is_empty() {
                                                    let value_preview =
                                                        text.chars().take(50).collect::<String>();
                                                    item.insert(
                                                        label.to_string(),
                                                        serde_json::Value::String(text),
                                                    );
                                                    item_diagnostics.push(serde_json::json!({
                                                        "field": label,
                                                        "status": "success",
                                                        "value_preview": value_preview,
                                                    }));
                                                } else {
                                                    item_diagnostics.push(serde_json::json!({
                                                        "field": label,
                                                        "status": "warning",
                                                        "message": "Текст пуст",
                                                    }));
                                                }
                                            }
                                            "href" => {
                                                if let Some(href) =
                                                    extract_element.value().attr("href")
                                                {
                                                    item.insert(
                                                        label.to_string(),
                                                        serde_json::Value::String(href.to_string()),
                                                    );
                                                    item_diagnostics.push(serde_json::json!({
                                                        "field": label,
                                                        "status": "success",
                                                        "value": href,
                                                    }));
                                                } else {
                                                    item_diagnostics.push(serde_json::json!({
                                                        "field": label,
                                                        "status": "warning",
                                                        "message": "Атрибут href не найден",
                                                    }));
                                                }
                                            }
                                            "src" => {
                                                if let Some(src) =
                                                    extract_element.value().attr("src")
                                                {
                                                    item.insert(
                                                        label.to_string(),
                                                        serde_json::Value::String(src.to_string()),
                                                    );
                                                    item_diagnostics.push(serde_json::json!({
                                                        "field": label,
                                                        "status": "success",
                                                        "value": src,
                                                    }));
                                                } else {
                                                    item_diagnostics.push(serde_json::json!({
                                                        "field": label,
                                                        "status": "warning",
                                                        "message": "Атрибут src не найден",
                                                    }));
                                                }
                                            }
                                            _ => {
                                                if let Some(attr_value) =
                                                    extract_element.value().attr(attribute)
                                                {
                                                    item.insert(
                                                        label.to_string(),
                                                        serde_json::Value::String(
                                                            attr_value.to_string(),
                                                        ),
                                                    );
                                                    item_diagnostics.push(serde_json::json!({
                                                        "field": label,
                                                        "status": "success",
                                                        "value": attr_value,
                                                    }));
                                                } else {
                                                    item_diagnostics.push(serde_json::json!({
                                                        "field": label,
                                                        "status": "warning",
                                                        "message": format!("Атрибут {} не найден", attribute),
                                                    }));
                                                }
                                            }
                                        }
                                    } else {
                                        item_diagnostics.push(serde_json::json!({
                                            "field": label,
                                            "status": "error",
                                            "message": format!("Элемент не найден по селектору: {}", extract_selector),
                                        }));
                                    }
                                }
                                Err(e) => {
                                    item_diagnostics.push(serde_json::json!({
                                        "field": label,
                                        "status": "error",
                                        "message": format!("Неверный селектор '{}': {}", extract_selector, e),
                                    }));
                                }
                            }
                        } else {
                            // Если селектор пустой, извлекаем из текущего элемента
                            match attribute {
                                "text" => {
                                    let text =
                                        element.text().collect::<String>().trim().to_string();
                                    if !text.is_empty() {
                                        let value_preview =
                                            text.chars().take(50).collect::<String>();
                                        item.insert(
                                            label.to_string(),
                                            serde_json::Value::String(text),
                                        );
                                        item_diagnostics.push(serde_json::json!({
                                            "field": label,
                                            "status": "success",
                                            "value_preview": value_preview,
                                        }));
                                    } else {
                                        item_diagnostics.push(serde_json::json!({
                                            "field": label,
                                            "status": "warning",
                                            "message": "Текст пуст",
                                        }));
                                    }
                                }
                                "href" => {
                                    if let Some(href) = element.value().attr("href") {
                                        item.insert(
                                            label.to_string(),
                                            serde_json::Value::String(href.to_string()),
                                        );
                                        item_diagnostics.push(serde_json::json!({
                                            "field": label,
                                            "status": "success",
                                            "value": href,
                                        }));
                                    } else {
                                        item_diagnostics.push(serde_json::json!({
                                            "field": label,
                                            "status": "warning",
                                            "message": "Атрибут href не найден",
                                        }));
                                    }
                                }
                                "src" => {
                                    if let Some(src) = element.value().attr("src") {
                                        item.insert(
                                            label.to_string(),
                                            serde_json::Value::String(src.to_string()),
                                        );
                                        item_diagnostics.push(serde_json::json!({
                                            "field": label,
                                            "status": "success",
                                            "value": src,
                                        }));
                                    } else {
                                        item_diagnostics.push(serde_json::json!({
                                            "field": label,
                                            "status": "warning",
                                            "message": "Атрибут src не найден",
                                        }));
                                    }
                                }
                                _ => {
                                    if let Some(attr_value) = element.value().attr(attribute) {
                                        item.insert(
                                            label.to_string(),
                                            serde_json::Value::String(attr_value.to_string()),
                                        );
                                        item_diagnostics.push(serde_json::json!({
                                            "field": label,
                                            "status": "success",
                                            "value": attr_value,
                                        }));
                                    } else {
                                        item_diagnostics.push(serde_json::json!({
                                            "field": label,
                                            "status": "warning",
                                            "message": format!("Атрибут {} не найден", attribute),
                                        }));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Если нет extract узлов, извлекаем хотя бы текст
        if item.is_empty() && extract_edges.is_empty() {
            let text = element.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                let value_preview = text.chars().take(50).collect::<String>();
                item.insert("text".to_string(), serde_json::Value::String(text));
                item_diagnostics.push(serde_json::json!({
                    "field": "text",
                    "status": "info",
                    "message": "Извлечен текст (нет extract узлов)",
                    "value_preview": value_preview,
                }));
            }
        }

        if !item.is_empty() {
            let fields_count = item.len();
            results.push(serde_json::Value::Object(item));
            extraction_stats.insert(
                format!("item_{}", index),
                serde_json::json!({
                    "fields_extracted": fields_count,
                    "diagnostics": item_diagnostics,
                }),
            );

            // Добавляем диагностику о текущем прогрессе в медленном режиме
            if is_slow_mode {
                diagnostics.push(serde_json::json!({
                    "type": "info",
                    "message": format!("✅ Обработан элемент #{} из {} (извлечено {} полей)", index + 1, processed_count, fields_count),
                }));
            }
        } else {
            diagnostics.push(serde_json::json!({
                "type": "warning",
                "message": format!("Элемент #{} не дал результатов", index + 1),
            }));
        }

        // Задержка между элементами в медленном режиме (только если не последний элемент)
        // Используем std::thread::sleep вместо tokio::time::sleep, так как ElementRef не Send
        if is_slow_mode && delay_per_element > 0 && index < test_elements.len() - 1 {
            std::thread::sleep(std::time::Duration::from_millis(delay_per_element));
        }
    }

    let execution_time = start_time.elapsed().as_millis();
    diagnostics.push(serde_json::json!({
        "type": "info",
        "message": format!("Время выполнения: {} мс. Обработано элементов: {}", execution_time, results.len()),
    }));

    Ok(serde_json::json!({
        "success": true,
        "count": results.len(),
        "results": results,
        "selector": list_selector,
        "diagnostics": diagnostics,
        "extraction_stats": extraction_stats,
        "elements_found": elements_count,
        "elements_processed": processed_count,
        "execution_time_ms": execution_time,
    }))
}

/// Протестировать парсер для указанного сайта
///
/// Загружает сайт из базы данных и запускает парсер для получения списка модов.
///
/// # Параметры
/// * `site_id` - ID сайта для тестирования парсера
///
/// # Возвращает
/// Вектор найденных модов или ошибку
#[tauri::command]
pub async fn test_parser(site_id: i64) -> Result<Vec<models::Mod>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    let site = db.get_site(site_id).await.map_err(|e| e.to_string())?;
    let engine = ParserEngine::new();
    engine.parse_site(&site).await.map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_build_parser_validation() {
        let result = build_parser("".to_string(), "".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_test_parser_from_nodes_empty_html() {
        let result = test_parser_from_nodes(
            "".to_string(),
            vec![],
            vec![],
            None,
            None,
            None,
            None,
        ).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "HTML страница пуста");
    }

    #[tokio::test]
    async fn test_test_parser_from_nodes_empty_nodes() {
        let result = test_parser_from_nodes(
            "<html></html>".to_string(),
            vec![],
            vec![],
            None,
            None,
            None,
            None,
        ).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Нет узлов парсера для тестирования");
    }

    #[tokio::test]
    async fn test_test_parser_validation() {
        let result = test_parser(1).await;
        assert!(result.is_err()); // Ожидаем ошибку подключения к БД
    }
}