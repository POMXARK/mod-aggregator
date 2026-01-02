import { derived, get, type Writable } from 'svelte/store';
import { statePersistence, usePersistentState, usePersistentStateGroup, type PersistentStore } from './useStatePersistence';

/**
 * Типы состояний приложения
 */
export interface UIState {
  // Панели видимости
  showPageViewer: boolean;
  showAIChat: boolean;
  showParserResults: boolean;
  showUISettingsMenu: boolean;
  showCodeEditor: boolean;

  // Активные вкладки
  activeBottomTab: string | null;

  // Размеры панелей
  pageViewerWidth: number;
  chatPanelWidth: number;
  bottomPanelHeight: number;

  // Состояние изменения размеров
  isResizingPageViewer: boolean;
  isResizingChatPanel: boolean;
  isResizingBottomPanel: boolean;

  // Текущий чат
  currentChatId: string | null;
}

export interface ParserState {
  // Узлы и связи
  nodes: any[];
  edges: any[];

  // Код
  generatedCode: string;
  editedCode: string;

  // Метаданные
  currentUrl: string;
  selectedSiteId: string | null;
}

export interface AISettings {
  modelType: 'ollama' | 'openai' | 'anthropic' | 'google';
  modelName: string;
  apiKey: string;
  ollamaUrl: string;
  description: string;
}

export interface AppState {
  // UI состояние
  ui: UIState;

  // Состояние парсера
  parser: ParserState;

  // Настройки AI
  ai: AISettings;
}

/**
 * Значения по умолчанию для состояний
 */
export const DEFAULT_UI_STATE: UIState = {
  // Панели видимости
  showPageViewer: false,
  showAIChat: false,
  showParserResults: false,
  showUISettingsMenu: false,
  showCodeEditor: false,

  // Активные вкладки
  activeBottomTab: null,

  // Размеры панелей (в пикселях)
  pageViewerWidth: 600,
  chatPanelWidth: 400,
  bottomPanelHeight: 300,

  // Состояние изменения размеров
  isResizingPageViewer: false,
  isResizingChatPanel: false,
  isResizingBottomPanel: false,

  // Текущий чат
  currentChatId: null,
};

export const DEFAULT_PARSER_STATE: ParserState = {
  // Узлы и связи
  nodes: [],
  edges: [],

  // Код
  generatedCode: '',
  editedCode: '',

  // Метаданные
  currentUrl: '',
  selectedSiteId: null,
};

export const DEFAULT_AI_SETTINGS: AISettings = {
  modelType: 'ollama',
  modelName: 'llama3.2:3b',
  apiKey: '',
  ollamaUrl: 'http://localhost:11434',
  description: 'Извлеки информацию о модах: название, ссылка, описание, изображение',
};

/**
 * Универсальная система управления состоянием приложения
 */
export class AppStateManager {
  private static instance: AppStateManager;
  private uiStates = new Map<string, PersistentStore<UIState>>();
  private parserStates = new Map<string, PersistentStore<ParserState>>();
  private aiSettings: PersistentStore<AISettings>;

  private constructor() {
    // Инициализируем AI настройки
    this.aiSettings = usePersistentState(
      'app.ai.settings',
      DEFAULT_AI_SETTINGS,
      { version: 1 }
    );
  }

  static getInstance(): AppStateManager {
    if (!AppStateManager.instance) {
      AppStateManager.instance = new AppStateManager();
    }
    return AppStateManager.instance;
  }

  /**
   * Получает или создает UI состояние для конкретного URL и чата
   */
  getUIState(url: string, chatId: string | null): PersistentStore<UIState> {
    const key = this.createUIStateKey(url, chatId);

    if (!this.uiStates.has(key)) {
      const store = usePersistentState(
        `app.ui.${key}`,
        DEFAULT_UI_STATE,
        { version: 2 } // Увеличиваем версию для миграции
      );
      this.uiStates.set(key, store);
    }

    return this.uiStates.get(key)!;
  }

  /**
   * Получает или создает parser состояние для конкретного URL и чата
   */
  getParserState(url: string, chatId: string | null): PersistentStore<ParserState> {
    const key = this.createParserStateKey(url, chatId);

    if (!this.parserStates.has(key)) {
      const store = usePersistentState(
        `app.parser.${key}`,
        DEFAULT_PARSER_STATE,
        { version: 1 }
      );
      this.parserStates.set(key, store);
    }

    return this.parserStates.get(key)!;
  }

  /**
   * Получает AI настройки
   */
  getAISettings(): PersistentStore<AISettings> {
    return this.aiSettings;
  }

  /**
   * Создает ключ для UI состояния
   */
  private createUIStateKey(url: string, chatId: string | null): string {
    const normalizedUrl = url.replace(/^https?:\/\//, '').replace(/[^a-zA-Z0-9]/g, '_');
    const chatPart = chatId ? `_${chatId}` : '';
    return `${normalizedUrl}${chatPart}`;
  }

  /**
   * Создает ключ для parser состояния
   */
  private createParserStateKey(url: string, chatId: string | null): string {
    return this.createUIStateKey(url, chatId);
  }

  /**
   * Сохраняет состояние для конкретного контекста
   */
  saveState(url: string, chatId: string | null, additionalData?: {
    nodes?: any[];
    edges?: any[];
    generatedCode?: string;
    editedCode?: string;
  }) {
    const uiState = this.getUIState(url, chatId);
    const parserState = this.getParserState(url, chatId);

    // Сохраняем UI состояние
    uiState.save();

    // Обновляем и сохраняем parser состояние, если есть дополнительные данные
    if (additionalData) {
      const currentParserState = get(parserState);
      const updatedParserState: ParserState = {
        ...currentParserState,
        nodes: additionalData.nodes ?? currentParserState.nodes,
        edges: additionalData.edges ?? currentParserState.edges,
        generatedCode: additionalData.generatedCode ?? currentParserState.generatedCode,
        editedCode: additionalData.editedCode ?? currentParserState.editedCode,
        currentUrl: url,
      };
      parserState.set(updatedParserState);
      parserState.save();
    } else {
      parserState.save();
    }
  }

  /**
   * Загружает состояние для конкретного контекста
   */
  loadState(url: string, chatId: string | null): {
    uiState: UIState;
    parserState: ParserState;
  } | null {
    try {
      const uiState = this.getUIState(url, chatId);
      const parserState = this.getParserState(url, chatId);

      uiState.load();
      parserState.load();

      return {
        uiState: get(uiState),
        parserState: get(parserState),
      };
    } catch (error) {
      console.error('Failed to load state:', error);
      return null;
    }
  }

  /**
   * Сбрасывает все состояния для конкретного URL
   */
  resetState(url: string) {
    // Ищем все состояния связанные с этим URL
    const keysToReset: string[] = [];

    this.uiStates.forEach((store, key) => {
      if (key.startsWith(url.replace(/^https?:\/\//, '').replace(/[^a-zA-Z0-9]/g, '_'))) {
        store.reset();
        keysToReset.push(key);
      }
    });

    this.parserStates.forEach((store, key) => {
      if (key.startsWith(url.replace(/^https?:\/\//, '').replace(/[^a-zA-Z0-9]/g, '_'))) {
        store.reset();
        keysToReset.push(key);
      }
    });

    // Удаляем из кэша
    keysToReset.forEach(key => {
      this.uiStates.delete(key);
      this.parserStates.delete(key);
    });
  }

  /**
   * Экспортирует все состояния
   */
  exportAllStates(): string {
    return statePersistence.exportAllStates();
  }

  /**
   * Импортирует состояния
   */
  importStates(jsonString: string): boolean {
    const success = statePersistence.importStates(jsonString);
    if (success) {
      // Очищаем кэш, чтобы при следующем обращении загрузились новые данные
      this.uiStates.clear();
      this.parserStates.clear();
    }
    return success;
  }

  /**
   * Получает статистику по сохраненным состояниям
   */
  getStateStats(): {
    uiStates: number;
    parserStates: number;
    aiSettings: boolean;
    totalSize: number;
  } {
    const allKeys = statePersistence.getAllStoredKeys();
    let totalSize = 0;

    allKeys.forEach(key => {
      try {
        const value = localStorage.getItem(key);
        if (value) {
          totalSize += value.length;
        }
      } catch (error) {
        // Игнорируем ошибки
      }
    });

    return {
      uiStates: Array.from(this.uiStates.keys()).length,
      parserStates: Array.from(this.parserStates.keys()).length,
      aiSettings: true, // Всегда есть
      totalSize,
    };
  }
}

// Экспортируем singleton instance
export const appStateManager = AppStateManager.getInstance();

/**
 * Composable для управления UI состоянием
 */
export function useUIState(url?: string, chatId?: string | null) {
  // Используем фиксированные ключи для общих состояний или динамические для контекстных
  const stateKey = url ? `${url}_${chatId || 'no-chat'}` : 'global';

  const uiState = appStateManager.getUIState(url || '', chatId || null);

  return {
    // Stores
    uiState,

    // Геттеры для удобства
    showPageViewer: derived(uiState, $state => $state.showPageViewer),
    showAIChat: derived(uiState, $state => $state.showAIChat),
    showParserResults: derived(uiState, $state => $state.showParserResults),
    showUISettingsMenu: derived(uiState, $state => $state.showUISettingsMenu),
    showCodeEditor: derived(uiState, $state => $state.showCodeEditor),
    activeBottomTab: derived(uiState, $state => $state.activeBottomTab),
    pageViewerWidth: derived(uiState, $state => $state.pageViewerWidth),
    chatPanelWidth: derived(uiState, $state => $state.chatPanelWidth),
    bottomPanelHeight: derived(uiState, $state => $state.bottomPanelHeight),
    isResizingPageViewer: derived(uiState, $state => $state.isResizingPageViewer),
    isResizingChatPanel: derived(uiState, $state => $state.isResizingChatPanel),
    isResizingBottomPanel: derived(uiState, $state => $state.isResizingBottomPanel),
    currentChatId: derived(uiState, $state => $state.currentChatId),

    // Сеттеры
    setShowPageViewer: (value: boolean) => {
      uiState.update(state => ({ ...state, showPageViewer: value }));
    },
    setShowAIChat: (value: boolean) => {
      uiState.update(state => ({ ...state, showAIChat: value }));
    },
    setShowParserResults: (value: boolean) => {
      uiState.update(state => ({ ...state, showParserResults: value }));
    },
    setShowUISettingsMenu: (value: boolean) => {
      uiState.update(state => ({ ...state, showUISettingsMenu: value }));
    },
    setShowCodeEditor: (value: boolean) => {
      uiState.update(state => ({ ...state, showCodeEditor: value }));
    },
    setActiveBottomTab: (value: string | null) => {
      uiState.update(state => ({ ...state, activeBottomTab: value }));
    },
    setPageViewerWidth: (value: number) => {
      uiState.update(state => ({ ...state, pageViewerWidth: value }));
    },
    setChatPanelWidth: (value: number) => {
      uiState.update(state => ({ ...state, chatPanelWidth: value }));
    },
    setBottomPanelHeight: (value: number) => {
      uiState.update(state => ({ ...state, bottomPanelHeight: value }));
    },
    setIsResizingPageViewer: (value: boolean) => {
      uiState.update(state => ({ ...state, isResizingPageViewer: value }));
    },
    setIsResizingChatPanel: (value: boolean) => {
      uiState.update(state => ({ ...state, isResizingChatPanel: value }));
    },
    setIsResizingBottomPanel: (value: boolean) => {
      uiState.update(state => ({ ...state, isResizingBottomPanel: value }));
    },
    setCurrentChatId: (value: string | null) => {
      uiState.update(state => ({ ...state, currentChatId: value }));
    },

    // Управление состоянием
    saveState: () => uiState.save(),
    loadState: () => uiState.load(),
    resetState: () => uiState.reset(),

    // Применение состояния
    applyUIState: (state: Partial<UIState>) => {
      uiState.set({ ...get(uiState), ...state });
    },
  };
}

/**
 * Composable для управления parser состоянием
 */
export function useParserState(url?: string, chatId?: string | null) {
  const parserState = appStateManager.getParserState(url || '', chatId || null);

  return {
    // Stores
    parserState,

    // Геттеры
    nodes: derived(parserState, $state => $state.nodes),
    edges: derived(parserState, $state => $state.edges),
    generatedCode: derived(parserState, $state => $state.generatedCode),
    editedCode: derived(parserState, $state => $state.editedCode),
    currentUrl: derived(parserState, $state => $state.currentUrl),
    selectedSiteId: derived(parserState, $state => $state.selectedSiteId),

    // Сеттеры
    setNodes: (nodes: any[]) => {
      parserState.update(state => ({ ...state, nodes }));
    },
    setEdges: (edges: any[]) => {
      parserState.update(state => ({ ...state, edges }));
    },
    setGeneratedCode: (code: string) => {
      parserState.update(state => ({ ...state, generatedCode: code }));
    },
    setEditedCode: (code: string) => {
      parserState.update(state => ({ ...state, editedCode: code }));
    },
    setCurrentUrl: (url: string) => {
      parserState.update(state => ({ ...state, currentUrl: url }));
    },
    setSelectedSiteId: (id: string | null) => {
      parserState.update(state => ({ ...state, selectedSiteId: id }));
    },

    // Управление состоянием
    saveState: () => parserState.save(),
    loadState: () => parserState.load(),
    resetState: () => parserState.reset(),
  };
}

/**
 * Composable для управления AI настройками
 */
export function useAISettings() {
  const aiSettings = appStateManager.getAISettings();

  return {
    // Stores
    aiSettings,

    // Геттеры
    modelType: derived(aiSettings, $state => $state.modelType),
    modelName: derived(aiSettings, $state => $state.modelName),
    apiKey: derived(aiSettings, $state => $state.apiKey),
    ollamaUrl: derived(aiSettings, $state => $state.ollamaUrl),
    description: derived(aiSettings, $state => $state.description),

    // Сеттеры
    setModelType: (value: 'ollama' | 'openai' | 'anthropic' | 'google') => {
      aiSettings.update(state => ({ ...state, modelType: value }));
    },
    setModelName: (value: string) => {
      aiSettings.update(state => ({ ...state, modelName: value }));
    },
    setApiKey: (value: string) => {
      aiSettings.update(state => ({ ...state, apiKey: value }));
    },
    setOllamaUrl: (value: string) => {
      aiSettings.update(state => ({ ...state, ollamaUrl: value }));
    },
    setDescription: (value: string) => {
      aiSettings.update(state => ({ ...state, description: value }));
    },

    // Управление состоянием
    saveSettings: () => aiSettings.save(),
    loadSettings: () => aiSettings.load(),
    resetSettings: () => aiSettings.reset(),
  };
}