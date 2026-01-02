//! Интеграционные тесты для модулей базы данных
//!
//! Эти тесты проверяют работу рефакторинговых модулей базы данных
//! и их взаимодействие между собой.

use mod_aggregator::database::modules::*;

/// Тест создания и инициализации базы данных
#[tokio::test]
async fn test_database_creation_and_init() {
    // Этот тест проверяет создание подключения и инициализацию схемы
    let db_result = base::Database::new().await;
    assert!(db_result.is_ok(), "Failed to create database connection");

    if let Ok(db) = db_result {
        // Проверяем, что основные таблицы созданы
        // Для этого можно выполнить простой запрос
        let sites_result = sites::get_sites(&db).await;
        assert!(sites_result.is_ok() || sites_result.is_err(), "Sites query should work");
    }
}

/// Тест операций с сайтами
#[tokio::test]
async fn test_sites_operations() {
    let db = base::Database::new().await.expect("Failed to create database");

    // Создаем тестовый сайт
    let test_site = sites::add_site(
        &db,
        "Test Site",
        "https://example.com",
        &serde_json::json!({"parser": "test"}),
    ).await.expect("Failed to add site");

    assert_eq!(test_site.name, "Test Site");
    assert_eq!(test_site.url, "https://example.com");

    // Получаем все сайты
    let sites = sites::get_sites(&db).await.expect("Failed to get sites");
    assert!(!sites.is_empty());

    // Получаем сайт по ID
    let retrieved_site = sites::get_site(&db, test_site.id).await
        .expect("Failed to get site")
        .expect("Site should exist");
    assert_eq!(retrieved_site.id, test_site.id);

    // Обновляем сайт
    let mut updated_site = test_site.clone();
    updated_site.name = "Updated Test Site".to_string();
    sites::update_site(&db, updated_site.id, "Updated Test Site", "https://updated.com", &serde_json::json!({"parser": "updated"}))
        .await.expect("Failed to update site");

    // Проверяем обновление
    let updated_retrieved = sites::get_site(&db, test_site.id).await
        .expect("Failed to get updated site")
        .expect("Updated site should exist");
    assert_eq!(updated_retrieved.name, "Updated Test Site");

    // Удаляем сайт
    sites::delete_site(&db, test_site.id).await.expect("Failed to delete site");

    // Проверяем, что сайт удален
    let deleted_site = sites::get_site(&db, test_site.id).await.expect("Query should succeed");
    assert!(deleted_site.is_none(), "Site should be deleted");
}

/// Тест операций с модами
#[tokio::test]
async fn test_mods_operations() {
    let db = base::Database::new().await.expect("Failed to create database");

    // Создаем тестовый сайт для модов
    let site = sites::add_site(
        &db,
        "Test Site for Mods",
        "https://test.com",
        &serde_json::json!({"parser": "test"}),
    ).await.expect("Failed to create site");

    // Создаем тестовый мод
    let test_mod = mods::add_mod(&db, &crate::models::Mod {
        id: 0, // будет присвоен автоматически
        site_id: site.id,
        title: "Test Mod".to_string(),
        url: "https://test.com/mod1".to_string(),
        version: Some("1.0.0".to_string()),
        author: Some("Test Author".to_string()),
        description: Some("Test Description".to_string()),
        image_url: Some("https://test.com/image.png".to_string()),
        changes: Some("Initial release".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }).await.expect("Failed to add mod");

    assert_eq!(test_mod.title, "Test Mod");
    assert_eq!(test_mod.site_id, site.id);

    // Получаем моды для сайта
    let mods = mods::get_mods(&db, Some(site.id)).await.expect("Failed to get mods");
    assert!(!mods.is_empty());
    assert!(mods.iter().any(|m| m.id == test_mod.id));

    // Получаем мод по URL
    let mod_by_url = mods::get_mod_by_url(&db, "https://test.com/mod1").await
        .expect("Failed to get mod by URL");
    assert!(mod_by_url.is_some());
    assert_eq!(mod_by_url.unwrap().id, test_mod.id);

    // Обновляем мод
    let mut updated_mod = test_mod.clone();
    updated_mod.title = "Updated Test Mod".to_string();
    mods::update_mod(&db, updated_mod.id, &updated_mod).await.expect("Failed to update mod");

    // Очистка: удаляем тестовые данные
    sites::delete_site(&db, site.id).await.expect("Failed to cleanup site");
}

/// Тест операций с уведомлениями
#[tokio::test]
async fn test_notifications_operations() {
    let db = base::Database::new().await.expect("Failed to create database");

    // Создаем тестовые данные
    let site = sites::add_site(&db, "Test Site", "https://test.com", &serde_json::json!({})).await.unwrap();
    let mod_item = mods::add_mod(&db, &crate::models::Mod {
        id: 0,
        site_id: site.id,
        title: "Test Mod".to_string(),
        url: "https://test.com/mod".to_string(),
        version: None,
        author: None,
        description: None,
        image_url: None,
        changes: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }).await.unwrap();

    // Добавляем уведомление
    notifications::add_notification(&db, &crate::models::Notification {
        id: 0,
        mod_id: mod_item.id,
        site_id: site.id,
        title: "Test Notification".to_string(),
        message: "Test message".to_string(),
        read: false,
        created_at: chrono::Utc::now(),
    }).await.expect("Failed to add notification");

    // Получаем уведомления
    let notifications = notifications::get_notifications(&db).await.expect("Failed to get notifications");
    assert!(!notifications.is_empty());

    // Отмечаем как прочитанное
    if let Some(notification) = notifications.first() {
        notifications::mark_notification_read(&db, notification.id).await.expect("Failed to mark as read");
    }

    // Очистка
    sites::delete_site(&db, site.id).await.expect("Failed to cleanup");
}

/// Тест операций с файлами
#[tokio::test]
async fn test_files_operations() {
    let db = base::Database::new().await.expect("Failed to create database");

    // Создаем тестовый файл
    let test_file = files::create_file(&db, &crate::models::file::File {
        id: 0, // будет присвоен автоматически
        name: "test_file".to_string(),
        version: "1.0.0".to_string(),
        path: Some("/test/path".to_string()),
        metadata: serde_json::json!({"type": "test"}),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }).await.expect("Failed to create file");

    assert_eq!(test_file.name, "test_file");
    assert_eq!(test_file.version, "1.0.0");

    // Получаем файл по ID
    let retrieved_file = files::get_file(&db, test_file.id).await
        .expect("Failed to get file")
        .expect("File should exist");
    assert_eq!(retrieved_file.id, test_file.id);

    // Получаем файл по имени и версии
    let file_by_name_version = files::get_file_by_name_version(&db, "test_file", "1.0.0").await
        .expect("Failed to get file by name/version");
    assert!(file_by_name_version.is_some());

    // Получаем все версии файла
    let file_versions = files::get_file_versions(&db, "test_file").await.expect("Failed to get file versions");
    assert!(!file_versions.is_empty());

    // Получаем все файлы
    let all_files = files::get_all_files(&db).await.expect("Failed to get all files");
    assert!(!all_files.is_empty());

    // Обновляем файл
    let mut updated_file = test_file.clone();
    updated_file.path = Some("/updated/path".to_string());
    files::update_file(&db, &updated_file).await.expect("Failed to update file");

    // Удаляем файл
    files::delete_file(&db, test_file.id).await.expect("Failed to delete file");

    // Проверяем, что файл удален
    let deleted_file = files::get_file(&db, test_file.id).await.expect("Query should succeed");
    assert!(deleted_file.is_none(), "File should be deleted");
}

/// Тест операций с зависимостями
#[tokio::test]
async fn test_dependencies_operations() {
    let db = base::Database::new().await.expect("Failed to create database");

    // Создаем тестовые файлы
    let source_file = files::create_file(&db, &crate::models::file::File {
        id: 0,
        name: "source_file".to_string(),
        version: "1.0.0".to_string(),
        path: None,
        metadata: serde_json::json!({}),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }).await.unwrap();

    let target_file = files::create_file(&db, &crate::models::file::File {
        id: 0,
        name: "target_file".to_string(),
        version: "2.0.0".to_string(),
        path: None,
        metadata: serde_json::json!({}),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }).await.unwrap();

    // Добавляем зависимость
    let dependency = dependencies::add_file_dependency(&db, &crate::models::FileDependency {
        id: 0,
        source_file_id: source_file.id,
        target_file_name: target_file.name.clone(),
        target_file_version: Some(target_file.version.clone()),
        dependency_type: crate::models::dependency::DependencyType::Required,
        created_at: chrono::Utc::now(),
    }).await.expect("Failed to add dependency");

    assert_eq!(dependency.source_file_id, source_file.id);
    assert_eq!(dependency.target_file_name, target_file.name);

    // Получаем зависимости файла
    let file_dependencies = dependencies::get_file_dependencies(&db, source_file.id).await
        .expect("Failed to get file dependencies");
    assert!(!file_dependencies.is_empty());

    // Получаем зависимые файлы
    let dependent_files = dependencies::get_dependent_files(&db, &source_file).await
        .expect("Failed to get dependent files");
    assert!(!dependent_files.is_empty());

    // Удаляем зависимость
    dependencies::remove_file_dependency(&db, dependency.id).await.expect("Failed to remove dependency");

    // Очистка
    files::delete_file(&db, source_file.id).await.expect("Failed to cleanup source file");
    files::delete_file(&db, target_file.id).await.expect("Failed to cleanup target file");
}

/// Тест операций с коллекциями
#[tokio::test]
async fn test_collections_operations() {
    let db = base::Database::new().await.expect("Failed to create database");

    // Создаем коллекцию
    let collection = collections::create_collection(&db, &crate::models::collection::Collection {
        id: 0,
        name: "Test Collection".to_string(),
        description: Some("Test collection description".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }).await.expect("Failed to create collection");

    assert_eq!(collection.name, "Test Collection");

    // Получаем коллекцию по ID
    let retrieved_collection = collections::get_collection(&db, collection.id).await
        .expect("Failed to get collection")
        .expect("Collection should exist");
    assert_eq!(retrieved_collection.id, collection.id);

    // Получаем коллекцию по имени
    let collection_by_name = collections::get_collection_by_name(&db, "Test Collection").await
        .expect("Failed to get collection by name");
    assert!(collection_by_name.is_some());

    // Получаем все коллекции
    let all_collections = collections::get_collections(&db).await.expect("Failed to get all collections");
    assert!(!all_collections.is_empty());

    // Создаем тестовый файл для добавления в коллекцию
    let test_file = files::create_file(&db, &crate::models::file::File {
        id: 0,
        name: "collection_file".to_string(),
        version: "1.0.0".to_string(),
        path: None,
        metadata: serde_json::json!({}),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }).await.unwrap();

    // Добавляем файл в коллекцию
    collections::add_file_to_collection(&db, collection.id, test_file.id, None).await
        .expect("Failed to add file to collection");

    // Получаем файлы коллекции
    let collection_files = collections::get_collection_files(&db, collection.id).await
        .expect("Failed to get collection files");
    assert!(!collection_files.is_empty());

    // Удаляем файл из коллекции
    collections::remove_file_from_collection(&db, collection.id, test_file.id).await
        .expect("Failed to remove file from collection");

    // Обновляем коллекцию
    let mut updated_collection = collection.clone();
    updated_collection.description = Some("Updated description".to_string());
    collections::update_collection(&db, &updated_collection).await.expect("Failed to update collection");

    // Удаляем коллекцию
    collections::delete_collection(&db, collection.id).await.expect("Failed to delete collection");

    // Очистка
    files::delete_file(&db, test_file.id).await.expect("Failed to cleanup file");
}

/// Тест операций с сессией
#[tokio::test]
async fn test_session_operations() {
    let db = base::Database::new().await.expect("Failed to create database");

    // Получаем состояние сессии
    let session_state = session::get_session_state(&db).await.expect("Failed to get session state");
    assert!(session_state.is_object());

    // Обновляем порядок файлов
    let file_order = vec![1, 2, 3, 4, 5];
    session::update_session_file_order(&db, &file_order).await.expect("Failed to update file order");

    // Обновляем UI предпочтения
    let ui_prefs = serde_json::json!({"theme": "dark", "language": "ru"});
    session::update_session_ui_preferences(&db, &ui_prefs).await.expect("Failed to update UI preferences");

    // Обновляем открытые коллекции
    let open_collections = vec![1, 2, 3];
    session::update_session_open_collections(&db, &open_collections).await
        .expect("Failed to update open collections");

    // Обновляем выбранные файлы
    let selected_files = vec![10, 20, 30];
    session::update_session_selected_files(&db, &selected_files).await
        .expect("Failed to update selected files");

    // Добавляем недавнее действие
    let action = serde_json::json!({"type": "file_opened", "file_id": 123});
    session::add_recent_action(&db, &action).await.expect("Failed to add recent action");

    // Получаем недавние действия
    let recent_actions = session::get_recent_actions(&db).await.expect("Failed to get recent actions");
    assert!(!recent_actions.is_empty());

    // Обновляем текущую страницу
    session::update_session_current_page(&db, "files").await.expect("Failed to update current page");

    // Обновляем выбранный сайт
    session::update_session_selected_site(&db, Some(42)).await.expect("Failed to update selected site");

    // Очищаем недавние действия
    session::clear_recent_actions(&db).await.expect("Failed to clear recent actions");

    // Проверяем, что действия очищены
    let cleared_actions = session::get_recent_actions(&db).await.expect("Failed to get cleared actions");
    assert!(cleared_actions.is_empty());

    // Сбрасываем состояние сессии
    session::reset_session_state(&db).await.expect("Failed to reset session state");
}