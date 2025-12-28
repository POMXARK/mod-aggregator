/**
 * Composable для управления хранением чатов в localStorage
 *
 * Управляет сохранением и загрузкой чатов из localStorage,
 * включая миграцию данных при изменении формата.
 */

import type { Chat, ChatMessage } from '../types';

const STORAGE_KEY = 'ai_chats';

/**
 * Загружает чаты из localStorage
 */
export function loadChats(): Chat[] {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      const parsed = JSON.parse(stored);
      return parsed.map((chat: unknown) => {
        const c = chat as { messages: unknown[]; createdAt: string; updatedAt: string };
        return {
          ...c,
          messages: c.messages.map((msg: unknown) => {
            const m = msg as Record<string, unknown>;
            return {
              ...m,
              timestamp: new Date(m.timestamp as string),
            } as ChatMessage;
          }),
          createdAt: new Date(c.createdAt),
          updatedAt: new Date(c.updatedAt),
        };
      });
    }
  } catch (e) {
    console.error('Failed to load chats:', e);
  }
  return [];
}

/**
 * Сохраняет чаты в localStorage
 */
export function saveChats(chats: Chat[]): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(chats));
  } catch (e) {
    console.error('Failed to save chats:', e);
  }
}

/**
 * Создает новый чат
 */
export function createNewChat(chats: Chat[]): Chat {
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
  return newChat;
}
