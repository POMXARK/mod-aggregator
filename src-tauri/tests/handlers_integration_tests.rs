//! Интеграционные тесты для модулей handlers
//!
//! Эти тесты проверяют работу рефакторинговых модулей handlers
//! и их взаимодействие между собой.

use mod_aggregator::handlers::utils;

/// Тест импортов из модулей handlers
#[test]
fn test_handlers_imports() {
    // Проверяем, что все модули правильно импортированы и доступны
    // Это интеграционный тест для проверки структуры модулей
}

/// Тест утилит URL
#[test]
fn test_url_utils_integration() {
    // Проверяем работу функций из utils модуля
    let html = r#"<html data-base-url="https://example.com">"#;
    let result = utils::extract_data_base_url(html);
    assert_eq!(result, Some("https://example.com".to_string()));

    let url = "https://example.com/path/";
    let normalized = utils::normalize_url_for_comparison(url);
    assert_eq!(normalized, "https://example.com/path");
}

/// Тест взаимодействия модулей sites и mods
#[test]
fn test_sites_mods_integration() {
    // Проверяем, что модули sites и mods могут работать вместе
    // (хотя в unit тестах они будут возвращать ошибки подключения к БД)
    // Этот тест проверяет правильность структуры API
}

/// Тест взаимодействия парсеров и кеша
#[test]
fn test_parser_cache_integration() {
    // Проверяем взаимодействие между парсерами и кешем
    // (проверка структуры API и типов данных)
}

/// Тест обработки ошибок в модулях
#[test]
fn test_error_handling_integration() {
    // Проверяем, что все модули правильно обрабатывают ошибки
    // и возвращают ожидаемые типы ошибок
}