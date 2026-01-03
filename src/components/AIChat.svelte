<script lang="ts">
  import type { Node } from '@xyflow/svelte';
  import { invoke } from '../lib/tauri-wrapper';
  import type { Chat, ChatMessage, ParserConfig, AIModelConfig } from '../lib/types';
  import { ChatHeader, ChatInput, ChatList, ChatMessages } from './ai-chat';

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
    onChatChange?: (chatId: string | null) => void;
  }

  let {
    aiModelType,
    aiModelName,
    aiApiKey,
    aiOllamaUrl,
    currentUrl,
    nodes,
    onCreateNodes,
    selectedElementInfo,
    onGenerateCode,
    onTestParser,
    onCheckCodeWithAI,
    generatedCode,
    onApplyCode,
    addAIMessage = $bindable(),
    onChatChange,
  }: Props = $props();

  // РЈРїСЂР°РІР»РµРЅРёРµ С‡Р°С‚Р°РјРё
  const STORAGE_KEY = 'ai_chats';
  let chats = $state<Chat[]>([]);
  let currentChatId = $state<string | null>(null);
  let showChatList = $state(false);

  // РўРµРєСѓС‰РёР№ С‡Р°С‚ - РёСЃРїРѕР»СЊР·СѓРµРј $state РґР»СЏ РёР·Р±РµР¶Р°РЅРёСЏ С†РёРєР»РёС‡РµСЃРєРёС… РѕР±РЅРѕРІР»РµРЅРёР№
  let messages = $state<ChatMessage[]>([]);

  // Р¤СѓРЅРєС†РёСЏ РґР»СЏ РѕР±РЅРѕРІР»РµРЅРёСЏ СЃРѕРѕР±С‰РµРЅРёР№ РёР· С‚РµРєСѓС‰РµРіРѕ С‡Р°С‚Р°
  function updateMessagesFromChat() {
    if (!currentChatId) {
      messages = [];
      return;
    }
    const chat = chats.find(c => c.id === currentChatId);
    messages = chat?.messages || [];
  }

  // РћС‚СЃР»РµР¶РёРІР°РµРј РёР·РјРµРЅРµРЅРёСЏ currentChatId Рё СѓРІРµРґРѕРјР»СЏРµРј СЂРѕРґРёС‚РµР»СЏ
  $effect(() => {
    if (onChatChange) {
      onChatChange(currentChatId);
    }
  });

  let inputMessage = $state('');
  let isSending = $state(false);
  let chatContainer: HTMLDivElement;
  let attachedElement = $state<ChatMessage['attachedElement'] | null>(null);

  // РЎРёСЃС‚РµРјРЅС‹Р№ РїСЂРѕРјРїС‚ РґР»СЏ AI
  const systemPrompt = `РўС‹ СЌРєСЃРїРµСЂС‚ РїРѕ РїР°СЂСЃРёРЅРіСѓ РІРµР±-СЃС‚СЂР°РЅРёС†. РўРІРѕСЏ Р·Р°РґР°С‡Р° - Р°РЅР°Р»РёР·РёСЂРѕРІР°С‚СЊ HTML СЌР»РµРјРµРЅС‚С‹ Рё СЃРѕР·РґР°РІР°С‚СЊ РєРѕРЅС„РёРіСѓСЂР°С†РёРё РїР°СЂСЃРµСЂРѕРІ.

Р’РђР–РќРћ: Р’РЎР•Р“Р”Рђ РѕС‚РІРµС‡Р°Р№ РўРћР›Р¬РљРћ РІ С„РѕСЂРјР°С‚Рµ JSON. РќРёРєР°РєРѕРіРѕ РґРѕРїРѕР»РЅРёС‚РµР»СЊРЅРѕРіРѕ С‚РµРєСЃС‚Р° РґРѕ РёР»Рё РїРѕСЃР»Рµ JSON.

Р¤РѕСЂРјР°С‚ РѕС‚РІРµС‚Р° (СЃС‚СЂРѕРіРѕ СЃРѕР±Р»СЋРґР°Р№):
{
  "list_selector": "CSS СЃРµР»РµРєС‚РѕСЂ РґР»СЏ СЃРїРёСЃРєР° РІСЃРµС… РїРѕС…РѕР¶РёС… СЌР»РµРјРµРЅС‚РѕРІ (РЅР°РїСЂРёРјРµСЂ: div.filekmod)",
  "title_selector": "CSS СЃРµР»РµРєС‚РѕСЂ РґР»СЏ Р·Р°РіРѕР»РѕРІРєР° РІРЅСѓС‚СЂРё РєР°Р¶РґРѕРіРѕ СЌР»РµРјРµРЅС‚Р° СЃРїРёСЃРєР° (РѕС‚РЅРѕСЃРёС‚РµР»СЊРЅС‹Р№, РЅР°РїСЂРёРјРµСЂ: h3, .title, a)",
  "url_selector": "CSS СЃРµР»РµРєС‚РѕСЂ РґР»СЏ СЃСЃС‹Р»РєРё (РѕС‚РЅРѕСЃРёС‚РµР»СЊРЅС‹Р№, РЅР°РїСЂРёРјРµСЂ: a[href], .link)",
  "description_selector": "CSS СЃРµР»РµРєС‚РѕСЂ РґР»СЏ РѕРїРёСЃР°РЅРёСЏ (РѕС‚РЅРѕСЃРёС‚РµР»СЊРЅС‹Р№, РЅР°РїСЂРёРјРµСЂ: .description, p, span)",
  "image_selector": "CSS СЃРµР»РµРєС‚РѕСЂ РґР»СЏ РёР·РѕР±СЂР°Р¶РµРЅРёСЏ (РѕС‚РЅРѕСЃРёС‚РµР»СЊРЅС‹Р№, РЅР°РїСЂРёРјРµСЂ: img, .thumb img)"
}

РџСЂР°РІРёР»Р°:
1. list_selector РґРѕР»Р¶РµРЅ РЅР°С…РѕРґРёС‚СЊ Р’РЎР• РїРѕС…РѕР¶РёРµ СЌР»РµРјРµРЅС‚С‹ РЅР° СЃС‚СЂР°РЅРёС†Рµ
2. РћСЃС‚Р°Р»СЊРЅС‹Рµ СЃРµР»РµРєС‚РѕСЂС‹ РґРѕР»Р¶РЅС‹ Р±С‹С‚СЊ РћРўРќРћРЎРРўР•Р›Р¬РќР«РњР (Р±РµР· СѓРєР°Р·Р°РЅРёСЏ list_selector РІ РЅР°С‡Р°Р»Рµ)
3. Р•СЃР»Рё РїРѕР»СЊР·РѕРІР°С‚РµР»СЊ РїСЂРёРєСЂРµРїРёР» СЌР»РµРјРµРЅС‚, РёСЃРїРѕР»СЊР·СѓР№ РµРіРѕ СЃРµР»РµРєС‚РѕСЂ РєР°Рє РѕСЃРЅРѕРІСѓ РґР»СЏ list_selector
4. Р•СЃР»Рё СЃРµР»РµРєС‚РѕСЂ РЅРµ РѕРїСЂРµРґРµР»РµРЅ, РёСЃРїРѕР»СЊР·СѓР№ РїСѓСЃС‚СѓСЋ СЃС‚СЂРѕРєСѓ ""
5. РћС‚РІРµС‡Р°Р№ РўРћР›Р¬РљРћ JSON, Р±РµР· РѕР±СЉСЏСЃРЅРµРЅРёР№ Рё РґРѕРїРѕР»РЅРёС‚РµР»СЊРЅРѕРіРѕ С‚РµРєСЃС‚Р°`;

  // Р—Р°РіСЂСѓР·РєР° С‡Р°С‚РѕРІ РёР· localStorage
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

        // Р•СЃР»Рё РµСЃС‚СЊ С‡Р°С‚С‹, РІС‹Р±РёСЂР°РµРј РїРѕСЃР»РµРґРЅРёР№
        if (chats.length > 0) {
          currentChatId = chats[chats.length - 1].id;
        } else {
          // Р•СЃР»Рё РЅРµС‚ С‡Р°С‚РѕРІ, СЃРѕР·РґР°РµРј РїРµСЂРІС‹Р№
          createNewChat();
        }
      } else {
        // Р•СЃР»Рё РЅРµС‚ СЃРѕС…СЂР°РЅРµРЅРЅС‹С… С‡Р°С‚РѕРІ, СЃРѕР·РґР°РµРј РїРµСЂРІС‹Р№
        createNewChat();
      }
    } catch (e) {
      console.error('Failed to load chats:', e);
      createNewChat();
    }
  }

  // РЎРѕС…СЂР°РЅРµРЅРёРµ С‡Р°С‚РѕРІ РІ localStorage
  function saveChats() {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(chats));
    } catch (e) {
      console.error('Failed to save chats:', e);
    }
  }

  // РЎРѕР·РґР°РЅРёРµ РЅРѕРІРѕРіРѕ С‡Р°С‚Р°
  function createNewChat() {
    const newChat: Chat = {
      id: `chat-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
      title: `Р§Р°С‚ ${chats.length + 1}`,
      messages: [
        {
          id: `msg-${Date.now()}`,
          role: 'assistant',
          content:
            'РџСЂРёРІРµС‚! РЇ РїРѕРјРѕРіСѓ С‚РµР±Рµ СЃРѕР·РґР°С‚СЊ РїР°СЂСЃРµСЂ РґР»СЏ РёР·РІР»РµС‡РµРЅРёСЏ РґР°РЅРЅС‹С… СЃ СЃР°Р№С‚РѕРІ. РћРїРёС€Рё, РєР°РєРёРµ РґР°РЅРЅС‹Рµ РЅСѓР¶РЅРѕ РёР·РІР»РµС‡СЊ, Рё СЏ СЃРѕР·РґР°Рј РєРѕРЅС„РёРіСѓСЂР°С†РёСЋ РїР°СЂСЃРµСЂР°.',
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

  // РЈРґР°Р»РµРЅРёРµ С‡Р°С‚Р°
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

  // РћР±РЅРѕРІР»РµРЅРёРµ С‡Р°С‚Р°
  function updateChat(chatId: string, updater: (chat: Chat) => Chat) {
    chats = chats.map(chat => {
      if (chat.id === chatId) {
        const updated = updater(chat);
        updated.updatedAt = new Date();
        return updated;
      }
      return chat;
    });

    // РћР±РЅРѕРІР»СЏРµРј СЃРѕРѕР±С‰РµРЅРёСЏ, РµСЃР»Рё РѕР±РЅРѕРІР»РµРЅ С‚РµРєСѓС‰РёР№ С‡Р°С‚
    if (chatId === currentChatId) {
      updateMessagesFromChat();
    }

    saveChats();
  }

  // Р¤СѓРЅРєС†РёСЏ РґР»СЏ РґРѕР±Р°РІР»РµРЅРёСЏ СЃРѕРѕР±С‰РµРЅРёСЏ РѕС‚ AI (РґР»СЏ РёСЃРїРѕР»СЊР·РѕРІР°РЅРёСЏ РёР·РІРЅРµ)
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

  // Р­РєСЃРїРѕСЂС‚РёСЂСѓРµРј С„СѓРЅРєС†РёСЋ С‡РµСЂРµР· bindable
  $effect(() => {
    addAIMessage = addAIMessageToChat;
  });

  // РџСЂРёРєСЂРµРїР»РµРЅРёРµ СЌР»РµРјРµРЅС‚Р°
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

  // РЈРґР°Р»РµРЅРёРµ РїСЂРёРєСЂРµРїР»РµРЅРЅРѕРіРѕ СЌР»РµРјРµРЅС‚Р°
  function removeAttachedElement() {
    attachedElement = null;
  }

  // РЈРґР°Р»РµРЅРёРµ СЃРѕРѕР±С‰РµРЅРёСЏ
  function deleteMessage(messageId: string) {
    if (!currentChatId) {
      return;
    }

    updateChat(currentChatId, chat => ({
      ...chat,
      messages: chat.messages.filter(m => m.id !== messageId),
    }));
  }

  // РР·РІР»РµС‡РµРЅРёРµ РєРѕРґР° РёР· СЃРѕРѕР±С‰РµРЅРёСЏ AI
  function extractCodeFromMessage(content: string): string | null {
    // РС‰РµРј РєРѕРґ РІ markdown Р±Р»РѕРєР°С…
    const codeBlockMatch = content.match(/```(?:rust|rs)?\s*([\s\S]*?)```/);
    if (codeBlockMatch) {
      return codeBlockMatch[1].trim();
    }

    // РС‰РµРј РєРѕРґ РјРµР¶РґСѓ С‚РµРіР°РјРё <code>
    const htmlCodeMatch = content.match(/<code>([\s\S]*?)<\/code>/);
    if (htmlCodeMatch) {
      return htmlCodeMatch[1].trim();
    }

    return null;
  }

  // РР·РІР»РµС‡РµРЅРёРµ JSON РєРѕРЅС„РёРіСѓСЂР°С†РёРё РёР· СЃРѕРѕР±С‰РµРЅРёСЏ AI
  function extractJSONConfigFromMessage(content: string): ParserConfig | null {
    try {
      // РЎС‚СЂР°С‚РµРіРёСЏ 1: РџРѕРёСЃРє JSON Р±Р»РѕРєР° РјРµР¶РґСѓ ```json Рё ```
      const jsonBlockMatch = content.match(/```json\s*([\s\S]*?)\s*```/);
      if (jsonBlockMatch) {
        try {
          const parsed = JSON.parse(jsonBlockMatch[1].trim());
          if (parsed && typeof parsed === 'object' && parsed.list_selector) {
            return parsed;
          }
        } catch {
          // РќРµРІР°Р»РёРґРЅС‹Р№ JSON - РїСЂРѕСЃС‚Рѕ РёРіРЅРѕСЂРёСЂСѓРµРј, РЅРµ Р»РѕРіРёСЂСѓРµРј
        }
      }

      // РЎС‚СЂР°С‚РµРіРёСЏ 2: РџРѕРёСЃРє РїРµСЂРІРѕРіРѕ JSON РѕР±СЉРµРєС‚Р° СЃ list_selector (Р±РѕР»РµРµ С‚РѕС‡РЅС‹Р№ РїР°С‚С‚РµСЂРЅ)
      const jsonMatch = content.match(/\{[^{}]*"list_selector"[^{}]*\}/);
      if (jsonMatch) {
        try {
          const parsed = JSON.parse(jsonMatch[0]);
          if (parsed && typeof parsed === 'object' && parsed.list_selector) {
            return parsed;
          }
        } catch {
          // РќРµРІР°Р»РёРґРЅС‹Р№ JSON - РїСЂРѕСЃС‚Рѕ РёРіРЅРѕСЂРёСЂСѓРµРј, РЅРµ Р»РѕРіРёСЂСѓРµРј
        }
      }

      // РЎС‚СЂР°С‚РµРіРёСЏ 3: РџРѕРїС‹С‚РєР° РЅР°Р№С‚Рё JSON РѕР±СЉРµРєС‚ СЃ Р±РѕР»РµРµ СѓРјРЅС‹Рј РїР°СЂСЃРёРЅРіРѕРј
      // РС‰РµРј РѕР±СЉРµРєС‚, РєРѕС‚РѕСЂС‹Р№ РЅР°С‡РёРЅР°РµС‚СЃСЏ СЃ { Рё СЃРѕРґРµСЂР¶РёС‚ list_selector
      const jsonPattern = /\{[\s\S]{0,2000}?"list_selector"[\s\S]{0,2000}?\}/;
      const anyJsonMatch = content.match(jsonPattern);
      if (anyJsonMatch) {
        try {
          // РџС‹С‚Р°РµРјСЃСЏ РЅР°Р№С‚Рё Р·Р°РєСЂС‹РІР°СЋС‰СѓСЋ СЃРєРѕР±РєСѓ
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
          // РќРµРІР°Р»РёРґРЅС‹Р№ JSON - РїСЂРѕСЃС‚Рѕ РёРіРЅРѕСЂРёСЂСѓРµРј, РЅРµ Р»РѕРіРёСЂСѓРµРј
        }
      }
    } catch {
      // РћР±С‰Р°СЏ РѕС€РёР±РєР° - РЅРµ Р»РѕРіРёСЂСѓРµРј, РїСЂРѕСЃС‚Рѕ РІРѕР·РІСЂР°С‰Р°РµРј null
    }

    return null;
  }

  // Р’РѕСЃСЃС‚Р°РЅРѕРІР»РµРЅРёРµ РєРѕРЅС„РёРіСѓСЂР°С†РёРё РїР°СЂСЃРµСЂР° РёР· СЃРѕРѕР±С‰РµРЅРёСЏ
  function restoreConfigFromMessage(messageId: string) {
    if (!currentChatId) {
      return;
    }

    const chat = chats.find(c => c.id === currentChatId);
    const message = chat?.messages.find(m => m.id === messageId);
    if (!message || message.role !== 'assistant') {
      return;
    }

    // РџС‹С‚Р°РµРјСЃСЏ РёР·РІР»РµС‡СЊ JSON РєРѕРЅС„РёРіСѓСЂР°С†РёСЋ
    const config = extractJSONConfigFromMessage(message.content);

    if (config && config.list_selector) {
      // Р•СЃР»Рё РµСЃС‚СЊ JSON РєРѕРЅС„РёРіСѓСЂР°С†РёСЏ, СЃРѕР·РґР°РµРј СѓР·Р»С‹ РёР· РЅРµС‘ (РєР°Рє РїСЂРё "РїРѕРІС‚РѕСЂРё")
      try {
        if (onCreateNodes) {
          onCreateNodes(config);
          // onCreateNodes Р°РІС‚РѕРјР°С‚РёС‡РµСЃРєРё РІС‹Р·РѕРІРµС‚ generateParserCode, РЅРѕ РґР»СЏ РЅР°РґРµР¶РЅРѕСЃС‚Рё РІС‹Р·С‹РІР°РµРј onGenerateCode С‚РѕР¶Рµ
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

        // Р”РѕР±Р°РІР»СЏРµРј СЃРѕРѕР±С‰РµРЅРёРµ РѕР± СѓСЃРїРµС…Рµ
        if (addAIMessage) {
          addAIMessage('вњ… РљРѕРЅС„РёРіСѓСЂР°С†РёСЏ РїР°СЂСЃРµСЂР° РІРѕСЃСЃС‚Р°РЅРѕРІР»РµРЅР° РёР· СЃРѕРѕР±С‰РµРЅРёСЏ (СѓР·Р»С‹ Рё РєРѕРґ СЃРѕР·РґР°РЅС‹)');
        }
      } catch (error: unknown) {
        console.error('Failed to restore config from message:', error);
        if (addAIMessage) {
          const errorMessage = error instanceof Error ? error.message : String(error);
          addAIMessage(`вќЊ РћС€РёР±РєР° РІРѕСЃСЃС‚Р°РЅРѕРІР»РµРЅРёСЏ РєРѕРЅС„РёРіСѓСЂР°С†РёРё: ${errorMessage}`);
        }
      }
    } else {
      // Р•СЃР»Рё РЅРµС‚ JSON РєРѕРЅС„РёРіСѓСЂР°С†РёРё, СЃРѕРѕР±С‰Р°РµРј РѕР± СЌС‚РѕРј
      if (addAIMessage) {
        addAIMessage('вќЊ Р’ СЌС‚РѕРј СЃРѕРѕР±С‰РµРЅРёРё РЅРµ РЅР°Р№РґРµРЅР° JSON РєРѕРЅС„РёРіСѓСЂР°С†РёСЏ РїР°СЂСЃРµСЂР°');
      }
    }
  }

  // Р“РµРЅРµСЂР°С†РёСЏ РєРѕРґР° РёР· СЃС‚Р°СЂРѕРіРѕ СЃРѕРѕР±С‰РµРЅРёСЏ (СЂР°Р±РѕС‚Р°РµС‚ РєР°Рє "РїРѕРІС‚РѕСЂРё")
  function generateCodeFromMessage(messageId: string) {
    if (!currentChatId) {
      return;
    }

    const chat = chats.find(c => c.id === currentChatId);
    const message = chat?.messages.find(m => m.id === messageId);
    if (!message || message.role !== 'assistant') {
      return;
    }

    // РџС‹С‚Р°РµРјСЃСЏ РёР·РІР»РµС‡СЊ JSON РєРѕРЅС„РёРіСѓСЂР°С†РёСЋ
    const config = extractJSONConfigFromMessage(message.content);

    if (config && config.list_selector) {
      // Р•СЃР»Рё РµСЃС‚СЊ JSON РєРѕРЅС„РёРіСѓСЂР°С†РёСЏ, СЃРѕР·РґР°РµРј СѓР·Р»С‹ РёР· РЅРµС‘ (РєР°Рє РїСЂРё "РїРѕРІС‚РѕСЂРё")
      try {
        if (onCreateNodes) {
          onCreateNodes(config);
          // onCreateNodes Р°РІС‚РѕРјР°С‚РёС‡РµСЃРєРё РІС‹Р·РѕРІРµС‚ generateParserCode, РЅРѕ РґР»СЏ РЅР°РґРµР¶РЅРѕСЃС‚Рё РІС‹Р·С‹РІР°РµРј onGenerateCode С‚РѕР¶Рµ
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

        // Р”РѕР±Р°РІР»СЏРµРј СЃРѕРѕР±С‰РµРЅРёРµ РѕР± СѓСЃРїРµС…Рµ
        if (addAIMessage) {
          addAIMessage('вњ… РЈР·Р»С‹ Рё РєРѕРґ РІРѕСЃСЃС‚Р°РЅРѕРІР»РµРЅС‹ РёР· СЃРѕРѕР±С‰РµРЅРёСЏ');
        }
      } catch (error: unknown) {
        console.error('Failed to create nodes from message:', error);
        // Р•СЃР»Рё РЅРµ СѓРґР°Р»РѕСЃСЊ СЃРѕР·РґР°С‚СЊ СѓР·Р»С‹, РїСЂРѕСЃС‚Рѕ РіРµРЅРµСЂРёСЂСѓРµРј РєРѕРґ РёР· СЃСѓС‰РµСЃС‚РІСѓСЋС‰РёС… СѓР·Р»РѕРІ
        if (onGenerateCode) {
          onGenerateCode();
        }
        if (addAIMessage) {
          const errorMessage = error instanceof Error ? error.message : String(error);
          addAIMessage(`вќЊ РћС€РёР±РєР° РІРѕСЃСЃС‚Р°РЅРѕРІР»РµРЅРёСЏ: ${errorMessage}`);
        }
      }
    } else {
      // Р•СЃР»Рё РЅРµС‚ JSON РєРѕРЅС„РёРіСѓСЂР°С†РёРё, РїСЂРѕСЃС‚Рѕ РіРµРЅРµСЂРёСЂСѓРµРј РєРѕРґ РёР· СЃСѓС‰РµСЃС‚РІСѓСЋС‰РёС… СѓР·Р»РѕРІ
      if (onGenerateCode) {
        onGenerateCode();
        if (addAIMessage) {
          addAIMessage('вљ пёЏ JSON РєРѕРЅС„РёРіСѓСЂР°С†РёСЏ РЅРµ РЅР°Р№РґРµРЅР°. РљРѕРґ СЃРіРµРЅРµСЂРёСЂРѕРІР°РЅ РёР· СЃСѓС‰РµСЃС‚РІСѓСЋС‰РёС… СѓР·Р»РѕРІ.');
        }
      }
    }
  }

  // РљРѕРїРёСЂРѕРІР°РЅРёРµ СЃРѕРґРµСЂР¶РёРјРѕРіРѕ СЃРѕРѕР±С‰РµРЅРёСЏ
  function copyMessageContent(messageId: string) {
    if (!currentChatId) {
      return;
    }

    const chat = chats.find(c => c.id === currentChatId);
    const message = chat?.messages.find(m => m.id === messageId);
    if (!message) {
      return;
    }

    // РљРѕРїРёСЂСѓРµРј С‚РµРєСЃС‚ СЃРѕРѕР±С‰РµРЅРёСЏ (Р±РµР· HTML С‚РµРіРѕРІ)
    const textContent = message.content
      .replace(/<[^>]*>/g, '') // РЈРґР°Р»СЏРµРј HTML С‚РµРіРё
      .replace(/```[\s\S]*?```/g, match => match) // РћСЃС‚Р°РІР»СЏРµРј РєРѕРґ Р±Р»РѕРєРё
      .trim();

    navigator.clipboard
      .writeText(textContent)
      .then(() => {
        // РњРѕР¶РЅРѕ РїРѕРєР°Р·Р°С‚СЊ СѓРІРµРґРѕРјР»РµРЅРёРµ РѕР± СѓСЃРїРµС€РЅРѕРј РєРѕРїРёСЂРѕРІР°РЅРёРё
        console.log('Message copied to clipboard');
      })
      .catch(err => {
        console.error('Failed to copy message:', err);
      });
  }

  // РљРѕРїРёСЂРѕРІР°РЅРёРµ С‚РѕР»СЊРєРѕ С‚РµРєСЃС‚Р° СЃРѕРѕР±С‰РµРЅРёСЏ (Р±РµР· РєРѕРґР°)
  function copyMessageText(messageId: string) {
    if (!currentChatId) {
      return;
    }

    const chat = chats.find(c => c.id === currentChatId);
    const message = chat?.messages.find(m => m.id === messageId);
    if (!message) {
      return;
    }

    // РљРѕРїРёСЂСѓРµРј С‚РѕР»СЊРєРѕ С‡РёСЃС‚С‹Р№ С‚РµРєСЃС‚ (Р±РµР· HTML С‚РµРіРѕРІ Рё Р±РµР· РєРѕРґР°)
    const textContent = message.content
      .replace(/<[^>]*>/g, '') // РЈРґР°Р»СЏРµРј HTML С‚РµРіРё
      .replace(/```[\s\S]*?```/g, '') // РЈРґР°Р»СЏРµРј РєРѕРґ Р±Р»РѕРєРё
      .replace(/^\s*[\r\n]+/gm, '') // РЈРґР°Р»СЏРµРј РїСѓСЃС‚С‹Рµ СЃС‚СЂРѕРєРё
      .trim();

    navigator.clipboard
      .writeText(textContent)
      .then(() => {
        console.log('Message text copied to clipboard');
      })
      .catch(err => {
        console.error('Failed to copy message text:', err);
      });
  }

  // РџСЂРёРјРµРЅРµРЅРёРµ РёСЃРїСЂР°РІР»РµРЅРёСЏ РєРѕРґР° РѕС‚ AI
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

  // РџСЂРѕРІРµСЂРєР° РіРµРЅРµСЂР°С†РёРё РєРѕРґР°
  async function checkGeneration() {
    if (!generatedCode) {
      const errorMsg: ChatMessage = {
        id: `msg-${Date.now()}`,
        role: 'assistant',
        content: 'вќЊ РљРѕРґ РµС‰Рµ РЅРµ СЃРіРµРЅРµСЂРёСЂРѕРІР°РЅ. РЎРЅР°С‡Р°Р»Р° СЃРіРµРЅРµСЂРёСЂСѓР№С‚Рµ РєРѕРґ РїР°СЂСЃРµСЂР°.',
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

  // Р—Р°РїСѓСЃРє РїР°СЂСЃРµСЂР° РёР· С‡Р°С‚Р° - РїСЂРѕСЃС‚Рѕ РѕС‚РєСЂС‹РІР°РµС‚ РІРєР»Р°РґРєСѓ runner, РіРґРµ РїРѕР»СЊР·РѕРІР°С‚РµР»СЊ РјРѕР¶РµС‚ Р·Р°РїСѓСЃС‚РёС‚СЊ РїР°СЂСЃРµСЂ
  function runParserFromChat() {
    if (nodes.length === 0) {
      if (addAIMessage) {
        addAIMessage(
          'вќЊ РќРµС‚ РЅРѕРґ РїР°СЂСЃРµСЂР°. РЎРЅР°С‡Р°Р»Р° СЃРѕР·РґР°Р№С‚Рµ РїР°СЂСЃРµСЂ С‡РµСЂРµР· AI РёР»Рё РІРѕСЃСЃС‚Р°РЅРѕРІРёС‚Рµ РёР· СЃРѕРѕР±С‰РµРЅРёСЏ.'
        );
      }
      return;
    }

    if (!currentUrl) {
      if (addAIMessage) {
        addAIMessage('вќЊ РќРµС‚ Р·Р°РіСЂСѓР¶РµРЅРЅРѕР№ СЃС‚СЂР°РЅРёС†С‹. РЎРЅР°С‡Р°Р»Р° Р·Р°РіСЂСѓР·РёС‚Рµ СЃС‚СЂР°РЅРёС†Сѓ РґР»СЏ С‚РµСЃС‚РёСЂРѕРІР°РЅРёСЏ.');
      }
      return;
    }

    // РћС‚РєСЂС‹РІР°РµРј РІРєР»Р°РґРєСѓ runner С‡РµСЂРµР· callback, РµСЃР»Рё РѕРЅ РµСЃС‚СЊ
    // Р’ ParserBuilder СЌС‚Рѕ РѕР±СЂР°Р±РѕС‚Р°РµС‚СЃСЏ Рё РѕС‚РєСЂРѕРµС‚СЃСЏ РІРєР»Р°РґРєР° runner
    if (onTestParser) {
      // onTestParser РјРѕР¶РµС‚ Р±С‹С‚СЊ С„СѓРЅРєС†РёРµР№, РєРѕС‚РѕСЂР°СЏ РѕС‚РєСЂС‹РІР°РµС‚ runner РІРєР»Р°РґРєСѓ
      // Р’С‹Р·С‹РІР°РµРј РµС‘, РЅРѕ РЅРµ Р¶РґРµРј СЂРµР·СѓР»СЊС‚Р°С‚Р°, С‚Р°Рє РєР°Рє Р·Р°РїСѓСЃРє РїР°СЂСЃРµСЂР° РґРѕР»Р¶РµРЅ РїСЂРѕРёСЃС…РѕРґРёС‚СЊ РІ ParserRunner
      onTestParser().catch((error: unknown) => {
        console.error('Error in onTestParser:', error);
        if (addAIMessage) {
          const errorMessage = error instanceof Error ? error.message : String(error);
          addAIMessage(`вќЊ РћС€РёР±РєР°: ${errorMessage}`);
        }
      });
    } else {
      if (addAIMessage) {
        addAIMessage('в„№пёЏ РџРµСЂРµР№РґРёС‚Рµ РІРѕ РІРєР»Р°РґРєСѓ "Р—Р°РїСѓСЃРє РїР°СЂСЃРµСЂР°" РІ РЅРёР¶РЅРµР№ РїР°РЅРµР»Рё РґР»СЏ Р·Р°РїСѓСЃРєР°.');
      }
    }
  }

  // РРЅРёС†РёР°Р»РёР·Р°С†РёСЏ - С‚РѕР»СЊРєРѕ РѕРґРёРЅ СЂР°Р· РїСЂРё РјРѕРЅС‚РёСЂРѕРІР°РЅРёРё
  let initialized = $state(false);

  $effect(() => {
    if (!initialized) {
      loadChats();
      initialized = true;
    }
  });

  // РћР±РЅРѕРІР»РµРЅРёРµ СЃРѕРѕР±С‰РµРЅРёР№ РїСЂРё РёР·РјРµРЅРµРЅРёРё С‚РµРєСѓС‰РµРіРѕ С‡Р°С‚Р°
  $effect(() => {
    updateMessagesFromChat();
  });

  // РџСЂРѕРєСЂСѓС‚РєР° РІРЅРёР· РїСЂРё РЅРѕРІС‹С… СЃРѕРѕР±С‰РµРЅРёСЏС…
  $effect(() => {
    if (chatContainer && messages.length > 0) {
      const timeoutId = setTimeout(() => {
        chatContainer.scrollTop = chatContainer.scrollHeight;
      }, 100);
      return () => clearTimeout(timeoutId);
    }
  });

  // РђРІС‚РѕРјР°С‚РёС‡РµСЃРєРѕРµ РїСЂРёРєСЂРµРїР»РµРЅРёРµ СЌР»РµРјРµРЅС‚Р° РїСЂРё РµРіРѕ РІС‹Р±РѕСЂРµ (С‚РѕР»СЊРєРѕ РµСЃР»Рё РЅРµ РѕС‚РєСЂС‹С‚ СЃРїРёСЃРѕРє С‡Р°С‚РѕРІ)
  $effect(() => {
    if (selectedElementInfo && !showChatList && !attachedElement) {
      // РђРІС‚РѕРјР°С‚РёС‡РµСЃРєРё РїСЂРёРєСЂРµРїР»СЏРµРј СЌР»РµРјРµРЅС‚, РµСЃР»Рё РѕРЅ РІС‹Р±СЂР°РЅ
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
        (attachedElement ? `РџСЂРёРєСЂРµРїР»РµРЅ СЌР»РµРјРµРЅС‚: ${attachedElement.tagName}` : ''),
      timestamp: new Date(),
      attachedElement: attachedElement || undefined,
    };

    // Р”РѕР±Р°РІР»СЏРµРј СЃРѕРѕР±С‰РµРЅРёРµ РІ С‡Р°С‚
    updateChat(currentChatId, chat => ({
      ...chat,
      messages: [...chat.messages, userMessage],
      title:
        chat.messages.length === 1 ? inputMessage.trim().slice(0, 30) || 'РќРѕРІС‹Р№ С‡Р°С‚' : chat.title,
    }));

    const messageText = inputMessage.trim();
    const elementInfo = attachedElement;
    inputMessage = '';
    attachedElement = null;
    isSending = true;

    try {
      // Р¤РѕСЂРјРёСЂСѓРµРј РёСЃС‚РѕСЂРёСЋ СЃРѕРѕР±С‰РµРЅРёР№ РґР»СЏ AI
      const currentChat = chats.find(c => c.id === currentChatId);
      const messageHistory: [string, string][] = [
        ['system', systemPrompt],
        ...(currentChat?.messages.slice(0, -1) || []).map(m => {
          let content = m.content;
          if (m.attachedElement) {
            content += `\n\n[РџСЂРёРєСЂРµРїР»РµРЅ HTML СЌР»РµРјРµРЅС‚]\nРЎРµР»РµРєС‚РѕСЂ: ${m.attachedElement.selector}\nРўРµРі: ${m.attachedElement.tagName}\nРўРµРєСЃС‚: ${m.attachedElement.text}\nРђС‚СЂРёР±СѓС‚С‹: ${JSON.stringify(m.attachedElement.attributes)}`;
          }
          return [m.role, content] as [string, string];
        }),
        [
          'user',
          messageText +
            (elementInfo
              ? `\n\n[РџСЂРёРєСЂРµРїР»РµРЅ HTML СЌР»РµРјРµРЅС‚]\nРЎРµР»РµРєС‚РѕСЂ: ${elementInfo.selector}\nРўРµРі: ${elementInfo.tagName}\nРўРµРєСЃС‚: ${elementInfo.text}\nРђС‚СЂРёР±СѓС‚С‹: ${JSON.stringify(elementInfo.attributes)}`
              : ''),
        ],
      ];

      // РћС‚РїСЂР°РІР»СЏРµРј Р·Р°РїСЂРѕСЃ Рє AI
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

      // РџС‹С‚Р°РµРјСЃСЏ РёР·РІР»РµС‡СЊ JSON РёР· РѕС‚РІРµС‚Р° (РЅРµСЃРєРѕР»СЊРєРѕ СЃС‚СЂР°С‚РµРіРёР№)
      let config: ParserConfig | null = null;

      // РЎС‚СЂР°С‚РµРіРёСЏ 1: РџСЂСЏРјРѕР№ РїР°СЂСЃРёРЅРі, РµСЃР»Рё РѕС‚РІРµС‚ - С‡РёСЃС‚С‹Р№ JSON
      try {
        config = JSON.parse(response.trim());
      } catch {
        // РЎС‚СЂР°С‚РµРіРёСЏ 2: РџРѕРёСЃРє JSON Р±Р»РѕРєР° РјРµР¶РґСѓ ```json Рё ```
        const jsonBlockMatch = response.match(/```json\s*([\s\S]*?)\s*```/);
        if (jsonBlockMatch) {
          try {
            config = JSON.parse(jsonBlockMatch[1].trim());
          } catch {
            // РЎС‚СЂР°С‚РµРіРёСЏ 3: РџРѕРёСЃРє РїРµСЂРІРѕРіРѕ JSON РѕР±СЉРµРєС‚Р°
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
          // РЎС‚СЂР°С‚РµРіРёСЏ 4: РџРѕРёСЃРє JSON РѕР±СЉРµРєС‚Р° Р±РµР· Р±Р»РѕРєРѕРІ РєРѕРґР°
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
        // РЈСЃРїРµС€РЅРѕ СЂР°СЃРїР°СЂСЃРёР»Рё JSON СЃ list_selector
        try {
          // РЎРѕР·РґР°РµРј СѓР·Р»С‹ РёР· РєРѕРЅС„РёРіСѓСЂР°С†РёРё
          onCreateNodes(config);

          const assistantMessage: ChatMessage = {
            id: `msg-${Date.now()}`,
            role: 'assistant',
            content: `вњ… РџР°СЂСЃРµСЂ СЃРѕР·РґР°РЅ! РЇ СЃРѕР·РґР°Р» РєРѕРЅС„РёРіСѓСЂР°С†РёСЋ СЃ СЃРµР»РµРєС‚РѕСЂР°РјРё.\n\nРќРѕРґС‹ СЃРѕР·РґР°РЅС‹ РЅР° РіСЂР°С„Рµ. РўРµРїРµСЂСЊ РјРѕР¶РЅРѕ РїСЂРѕС‚РµСЃС‚РёСЂРѕРІР°С‚СЊ РїР°СЂСЃРµСЂ.`,
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
            content: `вњ… РљРѕРЅС„РёРіСѓСЂР°С†РёСЏ РїРѕР»СѓС‡РµРЅР°.\n\nвљ пёЏ РћС€РёР±РєР° РїСЂРё СЃРѕР·РґР°РЅРёРё РЅРѕРґ: ${errorMessage}`,
            timestamp: new Date(),
          };

          updateChat(currentChatId, chat => ({
            ...chat,
            messages: [...chat.messages, assistantMessage],
          }));
        }
      } else {
        // РќРµ СѓРґР°Р»РѕСЃСЊ СЂР°СЃРїР°СЂСЃРёС‚СЊ JSON РёР»Рё РЅРµС‚ list_selector
        const assistantMessage: ChatMessage = {
          id: `msg-${Date.now()}`,
          role: 'assistant',
          content: `вљ пёЏ РќРµ СѓРґР°Р»РѕСЃСЊ СЂР°СЃРїР°СЂСЃРёС‚СЊ РѕС‚РІРµС‚ РєР°Рє JSON РєРѕРЅС„РёРіСѓСЂР°С†РёСЋ.\n\nРћС‚РІРµС‚ AI:\n${response}\n\nРџРѕРїСЂРѕР±СѓР№С‚Рµ РїРµСЂРµС„РѕСЂРјСѓР»РёСЂРѕРІР°С‚СЊ Р·Р°РїСЂРѕСЃ РёР»Рё СѓР±РµРґРёС‚РµСЃСЊ, С‡С‚Рѕ AI РІРѕР·РІСЂР°С‰Р°РµС‚ JSON РІ РїСЂР°РІРёР»СЊРЅРѕРј С„РѕСЂРјР°С‚Рµ.`,
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
            content: `вќЊ РћС€РёР±РєР°: ${errorMessage}`,
            timestamp: new Date(),
          },
        ],
      }));
    } finally {
      isSending = false;
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
          content: 'Р§Р°С‚ РѕС‡РёС‰РµРЅ. Р§РµРј РјРѕРіСѓ РїРѕРјРѕС‡СЊ?',
          timestamp: new Date(),
        },
      ],
    }));
  }
</script>

<div class="ai-chat">
  <ChatHeader
    title={chats.find(c => c.id === currentChatId)?.title || 'AI РџРѕРјРѕС‰РЅРёРє'}
    onToggleChatList={() => (showChatList = !showChatList)}
    onClear={clearChat}
  />

  {#if showChatList}
    <ChatList
      {chats}
      currentChatId={currentChatId}
      onSelectChat={(chatId) => {
        currentChatId = chatId;
        updateMessagesFromChat();
        showChatList = false;
      }}
      onDeleteChat={deleteChat}
      onCreateNew={createNewChat}
    />
  {/if}

  <!-- РљРѕРґ С‚РµРїРµСЂСЊ РїРѕРєР°Р·С‹РІР°РµС‚СЃСЏ С‚РѕР»СЊРєРѕ РІ РЅРёР¶РЅРµР№ РїР°РЅРµР»Рё, РЅРµ РІ С‡Р°С‚Рµ -->

  <ChatMessages
    {messages}
    hasJsonConfig={extractJSONConfigFromMessage}
    canRunParser={nodes.length > 0 && !!currentUrl}
    onApplyCode={applyCodeFix}
    onRestoreConfig={restoreConfigFromMessage}
    onGenerateCode={generateCodeFromMessage}
    onRunParser={runParserFromChat}
    onCopy={copyMessageContent}
    onCopyText={copyMessageText}
    onDelete={deleteMessage}
  />

  <ChatInput
    inputMessage={inputMessage}
    isSending={isSending}
    attachedElement={attachedElement}
    selectedElementInfo={selectedElementInfo}
    nodesCount={nodes.length}
    hasGeneratedCode={!!generatedCode}
    hasCurrentUrl={!!currentUrl}
    onInputChange={(value) => inputMessage = value}
    onSend={sendMessage}
    onAttachElement={attachElement}
    onRemoveAttachment={removeAttachedElement}
    onGenerateCode={() => onGenerateCode()}
    onCheckCode={checkGeneration}
    onRunParser={runParserFromChat}
  />
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
</style>
