import { invoke } from '@/lib/tauri-wrapper';
import type { Message } from './useChatStorage';

export interface AIChatOptions {
  aiModelType: 'ollama' | 'openai' | 'anthropic' | 'google';
  aiModelName: string;
  aiApiKey: string;
  aiOllamaUrl: string;
}

const SYSTEM_PROMPT = `Ты эксперт по парсингу веб-страниц. Твоя задача - анализировать HTML элементы и создавать конфигурации парсеров.

ВАЖНО: ВСЕГДА отвечай ТОЛЬКО в формате JSON. Никакого дополнительного текста до или после JSON.

Формат ответа (строго соблюдай):
{
  "list_selector": "CSS селектор для списка всех похожих элементов (например: div.filekmod)",
  "title_selector": "CSS селектор для заголовка внутри каждого элемента списка (относительный, например: h3, .title, a)",
  "url_selector": "CSS селектор для ссылки (относительный, например: a[href], .link)",
  "description_selector": "CSS селектор для описания (относительный, например: .description, p, span)",
  "image_selector": "CSS селектор для изображения (относительный, например: img, .thumb img)"
}

Правила:
1. list_selector должен находить ВСЕ похожие элементы на странице
2. Остальные селекторы должны быть ОТНОСИТЕЛЬНЫМИ (без указания list_selector в начале)
3. Если пользователь прикрепил элемент, используй его селектор как основу для list_selector
4. Если селектор не определен, используй пустую строку ""
5. Отвечай ТОЛЬКО JSON, без объяснений и дополнительного текста`;

/**
 * Composable для работы с AI чатом
 *
 * Управляет отправкой сообщений в AI, обработкой ответов,
 * извлечением JSON конфигураций из ответов AI.
 *
 * @param options - настройки AI
 * @returns Функции для работы с AI чатом
 */
export function useAIChat(options: () => AIChatOptions) {
  /**
   * Отправляет сообщение в AI и получает ответ
   *
   * @param messageHistory - история сообщений для контекста
   * @param messageText - текст сообщения пользователя
   * @param elementInfo - информация о прикрепленном элементе (опционально)
   * @returns Ответ от AI
   */
  async function sendMessageToAI(
    messageHistory: [string, string][],
    messageText: string,
    elementInfo?: Message['attachedElement']
  ): Promise<string> {
    const currentOptions = options();

    // Формируем историю сообщений для AI
    const history: [string, string][] = [
      ['system', SYSTEM_PROMPT],
      ...messageHistory,
      [
        'user',
        messageText +
          (elementInfo
            ? `\n\n[Прикреплен HTML элемент]\nСелектор: ${elementInfo.selector}\nТег: ${elementInfo.tagName}\nТекст: ${elementInfo.text}\nАтрибуты: ${JSON.stringify(elementInfo.attributes)}`
            : ''),
      ],
    ];

    // Отправляем запрос к AI
    let ollamaUrl = currentOptions.aiOllamaUrl;
    if (currentOptions.aiModelType === 'ollama' && ollamaUrl && !ollamaUrl.includes('/api/')) {
      ollamaUrl = `${ollamaUrl}/api/generate`;
    }

    const response = await invoke<string>('ai_chat', {
      messages: history,
      modelType: currentOptions.aiModelType,
      modelName: currentOptions.aiModelName || null,
      apiKey: currentOptions.aiApiKey || null,
      ollamaUrl: ollamaUrl || null,
    });

    return response;
  }

  /**
   * Извлекает JSON конфигурацию из ответа AI
   *
   * Использует несколько стратегий для извлечения JSON:
   * 1. Прямой парсинг, если ответ - чистый JSON
   * 2. Поиск JSON блока между ```json и ```
   * 3. Поиск первого JSON объекта
   *
   * @param response - ответ от AI
   * @returns JSON конфигурация или null
   */
  function extractJSONFromResponse(response: string): any | null {
    // Стратегия 1: Прямой парсинг, если ответ - чистый JSON
    try {
      const parsed = JSON.parse(response.trim());
      if (parsed && typeof parsed === 'object' && parsed.list_selector) {
        return parsed;
      }
    } catch {
      // Стратегия 2: Поиск JSON блока между ```json и ```
      const jsonBlockMatch = response.match(/```json\s*([\s\S]*?)\s*```/);
      if (jsonBlockMatch) {
        try {
          const parsed = JSON.parse(jsonBlockMatch[1].trim());
          if (parsed && typeof parsed === 'object' && parsed.list_selector) {
            return parsed;
          }
        } catch {
          // Стратегия 3: Поиск первого JSON объекта
          const jsonMatch = response.match(/\{[\s\S]*\}/);
          if (jsonMatch) {
            try {
              const parsed = JSON.parse(jsonMatch[0]);
              if (parsed && typeof parsed === 'object' && parsed.list_selector) {
                return parsed;
              }
            } catch (e3) {
              console.error('Failed to parse JSON:', e3);
            }
          }
        }
      } else {
        // Стратегия 4: Поиск JSON объекта без блоков кода
        const jsonMatch = response.match(/\{[\s\S]*\}/);
        if (jsonMatch) {
          try {
            const parsed = JSON.parse(jsonMatch[0]);
            if (parsed && typeof parsed === 'object' && parsed.list_selector) {
              return parsed;
            }
          } catch (e4) {
            console.error('Failed to parse JSON:', e4);
          }
        }
      }
    }

    return null;
  }

  /**
   * Извлекает код из сообщения AI
   *
   * Ищет код в markdown блоках или HTML тегах <code>
   *
   * @param content - содержимое сообщения
   * @returns Извлеченный код или null
   */
  function extractCodeFromMessage(content: string): string | null {
    // Ищем код в markdown блоках
    const codeBlockMatch = content.match(/```(?:rust|rs)?\s*([\s\S]*?)```/);
    if (codeBlockMatch) {
      return codeBlockMatch[1].trim();
    }

    // Ищем код между тегами <code>
    const htmlCodeMatch = content.match(/<code>([\s\S]*?)<\/code>/);
    if (htmlCodeMatch) {
      return htmlCodeMatch[1].trim();
    }

    return null;
  }

  /**
   * Извлекает JSON конфигурацию из сообщения AI
   *
   * Использует несколько стратегий для поиска JSON в тексте сообщения
   *
   * @param content - содержимое сообщения
   * @returns JSON конфигурация или null
   */
  function extractJSONConfigFromMessage(content: string): any | null {
    try {
      // Стратегия 1: Поиск JSON блока между ```json и ```
      const jsonBlockMatch = content.match(/```json\s*([\s\S]*?)\s*```/);
      if (jsonBlockMatch) {
        try {
          const parsed = JSON.parse(jsonBlockMatch[1].trim());
          if (parsed && typeof parsed === 'object' && parsed.list_selector) {
            return parsed;
          }
        } catch {
          // Невалидный JSON - просто игнорируем
        }
      }

      // Стратегия 2: Поиск первого JSON объекта с list_selector (более точный паттерн)
      const jsonMatch = content.match(/\{[^{}]*"list_selector"[^{}]*\}/);
      if (jsonMatch) {
        try {
          const parsed = JSON.parse(jsonMatch[0]);
          if (parsed && typeof parsed === 'object' && parsed.list_selector) {
            return parsed;
          }
        } catch {
          // Невалидный JSON - просто игнорируем
        }
      }

      // Стратегия 3: Попытка найти JSON объект с более умным парсингом
      const jsonPattern = /\{[\s\S]{0,2000}?"list_selector"[\s\S]{0,2000}?\}/;
      const anyJsonMatch = content.match(jsonPattern);
      if (anyJsonMatch) {
        try {
          // Пытаемся найти закрывающую скобку
          let jsonStr = anyJsonMatch[0];
          let braceCount = 0;
          let endIndex = -1;

          for (let i = 0; i < jsonStr.length; i++) {
            if (jsonStr[i] === '{') {
              braceCount++;
            }
            if (jsonStr[i] === '}') {
              braceCount--;
              if (braceCount === 0) {
                endIndex = i + 1;
                break;
              }
            }
          }

          if (endIndex > 0) {
            jsonStr = jsonStr.substring(0, endIndex);
            const parsed = JSON.parse(jsonStr);
            if (parsed && typeof parsed === 'object' && parsed.list_selector) {
              return parsed;
            }
          }
        } catch {
          // Невалидный JSON - просто игнорируем
        }
      }
    } catch {
      // Общая ошибка - не логируем, просто возвращаем null
    }

    return null;
  }

  return {
    sendMessageToAI,
    extractJSONFromResponse,
    extractCodeFromMessage,
    extractJSONConfigFromMessage,
  };
}
