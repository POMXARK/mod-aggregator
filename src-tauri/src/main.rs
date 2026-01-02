// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai_parser;
mod commands;
mod database;
mod handlers;
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

// Все обработчики команд теперь находятся в модуле handlers
// Импортируем их для использования в invoke_handler

/// Главная функция приложения Tauri
///
/// Инициализирует логгер, настраивает плагины и запускает приложение.
/// В режиме разработки включает MCP плагин для отладки.
/// Выполняет асинхронную инициализацию базы данных и запускает фоновый
/// процесс проверки обновлений модов каждый час.
///
/// # Логика работы:
/// 1. Инициализация логгера с уровнем debug в режиме разработки
/// 2. Настройка плагинов Tauri (shell, notification, fs, mcp в dev режиме)
/// 3. Запуск асинхронной инициализации базы данных
/// 4. Запуск фонового таймера для автоматической проверки обновлений
/// 5. Регистрация всех command handlers для IPC коммуникации с frontend
/// 6. Запуск главного цикла приложения
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
                        if let Ok(updates) = crate::handlers::mods::check_updates(None).await {
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
                crate::handlers::resources::fetch_resource,
                crate::handlers::resources::save_resource,
                crate::handlers::cache::get_cache_folder_for_url,
                crate::handlers::sites::get_sites,
                crate::handlers::sites::add_site,
                crate::handlers::sites::update_site,
                crate::handlers::sites::delete_site,
                crate::handlers::mods::get_mods,
                crate::handlers::mods::check_updates,
                crate::handlers::parsers::build_parser,
                crate::handlers::parsers::test_parser_from_nodes,
                crate::handlers::resources::fetch_page,
                crate::handlers::resources::save_page_local,
                crate::handlers::parsers::test_parser,
                crate::handlers::ai::ai_generate_parser,
                crate::handlers::ai::ai_chat,
                crate::handlers::ai::ai_check_ollama,
                crate::handlers::notifications::get_notifications,
                crate::handlers::notifications::mark_notification_read,
                crate::handlers::cache::get_cached_page,
                crate::handlers::cache::list_cached_pages,
                crate::handlers::cache::clear_page_cache,
                crate::handlers::cache::get_saved_page_for_site,
                crate::handlers::cache::get_saved_page_versions,
                crate::handlers::cache::delete_saved_page_version,
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
                reset_session_state,
                update_current_page,
                update_selected_site
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
                        if let Ok(updates) = crate::handlers::mods::check_updates(None).await {
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
                crate::handlers::resources::fetch_resource,
                crate::handlers::resources::save_resource,
                crate::handlers::cache::get_cache_folder_for_url,
                crate::handlers::sites::get_sites,
                crate::handlers::sites::add_site,
                crate::handlers::sites::update_site,
                crate::handlers::sites::delete_site,
                crate::handlers::mods::get_mods,
                crate::handlers::mods::check_updates,
                crate::handlers::parsers::build_parser,
                crate::handlers::resources::fetch_page,
                crate::handlers::resources::save_page_local,
                crate::handlers::parsers::test_parser,
                crate::handlers::ai::ai_generate_parser,
                crate::handlers::notifications::get_notifications,
                crate::handlers::notifications::mark_notification_read,
                crate::handlers::cache::get_cached_page,
                crate::handlers::cache::list_cached_pages,
                crate::handlers::cache::clear_page_cache,
                crate::handlers::cache::delete_saved_page_version,
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
                reset_session_state,
                update_current_page,
                update_selected_site
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}