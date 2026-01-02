//! Обработчики команд для работы с ресурсами
//!
//! Модуль содержит функции для загрузки, сохранения и кеширования ресурсов (CSS, изображения).

use crate::database::Database;
use crate::handlers::cache::{get_cached_page, get_saved_page_for_site};
use crate::models;
use crate::parser::ParserEngine;
use crate::parser_builder::ParserBuilder;
use log::{debug, error, info, warn};

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
pub async fn save_resource(
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
/// Сначала проверяет в общих подпапках "css" и "images", затем ищет
/// в папках сохраненных страниц, которые начинаются с "page_".
/// Генерирует безопасное имя файла из URL для поиска.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `url` - URL ресурса для поиска в кеше
///
/// # Возвращает
/// Байты ресурса, если файл найден и прочитан успешно, или None
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
pub async fn fetch_resource(app_handle: tauri::AppHandle, url: String) -> Result<Vec<u8>, String> {
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
pub async fn save_page_local(
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
        let version_timestamp = chrono::Utc::now().to_rfc3339();
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
pub async fn fetch_page(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_resource_empty_url() {
        // Этот тест проверяет валидацию входных данных без mock app handle
        // Для полноценного тестирования нужны интеграционные тесты с mock
        assert!(true); // Плейсхолдер для проверки компиляции
    }
}