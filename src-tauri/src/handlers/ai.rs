//! Обработчики команд для работы с AI
//!
//! Модуль содержит функции для генерации парсеров с помощью AI,
//! чата с AI и проверки доступности AI сервисов.

use crate::ai_parser::AIParser;
use log::{error, info};

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
pub async fn ai_generate_parser(
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
pub async fn ai_chat(
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
pub async fn ai_check_ollama(ollama_url: Option<String>) -> Result<Vec<String>, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_generate_parser_invalid_model() {
        let result = ai_generate_parser(
            "<html></html>".to_string(),
            "test".to_string(),
            "invalid".to_string(),
            None,
            None,
            None,
        ).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ai_chat_invalid_model() {
        let result = ai_chat(
            vec![("user".to_string(), "test".to_string())],
            "invalid".to_string(),
            None,
            None,
            None,
        ).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ai_check_ollama_connection() {
        let result = ai_check_ollama(Some("http://invalid-url".to_string())).await;
        // Ожидаем ошибку подключения
        assert!(result.is_err());
    }
}