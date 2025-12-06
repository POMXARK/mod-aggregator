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

#[tauri::command]
async fn get_sites() -> Result<Vec<models::Site>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.get_sites().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_site(name: String, url: String, parser_config: serde_json::Value) -> Result<models::Site, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.add_site(&name, &url, &parser_config).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_site(id: i64, name: String, url: String, parser_config: serde_json::Value) -> Result<(), String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.update_site(id, &name, &url, &parser_config).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_site(id: i64) -> Result<(), String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.delete_site(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_mods(site_id: Option<i64>) -> Result<Vec<models::Mod>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.get_mods(site_id).await.map_err(|e| e.to_string())
}

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

#[tauri::command]
async fn build_parser(html: String, selector: String) -> Result<serde_json::Value, String> {
    let builder = ParserBuilder::new();
    builder.build_from_selector(&html, &selector).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn fetch_page(url: String) -> Result<String, String> {
    info!("fetch_page called with URL: {}", url);
    
    // Validate URL format
    if url.is_empty() {
        warn!("Empty URL provided");
        return Err("URL не может быть пустым".to_string());
    }
    
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
    Ok(html)
}

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

#[tauri::command]
async fn fetch_resource(url: String) -> Result<Vec<u8>, String> {
    info!("fetch_resource called with URL: {}", url);
    
    if url.is_empty() {
        return Err("URL не может быть пустым".to_string());
    }
    
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
    
    info!("Successfully fetched resource, size: {} bytes", bytes.len());
    Ok(bytes.to_vec())
}

#[tauri::command]
async fn save_page_local(app_handle: tauri::AppHandle, html: String, filename: String) -> Result<String, String> {
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

#[tauri::command]
async fn test_parser(site_id: i64) -> Result<Vec<models::Mod>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    let site = db.get_site(site_id).await.map_err(|e| e.to_string())?;
    let engine = ParserEngine::new();
    engine.parse_site(&site).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_notifications() -> Result<Vec<models::Notification>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.get_notifications().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn mark_notification_read(id: i64) -> Result<(), String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.mark_notification_read(id).await.map_err(|e| e.to_string())
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
            mark_notification_read
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

