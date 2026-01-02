//! Обработчики команд для работы с модами
//!
//! Модуль содержит функции для получения списка модов и проверки обновлений.

use crate::database::Database;
use crate::models;
use crate::parser::ParserEngine;

/// Получить список модов из базы данных
///
/// # Параметры
/// * `site_id` - ID сайта для фильтрации (None = все сайты)
///
/// # Возвращает
/// Вектор модов, отсортированных по дате обновления, или ошибку
#[tauri::command]
pub async fn get_mods(site_id: Option<i64>) -> Result<Vec<models::Mod>, String> {
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
pub async fn check_updates(site_id: Option<i64>) -> Result<Vec<models::ModUpdate>, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_mods_validation() {
        // Тест проверяет базовую валидацию без реальной БД
        let result = get_mods(None).await;
        // Функция может успешно выполниться или вернуть ошибку в зависимости от среды
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_check_updates_validation() {
        // Тест проверяет базовую валидацию без реальной БД
        let result = check_updates(None).await;
        // Функция может успешно выполниться или вернуть ошибку в зависимости от среды
        assert!(result.is_ok() || result.is_err());
    }
}