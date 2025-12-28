use log::{error, info, warn};
use reqwest;
use serde::{Deserialize, Serialize};

/// Тип AI парсера
#[derive(Debug, Clone)]
pub enum AIParserType {
    Ollama { model: String, url: String },
    OpenAI { api_key: String, model: String },
    Anthropic { api_key: String, model: String },
    Google { api_key: String, model: String },
}

/// Структура для запросов к Ollama
#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

/// Структура для ответов от Ollama
#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

/// Структура для запросов к OpenAI
#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<OpenAIResponseFormat>,
}

#[derive(Debug, Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAIResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

/// Структура для ответов от OpenAI
#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
    error: Option<OpenAIError>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessageResponse,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessageResponse {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIError {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
}

/// AI парсер для генерации конфигураций парсеров
pub struct AIParser {
    parser_type: AIParserType,
}

impl AIParser {
    /// Создать новый AI парсер
    #[allow(dead_code)]
    pub fn new(parser_type: AIParserType) -> Self {
        Self { parser_type }
    }

    /// Создать Ollama парсер
    pub fn ollama(model: String, url: Option<String>) -> Self {
        Self {
            parser_type: AIParserType::Ollama {
                model,
                url: url.unwrap_or_else(|| "http://localhost:11434/api/generate".to_string()),
            },
        }
    }

    /// Создать OpenAI парсер
    pub fn openai(api_key: String, model: Option<String>) -> Self {
        Self {
            parser_type: AIParserType::OpenAI {
                api_key,
                model: model.unwrap_or_else(|| "gpt-4o-mini".to_string()),
            },
        }
    }

    /// Создать Anthropic Claude парсер
    pub fn anthropic(api_key: String, model: Option<String>) -> Self {
        Self {
            parser_type: AIParserType::Anthropic {
                api_key,
                model: model.unwrap_or_else(|| "claude-3-haiku-20240307".to_string()),
            },
        }
    }

    /// Создать Google Gemini парсер
    pub fn google(api_key: String, model: Option<String>) -> Self {
        Self {
            parser_type: AIParserType::Google {
                api_key,
                model: model.unwrap_or_else(|| "gemini-pro".to_string()),
            },
        }
    }

    /// Отправляет сообщение в чат и получает ответ
    ///
    /// # Параметры
    /// * `messages` - история сообщений (роль и содержимое)
    ///
    /// # Возвращает
    /// Ответ от AI
    pub async fn chat(
        &self,
        messages: Vec<(String, String)>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        info!("Sending chat message with AI: {:?}", self.parser_type);

        match &self.parser_type {
            AIParserType::Ollama { model, url } => {
                self.chat_with_ollama(model, url, messages).await
            }
            AIParserType::OpenAI { api_key, model } => {
                self.chat_with_openai(api_key, model, messages).await
            }
            AIParserType::Anthropic { api_key, model } => {
                self.chat_with_anthropic(api_key, model, messages).await
            }
            AIParserType::Google { api_key, model } => {
                self.chat_with_google(api_key, model, messages).await
            }
        }
    }

    /// Генерирует конфигурацию парсера на основе HTML и описания данных
    ///
    /// # Параметры
    /// * `html` - HTML код страницы (первые 5000 символов)
    /// * `description` - описание данных, которые нужно извлечь
    ///
    /// # Возвращает
    /// JSON конфигурацию парсера с CSS селекторами
    pub async fn generate_parser(
        &self,
        html: &str,
        description: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        info!("Generating parser with AI: {:?}", self.parser_type);

        // Ограничиваем размер HTML для экономии токенов
        let html_preview = html.chars().take(5000).collect::<String>();

        let prompt = format!(
            r#"Ты эксперт по парсингу веб-страниц. Проанализируй HTML и создай конфигурацию парсера.

Описание данных для извлечения: {}
HTML (первые 5000 символов): {}

Верни ТОЛЬКО валидный JSON в формате:
{{
  "list_selector": "CSS селектор для списка элементов (например, .item, .product, article)",
  "title_selector": "CSS селектор для заголовка (например, h2, .title, a.title)",
  "url_selector": "CSS селектор для ссылки (например, a, a.link, .url)",
  "description_selector": "CSS селектор для описания (например, .description, p, .text)",
  "image_selector": "CSS селектор для изображения (например, img, .image, img.thumbnail)"
}}

Важно:
- list_selector должен выбирать контейнер каждого элемента списка
- Остальные селекторы должны быть относительными к list_selector
- Если элемент не найден, используй пустую строку
- Только JSON, без markdown, без дополнительного текста"#,
            description, html_preview
        );

        match &self.parser_type {
            AIParserType::Ollama { model, url } => {
                self.generate_with_ollama(model, url, &prompt).await
            }
            AIParserType::OpenAI { api_key, model } => {
                self.generate_with_openai(api_key, model, &prompt).await
            }
            AIParserType::Anthropic { api_key, model } => {
                self.generate_with_anthropic(api_key, model, &prompt).await
            }
            AIParserType::Google { api_key, model } => {
                self.generate_with_google(api_key, model, &prompt).await
            }
        }
    }

    /// Генерирует парсер используя Ollama
    async fn generate_with_ollama(
        &self,
        model: &str,
        url: &str,
        prompt: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        info!("Sending request to Ollama: {}", url);

        let request = OllamaRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()?;

        let response = client.post(url).json(&request).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Ollama API error: {}", error_text);
            return Err(format!("Ollama API error: {}", error_text).into());
        }

        let ollama_response: OllamaResponse = response.json().await?;
        info!(
            "Received response from Ollama (length: {})",
            ollama_response.response.len()
        );

        // Извлекаем JSON из ответа
        let json_str = self.extract_json_from_response(&ollama_response.response)?;
        let config: serde_json::Value = serde_json::from_str(json_str)?;

        info!("Successfully parsed parser config from Ollama");
        Ok(config)
    }

    /// Генерирует парсер используя OpenAI
    async fn generate_with_openai(
        &self,
        api_key: &str,
        model: &str,
        prompt: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        info!("Sending request to OpenAI: {}", model);

        let request = OpenAIRequest {
            model: model.to_string(),
            messages: vec![OpenAIMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: 0.3,
            response_format: Some(OpenAIResponseFormat {
                format_type: "json_object".to_string(),
            }),
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()?;

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("OpenAI API error: {}", error_text);
            return Err(format!("OpenAI API error: {}", error_text).into());
        }

        let api_response: OpenAIResponse = response.json().await?;

        if let Some(err) = api_response.error {
            return Err(format!("OpenAI error: {} ({})", err.message, err.error_type).into());
        }

        if api_response.choices.is_empty() {
            return Err("OpenAI returned empty choices".into());
        }

        let content = &api_response.choices[0].message.content;
        info!("Received response from OpenAI (length: {})", content.len());

        // Очищаем ответ от markdown если есть
        let json_str = self.extract_json_from_response(content)?;
        let config: serde_json::Value = serde_json::from_str(json_str)?;

        info!("Successfully parsed parser config from OpenAI");
        Ok(config)
    }

    /// Генерирует парсер используя Anthropic Claude
    async fn generate_with_anthropic(
        &self,
        api_key: &str,
        model: &str,
        prompt: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct AnthropicRequest {
            model: String,
            max_tokens: u32,
            messages: Vec<AnthropicMessage>,
        }

        #[derive(Serialize)]
        struct AnthropicMessage {
            role: String,
            content: String,
        }

        #[derive(Deserialize)]
        struct AnthropicResponse {
            content: Vec<AnthropicContent>,
        }

        #[derive(Deserialize)]
        struct AnthropicContent {
            text: String,
        }

        let request = AnthropicRequest {
            model: model.to_string(),
            max_tokens: 4096,
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()?;

        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Anthropic API error: {}", error_text).into());
        }

        let api_response: AnthropicResponse = response.json().await?;

        if api_response.content.is_empty() {
            return Err("Anthropic returned empty content".into());
        }

        let content = &api_response.content[0].text;
        let json_str = self.extract_json_from_response(content)?;
        let config: serde_json::Value = serde_json::from_str(json_str)?;

        Ok(config)
    }

    /// Генерирует парсер используя Google Gemini
    async fn generate_with_google(
        &self,
        api_key: &str,
        model: &str,
        prompt: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct GoogleRequest {
            contents: Vec<GoogleContentReq>,
        }

        #[derive(Serialize)]
        struct GoogleContentReq {
            parts: Vec<GooglePart>,
        }

        #[derive(Serialize)]
        struct GooglePart {
            text: String,
        }

        #[derive(Deserialize)]
        struct GoogleResponse {
            candidates: Vec<GoogleCandidate>,
        }

        #[derive(Deserialize)]
        struct GoogleCandidate {
            content: GoogleContentResp,
        }

        #[derive(Deserialize)]
        struct GoogleContentResp {
            parts: Vec<GooglePartResp>,
        }

        #[derive(Deserialize)]
        struct GooglePartResp {
            text: String,
        }

        let request = GoogleRequest {
            contents: vec![GoogleContentReq {
                parts: vec![GooglePart {
                    text: prompt.to_string(),
                }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
        );

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()?;

        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Google API error: {}", error_text).into());
        }

        let api_response: GoogleResponse = response.json().await?;

        if api_response.candidates.is_empty() {
            return Err("Google returned empty candidates".into());
        }

        if api_response.candidates[0].content.parts.is_empty() {
            return Err("Google returned empty parts".into());
        }

        let content = &api_response.candidates[0].content.parts[0].text;
        let json_str = self.extract_json_from_response(content)?;
        let config: serde_json::Value = serde_json::from_str(json_str)?;

        Ok(config)
    }

    /// Извлекает JSON из ответа AI (убирает markdown, лишний текст)
    fn extract_json_from_response<'a>(
        &self,
        response: &'a str,
    ) -> Result<&'a str, Box<dyn std::error::Error>> {
        // Ищем JSON объект в ответе
        let json_start = response.find('{');
        let json_end = response.rfind('}');

        if let (Some(start), Some(end)) = (json_start, json_end) {
            let json_str = &response[start..=end];

            // Убираем markdown code blocks если есть
            let cleaned = json_str
                .trim_start_matches("```json")
                .trim_start_matches("```")
                .trim_end_matches("```")
                .trim();

            Ok(cleaned)
        } else {
            Err("Не удалось найти JSON в ответе AI".into())
        }
    }

    /// Извлекает данные из HTML используя AI
    ///
    /// # Параметры
    /// * `_html` - HTML код страницы
    /// * `_fields` - список полей для извлечения
    ///
    /// # Возвращает
    /// Вектор JSON объектов с извлеченными данными
    #[allow(dead_code)]
    pub async fn extract_data(
        &self,
        _html: &str,
        _fields: &[&str],
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        warn!("extract_data is not yet implemented for hybrid approach");
        // Для гибридного подхода извлечение данных лучше делать через обычный парсер
        // после генерации конфигурации
        Err(
            "Data extraction should be done through regular parser after AI generates config"
                .into(),
        )
    }

    /// Чат с Ollama
    async fn chat_with_ollama(
        &self,
        model: &str,
        url: &str,
        messages: Vec<(String, String)>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Объединяем все сообщения в один промпт
        let prompt = messages
            .iter()
            .map(|(role, content)| format!("{}: {}", role, content))
            .collect::<Vec<_>>()
            .join("\n\n");

        let request = OllamaRequest {
            model: model.to_string(),
            prompt,
            stream: false,
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()?;

        let response = client.post(url).json(&request).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Ollama API error: {}", error_text).into());
        }

        let ollama_response: OllamaResponse = response.json().await?;
        Ok(ollama_response.response)
    }

    /// Чат с OpenAI
    async fn chat_with_openai(
        &self,
        api_key: &str,
        model: &str,
        messages: Vec<(String, String)>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let openai_messages: Vec<OpenAIMessage> = messages
            .iter()
            .map(|(role, content)| OpenAIMessage {
                role: role.clone(),
                content: content.clone(),
            })
            .collect();

        let request = OpenAIRequest {
            model: model.to_string(),
            messages: openai_messages,
            temperature: 0.7,
            response_format: None,
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()?;

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("OpenAI API error: {}", error_text).into());
        }

        let api_response: OpenAIResponse = response.json().await?;

        if let Some(err) = api_response.error {
            return Err(format!("OpenAI error: {} ({})", err.message, err.error_type).into());
        }

        if api_response.choices.is_empty() {
            return Err("OpenAI returned empty choices".into());
        }

        Ok(api_response.choices[0].message.content.clone())
    }

    /// Чат с Anthropic Claude
    async fn chat_with_anthropic(
        &self,
        api_key: &str,
        model: &str,
        messages: Vec<(String, String)>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct AnthropicRequest {
            model: String,
            max_tokens: u32,
            messages: Vec<AnthropicMessage>,
        }

        #[derive(Serialize)]
        struct AnthropicMessage {
            role: String,
            content: String,
        }

        #[derive(Deserialize)]
        struct AnthropicResponse {
            content: Vec<AnthropicContent>,
        }

        #[derive(Deserialize)]
        struct AnthropicContent {
            text: String,
        }

        let anthropic_messages: Vec<AnthropicMessage> = messages
            .iter()
            .map(|(role, content)| AnthropicMessage {
                role: if role == "user" {
                    "user".to_string()
                } else {
                    "assistant".to_string()
                },
                content: content.clone(),
            })
            .collect();

        let request = AnthropicRequest {
            model: model.to_string(),
            max_tokens: 4096,
            messages: anthropic_messages,
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()?;

        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Anthropic API error: {}", error_text).into());
        }

        let api_response: AnthropicResponse = response.json().await?;

        if api_response.content.is_empty() {
            return Err("Anthropic returned empty content".into());
        }

        Ok(api_response.content[0].text.clone())
    }

    /// Чат с Google Gemini
    async fn chat_with_google(
        &self,
        api_key: &str,
        model: &str,
        messages: Vec<(String, String)>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct GoogleRequest {
            contents: Vec<GoogleContentReq>,
        }

        #[derive(Serialize)]
        struct GoogleContentReq {
            parts: Vec<GooglePart>,
        }

        #[derive(Serialize)]
        struct GooglePart {
            text: String,
        }

        #[derive(Deserialize)]
        struct GoogleResponse {
            candidates: Vec<GoogleCandidate>,
        }

        #[derive(Deserialize)]
        struct GoogleCandidate {
            content: GoogleContentResp,
        }

        #[derive(Deserialize)]
        struct GoogleContentResp {
            parts: Vec<GooglePartResp>,
        }

        #[derive(Deserialize)]
        struct GooglePartResp {
            text: String,
        }

        let contents: Vec<GoogleContentReq> = messages
            .iter()
            .map(|(_, content)| GoogleContentReq {
                parts: vec![GooglePart {
                    text: content.clone(),
                }],
            })
            .collect();

        let request = GoogleRequest { contents };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
        );

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()?;

        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Google API error: {}", error_text).into());
        }

        let api_response: GoogleResponse = response.json().await?;

        if api_response.candidates.is_empty() {
            return Err("Google returned empty candidates".into());
        }

        if api_response.candidates[0].content.parts.is_empty() {
            return Err("Google returned empty parts".into());
        }

        Ok(api_response.candidates[0].content.parts[0].text.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_from_response() {
        let parser = AIParser::ollama("test".to_string(), None);

        // Тест с чистым JSON
        let response1 = r#"{"list_selector": ".item", "title_selector": "h2"}"#;
        assert!(parser.extract_json_from_response(response1).is_ok());

        // Тест с markdown
        let response2 = r#"```json
{"list_selector": ".item", "title_selector": "h2"}
```"#;
        assert!(parser.extract_json_from_response(response2).is_ok());

        // Тест с текстом до и после JSON
        let response3 = r#"Вот конфигурация:
{"list_selector": ".item", "title_selector": "h2"}
Это правильный формат."#;
        assert!(parser.extract_json_from_response(response3).is_ok());
    }
}
