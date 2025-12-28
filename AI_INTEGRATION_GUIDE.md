# Руководство по интеграции AI для автоматического парсинга

## Обзор

Этот документ описывает варианты интеграции искусственного интеллекта для автоматического парсинга данных с сайтов и генерации парсеров в приложении Mod Aggregator.

## Варианты интеграции

### Вариант 1: Локальные модели через Ollama (Рекомендуется для приватности)

**Преимущества:**
- ✅ Полностью локально, данные не покидают компьютер
- ✅ Бесплатно
- ✅ Работает офлайн
- ✅ Нет лимитов на запросы

**Недостатки:**
- ❌ Требует установки Ollama (~4GB для легких моделей)
- ❌ Требует RAM (минимум 8GB для легких моделей)
- ❌ Медленнее чем API

**Подходящие модели:**
- `llama3.2:3b` - очень легкая (3GB), быстрая
- `phi3:mini` - Microsoft Phi-3 Mini (3.8GB)
- `mistral:7b` - более мощная, но тяжелее (4.1GB)
- `qwen2.5:7b` - хорошая для парсинга (4.4GB)

**Установка:**
```bash
# Скачать Ollama с https://ollama.ai
# Установить модель
ollama pull llama3.2:3b
```

**Интеграция в Rust:**
```rust
// src-tauri/Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Пример использования:**
```rust
// src-tauri/src/ai_parser.rs
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

pub struct AIParser {
    ollama_url: String,
    model: String,
}

impl AIParser {
    pub fn new(model: String) -> Self {
        Self {
            ollama_url: "http://localhost:11434/api/generate".to_string(),
            model,
        }
    }

    /// Генерирует парсер на основе HTML и описания данных
    pub async fn generate_parser(
        &self,
        html: &str,
        description: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let prompt = format!(
            r#"Ты эксперт по парсингу веб-страниц. Проанализируй HTML и создай конфигурацию парсера.

Описание данных для извлечения: {}
HTML (первые 5000 символов): {}

Верни JSON конфигурацию в формате:
{{
  "list_selector": "CSS селектор для списка элементов",
  "title_selector": "CSS селектор для заголовка",
  "url_selector": "CSS селектор для ссылки",
  "description_selector": "CSS селектор для описания",
  "image_selector": "CSS селектор для изображения"
}}

Только JSON, без дополнительного текста."#,
            description,
            &html.chars().take(5000).collect::<String>()
        );

        let request = OllamaRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&self.ollama_url)
            .json(&request)
            .send()
            .await?;

        let ollama_response: OllamaResponse = response.json().await?;
        
        // Парсим JSON из ответа
        let json_start = ollama_response.response.find('{');
        let json_end = ollama_response.response.rfind('}');
        
        if let (Some(start), Some(end)) = (json_start, json_end) {
            let json_str = &ollama_response.response[start..=end];
            let config: serde_json::Value = serde_json::from_str(json_str)?;
            Ok(config)
        } else {
            Err("Не удалось найти JSON в ответе".into())
        }
    }

    /// Извлекает данные из HTML используя AI
    pub async fn extract_data(
        &self,
        html: &str,
        fields: &[&str],
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let fields_str = fields.join(", ");
        let prompt = format!(
            r#"Извлеки следующие данные из HTML: {}

HTML:
{}

Верни массив объектов в JSON формате. Каждый объект должен содержать поля: {}
Только JSON массив, без дополнительного текста."#,
            fields_str,
            &html.chars().take(8000).collect::<String>(),
            fields_str
        );

        let request = OllamaRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&self.ollama_url)
            .json(&request)
            .send()
            .await?;

        let ollama_response: OllamaResponse = response.json().await?;
        
        // Парсим JSON массив из ответа
        let json_start = ollama_response.response.find('[');
        let json_end = ollama_response.response.rfind(']');
        
        if let (Some(start), Some(end)) = (json_start, json_end) {
            let json_str = &ollama_response.response[start..=end];
            let data: Vec<serde_json::Value> = serde_json::from_str(json_str)?;
            Ok(data)
        } else {
            Err("Не удалось найти JSON массив в ответе".into())
        }
    }
}
```

---

### Вариант 2: API модели (OpenAI, Anthropic, Google)

**Преимущества:**
- ✅ Очень точные результаты
- ✅ Не требует локальных ресурсов
- ✅ Быстро работает
- ✅ Простая интеграция

**Недостатки:**
- ❌ Требует API ключ (платно)
- ❌ Данные отправляются на сервер
- ❌ Зависимость от интернета
- ❌ Лимиты на запросы

**Рекомендуемые модели:**
- **OpenAI GPT-4o-mini** - $0.15/1M tokens (дешево, быстро)
- **Anthropic Claude Haiku** - $0.25/1M tokens (очень быстро)
- **Google Gemini Flash** - $0.075/1M tokens (самое дешевое)

**Интеграция в Rust:**
```rust
// src-tauri/Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

**Пример использования (OpenAI):**
```rust
// src-tauri/src/ai_parser_api.rs
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    response_format: ResponseFormat,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: String,
}

pub struct OpenAIParser {
    api_key: String,
    model: String,
}

impl OpenAIParser {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            api_key,
            model: model.unwrap_or_else(|| "gpt-4o-mini".to_string()),
        }
    }

    pub async fn generate_parser(
        &self,
        html: &str,
        description: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let prompt = format!(
            r#"Ты эксперт по парсингу веб-страниц. Проанализируй HTML и создай конфигурацию парсера.

Описание данных для извлечения: {}
HTML (первые 5000 символов): {}

Верни ТОЛЬКО валидный JSON в формате:
{{
  "list_selector": "CSS селектор для списка элементов",
  "title_selector": "CSS селектор для заголовка",
  "url_selector": "CSS селектор для ссылки",
  "description_selector": "CSS селектор для описания",
  "image_selector": "CSS селектор для изображения"
}}

Только JSON, без markdown, без дополнительного текста."#,
            description,
            &html.chars().take(5000).collect::<String>()
        );

        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: 0.3,
            response_format: ResponseFormat {
                format_type: "json_object".to_string(),
            },
        };

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("OpenAI API error: {}", error_text).into());
        }

        let api_response: OpenAIResponse = response.json().await?;
        let content = &api_response.choices[0].message.content;
        
        // Очищаем ответ от markdown если есть
        let json_str = content
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();
        
        let config: serde_json::Value = serde_json::from_str(json_str)?;
        Ok(config)
    }
}
```

---

### Вариант 3: Гибридный подход (Рекомендуется)

**Стратегия:**
1. Использовать API для обучения/генерации парсеров (редко, но точно)
2. Использовать локальную модель для регулярного парсинга (часто, но приватно)

**Преимущества:**
- ✅ Баланс между точностью и приватностью
- ✅ Экономия на API запросах
- ✅ Работает офлайн для парсинга

**Реализация:**
```rust
// src-tauri/src/ai_parser_hybrid.rs
pub enum AIParserType {
    Ollama(String), // model name
    OpenAI(String), // API key
    Anthropic(String), // API key
}

pub struct HybridAIParser {
    parser_type: AIParserType,
}

impl HybridAIParser {
    pub fn new(parser_type: AIParserType) -> Self {
        Self { parser_type }
    }

    /// Генерирует парсер (использует API для точности)
    pub async fn generate_parser(
        &self,
        html: &str,
        description: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        match &self.parser_type {
            AIParserType::OpenAI(key) => {
                let parser = OpenAIParser::new(key.clone(), None);
                parser.generate_parser(html, description).await
            }
            AIParserType::Ollama(model) => {
                let parser = AIParser::new(model.clone());
                parser.generate_parser(html, description).await
            }
            _ => Err("Unsupported parser type for generation".into()),
        }
    }

    /// Извлекает данные (использует локальную модель для приватности)
    pub async fn extract_data(
        &self,
        html: &str,
        fields: &[&str],
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        match &self.parser_type {
            AIParserType::Ollama(model) => {
                let parser = AIParser::new(model.clone());
                parser.extract_data(html, fields).await
            }
            _ => Err("Data extraction requires local model".into()),
        }
    }
}
```

---

## Интеграция в UI (Svelte)

### Добавление AI кнопки в ParserBuilder

```svelte
<!-- src/components/ParserBuilder.svelte -->
<script lang="ts">
  // ... existing code ...
  
  let aiEnabled = $state(false);
  let aiModel = $state('ollama'); // 'ollama' | 'openai' | 'anthropic'
  let aiApiKey = $state('');
  let isGenerating = $state(false);

  async function handleAIGenerate() {
    if (!currentUrl || !selectedSite) {
      alert('Загрузите страницу сайта для генерации парсера');
      return;
    }

    isGenerating = true;
    try {
      // Получаем HTML страницы
      const html = await invoke('get_cached_page', { url: currentUrl });
      
      // Описание данных для извлечения
      const description = `Извлеки информацию о модах: название, ссылка, описание, изображение`;
      
      // Генерируем парсер через AI
      const config = await invoke('ai_generate_parser', {
        html,
        description,
        model: aiModel,
        apiKey: aiApiKey || null,
      });

      // Создаем узлы из конфигурации
      createNodesFromConfig(config);
      
      // Автоматически генерируем код
      generateParserCode();
      
      alert('Парсер успешно сгенерирован с помощью AI!');
    } catch (error) {
      console.error('AI generation error:', error);
      alert('Ошибка генерации парсера: ' + error);
    } finally {
      isGenerating = false;
    }
  }

  function createNodesFromConfig(config: any) {
    // Очищаем существующие узлы
    nodes = [];
    edges = [];

    // Создаем selector node
    const selectorNode: Node = {
      id: 'selector-root',
      type: 'selector',
      position: { x: 100, y: 200 },
      data: { selector: config.list_selector || '' },
    };
    nodes = [selectorNode];

    // Создаем extract nodes
    const extractFields = [
      { key: 'title_selector', attribute: 'text', label: 'Название' },
      { key: 'url_selector', attribute: 'href', label: 'Ссылка' },
      { key: 'description_selector', attribute: 'text', label: 'Описание' },
      { key: 'image_selector', attribute: 'src', label: 'Изображение' },
    ];

    extractFields.forEach((field, index) => {
      if (config[field.key]) {
        const extractNode: Node = {
          id: `extract-${field.key}`,
          type: 'extract',
          position: { x: 400, y: 200 + index * 100 },
          data: {
            selector: config[field.key],
            attribute: field.attribute,
            label: field.label,
          },
        };
        
        nodes = [...nodes, extractNode];
        edges = [...edges, {
          id: `edge-${field.key}`,
          source: selectorNode.id,
          target: extractNode.id,
          type: 'smoothstep',
        }];
      }
    });
  }
</script>

<!-- В header-actions добавить: -->
<button 
  class="btn-primary" 
  onclick={handleAIGenerate}
  disabled={isGenerating}
  title="Сгенерировать парсер с помощью AI"
>
  {#if isGenerating}
    <span>Генерация...</span>
  {:else}
    <span>🤖 AI Генерация</span>
  {/if}
</button>

<!-- Настройки AI (можно в отдельном модальном окне) -->
<div class="ai-settings">
  <label>
    <input type="radio" bind:group={aiModel} value="ollama" />
    Ollama (локально)
  </label>
  <label>
    <input type="radio" bind:group={aiModel} value="openai" />
    OpenAI API
  </label>
  {#if aiModel === 'openai'}
    <input 
      type="password" 
      bind:value={aiApiKey} 
      placeholder="API ключ OpenAI"
    />
  {/if}
</div>
```

---

## Добавление Tauri команд

```rust
// src-tauri/src/main.rs
use crate::ai_parser::{AIParser, HybridAIParser, AIParserType};

#[tauri::command]
async fn ai_generate_parser(
    html: String,
    description: String,
    model: String,
    api_key: Option<String>,
) -> Result<serde_json::Value, String> {
    let parser = match model.as_str() {
        "ollama" => {
            let ai_parser = AIParser::new("llama3.2:3b".to_string());
            ai_parser.generate_parser(&html, &description).await
        }
        "openai" => {
            let api_key = api_key.ok_or("API key required for OpenAI")?;
            let ai_parser = OpenAIParser::new(api_key, None);
            ai_parser.generate_parser(&html, &description).await
        }
        _ => Err("Unknown model type".into()),
    };

    parser.map_err(|e| e.to_string())
}

#[tauri::command]
async fn ai_extract_data(
    html: String,
    fields: Vec<String>,
    model: String,
) -> Result<Vec<serde_json::Value>, String> {
    let ai_parser = AIParser::new(model);
    let field_refs: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
    ai_parser
        .extract_data(&html, &field_refs)
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_mcp::init_with_config(/* ... */))
        .invoke_handler(tauri::generate_handler![
            // ... existing commands ...
            ai_generate_parser,
            ai_extract_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Рекомендации по выбору

### Для начала (быстрый старт):
1. **Ollama + llama3.2:3b** - бесплатно, локально, достаточно для простых парсеров

### Для продакшена:
1. **Гибридный подход** - OpenAI для генерации, Ollama для парсинга
2. Или **только OpenAI GPT-4o-mini** - если бюджет позволяет

### Для максимальной приватности:
1. **Только Ollama** с более мощной моделью (mistral:7b или qwen2.5:7b)

---

## Следующие шаги

1. Выберите вариант интеграции
2. Добавьте зависимости в `Cargo.toml`
3. Создайте модуль `ai_parser.rs` с выбранным вариантом
4. Добавьте Tauri команды в `main.rs`
5. Интегрируйте UI кнопку в `ParserBuilder.svelte`
6. Протестируйте на реальных сайтах

---

## Примеры промптов для обучения

### Промпт для генерации парсера:
```
Ты эксперт по парсингу веб-страниц. Проанализируй HTML и создай конфигурацию парсера.

Описание данных для извлечения: {описание}
HTML: {html}

Верни JSON конфигурацию с CSS селекторами для:
- list_selector: селектор для списка элементов
- title_selector: селектор для заголовка
- url_selector: селектор для ссылки
- description_selector: селектор для описания
- image_selector: селектор для изображения
```

### Промпт для извлечения данных:
```
Извлеки следующие данные из HTML: {поля}

HTML: {html}

Верни массив объектов в JSON формате. Каждый объект должен содержать указанные поля.
```

---

## Troubleshooting

### Ollama не запускается:
- Проверьте, что Ollama установлен и запущен
- Проверьте порт 11434: `curl http://localhost:11434/api/tags`

### OpenAI API ошибки:
- Проверьте API ключ
- Проверьте баланс аккаунта
- Убедитесь, что модель доступна

### Плохие результаты парсинга:
- Увеличьте контекст HTML (больше символов)
- Уточните описание данных
- Попробуйте другую модель


















