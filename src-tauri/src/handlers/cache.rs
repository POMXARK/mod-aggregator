//! Обработчики команд для работы с кешем страниц
//!
//! Модуль содержит функции для кеширования страниц, ресурсов и управления кешем.

use crate::database::Database;
use crate::handlers::utils::*;
use crate::models;
use log::{debug, error, info, warn};

/// Получить закешированную страницу из общего кеша
///
/// Ищет сохраненную страницу в директории saved_pages по hostname из URL.
/// Сканирует все папки, начинающиеся с "page_" и содержащие hostname,
/// сортирует их по дате модификации и проверяет каждую на соответствие URL.
/// Для каждой кандидата извлекает data-base-url из HTML и сравнивает
/// с нормализованным запрашиваемым URL.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `url` - URL страницы для поиска в общем кеше
///
/// # Возвращает
/// HTML содержимое страницы, если найдено совпадение, или None
#[tauri::command]
pub async fn get_cached_page(
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
/// Для каждой папки с index.html собирает метаданные: имя папки, полный путь,
/// время модификации и размер файла. Сортирует результаты по времени модификации
/// (новые страницы первыми).
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
///
/// # Возвращает
/// Вектор объектов с информацией о страницах (folder, path, modified, size),
/// отсортированных по дате изменения (новые первыми)
#[tauri::command]
pub async fn list_cached_pages(app_handle: tauri::AppHandle) -> Result<Vec<serde_json::Value>, String> {
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
/// При удалении всего кеша пересоздает директорию saved_pages.
/// Логирует операции удаления для отслеживания.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `folder` - имя папки для удаления (None = удалить весь кеш)
///
/// # Возвращает
/// Пустой результат при успехе или ошибку при неудаче удаления
#[tauri::command]
pub async fn clear_page_cache(
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

/// Получить папку кеша для URL (для поиска ресурсов)
///
/// Ищет папку кеша для указанного URL, чтобы можно было найти сохраненные ресурсы.
/// Анализирует hostname из URL, ищет папки в saved_pages, которые начинаются
/// с "page_" и содержат hostname. Возвращает самую новую найденную папку.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `url` - URL страницы для поиска папки кеша
///
/// # Возвращает
/// Имя папки кеша (например, "page_1234567890_hostname"), если найдено, или None
#[tauri::command]
pub async fn get_cache_folder_for_url(
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

/// Получить сохраненную страницу для сайта (последняя версия)
///
/// Выполняет запрос к базе данных для поиска сохраненной страницы по site_id и url.
/// Если запись найдена, извлекает folder_path и читает HTML файл из
/// соответствующей директории в saved_pages. Логирует все этапы операции.
///
/// # Параметры
/// * `app_handle` - handle приложения Tauri для доступа к файловой системе
/// * `site_id` - ID сайта для поиска сохраненной страницы
/// * `url` - URL страницы для поиска в базе данных
///
/// # Возвращает
/// HTML содержимое страницы, если найдено и прочитано успешно, или None
#[tauri::command]
pub async fn get_saved_page_for_site(
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
        Ok(Some((_id, folder_path, _timestamp))) => {
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

/// Получить все версии сохраненной страницы для сайта
///
/// Выполняет запрос к базе данных для получения всех сохраненных версий
/// страницы по указанным site_id и URL. Каждая версия содержит информацию
/// о времени сохранения и пути к папке с файлами.
///
/// # Параметры
/// * `site_id` - ID сайта для поиска версий
/// * `url` - URL страницы для поиска версий
///
/// # Возвращает
/// Вектор объектов с информацией о версиях (id, folder_path, timestamp)
/// или пустой вектор при ошибке
#[tauri::command]
pub async fn get_saved_page_versions(
    site_id: i64,
    url: String,
) -> Result<Vec<serde_json::Value>, String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;

    match db.get_saved_page_versions(site_id, &url).await {
        Ok(versions) => Ok(versions),
        Err(e) => {
            warn!("Failed to get saved page versions: {}", e);
            Ok(vec![])
        }
    }
}

/// Удалить конкретную версию сохраненной страницы
///
/// Выполняет запрос к базе данных для удаления записи о сохраненной версии страницы.
/// Файлы на диске остаются нетронутыми - удаляется только ссылка в базе данных.
/// Логирует результат операции.
///
/// # Параметры
/// * `page_id` - ID версии страницы для удаления из базы данных
///
/// # Возвращает
/// Пустой результат при успешном удалении или ошибку при неудаче
#[tauri::command]
pub async fn delete_saved_page_version(page_id: i64) -> Result<(), String> {
    let db = Database::new().await.map_err(|e| e.to_string())?;
    db.delete_saved_page_version(page_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Unit тесты для модуля cache
    // Для интеграционных тестов нужны mock объекты Tauri AppHandle
    // Эти тесты проверяют только компиляцию и базовую структуру API
}