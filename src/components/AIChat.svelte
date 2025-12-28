<script lang="ts">
  import type { Node } from '@xyflow/svelte';
  import { invoke } from '../lib/tauri-wrapper';
  import type { Chat, ChatMessage, ParserConfig, AIModelConfig } from '../lib/types';

  interface Props {
    aiModelType?: AIModelConfig['type'];
    aiModelName?: string;
    aiApiKey?: string;
    aiOllamaUrl?: string;
    currentUrl?: string;
    nodes?: Node[];
    onCreateNodes?: (config: ParserConfig) => void;
    selectedElementInfo?: {
      selector: string;
      elementInfo: {
        tagName: string;
        text: string;
        attributes: Record<string, string>;
        similarElements?: number;
      };
    } | null;
    onGenerateCode?: () => void;
    onTestParser?: () => Promise<void>;
    onCheckCodeWithAI?: () => Promise<void>;
    generatedCode?: string;
    onApplyCode?: (code: string) => void;
    addAIMessage?: (content: string) => void;
  }

  let {
    aiModelType = 'ollama',
    aiModelName = 'llama3.2:3b',
    aiApiKey = '',
    aiOllamaUrl = 'http://localhost:11434',
    currentUrl = '',
    nodes = [],
    onCreateNodes = () => {},
    selectedElementInfo = null,
    onGenerateCode = () => {},
    onTestParser = async () => {},
    onCheckCodeWithAI = async () => {},
    generatedCode = '',
    onApplyCode = () => {},
    addAIMessage = $bindable(),
  }: Props = $props();

  // Управление чатами
  const STORAGE_KEY = 'ai_chats';
  let chats = $state<Chat[]>([]);
  let currentChatId = $state<string | null>(null);
  let showChatList = $state(false);

  // Текущий чат - используем $state для избежания циклических обновлений
  let messages = $state<ChatMessage[]>([]);

  // Функция для обновления сообщений из текущего чата
  function updateMessagesFromChat() {
    if (!currentChatId) {
      messages = [];
      return;
    }
    const chat = chats.find(c => c.id === currentChatId);
    messages = chat?.messages || [];
  }

  let inputMessage = $state('');
  let isSending = $state(false);
  let chatContainer: HTMLDivElement;
  let attachedElement = $state<ChatMessage['attachedElement'] | null>(null);

  // Системный промпт для AI
  const systemPrompt = `Ты эксперт по парсингу веб-страниц. Твоя задача - анализировать HTML элементы и создавать конфигурации парсеров.

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

  // Загрузка чатов из localStorage
  function loadChats() {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        const parsed = JSON.parse(stored);
        const loadedChats = parsed.map((chat: unknown) => {
          const c = chat as { messages: unknown[]; createdAt: string; updatedAt: string };
          return {
            ...c,
            messages: c.messages.map((msg: unknown) => ({
              ...(msg as Record<string, unknown>),
              timestamp: new Date((msg as { timestamp: string }).timestamp),
            })),
            createdAt: new Date(c.createdAt),
            updatedAt: new Date(c.updatedAt),
          };
        });

        chats = loadedChats;

        // Если есть чаты, выбираем последний
        if (chats.length > 0) {
          currentChatId = chats[chats.length - 1].id;
        } else {
          // Если нет чатов, создаем первый
          createNewChat();
        }
      } else {
        // Если нет сохраненных чатов, создаем первый
        createNewChat();
      }
    } catch (e) {
      console.error('Failed to load chats:', e);
      createNewChat();
    }
  }

  // Сохранение чатов в localStorage
  function saveChats() {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(chats));
    } catch (e) {
      console.error('Failed to save chats:', e);
    }
  }

  // Создание нового чата
  function createNewChat() {
    const newChat: Chat = {
      id: `chat-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
      title: `Чат ${chats.length + 1}`,
      messages: [
        {
          id: `msg-${Date.now()}`,
          role: 'assistant',
          content:
            'Привет! Я помогу тебе создать парсер для извлечения данных с сайтов. Опиши, какие данные нужно извлечь, и я создам конфигурацию парсера.',
          timestamp: new Date(),
        },
      ],
      createdAt: new Date(),
      updatedAt: new Date(),
    };

    chats = [...chats, newChat];
    currentChatId = newChat.id;
    updateMessagesFromChat();
    saveChats();
  }

  // Удаление чата
  function deleteChat(chatId: string) {
    chats = chats.filter(c => c.id !== chatId);
    if (currentChatId === chatId) {
      if (chats.length > 0) {
        currentChatId = chats[0].id;
        updateMessagesFromChat();
      } else {
        createNewChat();
      }
    }
    saveChats();
  }

  // Обновление чата
  function updateChat(chatId: string, updater: (chat: Chat) => Chat) {
    chats = chats.map(chat => {
      if (chat.id === chatId) {
        const updated = updater(chat);
        updated.updatedAt = new Date();
        return updated;
      }
      return chat;
    });

    // Обновляем сообщения, если обновлен текущий чат
    if (chatId === currentChatId) {
      updateMessagesFromChat();
    }

    saveChats();
  }

  // Функция для добавления сообщения от AI (для использования извне)
  function addAIMessageToChat(content: string) {
    if (!currentChatId) {
      createNewChat();
    }

    const aiMessage: ChatMessage = {
      id: `msg-${Date.now()}`,
      role: 'assistant',
      content: content,
      timestamp: new Date(),
    };

    updateChat(currentChatId!, chat => ({
      ...chat,
      messages: [...chat.messages, aiMessage],
    }));
  }

  // Экспортируем функцию через bindable
  $effect(() => {
    addAIMessage = addAIMessageToChat;
  });

  // Прикрепление элемента
  function attachElement() {
    if (selectedElementInfo) {
      attachedElement = {
        selector: selectedElementInfo.selector,
        tagName: selectedElementInfo.elementInfo.tagName,
        text: selectedElementInfo.elementInfo.text,
        attributes: selectedElementInfo.elementInfo.attributes,
      };
    }
  }

  // Удаление прикрепленного элемента
  function removeAttachedElement() {
    attachedElement = null;
  }

  // Удаление сообщения
  function deleteMessage(messageId: string) {
    if (!currentChatId) {
      return;
    }

    updateChat(currentChatId, chat => ({
      ...chat,
      messages: chat.messages.filter(m => m.id !== messageId),
    }));
  }

  // Извлечение кода из сообщения AI
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

  // Извлечение JSON конфигурации из сообщения AI
  function extractJSONConfigFromMessage(content: string): ParserConfig | null {
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
          // Невалидный JSON - просто игнорируем, не логируем
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
          // Невалидный JSON - просто игнорируем, не логируем
        }
      }

      // Стратегия 3: Попытка найти JSON объект с более умным парсингом
      // Ищем объект, который начинается с { и содержит list_selector
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
          // Невалидный JSON - просто игнорируем, не логируем
        }
      }
    } catch {
      // Общая ошибка - не логируем, просто возвращаем null
    }

    return null;
  }

  // Восстановление конфигурации парсера из сообщения
  function restoreConfigFromMessage(messageId: string) {
    if (!currentChatId) {
      return;
    }

    const chat = chats.find(c => c.id === currentChatId);
    const message = chat?.messages.find(m => m.id === messageId);
    if (!message || message.role !== 'assistant') {
      return;
    }

    // Пытаемся извлечь JSON конфигурацию
    const config = extractJSONConfigFromMessage(message.content);

    if (config && config.list_selector) {
      // Если есть JSON конфигурация, создаем узлы из неё (как при "повтори")
      try {
        if (onCreateNodes) {
          onCreateNodes(config);
          // onCreateNodes автоматически вызовет generateParserCode, но для надежности вызываем onGenerateCode тоже
          setTimeout(() => {
            if (onGenerateCode) {
              onGenerateCode();
            }
          }, 150);
        } else {
          console.warn('onCreateNodes callback not available');
          if (onGenerateCode) {
            onGenerateCode();
          }
        }

        // Добавляем сообщение об успехе
        if (addAIMessage) {
          addAIMessage('✅ Конфигурация парсера восстановлена из сообщения (узлы и код созданы)');
        }
      } catch (error: unknown) {
        console.error('Failed to restore config from message:', error);
        if (addAIMessage) {
          const errorMessage = error instanceof Error ? error.message : String(error);
          addAIMessage(`❌ Ошибка восстановления конфигурации: ${errorMessage}`);
        }
      }
    } else {
      // Если нет JSON конфигурации, сообщаем об этом
      if (addAIMessage) {
        addAIMessage('❌ В этом сообщении не найдена JSON конфигурация парсера');
      }
    }
  }

  // Генерация кода из старого сообщения (работает как "повтори")
  function generateCodeFromMessage(messageId: string) {
    if (!currentChatId) {
      return;
    }

    const chat = chats.find(c => c.id === currentChatId);
    const message = chat?.messages.find(m => m.id === messageId);
    if (!message || message.role !== 'assistant') {
      return;
    }

    // Пытаемся извлечь JSON конфигурацию
    const config = extractJSONConfigFromMessage(message.content);

    if (config && config.list_selector) {
      // Если есть JSON конфигурация, создаем узлы из неё (как при "повтори")
      try {
        if (onCreateNodes) {
          onCreateNodes(config);
          // onCreateNodes автоматически вызовет generateParserCode, но для надежности вызываем onGenerateCode тоже
          setTimeout(() => {
            if (onGenerateCode) {
              onGenerateCode();
            }
          }, 150);
        } else {
          console.warn('onCreateNodes callback not available');
          if (onGenerateCode) {
            onGenerateCode();
          }
        }

        // Добавляем сообщение об успехе
        if (addAIMessage) {
          addAIMessage('✅ Узлы и код восстановлены из сообщения');
        }
      } catch (error: unknown) {
        console.error('Failed to create nodes from message:', error);
        // Если не удалось создать узлы, просто генерируем код из существующих узлов
        if (onGenerateCode) {
          onGenerateCode();
        }
        if (addAIMessage) {
          const errorMessage = error instanceof Error ? error.message : String(error);
          addAIMessage(`❌ Ошибка восстановления: ${errorMessage}`);
        }
      }
    } else {
      // Если нет JSON конфигурации, просто генерируем код из существующих узлов
      if (onGenerateCode) {
        onGenerateCode();
        if (addAIMessage) {
          addAIMessage('⚠️ JSON конфигурация не найдена. Код сгенерирован из существующих узлов.');
        }
      }
    }
  }

  // Копирование содержимого сообщения
  function copyMessageContent(messageId: string) {
    if (!currentChatId) {
      return;
    }

    const chat = chats.find(c => c.id === currentChatId);
    const message = chat?.messages.find(m => m.id === messageId);
    if (!message) {
      return;
    }

    // Копируем текст сообщения (без HTML тегов)
    const textContent = message.content
      .replace(/<[^>]*>/g, '') // Удаляем HTML теги
      .replace(/```[\s\S]*?```/g, match => match) // Оставляем код блоки
      .trim();

    navigator.clipboard
      .writeText(textContent)
      .then(() => {
        // Можно показать уведомление об успешном копировании
        console.log('Message copied to clipboard');
      })
      .catch(err => {
        console.error('Failed to copy message:', err);
      });
  }

  // Применение исправления кода от AI
  function applyCodeFix(messageId: string) {
    if (!currentChatId) {
      return;
    }

    const chat = chats.find(c => c.id === currentChatId);
    const message = chat?.messages.find(m => m.id === messageId);
    if (!message || message.role !== 'assistant') {
      return;
    }

    const code = extractCodeFromMessage(message.content);
    if (code) {
      onApplyCode(code);
    }
  }

  // Проверка генерации кода
  async function checkGeneration() {
    if (!generatedCode) {
      const errorMsg: ChatMessage = {
        id: `msg-${Date.now()}`,
        role: 'assistant',
        content: '❌ Код еще не сгенерирован. Сначала сгенерируйте код парсера.',
        timestamp: new Date(),
      };
      updateChat(currentChatId!, chat => ({
        ...chat,
        messages: [...chat.messages, errorMsg],
      }));
      return;
    }

    await onCheckCodeWithAI();
  }

  // Запуск парсера из чата - просто открывает вкладку runner, где пользователь может запустить парсер
  function runParserFromChat() {
    if (nodes.length === 0) {
      if (addAIMessage) {
        addAIMessage(
          '❌ Нет нод парсера. Сначала создайте парсер через AI или восстановите из сообщения.'
        );
      }
      return;
    }

    if (!currentUrl) {
      if (addAIMessage) {
        addAIMessage('❌ Нет загруженной страницы. Сначала загрузите страницу для тестирования.');
      }
      return;
    }

    // Открываем вкладку runner через callback, если он есть
    // В ParserBuilder это обработается и откроется вкладка runner
    if (onTestParser) {
      // onTestParser может быть функцией, которая открывает runner вкладку
      // Вызываем её, но не ждем результата, так как запуск парсера должен происходить в ParserRunner
      onTestParser().catch((error: unknown) => {
        console.error('Error in onTestParser:', error);
        if (addAIMessage) {
          const errorMessage = error instanceof Error ? error.message : String(error);
          addAIMessage(`❌ Ошибка: ${errorMessage}`);
        }
      });
    } else {
      if (addAIMessage) {
        addAIMessage('ℹ️ Перейдите во вкладку "Запуск парсера" в нижней панели для запуска.');
      }
    }
  }

  // Инициализация - только один раз при монтировании
  let initialized = $state(false);

  $effect(() => {
    if (!initialized) {
      loadChats();
      initialized = true;
    }
  });

  // Обновление сообщений при изменении текущего чата
  $effect(() => {
    updateMessagesFromChat();
  });

  // Прокрутка вниз при новых сообщениях
  $effect(() => {
    if (chatContainer && messages.length > 0) {
      const timeoutId = setTimeout(() => {
        chatContainer.scrollTop = chatContainer.scrollHeight;
      }, 100);
      return () => clearTimeout(timeoutId);
    }
  });

  // Автоматическое прикрепление элемента при его выборе (только если не открыт список чатов)
  $effect(() => {
    if (selectedElementInfo && !showChatList && !attachedElement) {
      // Автоматически прикрепляем элемент, если он выбран
      attachElement();
    }
  });

  async function sendMessage() {
    if ((!inputMessage.trim() && !attachedElement) || isSending || !currentChatId) {
      return;
    }

    const messageId = `msg-${Date.now()}`;
    const userMessage: ChatMessage = {
      id: messageId,
      role: 'user',
      content:
        inputMessage.trim() ||
        (attachedElement ? `Прикреплен элемент: ${attachedElement.tagName}` : ''),
      timestamp: new Date(),
      attachedElement: attachedElement || undefined,
    };

    // Добавляем сообщение в чат
    updateChat(currentChatId, chat => ({
      ...chat,
      messages: [...chat.messages, userMessage],
      title:
        chat.messages.length === 1 ? inputMessage.trim().slice(0, 30) || 'Новый чат' : chat.title,
    }));

    const messageText = inputMessage.trim();
    const elementInfo = attachedElement;
    inputMessage = '';
    attachedElement = null;
    isSending = true;

    try {
      // Формируем историю сообщений для AI
      const currentChat = chats.find(c => c.id === currentChatId);
      const messageHistory: [string, string][] = [
        ['system', systemPrompt],
        ...(currentChat?.messages.slice(0, -1) || []).map(m => {
          let content = m.content;
          if (m.attachedElement) {
            content += `\n\n[Прикреплен HTML элемент]\nСелектор: ${m.attachedElement.selector}\nТег: ${m.attachedElement.tagName}\nТекст: ${m.attachedElement.text}\nАтрибуты: ${JSON.stringify(m.attachedElement.attributes)}`;
          }
          return [m.role, content] as [string, string];
        }),
        [
          'user',
          messageText +
            (elementInfo
              ? `\n\n[Прикреплен HTML элемент]\nСелектор: ${elementInfo.selector}\nТег: ${elementInfo.tagName}\nТекст: ${elementInfo.text}\nАтрибуты: ${JSON.stringify(elementInfo.attributes)}`
              : ''),
        ],
      ];

      // Отправляем запрос к AI
      let ollamaUrl = aiOllamaUrl;
      if (aiModelType === 'ollama' && ollamaUrl && !ollamaUrl.includes('/api/')) {
        ollamaUrl = `${ollamaUrl}/api/generate`;
      }

      const response = await invoke<string>('ai_chat', {
        messages: messageHistory,
        modelType: aiModelType,
        modelName: aiModelName || null,
        apiKey: aiApiKey || null,
        ollamaUrl: ollamaUrl || null,
      });

      // Пытаемся извлечь JSON из ответа (несколько стратегий)
      let config: ParserConfig | null = null;

      // Стратегия 1: Прямой парсинг, если ответ - чистый JSON
      try {
        config = JSON.parse(response.trim());
      } catch {
        // Стратегия 2: Поиск JSON блока между ```json и ```
        const jsonBlockMatch = response.match(/```json\s*([\s\S]*?)\s*```/);
        if (jsonBlockMatch) {
          try {
            config = JSON.parse(jsonBlockMatch[1].trim());
          } catch {
            // Стратегия 3: Поиск первого JSON объекта
            const jsonMatch = response.match(/\{[\s\S]*\}/);
            if (jsonMatch) {
              try {
                config = JSON.parse(jsonMatch[0]);
              } catch {
                console.error('Failed to parse JSON');
              }
            }
          }
        } else {
          // Стратегия 4: Поиск JSON объекта без блоков кода
          const jsonMatch = response.match(/\{[\s\S]*\}/);
          if (jsonMatch) {
            try {
              config = JSON.parse(jsonMatch[0]);
            } catch {
              console.error('Failed to parse JSON');
            }
          }
        }
      }

      if (config && typeof config === 'object' && config.list_selector) {
        // Успешно распарсили JSON с list_selector
        try {
          // Создаем узлы из конфигурации
          onCreateNodes(config);

          const assistantMessage: ChatMessage = {
            id: `msg-${Date.now()}`,
            role: 'assistant',
            content: `✅ Парсер создан! Я создал конфигурацию с селекторами.\n\nНоды созданы на графе. Теперь можно протестировать парсер.`,
            timestamp: new Date(),
          };

          updateChat(currentChatId, chat => ({
            ...chat,
            messages: [...chat.messages, assistantMessage],
          }));
        } catch (createError: unknown) {
          console.error('Failed to create nodes:', createError);
          const errorMessage =
            createError instanceof Error ? createError.message : String(createError);
          const assistantMessage: ChatMessage = {
            id: `msg-${Date.now()}`,
            role: 'assistant',
            content: `✅ Конфигурация получена.\n\n⚠️ Ошибка при создании нод: ${errorMessage}`,
            timestamp: new Date(),
          };

          updateChat(currentChatId, chat => ({
            ...chat,
            messages: [...chat.messages, assistantMessage],
          }));
        }
      } else {
        // Не удалось распарсить JSON или нет list_selector
        const assistantMessage: ChatMessage = {
          id: `msg-${Date.now()}`,
          role: 'assistant',
          content: `⚠️ Не удалось распарсить ответ как JSON конфигурацию.\n\nОтвет AI:\n${response}\n\nПопробуйте переформулировать запрос или убедитесь, что AI возвращает JSON в правильном формате.`,
          timestamp: new Date(),
        };

        updateChat(currentChatId, chat => ({
          ...chat,
          messages: [...chat.messages, assistantMessage],
        }));
      }
    } catch (error: unknown) {
      console.error('Chat error:', error);
      const errorMessage = error instanceof Error ? error.message : String(error);

      updateChat(currentChatId, (chat: Chat) => ({
        ...chat,
        messages: [
          ...chat.messages,
          {
            id: `msg-${Date.now()}`,
            role: 'assistant',
            content: `❌ Ошибка: ${errorMessage}`,
            timestamp: new Date(),
          },
        ],
      }));
    } finally {
      isSending = false;
    }
  }

  function handleKeyPress(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      sendMessage();
    }
  }

  function clearChat() {
    if (!currentChatId) {
      return;
    }
    updateChat(currentChatId, chat => ({
      ...chat,
      messages: [
        {
          id: `msg-${Date.now()}`,
          role: 'assistant',
          content: 'Чат очищен. Чем могу помочь?',
          timestamp: new Date(),
        },
      ],
    }));
  }
</script>

<div class="ai-chat">
  <div class="chat-header">
    <div class="chat-header-left">
      <button
        class="btn-chat-list"
        onclick={() => (showChatList = !showChatList)}
        title="Список чатов"
      >
        💬
      </button>
      <h3>{chats.find(c => c.id === currentChatId)?.title || 'AI Помощник'}</h3>
    </div>
    <div class="chat-header-right">
      <button class="btn-clear" onclick={clearChat} title="Очистить чат"> 🗑️ </button>
    </div>
  </div>

  {#if showChatList}
    <div class="chat-list-panel">
      <div class="chat-list-header">
        <h4>Чаты</h4>
        <button class="btn-new-chat" onclick={createNewChat} title="Новый чат"> ➕ </button>
      </div>
      <div class="chat-list">
        {#each chats as chat (chat.id)}
          <div
            class="chat-item"
            class:active={chat.id === currentChatId}
            onclick={() => {
              currentChatId = chat.id;
              updateMessagesFromChat();
              showChatList = false;
            }}
          >
            <div class="chat-item-title">{chat.title}</div>
            <div class="chat-item-meta">
              {chat.messages.length} сообщений • {new Date(chat.updatedAt).toLocaleDateString(
                'ru-RU'
              )}
            </div>
            <button
              class="btn-delete-chat"
              onclick={e => {
                e.stopPropagation();
                deleteChat(chat.id);
              }}
              title="Удалить чат"
            >
              ×
            </button>
          </div>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Код теперь показывается только в нижней панели, не в чате -->

  <div class="chat-messages" bind:this={chatContainer}>
    {#each messages as message (message.id)}
      <div class="message message-{message.role}">
        <div class="message-header">
          <div class="message-actions">
            {#if message.role === 'assistant'}
              {@const hasJsonConfig = extractJSONConfigFromMessage(message.content)}
              {@const hasCode =
                message.content.includes('```') || message.content.includes('<code>')}
              {@const hasParserMention =
                message.content.includes('Парсер') ||
                message.content.includes('парсер') ||
                message.content.includes('конфигурацию') ||
                message.content.includes('селектор')}
              {#if hasCode}
                <button
                  class="btn-message-action"
                  onclick={() => applyCodeFix(message.id)}
                  title="Применить исправление кода"
                >
                  ✅ Применить код
                </button>
              {/if}
              {#if hasJsonConfig || hasParserMention}
                {#if hasJsonConfig}
                  <button
                    class="btn-message-action"
                    onclick={() => restoreConfigFromMessage(message.id)}
                    title="Восстановить конфигурацию парсера из этого сообщения (создаст узлы и код)"
                  >
                    🔄 Восстановить конфигурацию
                  </button>
                  <button
                    class="btn-message-action"
                    onclick={() => generateCodeFromMessage(message.id)}
                    title="Сгенерировать код из этого сообщения (создаст узлы и код, как при 'повтори')"
                  >
                    📄 Генерировать код
                  </button>
                {/if}
                <button
                  class="btn-message-action"
                  onclick={runParserFromChat}
                  title="Запустить парсер"
                  disabled={nodes.length === 0 || !currentUrl}
                >
                  ▶️ Запустить парсер
                </button>
              {/if}
            {/if}
            <button
              class="btn-message-action"
              onclick={() => copyMessageContent(message.id)}
              title="Копировать сообщение"
            >
              📋 Копировать
            </button>
            <button
              class="btn-delete-message"
              onclick={() => deleteMessage(message.id)}
              title="Удалить сообщение"
            >
              ×
            </button>
          </div>
        </div>
        <div class="message-content">
          {#if message.attachedElement}
            <div class="attached-element">
              <div class="attached-element-header">
                <span class="attached-icon">📎</span>
                <span class="attached-label">Прикреплен элемент</span>
              </div>
              <div class="attached-element-info">
                <div class="attached-info-row">
                  <strong>Селектор:</strong> <code>{message.attachedElement.selector}</code>
                </div>
                <div class="attached-info-row">
                  <strong>Тег:</strong>
                  <span class="tag-badge">{message.attachedElement.tagName}</span>
                </div>
                {#if message.attachedElement.text}
                  <div class="attached-info-row">
                    <strong>Текст:</strong>
                    <span class="text-preview">{message.attachedElement.text.slice(0, 100)}</span>
                  </div>
                {/if}
              </div>
            </div>
          {/if}

          <!-- eslint-disable-next-line svelte/no-at-html-tags -->
          {@html message.content
            .replace(
              /```(?:rust|rs|json)?\s*([\s\S]*?)\s*```/g,
              '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
            )
            .replace(
              /\{[\s\S]*?"list_selector"[\s\S]*?\}/g,
              '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
            )
            .replace(
              /<pre[^>]*>[\s\S]*?<\/pre>/gi,
              '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
            )
            .replace(
              /<code[^>]*>[\s\S]*?<\/code>/gi,
              '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
            )
            .replace(
              /<div[^>]*class="code-header"[^>]*>[\s\S]*?<\/div>/gi,
              '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
            )
            .replace(
              /<div[^>]*class="code-content"[^>]*>[\s\S]*?<\/div>/gi,
              '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
            )
            .replace(
              /<h3[^>]*>Код<\/h3>/gi,
              '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
            )
            .replace(/<button[^>]*>Проверить AI<\/button>/gi, '')
            .replace(/<button[^>]*>Редактировать<\/button>/gi, '')
            .replace(/<button[^>]*>Копировать<\/button>/gi, '')
            .replace(/\n/g, '<br>')}
        </div>
        <div class="message-time">
          {message.timestamp.toLocaleTimeString('ru-RU', { hour: '2-digit', minute: '2-digit' })}
        </div>
      </div>
    {/each}

    {#if isSending}
      <div class="message message-assistant">
        <div class="message-content">
          <span class="typing-indicator">...</span>
        </div>
      </div>
    {/if}
  </div>

  <div class="chat-input">
    {#if attachedElement}
      <div class="attached-element-preview">
        <div class="attached-preview-header">
          <span class="attached-icon">📎</span>
          <span>Элемент: {attachedElement.tagName} ({attachedElement.selector})</span>
          <button class="btn-remove-attachment" onclick={removeAttachedElement} title="Убрать">
            ×
          </button>
        </div>
      </div>
    {/if}

    {#if selectedElementInfo && !attachedElement}
      <div class="attach-element-hint">
        <button class="btn-attach" onclick={attachElement} title="Прикрепить выделенный элемент">
          📎 Прикрепить элемент
        </button>
      </div>
    {/if}

    <div class="chat-input-actions">
      <button
        class="btn-chat-action"
        onclick={() => onGenerateCode()}
        disabled={nodes.length === 0}
        title="Генерировать код парсера"
      >
        📄 Код
      </button>
      <button
        class="btn-chat-action"
        onclick={checkGeneration}
        disabled={!generatedCode}
        title="Проверить код с помощью AI"
      >
        🤖 Проверить
      </button>
      <button
        class="btn-chat-action"
        onclick={runParserFromChat}
        disabled={nodes.length === 0 || !currentUrl}
        title="Запустить парсер"
      >
        ▶️ Тест
      </button>
    </div>
    <div class="chat-input-row">
      <textarea
        bind:value={inputMessage}
        onkeydown={handleKeyPress}
        placeholder="Опиши, какие данные нужно извлечь с сайта..."
        rows="2"
        disabled={isSending}
      ></textarea>
      <button
        class="btn-send"
        onclick={sendMessage}
        disabled={isSending || (!inputMessage.trim() && !attachedElement)}
      >
        {#if isSending}
          ⏳
        {:else}
          ➤
        {/if}
      </button>
    </div>
  </div>
</div>

<style>
  .ai-chat {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: #1e293b;
    border-left: 1px solid #334155;
    position: relative;
  }

  .chat-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: clamp(0.75rem, 1.5vh, 1rem) clamp(1rem, 2vw, 1.25rem);
    border-bottom: 1px solid #334155;
    background: #0f172a;
  }

  .chat-header-left {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex: 1;
    min-width: 0;
  }

  .btn-chat-list {
    background: transparent;
    border: none;
    color: #cbd5e1;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
    font-size: 1rem;
    transition: background-color 0.2s ease;
    flex-shrink: 0;
  }

  .btn-chat-list:hover {
    background: #334155;
  }

  .chat-header h3 {
    margin: 0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    color: #e2e8f0;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .chat-header-right {
    display: flex;
    gap: 0.5rem;
  }

  .btn-clear {
    background: transparent;
    border: none;
    color: #cbd5e1;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
    font-size: 1rem;
    transition: background-color 0.2s ease;
  }

  .btn-clear:hover {
    background: #334155;
  }

  /* Список чатов */
  .chat-list-panel {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: #1e293b;
    border-bottom: 1px solid #334155;
    z-index: 100;
    max-height: 400px;
    display: flex;
    flex-direction: column;
  }

  .chat-list-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: clamp(0.75rem, 1.5vh, 1rem);
    border-bottom: 1px solid #334155;
  }

  .chat-list-header h4 {
    margin: 0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    color: #e2e8f0;
  }

  .btn-new-chat {
    background: #0ea5e9;
    border: none;
    color: white;
    cursor: pointer;
    padding: 0.375rem 0.75rem;
    border-radius: 0.375rem;
    font-size: 1rem;
    transition: background-color 0.2s ease;
  }

  .btn-new-chat:hover {
    background: #0284c7;
  }

  .chat-list {
    overflow-y: auto;
    flex: 1;
  }

  .chat-item {
    padding: clamp(0.75rem, 1.5vh, 1rem);
    border-bottom: 1px solid #334155;
    cursor: pointer;
    transition: background-color 0.2s ease;
    position: relative;
  }

  .chat-item:hover {
    background: #334155;
  }

  .chat-item.active {
    background: #0ea5e9;
  }

  .chat-item-title {
    font-weight: 600;
    color: #e2e8f0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    margin-bottom: 0.25rem;
  }

  .chat-item.active .chat-item-title {
    color: white;
  }

  .chat-item-meta {
    font-size: clamp(0.625rem, 0.8vw, 0.75rem);
    color: #64748b;
  }

  .chat-item.active .chat-item-meta {
    color: rgba(255, 255, 255, 0.8);
  }

  .btn-delete-chat {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    background: transparent;
    border: none;
    color: #cbd5e1;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
    font-size: 1.25rem;
    line-height: 1;
    transition:
      background-color 0.2s ease,
      color 0.2s ease;
    opacity: 0;
  }

  .chat-item:hover .btn-delete-chat {
    opacity: 1;
  }

  .btn-delete-chat:hover {
    background: rgba(239, 68, 68, 0.2);
    color: #ef4444;
  }

  .chat-messages {
    flex: 1;
    overflow-y: auto;
    padding: clamp(0.75rem, 1.5vh, 1rem);
    display: flex;
    flex-direction: column;
    gap: clamp(0.75rem, 1.5vh, 1rem);
  }

  .message {
    display: flex;
    flex-direction: column;
    max-width: 85%;
    animation: messageFadeIn 0.3s ease-out;
    position: relative;
  }

  @keyframes messageFadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .message-header {
    display: flex;
    justify-content: flex-end;
    margin-bottom: 0.25rem;
  }

  .message-actions {
    display: flex;
    gap: 0.25rem;
    align-items: center;
    opacity: 0;
    transition: opacity 0.2s ease;
  }

  .message:hover .message-actions {
    opacity: 1;
  }

  .btn-message-action {
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    color: #0ea5e9;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    line-height: 1;
    transition: all 0.2s ease;
  }

  .btn-message-action:hover {
    background: rgba(14, 165, 233, 0.2);
    border-color: #0ea5e9;
  }

  .btn-delete-message {
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    font-size: 0.875rem;
    line-height: 1;
    transition:
      background-color 0.2s ease,
      color 0.2s ease;
  }

  .btn-delete-message:hover {
    background: rgba(239, 68, 68, 0.2);
    color: #ef4444;
  }

  .chat-input-actions {
    display: flex;
    gap: 0.375rem;
    margin-bottom: 0.75rem;
    flex-wrap: wrap;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid rgba(51, 65, 85, 0.5);
  }

  .btn-chat-action {
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    color: #0ea5e9;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.6875rem;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .btn-chat-action:hover:not(:disabled) {
    background: rgba(14, 165, 233, 0.2);
    border-color: #0ea5e9;
  }

  .btn-chat-action:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .message-user {
    align-self: flex-end;
  }

  .message-assistant {
    align-self: flex-start;
  }

  .message-content {
    padding: clamp(0.625rem, 1.25vh, 0.75rem) clamp(0.875rem, 1.75vw, 1rem);
    border-radius: 0.75rem;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    line-height: 1.5;
    word-wrap: break-word;
  }

  .message-user .message-content {
    background: #0ea5e9;
    color: white;
    border-bottom-right-radius: 0.25rem;
  }

  .message-assistant .message-content {
    background: #334155;
    color: #e2e8f0;
    border-bottom-left-radius: 0.25rem;
  }

  .attached-element {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 0.5rem;
    padding: clamp(0.5rem, 1vh, 0.75rem);
    margin-bottom: 0.75rem;
  }

  .attached-element-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
    font-weight: 600;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
  }

  .attached-icon {
    font-size: 1rem;
  }

  .attached-label {
    color: rgba(255, 255, 255, 0.9);
  }

  .attached-element-info {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
  }

  .attached-info-row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: center;
  }

  .attached-info-row strong {
    color: rgba(255, 255, 255, 0.9);
  }

  .attached-info-row code {
    background: rgba(0, 0, 0, 0.3);
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    font-family: 'Courier New', monospace;
    font-size: 0.875em;
    color: #10b981;
  }

  .tag-badge {
    background: rgba(14, 165, 233, 0.2);
    color: #0ea5e9;
    padding: 0.125rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.875em;
    font-weight: 600;
  }

  .text-preview {
    color: rgba(255, 255, 255, 0.8);
    font-style: italic;
  }

  /* Скрываем все блоки кода в сообщениях - код теперь только в нижней панели */
  .message-content pre,
  .message-content pre code,
  .message-content code:not(.attached-info-row code) {
    display: none !important;
  }

  /* JSON конфигурация уже заменяется на ссылку в обработке сообщений */

  /* Показываем только селекторы в прикрепленных элементах */
  .attached-info-row code {
    display: inline !important;
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
    color: #10b981;
  }

  .code-reference {
    padding: 0.5rem;
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    border-radius: 0.375rem;
    color: #0ea5e9;
    font-size: 0.875rem;
    margin: 0.5rem 0;
    font-style: italic;
  }

  /* Скрываем любые панели с кодом внутри чата - код теперь только в нижней панели */
  .code-panel-in-chat {
    display: none !important;
  }

  /* Скрываем все элементы с кодом внутри чата, кроме селекторов в прикрепленных элементах */
  .ai-chat .code-header,
  .ai-chat .code-content,
  .ai-chat .code-panel,
  .ai-chat pre:not(.attached-info-row pre),
  .ai-chat code:not(.attached-info-row code) {
    display: none !important;
  }

  /* Скрываем все заголовки h3 внутри чата (включая "Код") */
  .ai-chat h3:not(.chat-header h3) {
    display: none !important;
  }

  /* Скрываем все элементы с классом code-header или code-content */
  .ai-chat .code-header,
  .ai-chat .code-content,
  .ai-chat .code-panel {
    display: none !important;
  }

  /* Скрываем все элементы, которые содержат JSON конфигурацию */
  .ai-chat pre:not(.attached-info-row pre),
  .ai-chat code:not(.attached-info-row code),
  .ai-chat button[title*='Проверить AI'],
  .ai-chat button[title*='Редактировать'],
  .ai-chat button[title*='Копировать'] {
    display: none !important;
  }

  /* Показываем только селекторы в прикрепленных элементах */
  .attached-info-row code {
    display: inline !important;
  }

  /* JSON конфигурация уже заменяется на ссылку в обработке сообщений */

  .message-time {
    font-size: clamp(0.625rem, 0.8vw, 0.75rem);
    color: #64748b;
    margin-top: 0.25rem;
    padding: 0 0.5rem;
  }

  .message-user .message-time {
    text-align: right;
  }

  .typing-indicator {
    display: inline-block;
    animation: typing 1.4s infinite;
  }

  @keyframes typing {
    0%,
    60%,
    100% {
      opacity: 0.3;
    }
    30% {
      opacity: 1;
    }
  }

  .chat-input {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: clamp(0.75rem, 1.5vh, 1rem);
    border-top: 1px solid #334155;
    background: #0f172a;
  }

  .attached-element-preview {
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    border-radius: 0.5rem;
    padding: clamp(0.5rem, 1vh, 0.75rem);
  }

  .attached-preview-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
    color: #0ea5e9;
  }

  .btn-remove-attachment {
    background: transparent;
    border: none;
    color: #0ea5e9;
    cursor: pointer;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    font-size: 1rem;
    line-height: 1;
    margin-left: auto;
    transition: background-color 0.2s ease;
  }

  .btn-remove-attachment:hover {
    background: rgba(14, 165, 233, 0.2);
  }

  .attach-element-hint {
    display: flex;
    justify-content: flex-start;
  }

  .btn-attach {
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    color: #0ea5e9;
    cursor: pointer;
    padding: clamp(0.375rem, 0.75vh, 0.5rem) clamp(0.75rem, 1.5vw, 1rem);
    border-radius: 0.5rem;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
    transition: background-color 0.2s ease;
  }

  .btn-attach:hover {
    background: rgba(14, 165, 233, 0.2);
  }

  .chat-input-row {
    display: flex;
    gap: 0.375rem;
    align-items: flex-start;
  }

  .chat-input textarea {
    flex: 1;
    padding: clamp(0.625rem, 1.25vh, 0.75rem) clamp(0.875rem, 1.75vw, 1rem);
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    color: #e2e8f0;
    font-size: clamp(0.8125rem, 1vw, 0.9375rem);
    font-family: inherit;
    resize: vertical;
    min-height: 3rem;
    max-height: 8rem;
    line-height: 1.5;
  }

  .chat-input textarea:focus {
    outline: none;
    border-color: #0ea5e9;
    box-shadow: 0 0 0 3px rgba(14, 165, 233, 0.1);
  }

  .chat-input textarea:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-send {
    padding: clamp(0.5rem, 1vh, 0.625rem) clamp(0.875rem, 1.75vw, 1rem);
    background: #0ea5e9;
    border: none;
    border-radius: 0.375rem;
    color: white;
    cursor: pointer;
    font-size: 1rem;
    transition: background-color 0.2s ease;
    min-width: 2.5rem;
    align-self: flex-start;
  }

  .btn-send:hover:not(:disabled) {
    background: #0284c7;
  }

  .btn-send:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  @media (max-width: 768px) {
    .message {
      max-width: 95%;
    }

    .chat-header,
    .chat-input {
      padding: clamp(0.5rem, 1vh, 0.75rem);
    }
  }
</style>
