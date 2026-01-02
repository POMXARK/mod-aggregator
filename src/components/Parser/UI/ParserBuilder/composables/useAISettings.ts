import { writable, get } from 'svelte/store';
import { invoke } from '@/lib/tauri-wrapper';
import type { AISettings, OllamaState } from '../types/parser-builder.types';

/**
 * Composable для управления настройками AI
 * Управляет подключением к AI сервисам, моделями и генерацией кода
 */
export function useAISettings() {
  // AI настройки
  const aiSettingsStore = writable<AISettings>({
    modelType: 'ollama',
    modelName: 'llama3.2:3b',
    apiKey: '',
    ollamaUrl: 'http://localhost:11434',
    description: 'Извлеки информацию о модах: название, ссылка, описание, изображение'
  });

  // Состояние AI операций
  const isAIGeneratingStore = writable(false);
  const showAISettingsStore = writable(false);

  // Состояние Ollama
  let ollamaState: OllamaState = {
    models: [],
    status: null,
    error: null
  };

  // Callbacks для чата
  let addAIMessageToChat: ((content: string) => void) | null = null;

  // Геттеры
  const getIsAIGenerating = () => isAIGeneratingStore;
  const getShowAISettings = () => showAISettingsStore;
  const getOllamaState = () => ollamaState;
  const getAddAIMessageToChat = () => addAIMessageToChat;

  // Сеттеры
  const setAISettings = (value: AISettings) => aiSettingsStore.set(value);
  const setIsAIGenerating = (value: boolean) => isAIGeneratingStore.set(value);
  const setShowAISettings = (value: boolean) => showAISettingsStore.set(value);
  const setOllamaState = (value: OllamaState) => { ollamaState = value; };
  const setAddAIMessageToChat = (value: ((content: string) => void) | null) => { addAIMessageToChat = value; };

  // Специфические функции для работы с AI

  /**
   * Проверяет статус Ollama и загружает список моделей
   */
  async function checkOllama() {
    ollamaState.status = 'checking';
    ollamaState.error = null;

    try {
      const currentSettings = get(aiSettingsStore);
      const baseUrl = currentSettings.ollamaUrl.replace('/api/generate', '').replace('/api/tags', '');
      const models = await invoke<string[]>('ai_check_ollama', {
        ollamaUrl: baseUrl || null,
      });

      ollamaState.models = models;
      ollamaState.status = 'connected';

      // Если выбранная модель не в списке, выбираем первую доступную
      if (models.length > 0 && !models.includes(currentSettings.modelName)) {
        aiSettingsStore.update(settings => ({ ...settings, modelName: models[0] }));
      }
    } catch (error: unknown) {
      console.error('Ollama check error:', error);
      ollamaState.status = 'error';
      ollamaState.error = error instanceof Error ? error.message : String(error);
      ollamaState.models = [];
    }
  }

  /**
   * Обрабатывает изменение типа AI модели
   */
  async function handleAIModelTypeChange() {
    const currentSettings = get(aiSettingsStore);
    if (currentSettings.modelType === 'ollama') {
      await checkOllama();
    } else {
      ollamaState.status = null;
      ollamaState.error = null;
      ollamaState.models = [];
    }
  }

  /**
   * Генерирует парсер с помощью AI
   */
  async function generateParserWithAI(
    html: string,
    onSuccess: (config: unknown) => void,
    onError: (error: string) => void
  ) {
    isAIGeneratingStore.set(true);

    try {
      const currentSettings = get(aiSettingsStore);
      // Для Ollama добавляем /api/generate к базовому URL
      let ollamaUrl = currentSettings.ollamaUrl;
      if (currentSettings.modelType === 'ollama' && ollamaUrl && !ollamaUrl.includes('/api/')) {
        ollamaUrl = `${ollamaUrl}/api/generate`;
      }

      const config = await invoke('ai_generate_parser', {
        html,
        description: currentSettings.description,
        modelType: currentSettings.modelType,
        modelName: currentSettings.modelName || null,
        apiKey: currentSettings.apiKey || null,
        ollamaUrl: ollamaUrl || null,
      });

      onSuccess(config);
      showAISettingsStore.set(false);
    } catch (err: unknown) {
      console.error('AI generation error:', err);
      const errorMessage = err instanceof Error ? err.message : String(err);
      onError(`Ошибка генерации парсера: ${errorMessage}`);
    } finally {
      isAIGeneratingStore.set(false);
    }
  }

  /**
   * Проверяет код с помощью AI
   */
  async function checkCodeWithAI(
    code: string,
    parserConfig: unknown,
    onMessage: (message: string) => void,
    onError: (error: string) => void
  ) {
    try {
      // Добавляем JSON конфигурацию в чат для просмотра
      const configJson = JSON.stringify(parserConfig, null, 2);
      onMessage(`📋 **JSON конфигурация парсера:**\n\n\`\`\`json\n${configJson}\n\`\`\`\n\nПроверяю сгенерированный Rust код...`);

      // Формируем промпт с четким указанием, что нужно проверить именно Rust код
      const prompt = `Проверь следующий Rust код парсера на ошибки, оптимизацию и лучшие практики. Укажи конкретные проблемы и предложи улучшения.

ВАЖНО: Проверяй именно Rust код ниже, а не JSON конфигурацию!

Rust код парсера:
\`\`\`rust
${code.trim()}
\`\`\`

Проанализируй этот код на:
1. Синтаксические ошибки
2. Логические ошибки
3. Оптимизацию производительности
4. Лучшие практики Rust
5. Обработку ошибок
6. Безопасность парсинга

Ответь на русском языке, структурированно и конкретно.`;

      const currentSettings = get(aiSettingsStore);
      let ollamaUrl = currentSettings.ollamaUrl;
      if (currentSettings.modelType === 'ollama' && ollamaUrl && !ollamaUrl.includes('/api/')) {
        ollamaUrl = `${ollamaUrl}/api/generate`;
      }

      const review = await invoke<string>('ai_chat', {
        messages: [
          [
            'system',
            'Ты эксперт по Rust и парсингу веб-страниц. Анализируй Rust код парсера и давай конкретные рекомендации. НЕ анализируй JSON конфигурацию, только Rust код.',
          ],
          ['user', prompt],
        ],
        modelType: currentSettings.modelType,
        modelName: currentSettings.modelName || null,
        apiKey: currentSettings.apiKey || null,
        ollamaUrl: ollamaUrl || null,
      });

      // Добавляем результат проверки в чат
      onMessage(`🤖 **Отзыв AI по коду:**\n\n${review}`);
    } catch (err: unknown) {
      console.error('AI code review error:', err);
      const errorMsg = `Ошибка проверки кода: ${err instanceof Error ? err.message : String(err)}`;
      onError(errorMsg);
    }
  }

  /**
   * Обновляет настройки AI
   */
  // function updateAISettings(updates: Partial<AISettings>) {
  //   aiSettings = { ...aiSettings, ...updates };
  // }

  /**
   * Сбрасывает настройки AI к значениям по умолчанию
   */
  // function resetAISettings() {
  //   aiSettings = {
  //     modelType: 'ollama',
  //     modelName: 'llama3.2:3b',
  //     apiKey: '',
  //     ollamaUrl: 'http://localhost:11434',
  //     description: 'Извлеки информацию о модах: название, ссылка, описание, изображение'
  //   };
  //   ollamaState = {
  //     models: [],
  //     status: null,
  //     error: null
  //   };
  // }

  return {
    // Stores
    aiSettingsStore,
    isAIGeneratingStore,
    showAISettingsStore,

    // Геттеры
    getIsAIGenerating,
    getShowAISettings,
    getOllamaState,
    getAddAIMessageToChat,

    // Сеттеры
    setAISettings,
    setIsAIGenerating,
    setShowAISettings,
    setOllamaState,
    setAddAIMessageToChat,

    // Действия
    checkOllama,
    handleAIModelTypeChange,
    generateParserWithAI,
    checkCodeWithAI,
  };
}
