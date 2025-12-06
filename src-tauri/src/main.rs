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
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Initialize database
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = Database::new().await {
                    eprintln!("Failed to initialize database: {}", e);
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
            get_sites,
            add_site,
            update_site,
            delete_site,
            get_mods,
            check_updates,
            build_parser,
            test_parser,
            get_notifications,
            mark_notification_read
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

