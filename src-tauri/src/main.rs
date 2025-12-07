// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod parser;
mod parser_builder;
mod notification;
mod models;

// mod parsers;

use database::Database;
use parser::ParserEngine;
use parser_builder::ParserBuilder;
use notification::NotificationService;
use log::{info, warn, error, debug};
use chrono::Utc;

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
        Err(_) => url.to_string()
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
async fn add_site(name: String, url: String, parser_config: serde_json::Value) -> Result<models::Site, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.add_site(&name, &url, &parser_config).await.map_err(|e| e.to_string())
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
async fn update_site(id: i64, name: String, url: String, parser_config: serde_json::Value) -> Result<(), String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.update_site(id, &name, &url, &parser_config).await.map_err(|e| e.to_string())
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
    builder.build_from_selector(&html, &selector).await.map_err(|e| e.to_string())
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
async fn fetch_page(app_handle: tauri::AppHandle, url: String, force_refresh: bool, site_id: Option<i64>) -> Result<String, String> {
    info!("fetch_page called with URL: {}, force_refresh: {}, site_id: {:?}", url, force_refresh, site_id);
    
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
        info!("Checking cache for URL: {} (normalized: {})", url, normalized_url);
        
        // Сначала проверяем сохраненную страницу для сайта (если site_id указан)
        if let Some(site_id_val) = site_id {
            info!("Checking saved page for site {} with URL: {}", site_id_val, normalized_url);
            match get_saved_page_for_site(app_handle.clone(), site_id_val, normalized_url.clone()).await {
                Ok(Some(cached_html)) => {
                    info!("✓ Found cached page for site {}: {} (HTML length: {} chars)", site_id_val, normalized_url, cached_html.len());
                    return Ok(cached_html);
                }
                Ok(None) => {
                    info!("✗ No cached page found for site {}: {}", site_id_val, normalized_url);
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
                info!("✓ Found cached page from general cache: {} (HTML length: {} chars)", normalized_url, cached_html.len());
                return Ok(cached_html);
            }
            Ok(None) => {
                info!("✗ No cached page found in general cache: {}", normalized_url);
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
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            error!("Request failed: {}", e);
            format!("Ошибка запроса: {}", e)
        })?;
    
    info!("Response status: {}", response.status());
    
    // Check content length
    if let Some(content_length) = response.content_length() {
        info!("Content-Length: {} bytes ({}MB)", content_length, content_length / 1024 / 1024);
        if content_length > MAX_SIZE {
            warn!("Content too large: {}MB", content_length / 1024 / 1024);
            return Err(format!("Страница слишком большая ({}MB). Максимум: 10MB", content_length / 1024 / 1024));
        }
    }
    
    debug!("Reading response bytes...");
    // Read response with size limit
    let bytes = response.bytes().await
        .map_err(|e| {
            error!("Failed to read response: {}", e);
            format!("Ошибка чтения ответа: {}", e)
        })?;
    
    info!("Received {} bytes ({}MB)", bytes.len(), bytes.len() / 1024 / 1024);
    
    if bytes.len() > MAX_SIZE as usize {
        warn!("Response too large: {}MB", bytes.len() / 1024 / 1024);
        return Err(format!("Страница слишком большая ({}MB). Максимум: 10MB", bytes.len() / 1024 / 1024));
    }
    
    debug!("Converting bytes to UTF-8 string...");
    let html = String::from_utf8(bytes.to_vec())
        .map_err(|e| {
            error!("UTF-8 decode error: {}", e);
            format!("Ошибка декодирования UTF-8: {}", e)
        })?;
    
    info!("Successfully fetched page, HTML length: {} chars", html.len());
    
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
        match save_page_local(app_handle.clone(), html.clone(), filename, Some(site_id_val), Some(save_url.clone())).await {
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
        
        match save_page_local(app_handle.clone(), html.clone(), filename, None, Some(save_url.clone())).await {
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
async fn save_resource(app_handle: tauri::AppHandle, url: String, data: Vec<u8>, subfolder: String) -> Result<String, String> {
    use std::fs;
    use tauri::Manager;
    
    info!("save_resource called, URL: {}, size: {} bytes, subfolder: {}", url, data.len(), subfolder);
    
    let app_data_dir = app_handle.path()
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
    fs::write(&file_path, data)
        .map_err(|e| format!("Не удалось сохранить ресурс: {}", e))?;
    
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
async fn get_cached_resource(app_handle: &tauri::AppHandle, url: &str) -> Result<Option<Vec<u8>>, String> {
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
    
    let app_data_dir = app_handle.path()
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
                    debug!("Failed to read cached resource from {:?}: {}", resource_path, e);
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
                            let folder_name = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("");
                            
                            if folder_name.starts_with("page_") {
                                // Проверяем в подпапках css и images
                                for subfolder in &["css", "images"] {
                                    let resource_path = path.join(subfolder).join(&safe_filename);
                                    if resource_path.exists() {
                                        match fs::read(&resource_path) {
                                            Ok(data) => {
                                                info!("Found cached resource: {} in {}/{}", url, folder_name, subfolder);
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
            return Err(format!("Ресурс слишком большой ({}MB). Максимум: 5MB", content_length / 1024 / 1024));
        }
    }
    
    let bytes = response.bytes().await
        .map_err(|e| format!("Ошибка чтения ответа: {}", e))?;
    
    if bytes.len() > MAX_SIZE as usize {
        return Err(format!("Ресурс слишком большой ({}MB). Максимум: 5MB", bytes.len() / 1024 / 1024));
    }
    
    info!("Successfully fetched resource from server, size: {} bytes", bytes.len());
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
    
    info!("save_page_local called, filename: {}, HTML size: {} bytes", filename, html.len());
    
    // Limit HTML size to prevent crashes (5MB max)
    const MAX_SIZE: usize = 5 * 1024 * 1024;
    if html.len() > MAX_SIZE {
        warn!("HTML too large: {}MB", html.len() / 1024 / 1024);
        return Err(format!("HTML слишком большой ({}MB). Максимум: 5MB", html.len() / 1024 / 1024));
    }
    
    // Use AppData directory via Tauri API to avoid triggering file watcher
    // This prevents Tauri from restarting when files are saved
    // In Tauri 2.0, use app_handle.path() API with Manager trait
    let app_data_dir = app_handle.path()
        .app_data_dir()
        .map_err(|e| {
            error!("Failed to get app data directory: {}", e);
            format!("Не удалось получить директорию данных приложения: {}", e)
        })?
        .join("saved_pages");
    
    debug!("Saving to directory: {:?}", app_data_dir);
    
    // Create directory if it doesn't exist
    fs::create_dir_all(&app_data_dir)
        .map_err(|e| {
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
        fs::create_dir_all(parent)
            .map_err(|e| {
                error!("Failed to create parent directory: {}", e);
                format!("Не удалось создать директорию: {}", e)
            })?;
    }
    debug!("Writing to file: {:?}", file_path);
    fs::write(&file_path, html)
        .map_err(|e| {
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
        if let Err(e) = db.save_page_for_site(site_id, &page_url, folder_path, &version_timestamp).await {
            warn!("Failed to save page link to database: {}", e);
        } else {
            info!("Page linked to site {}: {} (version: {})", site_id, page_url, version_timestamp);
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
    db.mark_notification_read(id).await.map_err(|e| e.to_string())
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
async fn get_cached_page(app_handle: tauri::AppHandle, url: String) -> Result<Option<String>, String> {
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
    info!("[CACHE] Parsed URL - hostname: {}, pathname: {}", hostname, pathname);
    
    let app_data_dir = app_handle.path()
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
                        let folder_name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                        
                        // Проверяем, соответствует ли папка этому URL
                        if folder_name.starts_with("page_") && folder_name.contains(&hostname) {
                            let index_file = path.join("index.html");
                            if index_file.exists() {
                                // Получаем время модификации для сортировки
                                if let Ok(metadata) = fs::metadata(&index_file) {
                                    if let Ok(modified) = metadata.modified() {
                                        candidates.push((path.clone(), modified, folder_name.to_string()));
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
    
    info!("[CACHE] Found {} candidate folders for hostname: {}", candidates.len(), hostname);
    
    // Сортируем по дате модификации (новые первыми)
    candidates.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Проверяем кандидатов, начиная с самых новых
    // Для каждого кандидата проверяем, соответствует ли он запрашиваемому URL
    // Проверяем по pathname через data-base-url в HTML
    let normalized_request_url = normalize_url_for_comparison(&url);
    info!("[CACHE] Normalized request URL for comparison: {}", normalized_request_url);
    
    for (path, _, folder_name) in candidates {
        let index_file = path.join("index.html");
        if index_file.exists() {
            match fs::read_to_string(&index_file) {
                Ok(html) => {
                    info!("[CACHE] Checking candidate folder: {} for URL: {}", folder_name, url);
                    
                    // Проверяем, содержит ли HTML data-base-url атрибут с нужным URL
                    let url_matches = if let Some(data_base_url) = extract_data_base_url(&html) {
                        let normalized_data_url = normalize_url_for_comparison(&data_base_url);
                        let matches = normalized_data_url == normalized_request_url || 
                                     normalized_data_url.starts_with(&normalized_request_url) ||
                                     normalized_request_url.starts_with(&normalized_data_url);
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
                    warn!("[CACHE] Failed to read cached page from folder {}: {}", folder_name, e);
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
    
    let app_data_dir = app_handle.path()
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
                                let modified = metadata.modified()
                                    .ok()
                                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                    .map(|d| d.as_secs());
                                
                                let folder_name = path.file_name()
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
async fn clear_page_cache(app_handle: tauri::AppHandle, folder: Option<String>) -> Result<(), String> {
    use std::fs;
    use tauri::Manager;
    
    let app_data_dir = app_handle.path()
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
async fn get_saved_page_for_site(app_handle: tauri::AppHandle, site_id: i64, url: String) -> Result<Option<String>, String> {
    use std::fs;
    use tauri::Manager;
    
    info!("[DB_CACHE] get_saved_page_for_site called for site_id: {}, URL: {}", site_id, url);
    
    let db = Database::new().await.map_err(|e| e.to_string())?;
    
    match db.get_saved_page(site_id, &url).await {
        Ok(Some(folder_path)) => {
            info!("[DB_CACHE] Found saved page in database for site {}: {} -> folder: {}", site_id, url, folder_path);
            
            // Читаем сохраненную страницу из папки
            let app_data_dir = app_handle.path()
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
            info!("[DB_CACHE] ✗ No saved page found in database for site {}: {}", site_id, url);
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
async fn get_cache_folder_for_url(app_handle: tauri::AppHandle, url: String) -> Result<Option<String>, String> {
    use std::fs;
    use tauri::Manager;
    
    let url_obj = match url::Url::parse(&url) {
        Ok(u) => u,
        Err(_) => return Ok(None),
    };
    
    let hostname = url_obj.host_str().unwrap_or("unknown").replace('.', "_");
    
    let app_data_dir = app_handle.path()
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
                        let folder_name = path.file_name()
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
async fn get_saved_page_versions(site_id: i64, url: String) -> Result<Vec<serde_json::Value>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    
    match db.get_saved_page_versions(site_id, &url).await {
        Ok(versions) => {
            Ok(versions
                .iter()
                .map(|(id, folder_path, timestamp)| {
                    serde_json::json!({
                        "id": id,
                        "folder_path": folder_path,
                        "timestamp": timestamp
                    })
                })
                .collect())
        }
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
    db.delete_saved_page_version(page_id).await.map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    // Initialize logger - show all logs in debug mode
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .format_timestamp_secs()
        .init();
    
    info!("Starting Tauri application");
    
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
                let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600));
                loop {
                    interval.tick().await;
                    if let Ok(updates) = check_updates(None).await {
                        if !updates.is_empty() {
                            let notification_service = NotificationService::new(app_handle_clone.clone());
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
            get_notifications,
            mark_notification_read,
            get_cached_page,
            list_cached_pages,
            clear_page_cache,
            get_saved_page_for_site,
            get_saved_page_versions,
            delete_saved_page_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

