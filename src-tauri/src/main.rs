// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai_parser;
mod commands;
mod database;
mod models;
mod notification;
mod parser;
mod parser_builder;
mod services;

// mod parsers;

use ai_parser::AIParser;
use chrono::Utc;
use commands::batch_operations::*;
use commands::collections::*;
use commands::dependencies::*;
use commands::files::*;
use commands::import_export::*;
use commands::session_state::*;
use database::Database;
use log::{debug, error, info, warn};
use notification::NotificationService;
use parser::ParserEngine;
use parser_builder::ParserBuilder;

/// Извлекает data-base-url из HTML
///
/// Ищет атрибут data-base-url в тегах html или body
fn extract_data_base_url(html: &str) -> Option<String> {
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
/// Приводит URL к единому формату для сравнения
fn normalize_url_for_comparison(url: &str) -> String {
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

/// Получить список всех сайтов из базы данных
///
/// # Возвращает
/// Вектор всех сайтов, отсортированных по имени, или ошибку
#[tauri::command]
async fn get_sites() -> Result<Vec<models::Site>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.get_sites().await.map_err(|e| e.to_string())
}

/// Добавить новый сайт в базу данных
///
/// # Параметры
/// * `name` - название сайта
/// * `url` - URL сайта
/// * `parser_config` - конфигурация парсера в формате JSON
///
/// # Возвращает
/// Созданный сайт или ошибку
#[tauri::command]
async fn add_site(
    name: String,
    url: String,
    parser_config: serde_json::Value,
) -> Result<models::Site, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.add_site(&name, &url, &parser_config)
        .await
        .map_err(|e| e.to_string())
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
#[tauri::command]
async fn update_site(
    id: i64,
    name: String,
    url: String,
    parser_config: serde_json::Value,
) -> Result<(), String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.update_site(id, &name, &url, &parser_config)
        .await
        .map_err(|e| e.to_string())
}

/// Удалить сайт из базы данных
///
/// # Параметры
/// * `id` - идентификатор сайта для удаления
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
#[tauri::command]
async fn delete_site(id: i64) -> Result<(), String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.delete_site(id).await.map_err(|e| e.to_string())
}

/// Получить список модов из базы данных
///
/// # Параметры
/// * `site_id` - ID сайта для фильтрации (None = все сайты)
///
/// # Возвращает
/// Вектор модов, отсортированных по дате обновления, или ошибку
#[tauri::command]
async fn get_mods(site_id: Option<i64>) -> Result<Vec<models::Mod>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.get_mods(site_id).await.map_err(|e| e.to_string())
}

/// Проверить обновления модов для указанного сайта или всех сайтов
///
/// Загружает страницы сайтов, парсит моды и сравнивает с существующими в базе данных.
/// Создает записи о новых модах и обновлениях существующих.
///
/// # Параметры
/// * `site_id` - ID сайта для проверки (None = все сайты)
///
/// # Возвращает
/// Вектор обновлений модов (ModUpdate) или ошибку
#[tauri::command]
async fn check_updates(site_id: Option<i64>) -> Result<Vec<models::ModUpdate>, String> {
    let engine = ParserEngine::new();
    let db = Database::new().await.map_err(|e| e.to_string())?;

    let sites = if let Some(id) = site_id {
        vec![db.get_site(id).await.map_err(|e| e.to_string())?]
    } else {
        db.get_sites().await.map_err(|e| e.to_string())?
    };

    let mut updates = Vec::new();

    for site in sites {
        match engine.parse_site(&site).await {
            Ok(mods) => {
                for mod_item in mods {
                    if let Some(existing) = db.get_mod_by_url(&mod_item.url).await.ok().flatten() {
                        if existing.updated_at < mod_item.updated_at {
                            updates.push(models::ModUpdate {
                                mod_id: existing.id,
                                site_id: site.id,
                                old_version: existing.version.clone(),
                                new_version: mod_item.version.clone(),
                                changes: mod_item.changes.clone(),
                            });
                            db.update_mod(existing.id, &mod_item).await.ok();
                        }
                    } else {
                        db.add_mod(&mod_item).await.ok();
                    }
                }
            }
            Err(e) => eprintln!("Error parsing site {}: {}", site.name, e),
        }
    }

    Ok(updates)
}

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
async fn build_parser(html: String, selector: String) -> Result<serde_json::Value, String> {
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
///
/// # Возвращает
/// Результаты парсинга в формате JSON или ошибку
#[tauri::command]
async fn test_parser_from_nodes(
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

/// Загрузить HTML страницу с указанного URL или из кеша
///
/// Сначала проверяет кеш (если `force_refresh` = false), затем загружает с сервера.
/// После загрузки сохраняет страницу в кеш для будущего использования.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `url` - URL страницы для загрузки
/// * `force_refresh` - если true, игнорирует кеш и загружает с сервера
/// * `site_id` - ID сайта для привязки кеша (опционально)
///
/// # Возвращает
/// HTML содержимое страницы или ошибку
#[tauri::command]
async fn fetch_page(
    app_handle: tauri::AppHandle,
    url: String,
    force_refresh: bool,
    site_id: Option<i64>,
) -> Result<String, String> {
    info!(
        "fetch_page called with URL: {}, force_refresh: {}, site_id: {:?}",
        url, force_refresh, site_id
    );

    // Validate URL format
    if url.is_empty() {
        warn!("Empty URL provided");
        return Err("URL не может быть пустым".to_string());
    }

    // Нормализуем URL перед поиском в кеше
    let normalized_url = match url::Url::parse(&url) {
        Ok(url_obj) => {
            let mut normalized = url_obj.to_string();
            // Убираем trailing slash для консистентности (если это не корневой путь)
            if normalized.ends_with('/') && normalized.split('/').count() > 4 {
                normalized.pop();
            }
            info!("Normalized URL: {} -> {}", url, normalized);
            normalized
        }
        Err(e) => {
            warn!("Invalid URL format: {}, error: {}", url, e);
            url.clone()
        }
    };

    // Проверяем кэш перед загрузкой с сервера (если не принудительное обновление)
    if !force_refresh {
        info!(
            "Checking cache for URL: {} (normalized: {})",
            url, normalized_url
        );

        // Сначала проверяем сохраненную страницу для сайта (если site_id указан)
        if let Some(site_id_val) = site_id {
            info!(
                "Checking saved page for site {} with URL: {}",
                site_id_val, normalized_url
            );
            match get_saved_page_for_site(app_handle.clone(), site_id_val, normalized_url.clone())
                .await
            {
                Ok(Some(cached_html)) => {
                    info!(
                        "✓ Found cached page for site {}: {} (HTML length: {} chars)",
                        site_id_val,
                        normalized_url,
                        cached_html.len()
                    );
                    return Ok(cached_html);
                }
                Ok(None) => {
                    info!(
                        "✗ No cached page found for site {}: {}",
                        site_id_val, normalized_url
                    );
                }
                Err(e) => {
                    warn!("Error checking cached page for site {}: {}", site_id_val, e);
                }
            }
        }

        // Если не нашли для сайта, проверяем общий кэш
        info!("Checking general cache for URL: {}", normalized_url);
        match get_cached_page(app_handle.clone(), normalized_url.clone()).await {
            Ok(Some(cached_html)) => {
                info!(
                    "✓ Found cached page from general cache: {} (HTML length: {} chars)",
                    normalized_url,
                    cached_html.len()
                );
                return Ok(cached_html);
            }
            Ok(None) => {
                info!(
                    "✗ No cached page found in general cache: {}",
                    normalized_url
                );
            }
            Err(e) => {
                warn!("Error checking general cache: {}", e);
            }
        }
    }

    info!("Loading page from server: {}", url);

    // Загружаем страницу с сервера
    // Limit response size to prevent memory issues (10MB max)
    const MAX_SIZE: u64 = 10 * 1024 * 1024;

    debug!("Creating HTTP client...");
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| {
            error!("Failed to create HTTP client: {}", e);
            format!("Не удалось создать HTTP клиент: {}", e)
        })?;

    debug!("Sending GET request to: {}", url);
    let response = client.get(&url).send().await.map_err(|e| {
        error!("Request failed: {}", e);
        format!("Ошибка запроса: {}", e)
    })?;

    info!("Response status: {}", response.status());

    // Check content length
    if let Some(content_length) = response.content_length() {
        info!(
            "Content-Length: {} bytes ({}MB)",
            content_length,
            content_length / 1024 / 1024
        );
        if content_length > MAX_SIZE {
            warn!("Content too large: {}MB", content_length / 1024 / 1024);
            return Err(format!(
                "Страница слишком большая ({}MB). Максимум: 10MB",
                content_length / 1024 / 1024
            ));
        }
    }

    debug!("Reading response bytes...");
    // Read response with size limit
    let bytes = response.bytes().await.map_err(|e| {
        error!("Failed to read response: {}", e);
        format!("Ошибка чтения ответа: {}", e)
    })?;

    info!(
        "Received {} bytes ({}MB)",
        bytes.len(),
        bytes.len() / 1024 / 1024
    );

    if bytes.len() > MAX_SIZE as usize {
        warn!("Response too large: {}MB", bytes.len() / 1024 / 1024);
        return Err(format!(
            "Страница слишком большая ({}MB). Максимум: 10MB",
            bytes.len() / 1024 / 1024
        ));
    }

    debug!("Converting bytes to UTF-8 string...");
    let html = String::from_utf8(bytes.to_vec()).map_err(|e| {
        error!("UTF-8 decode error: {}", e);
        format!("Ошибка декодирования UTF-8: {}", e)
    })?;

    info!(
        "Successfully fetched page, HTML length: {} chars",
        html.len()
    );

    // Сохраняем страницу в кэш для будущего использования
    // Используем нормализованный URL для сохранения
    let save_url = normalized_url.clone();

    if let Some(site_id_val) = site_id {
        // Сохраняем страницу с привязкой к сайту
        let url_obj = match url::Url::parse(&save_url) {
            Ok(u) => u,
            Err(_) => {
                warn!("Failed to parse URL for saving: {}", save_url);
                return Ok(html);
            }
        };

        let hostname = url_obj.host_str().unwrap_or("unknown").replace('.', "_");
        let timestamp = chrono::Utc::now().timestamp_millis();
        let folder_name = format!("page_{}_{}", timestamp, hostname);
        let filename = format!("{}/index.html", folder_name);

        // Сохраняем страницу локально с нормализованным URL
        match save_page_local(
            app_handle.clone(),
            html.clone(),
            filename,
            Some(site_id_val),
            Some(save_url.clone()),
        )
        .await
        {
            Ok(_) => {
                info!("Page saved to cache for site {}: {}", site_id_val, save_url);
            }
            Err(e) => {
                warn!("Failed to save page to cache: {}", e);
            }
        }
    } else {
        // Сохраняем в общий кэш (без привязки к сайту)
        let url_obj = match url::Url::parse(&save_url) {
            Ok(u) => u,
            Err(_) => {
                warn!("Failed to parse URL for saving: {}", save_url);
                return Ok(html);
            }
        };

        let hostname = url_obj.host_str().unwrap_or("unknown").replace('.', "_");
        let timestamp = chrono::Utc::now().timestamp_millis();
        let folder_name = format!("page_{}_{}", timestamp, hostname);
        let filename = format!("{}/index.html", folder_name);

        match save_page_local(
            app_handle.clone(),
            html.clone(),
            filename,
            None,
            Some(save_url.clone()),
        )
        .await
        {
            Ok(_) => {
                info!("Page saved to general cache: {}", save_url);
            }
            Err(e) => {
                warn!("Failed to save page to general cache: {}", e);
            }
        }
    }

    Ok(html)
}

/// Сохранить ресурс (CSS, изображение) локально
///
/// Сохраняет ресурс в указанную подпапку в директории saved_pages.
/// Генерирует безопасное имя файла из URL.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `url` - URL ресурса (используется для генерации имени файла)
/// * `data` - байты ресурса для сохранения
/// * `subfolder` - подпапка для сохранения (например, "images" или "css")
///
/// # Возвращает
/// Относительный путь к сохраненному файлу или ошибку
#[tauri::command]
async fn save_resource(
    app_handle: tauri::AppHandle,
    url: String,
    data: Vec<u8>,
    subfolder: String,
) -> Result<String, String> {
    use std::fs;
    use tauri::Manager;

    info!(
        "save_resource called, URL: {}, size: {} bytes, subfolder: {}",
        url,
        data.len(),
        subfolder
    );

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Не удалось получить директорию данных приложения: {}", e))?
        .join("saved_pages");

    // Создаем подпапку для ресурсов
    let resource_dir = app_data_dir.join(&subfolder);
    fs::create_dir_all(&resource_dir)
        .map_err(|e| format!("Не удалось создать директорию для ресурсов: {}", e))?;

    // Генерируем безопасное имя файла из URL
    let url_path = url.split('?').next().unwrap_or(&url); // Убираем query параметры
    let file_name = url_path.split('/').last().unwrap_or("resource");
    let safe_filename = file_name
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '_' || *c == '-')
        .collect::<String>();

    if safe_filename.is_empty() {
        return Err("Неверное имя файла".to_string());
    }

    let file_path = resource_dir.join(&safe_filename);
    fs::write(&file_path, data).map_err(|e| format!("Не удалось сохранить ресурс: {}", e))?;

    info!("Resource saved: {:?}", file_path);

    // Возвращаем относительный путь от HTML файла
    Ok(format!("{}/{}", subfolder, safe_filename))
}

/// Получить закешированный ресурс из локального кеша
///
/// Ищет сохраненный ресурс (CSS, изображение) в директории saved_pages.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `url` - URL ресурса для поиска
///
/// # Возвращает
/// Байты ресурса, если найдено, или None
async fn get_cached_resource(
    app_handle: &tauri::AppHandle,
    url: &str,
) -> Result<Option<Vec<u8>>, String> {
    use std::fs;
    use tauri::Manager;

    let url_obj = match url::Url::parse(url) {
        Ok(u) => u,
        Err(_) => return Ok(None),
    };

    // Генерируем имя файла из URL
    let url_path = url_obj.path();
    let file_name = url_path.split('/').last().unwrap_or("resource");
    let safe_filename = file_name
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '_' || *c == '-')
        .collect::<String>();

    if safe_filename.is_empty() {
        return Ok(None);
    }

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Не удалось получить директорию данных: {}", e))?
        .join("saved_pages");

    // Ищем ресурс в подпапках css и images
    for subfolder in &["css", "images"] {
        let resource_path = app_data_dir.join(subfolder).join(&safe_filename);
        if resource_path.exists() {
            match fs::read(&resource_path) {
                Ok(data) => {
                    info!("Found cached resource: {} in {}", url, subfolder);
                    return Ok(Some(data));
                }
                Err(e) => {
                    debug!(
                        "Failed to read cached resource from {:?}: {}",
                        resource_path, e
                    );
                }
            }
        }
    }

    // Также ищем в папках сохраненных страниц
    if app_data_dir.exists() {
        match fs::read_dir(&app_data_dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_dir() {
                            let folder_name =
                                path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                            if folder_name.starts_with("page_") {
                                // Проверяем в подпапках css и images
                                for subfolder in &["css", "images"] {
                                    let resource_path = path.join(subfolder).join(&safe_filename);
                                    if resource_path.exists() {
                                        match fs::read(&resource_path) {
                                            Ok(data) => {
                                                info!(
                                                    "Found cached resource: {} in {}/{}",
                                                    url, folder_name, subfolder
                                                );
                                                return Ok(Some(data));
                                            }
                                            Err(_) => continue,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }

    Ok(None)
}

/// Загрузить ресурс (CSS, изображение) с указанного URL или из кеша
///
/// Сначала проверяет локальный кеш, затем загружает с сервера с ограничением размера (максимум 5MB).
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `url` - URL ресурса для загрузки
///
/// # Возвращает
/// Байты ресурса или ошибку
#[tauri::command]
async fn fetch_resource(app_handle: tauri::AppHandle, url: String) -> Result<Vec<u8>, String> {
    info!("fetch_resource called with URL: {}", url);

    if url.is_empty() {
        return Err("URL не может быть пустым".to_string());
    }

    // Сначала проверяем локальный кеш
    match get_cached_resource(&app_handle, &url).await {
        Ok(Some(cached_data)) => {
            info!("Using cached resource: {}", url);
            return Ok(cached_data);
        }
        Ok(None) => {
            debug!("No cached resource found for: {}", url);
        }
        Err(e) => {
            warn!("Error checking cache for resource {}: {}", url, e);
        }
    }

    // Если не нашли в кеше, загружаем с сервера
    info!("Loading resource from server: {}", url);

    const MAX_SIZE: u64 = 5 * 1024 * 1024; // 5MB max per resource

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Не удалось создать HTTP клиент: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Ошибка запроса: {}", e))?;

    if let Some(content_length) = response.content_length() {
        if content_length > MAX_SIZE {
            return Err(format!(
                "Ресурс слишком большой ({}MB). Максимум: 5MB",
                content_length / 1024 / 1024
            ));
        }
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Ошибка чтения ответа: {}", e))?;

    if bytes.len() > MAX_SIZE as usize {
        return Err(format!(
            "Ресурс слишком большой ({}MB). Максимум: 5MB",
            bytes.len() / 1024 / 1024
        ));
    }

    info!(
        "Successfully fetched resource from server, size: {} bytes",
        bytes.len()
    );
    Ok(bytes.to_vec())
}

/// Сохранить HTML страницу локально с привязкой к сайту
///
/// Сохраняет HTML страницу в файловую систему и создает запись в базе данных
/// с привязкой к сайту (если указаны site_id и url). Создает новую версию страницы
/// с временной меткой.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `html` - HTML содержимое страницы для сохранения
/// * `filename` - имя файла для сохранения (может включать подпапки, например "page_123/index.html")
/// * `site_id` - ID сайта для привязки в базе данных (опционально)
/// * `url` - URL страницы для привязки в базе данных (опционально)
///
/// # Возвращает
/// file:// URL сохраненного файла или ошибку
#[tauri::command]
async fn save_page_local(
    app_handle: tauri::AppHandle,
    html: String,
    filename: String,
    site_id: Option<i64>,
    url: Option<String>,
) -> Result<String, String> {
    use std::fs;
    use tauri::Manager;

    info!(
        "save_page_local called, filename: {}, HTML size: {} bytes",
        filename,
        html.len()
    );

    // Limit HTML size to prevent crashes (5MB max)
    const MAX_SIZE: usize = 5 * 1024 * 1024;
    if html.len() > MAX_SIZE {
        warn!("HTML too large: {}MB", html.len() / 1024 / 1024);
        return Err(format!(
            "HTML слишком большой ({}MB). Максимум: 5MB",
            html.len() / 1024 / 1024
        ));
    }

    // Use AppData directory via Tauri API to avoid triggering file watcher
    // This prevents Tauri from restarting when files are saved
    // In Tauri 2.0, use app_handle.path() API with Manager trait
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| {
            error!("Failed to get app data directory: {}", e);
            format!("Не удалось получить директорию данных приложения: {}", e)
        })?
        .join("saved_pages");

    debug!("Saving to directory: {:?}", app_data_dir);

    // Create directory if it doesn't exist
    fs::create_dir_all(&app_data_dir).map_err(|e| {
        error!("Failed to create directory: {}", e);
        format!("Не удалось создать директорию: {}", e)
    })?;

    // Validate filename to prevent path traversal
    // Разрешаем слэши для подпапок, но проверяем на path traversal
    let safe_filename = filename
        .replace('\\', "/") // Нормализуем слэши
        .split('/')
        .map(|part| {
            part.chars()
                .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '_' || *c == '-')
                .collect::<String>()
        })
        .filter(|s| !s.is_empty() && s != ".." && s != ".")
        .collect::<Vec<_>>()
        .join("/");

    if safe_filename.is_empty() {
        warn!("Invalid filename after sanitization");
        return Err("Неверное имя файла".to_string());
    }

    // Save file (создаем подпапки если нужно)
    let file_path = app_data_dir.join(&safe_filename);

    // Создаем родительские директории если нужно
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            error!("Failed to create parent directory: {}", e);
            format!("Не удалось создать директорию: {}", e)
        })?;
    }
    debug!("Writing to file: {:?}", file_path);
    fs::write(&file_path, html).map_err(|e| {
        error!("Failed to write file: {}", e);
        format!("Не удалось сохранить файл: {}", e)
    })?;

    info!("File saved successfully: {:?}", file_path);

    // Привязываем сохраненную страницу к сайту, если указаны site_id и url
    // Создаем новую версию с временной меткой
    if let (Some(site_id), Some(page_url)) = (site_id, url) {
        let db = Database::new().await.map_err(|e| e.to_string())?;
        let folder_path = safe_filename.trim_end_matches("/index.html");
        let version_timestamp = Utc::now().to_rfc3339();
        if let Err(e) = db
            .save_page_for_site(site_id, &page_url, folder_path, &version_timestamp)
            .await
        {
            warn!("Failed to save page link to database: {}", e);
        } else {
            info!(
                "Page linked to site {}: {} (version: {})",
                site_id, page_url, version_timestamp
            );
        }
    }

    // Return file:// URL for loading in iframe
    // Windows: file:///C:/path/to/file
    // Unix: file:///path/to/file
    let path_str = file_path.to_string_lossy().replace('\\', "/");
    let file_url = if cfg!(windows) {
        // On Windows, ensure we have the drive letter format
        format!("file:///{}", path_str)
    } else {
        format!("file://{}", path_str)
    };

    debug!("File URL: {}", file_url);
    Ok(file_url)
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
async fn test_parser(site_id: i64) -> Result<Vec<models::Mod>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    let site = db.get_site(site_id).await.map_err(|e| e.to_string())?;
    let engine = ParserEngine::new();
    engine.parse_site(&site).await.map_err(|e| e.to_string())
}

/// Генерирует конфигурацию парсера с помощью AI
///
/// Использует AI (Ollama или OpenAI) для анализа HTML и автоматической генерации
/// конфигурации парсера с CSS селекторами.
///
/// # Параметры
/// * `html` - HTML код страницы для анализа
/// * `description` - описание данных, которые нужно извлечь (например, "Извлеки информацию о модах: название, ссылка, описание, изображение")
/// * `model_type` - тип модели: "ollama" или "openai"
/// * `model_name` - название модели (для Ollama: "llama3.2:3b", для OpenAI: "gpt-4o-mini")
/// * `api_key` - API ключ (только для OpenAI, опционально)
/// * `ollama_url` - URL Ollama сервера (опционально, по умолчанию "http://localhost:11434/api/generate")
///
/// # Возвращает
/// JSON конфигурацию парсера с CSS селекторами или ошибку
#[tauri::command]
async fn ai_generate_parser(
    html: String,
    description: String,
    model_type: String,
    model_name: Option<String>,
    api_key: Option<String>,
    ollama_url: Option<String>,
) -> Result<serde_json::Value, String> {
    info!(
        "AI generate parser called: model_type={}, model_name={:?}",
        model_type, model_name
    );

    let parser = match model_type.as_str() {
        "ollama" => {
            let model = model_name.unwrap_or_else(|| "llama3.2:3b".to_string());
            AIParser::ollama(model, ollama_url)
        }
        "openai" => {
            let api_key = api_key.ok_or("API key required for OpenAI")?;
            let model = model_name.unwrap_or_else(|| "gpt-4o-mini".to_string());
            AIParser::openai(api_key, Some(model))
        }
        _ => {
            return Err(format!(
                "Unknown model type: {}. Use 'ollama' or 'openai'",
                model_type
            ));
        }
    };

    parser
        .generate_parser(&html, &description)
        .await
        .map_err(|e| {
            error!("AI parser error: {}", e);
            format!("Ошибка генерации парсера: {}", e)
        })
}

/// Отправляет сообщение в чат с AI
///
/// # Параметры
/// * `messages` - история сообщений (роль и содержимое)
/// * `model_type` - тип модели: "ollama", "openai", "anthropic", "google"
/// * `model_name` - название модели
/// * `api_key` - API ключ (для OpenAI, Anthropic, Google)
/// * `ollama_url` - URL Ollama сервера (опционально)
///
/// # Возвращает
/// Ответ от AI
#[tauri::command]
async fn ai_chat(
    messages: Vec<(String, String)>,
    model_type: String,
    model_name: Option<String>,
    api_key: Option<String>,
    ollama_url: Option<String>,
) -> Result<String, String> {
    info!(
        "AI chat called: model_type={}, model_name={:?}",
        model_type, model_name
    );

    let parser = match model_type.as_str() {
        "ollama" => {
            let model = model_name.unwrap_or_else(|| "llama3.2:3b".to_string());
            AIParser::ollama(model, ollama_url)
        }
        "openai" => {
            let api_key = api_key.ok_or("API key required for OpenAI")?;
            let model = model_name.unwrap_or_else(|| "gpt-4o-mini".to_string());
            AIParser::openai(api_key, Some(model))
        }
        "anthropic" => {
            let api_key = api_key.ok_or("API key required for Anthropic")?;
            let model = model_name.unwrap_or_else(|| "claude-3-haiku-20240307".to_string());
            AIParser::anthropic(api_key, Some(model))
        }
        "google" => {
            let api_key = api_key.ok_or("API key required for Google")?;
            let model = model_name.unwrap_or_else(|| "gemini-pro".to_string());
            AIParser::google(api_key, Some(model))
        }
        _ => {
            return Err(format!(
                "Unknown model type: {}. Use 'ollama', 'openai', 'anthropic', or 'google'",
                model_type
            ));
        }
    };

    parser.chat(messages).await.map_err(|e| {
        error!("AI chat error: {}", e);
        format!("Ошибка чата: {}", e)
    })
}

/// Проверяет статус Ollama и возвращает список доступных моделей
///
/// # Параметры
/// * `ollama_url` - URL Ollama сервера (опционально)
///
/// # Возвращает
/// Список моделей или ошибку
#[tauri::command]
async fn ai_check_ollama(ollama_url: Option<String>) -> Result<Vec<String>, String> {
    use reqwest;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct OllamaListResponse {
        models: Vec<OllamaModel>,
    }

    #[derive(Deserialize)]
    struct OllamaModel {
        name: String,
    }

    let url = ollama_url.unwrap_or_else(|| "http://localhost:11434".to_string());
    let list_url = format!("{}/api/tags", url);

    info!("Checking Ollama at: {}", list_url);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client.get(&list_url).send().await.map_err(|e| {
        format!(
            "Failed to connect to Ollama: {}. Убедитесь, что Ollama запущен.",
            e
        )
    })?;

    if !response.status().is_success() {
        return Err(format!(
            "Ollama returned error status: {}",
            response.status()
        ));
    }

    let api_response: OllamaListResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    let models: Vec<String> = api_response.models.into_iter().map(|m| m.name).collect();

    info!("Found {} Ollama models", models.len());
    Ok(models)
}

/// Получить список всех уведомлений
///
/// Возвращает последние 100 уведомлений, отсортированных по дате создания.
///
/// # Возвращает
/// Вектор уведомлений или ошибку
#[tauri::command]
async fn get_notifications() -> Result<Vec<models::Notification>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.get_notifications().await.map_err(|e| e.to_string())
}

/// Отметить уведомление как прочитанное
///
/// # Параметры
/// * `id` - ID уведомления для отметки
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
#[tauri::command]
async fn mark_notification_read(id: i64) -> Result<(), String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.mark_notification_read(id)
        .await
        .map_err(|e| e.to_string())
}

/// Получить закешированную страницу из общего кеша
///
/// Ищет сохраненную страницу в директории saved_pages по hostname и pathname из URL.
/// Возвращает самую новую найденную версию страницы.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `url` - URL страницы для поиска в кеше
///
/// # Возвращает
/// HTML содержимое страницы, если найдено, или None
#[tauri::command]
async fn get_cached_page(
    app_handle: tauri::AppHandle,
    url: String,
) -> Result<Option<String>, String> {
    use std::fs;
    use tauri::Manager;

    info!("[CACHE] get_cached_page called for URL: {}", url);

    let url_obj = match url::Url::parse(&url) {
        Ok(u) => u,
        Err(e) => {
            warn!("[CACHE] Invalid URL format: {}, error: {}", url, e);
            return Ok(None);
        }
    };

    let hostname = url_obj.host_str().unwrap_or("unknown").replace('.', "_");
    let pathname = url_obj.path();
    info!(
        "[CACHE] Parsed URL - hostname: {}, pathname: {}",
        hostname, pathname
    );

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Не удалось получить директорию данных: {}", e))?
        .join("saved_pages");

    if !app_data_dir.exists() {
        info!("[CACHE] Cache directory does not exist: {:?}", app_data_dir);
        return Ok(None);
    }

    info!("[CACHE] Searching in directory: {:?}", app_data_dir);

    // Ищем все папки, которые начинаются с page_ и содержат hostname
    // Сортируем по дате модификации (новые первыми)
    let mut candidates: Vec<(std::path::PathBuf, std::time::SystemTime, String)> = Vec::new();

    match fs::read_dir(&app_data_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        let folder_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                        // Проверяем, соответствует ли папка этому URL
                        if folder_name.starts_with("page_") && folder_name.contains(&hostname) {
                            let index_file = path.join("index.html");
                            if index_file.exists() {
                                // Получаем время модификации для сортировки
                                if let Ok(metadata) = fs::metadata(&index_file) {
                                    if let Ok(modified) = metadata.modified() {
                                        candidates.push((
                                            path.clone(),
                                            modified,
                                            folder_name.to_string(),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            warn!("[CACHE] Failed to read cache directory: {}", e);
        }
    }

    info!(
        "[CACHE] Found {} candidate folders for hostname: {}",
        candidates.len(),
        hostname
    );

    // Сортируем по дате модификации (новые первыми)
    candidates.sort_by(|a, b| b.1.cmp(&a.1));

    // Проверяем кандидатов, начиная с самых новых
    // Для каждого кандидата проверяем, соответствует ли он запрашиваемому URL
    // Проверяем по pathname через data-base-url в HTML
    let normalized_request_url = normalize_url_for_comparison(&url);
    info!(
        "[CACHE] Normalized request URL for comparison: {}",
        normalized_request_url
    );

    for (path, _, folder_name) in candidates {
        let index_file = path.join("index.html");
        if index_file.exists() {
            match fs::read_to_string(&index_file) {
                Ok(html) => {
                    info!(
                        "[CACHE] Checking candidate folder: {} for URL: {}",
                        folder_name, url
                    );

                    // Проверяем, содержит ли HTML data-base-url атрибут с нужным URL
                    let url_matches = if let Some(data_base_url) = extract_data_base_url(&html) {
                        let normalized_data_url = normalize_url_for_comparison(&data_base_url);
                        let matches = normalized_data_url == normalized_request_url
                            || normalized_data_url.starts_with(&normalized_request_url)
                            || normalized_request_url.starts_with(&normalized_data_url);
                        info!("[CACHE] Comparing URLs - saved: {} (normalized: {}), requested: {} (normalized: {}), matches: {}", 
                              data_base_url, normalized_data_url, url, normalized_request_url, matches);
                        matches
                    } else {
                        // Если data-base-url не найден, проверяем по pathname из URL
                        let matches = match url::Url::parse(&url) {
                            Ok(url_obj) => {
                                let requested_path = url_obj.path();
                                // Если pathname пустой или только "/", считаем совпадением
                                // (это может быть главная страница)
                                let matches = requested_path == "/" || requested_path.is_empty();
                                info!("[CACHE] No data-base-url found in HTML, checking pathname: {}, matches: {}", requested_path, matches);
                                matches
                            }
                            Err(_) => {
                                warn!("[CACHE] Failed to parse URL for comparison: {}", url);
                                false
                            }
                        };
                        matches
                    };

                    if url_matches {
                        info!("[CACHE] ✓ Found matching cached page for URL: {} in folder: {} (HTML length: {} chars)", url, folder_name, html.len());
                        return Ok(Some(html));
                    } else {
                        info!("[CACHE] URL mismatch in folder: {}, skipping", folder_name);
                    }
                }
                Err(e) => {
                    warn!(
                        "[CACHE] Failed to read cached page from folder {}: {}",
                        folder_name, e
                    );
                }
            }
        }
    }

    info!("[CACHE] ✗ No cached page found for URL: {}", url);
    Ok(None)
}

/// Получить список всех закешированных страниц
///
/// Сканирует директорию saved_pages и возвращает информацию о всех сохраненных страницах.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
///
/// # Возвращает
/// Вектор объектов с информацией о страницах (folder, path, modified, size), отсортированных по дате изменения
#[tauri::command]
async fn list_cached_pages(app_handle: tauri::AppHandle) -> Result<Vec<serde_json::Value>, String> {
    use std::fs;
    use tauri::Manager;

    info!("list_cached_pages called");

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Не удалось получить директорию данных: {}", e))?
        .join("saved_pages");

    let mut cached_pages = Vec::new();

    if !app_data_dir.exists() {
        return Ok(cached_pages);
    }

    match fs::read_dir(&app_data_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        let index_file = path.join("index.html");
                        if index_file.exists() {
                            if let Ok(metadata) = fs::metadata(&index_file) {
                                let modified = metadata
                                    .modified()
                                    .ok()
                                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                    .map(|d| d.as_secs());

                                let folder_name = path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("unknown")
                                    .to_string();

                                cached_pages.push(serde_json::json!({
                                    "folder": folder_name,
                                    "path": path.to_string_lossy().to_string(),
                                    "modified": modified,
                                    "size": metadata.len()
                                }));
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            warn!("Failed to read cache directory: {}", e);
        }
    }

    // Сортируем по дате изменения (новые первыми)
    cached_pages.sort_by(|a, b| {
        let a_time = a["modified"].as_u64().unwrap_or(0);
        let b_time = b["modified"].as_u64().unwrap_or(0);
        b_time.cmp(&a_time)
    });

    info!("Found {} cached pages", cached_pages.len());
    Ok(cached_pages)
}

/// Очистить кеш сохраненных страниц
///
/// Удаляет указанную папку из кеша или весь кеш, если папка не указана.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `folder` - имя папки для удаления (None = удалить весь кеш)
///
/// # Возвращает
/// Пустой результат при успехе или ошибку
#[tauri::command]
async fn clear_page_cache(
    app_handle: tauri::AppHandle,
    folder: Option<String>,
) -> Result<(), String> {
    use std::fs;
    use tauri::Manager;

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Не удалось получить директорию данных: {}", e))?
        .join("saved_pages");

    if let Some(folder_name) = folder {
        // Удаляем конкретную папку
        let folder_path = app_data_dir.join(&folder_name);
        if folder_path.exists() {
            info!("Clearing cache folder: {}", folder_name);
            fs::remove_dir_all(&folder_path)
                .map_err(|e| format!("Не удалось удалить папку кеша: {}", e))?;
        }
    } else {
        // Удаляем весь кеш
        info!("Clearing all page cache");
        if app_data_dir.exists() {
            fs::remove_dir_all(&app_data_dir)
                .map_err(|e| format!("Не удалось очистить кеш: {}", e))?;
            // Создаем директорию заново
            fs::create_dir_all(&app_data_dir)
                .map_err(|e| format!("Не удалось создать директорию: {}", e))?;
        }
    }

    Ok(())
}

/// Получить сохраненную страницу для сайта (последняя версия)
///
/// Ищет сохраненную страницу в базе данных по site_id и url, затем загружает
/// HTML содержимое из файловой системы.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `site_id` - ID сайта для поиска
/// * `url` - URL страницы для поиска
///
/// # Возвращает
/// HTML содержимое страницы, если найдено, или None
#[tauri::command]
async fn get_saved_page_for_site(
    app_handle: tauri::AppHandle,
    site_id: i64,
    url: String,
) -> Result<Option<String>, String> {
    use std::fs;
    use tauri::Manager;

    info!(
        "[DB_CACHE] get_saved_page_for_site called for site_id: {}, URL: {}",
        site_id, url
    );

    let db = Database::new().await.map_err(|e| e.to_string())?;

    match db.get_saved_page(site_id, &url).await {
        Ok(Some(folder_path)) => {
            info!(
                "[DB_CACHE] Found saved page in database for site {}: {} -> folder: {}",
                site_id, url, folder_path
            );

            // Читаем сохраненную страницу из папки
            let app_data_dir = app_handle
                .path()
                .app_data_dir()
                .map_err(|e| format!("Не удалось получить директорию данных: {}", e))?
                .join("saved_pages")
                .join(&folder_path)
                .join("index.html");

            info!("[DB_CACHE] Reading HTML from: {:?}", app_data_dir);

            if app_data_dir.exists() {
                match fs::read_to_string(&app_data_dir) {
                    Ok(html) => {
                        info!("[DB_CACHE] ✓ Loaded saved page for site {}: {} from folder: {} (HTML length: {} chars)", site_id, url, folder_path, html.len());
                        Ok(Some(html))
                    }
                    Err(e) => {
                        warn!("[DB_CACHE] Failed to read saved page: {}", e);
                        Ok(None)
                    }
                }
            } else {
                warn!("[DB_CACHE] File does not exist: {:?}", app_data_dir);
                Ok(None)
            }
        }
        Ok(None) => {
            info!(
                "[DB_CACHE] ✗ No saved page found in database for site {}: {}",
                site_id, url
            );
            Ok(None)
        }
        Err(e) => {
            warn!("[DB_CACHE] Failed to get saved page from database: {}", e);
            Ok(None)
        }
    }
}

/// Получить папку кеша для URL (для поиска ресурсов)
///
/// Ищет папку кеша для указанного URL, чтобы можно было найти сохраненные ресурсы.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `url` - URL страницы для поиска
///
/// # Возвращает
/// Путь к папке кеша, если найдено, или None
#[tauri::command]
async fn get_cache_folder_for_url(
    app_handle: tauri::AppHandle,
    url: String,
) -> Result<Option<String>, String> {
    use std::fs;
    use tauri::Manager;

    let url_obj = match url::Url::parse(&url) {
        Ok(u) => u,
        Err(_) => return Ok(None),
    };

    let hostname = url_obj.host_str().unwrap_or("unknown").replace('.', "_");

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Не удалось получить директорию данных: {}", e))?
        .join("saved_pages");

    if !app_data_dir.exists() {
        return Ok(None);
    }

    // Ищем папки, которые начинаются с page_ и содержат hostname
    let mut candidates: Vec<(String, std::time::SystemTime)> = Vec::new();

    match fs::read_dir(&app_data_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        let folder_name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .map(|s| s.to_string())
                            .unwrap_or_default();

                        if folder_name.starts_with("page_") && folder_name.contains(&hostname) {
                            let index_file = path.join("index.html");
                            if index_file.exists() {
                                if let Ok(metadata) = fs::metadata(&index_file) {
                                    if let Ok(modified) = metadata.modified() {
                                        candidates.push((folder_name, modified));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(_) => {}
    }

    // Сортируем по дате модификации (новые первыми)
    candidates.sort_by(|a, b| b.1.cmp(&a.1));

    // Возвращаем самую новую папку
    if let Some((folder_name, _)) = candidates.first() {
        Ok(Some(folder_name.clone()))
    } else {
        Ok(None)
    }
}

/// Получить все версии сохраненной страницы для сайта
#[tauri::command]
async fn get_saved_page_versions(
    site_id: i64,
    url: String,
) -> Result<Vec<serde_json::Value>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;

    match db.get_saved_page_versions(site_id, &url).await {
        Ok(versions) => Ok(versions
            .iter()
            .map(|(id, folder_path, timestamp)| {
                serde_json::json!({
                    "id": id,
                    "folder_path": folder_path,
                    "timestamp": timestamp
                })
            })
            .collect()),
        Err(e) => {
            warn!("Failed to get saved page versions: {}", e);
            Ok(vec![])
        }
    }
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
#[tauri::command]
async fn delete_saved_page_version(page_id: i64) -> Result<(), String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.delete_saved_page_version(page_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    // Initialize logger - show all logs in debug mode
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .format_timestamp_secs()
        .init();

    info!("Starting Tauri application");

    // Only include the MCP plugin in development builds
    #[cfg(debug_assertions)]
    {
        info!("Development build detected, enabling MCP plugin");
        use tauri_plugin_mcp::PluginConfig;
        tauri::Builder::default()
            .plugin(tauri_plugin_shell::init())
            .plugin(tauri_plugin_notification::init())
            .plugin(tauri_plugin_fs::init())
            .plugin(tauri_plugin_mcp::init_with_config(
                PluginConfig::new("Mod Aggregator".to_string())
                    .start_socket_server(true)
                    // Use TCP mode for more reliable connection
                    .tcp("127.0.0.1".to_string(), 4000),
            ))
            .setup(|app| {
                info!("Tauri app setup started");

                // Initialize database
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    info!("Initializing database...");
                    if let Err(e) = Database::new().await {
                        error!("Failed to initialize database: {}", e);
                    } else {
                        info!("Database initialized successfully");
                    }
                });

                // Start background update checker
                let app_handle_clone = app_handle.clone();
                tauri::async_runtime::spawn(async move {
                    let mut interval =
                        tokio::time::interval(tokio::time::Duration::from_secs(3600));
                    loop {
                        interval.tick().await;
                        if let Ok(updates) = check_updates(None).await {
                            if !updates.is_empty() {
                                let notification_service =
                                    NotificationService::new(app_handle_clone.clone());
                                for update in updates {
                                    notification_service.notify_update(&update).await.ok();
                                }
                            }
                        }
                    }
                });

                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                fetch_resource,
                save_resource,
                get_cache_folder_for_url,
                get_sites,
                add_site,
                update_site,
                delete_site,
                get_mods,
                check_updates,
                build_parser,
                test_parser_from_nodes,
                fetch_page,
                save_page_local,
                test_parser,
                ai_generate_parser,
                ai_chat,
                ai_check_ollama,
                get_notifications,
                mark_notification_read,
                get_cached_page,
                list_cached_pages,
                clear_page_cache,
                get_saved_page_for_site,
                get_saved_page_versions,
                delete_saved_page_version,
                // Dependency commands
                get_file_dependencies,
                add_file_dependency,
                remove_file_dependency,
                check_dependencies,
                check_circular_dependencies,
                get_dependent_files,
                resolve_dependency_version,
                get_dependency_graph,
                get_installation_order,
                // File commands
                create_file,
                update_file,
                delete_file,
                upload_file_version,
                get_file_by_name_version,
                get_file_versions,
                get_all_files,
                // Import/Export commands
                export_file,
                export_collection,
                export_build,
                import_file,
                import_collection,
                import_build,
                validate_import_json,
                // Batch operations commands
                batch_delete_files,
                batch_move_files_to_collection,
                batch_update_file_properties,
                batch_add_dependencies,
                batch_check_dependencies,
                batch_export_files,
                validate_batch_operation,
                // Collection commands
                get_collections,
                create_collection,
                update_collection,
                delete_collection,
                get_collection_files,
                add_file_to_collection,
                remove_file_from_collection,
                reorder_collection_files,
                get_collection_logic_rules,
                create_collection_logic_rule,
                update_collection_logic_rule,
                delete_collection_logic_rule,
                evaluate_collection_logic,
                combine_collections,
                get_files_from_multiple_collections,
                // Batch operations commands
                batch_delete_files,
                batch_move_files_to_collection,
                batch_update_file_properties,
                batch_add_dependencies,
                batch_check_dependencies,
                batch_export_files,
                validate_batch_operation,
                // Session state commands
                get_session_state,
                update_file_order,
                update_ui_preferences,
                update_open_collections,
                update_selected_files,
                add_recent_action,
                get_recent_actions,
                clear_recent_actions,
                restore_session,
                reset_session_state
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }

    #[cfg(not(debug_assertions))]
    {
        tauri::Builder::default()
            .plugin(tauri_plugin_shell::init())
            .plugin(tauri_plugin_notification::init())
            .plugin(tauri_plugin_fs::init())
            .setup(|app| {
                info!("Tauri app setup started");

                // Initialize database
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    info!("Initializing database...");
                    if let Err(e) = Database::new().await {
                        error!("Failed to initialize database: {}", e);
                    } else {
                        info!("Database initialized successfully");
                    }
                });

                // Start background update checker
                let app_handle_clone = app_handle.clone();
                tauri::async_runtime::spawn(async move {
                    let mut interval =
                        tokio::time::interval(tokio::time::Duration::from_secs(3600));
                    loop {
                        interval.tick().await;
                        if let Ok(updates) = check_updates(None).await {
                            if !updates.is_empty() {
                                let notification_service =
                                    NotificationService::new(app_handle_clone.clone());
                                for update in updates {
                                    notification_service.notify_update(&update).await.ok();
                                }
                            }
                        }
                    }
                });

                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                fetch_resource,
                save_resource,
                get_cache_folder_for_url,
                get_sites,
                add_site,
                update_site,
                delete_site,
                get_mods,
                check_updates,
                build_parser,
                fetch_page,
                save_page_local,
                test_parser,
                ai_generate_parser,
                get_notifications,
                mark_notification_read,
                get_cached_page,
                list_cached_pages,
                clear_page_cache,
                delete_saved_page_version,
                // File commands
                get_all_files,
                add_file,
                update_file,
                delete_file,
                get_file_by_id,
                // Dependency commands
                add_dependency,
                remove_dependency,
                get_dependencies,
                check_dependencies,
                get_dependency_graph,
                detect_circular_dependencies,
                // Collection commands
                create_collection,
                update_collection,
                delete_collection,
                get_collections,
                get_collection_by_id,
                add_file_to_collection,
                remove_file_from_collection,
                get_files_in_collection,
                create_collection_logic_rule,
                update_collection_logic_rule,
                delete_collection_logic_rule,
                evaluate_collection_logic,
                combine_collections,
                get_files_from_multiple_collections,
                // Batch operations commands
                batch_delete_files,
                batch_move_files_to_collection,
                batch_update_file_properties,
                batch_add_dependencies,
                batch_check_dependencies,
                batch_export_files,
                validate_batch_operation,
                // Session state commands
                get_session_state,
                update_file_order,
                update_ui_preferences,
                update_open_collections,
                update_selected_files,
                add_recent_action,
                get_recent_actions,
                clear_recent_actions,
                restore_session,
                reset_session_state
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
