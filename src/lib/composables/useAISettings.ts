import { invoke } from '@/lib/tauri-wrapper';

export interface AISettings {
  modelType: 'ollama' | 'openai' | 'anthropic' | 'google';
  modelName: string;
  apiKey: string;
  ollamaUrl: string;
  description: string;
}

export interface OllamaStatus {
  status: 'checking' | 'connected' | 'error' | null;
  models: string[];
  error: string | null;
}

/**
 * Composable для управления настройками AI
 *
 * Управляет настройками AI моделей, проверкой подключения к Ollama,
 * загрузкой списка моделей.
 *
 * @param initialSettings - начальные настройки AI
 * @returns Объект с настройками и методами для управления
 */
export function useAISettings(initialSettings: Partial<AISettings> = {}) {
  const modelType = $state<AISettings['modelType']>(initialSettings.modelType || 'ollama');
  let modelName = $state<string>(initialSettings.modelName || 'llama3.2:3b');
  const apiKey = $state<string>(initialSettings.apiKey || '');
  const ollamaUrl = $state<string>(initialSettings.ollamaUrl || 'http://localhost:11434');
  const description = $state<string>(
    initialSettings.description ||
      'Извлеки информацию о модах: название, ссылка, описание, изображение'
  );

  let ollamaStatus = $state<OllamaStatus['status']>(null);
  let ollamaModels = $state<string[]>([]);
  let ollamaError = $state<string | null>(null);

  /**
   * Проверяет статус Ollama и загружает список моделей
   */
  async function checkOllama() {
    ollamaStatus = 'checking';
    ollamaError = null;

    try {
      const baseUrl = ollamaUrl.replace('/api/generate', '').replace('/api/tags', '');
      const models = await invoke<string[]>('ai_check_ollama', {
        ollamaUrl: baseUrl || null,
      });

      ollamaModels = models;
      ollamaStatus = 'connected';

      // Если выбранная модель не в списке, выбираем первую доступную
      if (models.length > 0 && !models.includes(modelName)) {
        modelName = models[0];
      }
    } catch (error: any) {
      console.error('Ollama check error:', error);
      ollamaStatus = 'error';
      ollamaError = error.message || error;
      ollamaModels = [];
    }
  }

  /**
   * Обрабатывает изменение типа AI модели
   */
  async function handleModelTypeChange() {
    if (modelType === 'ollama') {
      await checkOllama();
    } else {
      ollamaStatus = null;
      ollamaError = null;
      ollamaModels = [];
    }
  }

  return {
    // Settings
    modelType,
    modelName,
    apiKey,
    ollamaUrl,
    description,
    // Ollama status
    ollamaStatus,
    ollamaModels,
    ollamaError,
    // Methods
    checkOllama,
    handleModelTypeChange,
  };
}
