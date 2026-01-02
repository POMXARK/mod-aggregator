import { saveUIState, restoreUIState as restoreUIStateFromStorage, type ParserBuilderUIState } from '@/lib/composables/useParserBuilderState';
import { writable, get } from 'svelte/store';
import type { BottomTabType, UIStateConfig } from '../types/parser-builder.types';
import type { Node, Edge } from '@xyflow/svelte';

/**
 * Composable для управления состоянием интерфейса
 * Управляет размерами панелей, видимостью компонентов и сохранением состояния
 */
export function useUIState(getCurrentUrlFn?: () => string, getCurrentChatIdFn?: () => string | null) {
  // UI константы
  const UI_CONSTANTS = {
    // Размеры панелей по умолчанию
    DEFAULT_PAGE_VIEWER_WIDTH: 600,
    DEFAULT_CHAT_PANEL_WIDTH: 400,
    DEFAULT_BOTTOM_PANEL_HEIGHT: 300,

    // Ограничения размеров
    PAGE_VIEWER_WIDTH_MIN: 300,
    PAGE_VIEWER_WIDTH_MAX: 1000,
    CHAT_PANEL_WIDTH_MIN: 250,
    CHAT_PANEL_WIDTH_MAX: 800,
    BOTTOM_PANEL_HEIGHT_MIN: 150,

    // Предполагаемая ширина контейнера для конвертации процентов в пиксели
    CONTAINER_WIDTH_ESTIMATE: 1200,

    // Задержка автосохранения в мс
    SAVE_DELAY_MS: 100,
  } as const;

  // Состояние UI
  const showPageViewerStore = writable(false);
  const showAIChatStore = writable(false);
  const showParserResultsStore = writable(false);
  const showUISettingsMenuStore = writable(false);
  const showCodeEditorStore = writable(false);
  const activeBottomTabStore = writable<BottomTabType>(null);

  // Размеры панелей
  const pageViewerWidthStore = writable(600); // пиксели от левого края
  const chatPanelWidthStore = writable(400); // пиксели от правого края
  const bottomPanelHeightStore = writable(300);
  const isResizingPageViewerStore = writable(false);
  const isResizingChatPanelStore = writable(false);
  const isResizingBottomPanelStore = writable(false);

  // Внутренние ID для привязки состояния
  const currentChatIdStore = writable<string | null>(null);

  // Геттеры для получения текущих значений stores
  const getCurrentValues = () => ({
    showPageViewer: get(showPageViewerStore),
    showAIChat: get(showAIChatStore),
    showParserResults: get(showParserResultsStore),
    showUISettingsMenu: get(showUISettingsMenuStore),
    showCodeEditor: get(showCodeEditorStore),
    activeBottomTab: get(activeBottomTabStore),
    bottomPanelHeight: get(bottomPanelHeightStore),
    pageViewerWidth: get(pageViewerWidthStore),
    chatPanelWidth: get(chatPanelWidthStore),
    isResizingPageViewer: get(isResizingPageViewerStore),
    isResizingChatPanel: get(isResizingChatPanelStore),
    isResizingBottomPanel: get(isResizingBottomPanelStore),
    currentChatId: get(currentChatIdStore),
  });

  /**
   * Простая debounce функция для предотвращения частых сохранений
   */
  function debounce<T extends (...args: any[]) => void>(
    func: T,
    delay: number
  ): (...args: Parameters<T>) => void {
    let timeoutId: number;
    return (...args: Parameters<T>) => {
      clearTimeout(timeoutId);
      timeoutId = window.setTimeout(() => func(...args), delay);
    };
  }

  /**
   * Debounced функция сохранения состояния
   */
  const debouncedSaveState = debounce(() => {
    saveCurrentState(
      getCurrentUrlFn ? getCurrentUrlFn() : '',
      getCurrentChatIdFn ? getCurrentChatIdFn() : null,
      {}
    );
  }, UI_CONSTANTS.SAVE_DELAY_MS);

  // Функции-сеттеры
  const setShowPageViewer = (value: boolean) => {
    showPageViewerStore.set(value);
    debouncedSaveState();
  };
  const setShowAIChat = (value: boolean) => {
    showAIChatStore.set(value);
    debouncedSaveState();
  };
  const setShowParserResults = (value: boolean) => {
    showParserResultsStore.set(value);
    debouncedSaveState();
  };
  const setShowUISettingsMenu = (value: boolean) => showUISettingsMenuStore.set(value);
  const setShowCodeEditor = (value: boolean) => showCodeEditorStore.set(value);
  const setActiveBottomTab = (value: BottomTabType) => activeBottomTabStore.set(value);
  const setBottomPanelHeight = (value: number) => {
    bottomPanelHeightStore.set(value);
    debouncedSaveState();
  };
  const setPageViewerWidth = (value: number) => {
    pageViewerWidthStore.set(value);
    debouncedSaveState();
  };
  const setChatPanelWidth = (value: number) => {
    chatPanelWidthStore.set(value);
    debouncedSaveState();
  };
  const setIsResizingPageViewer = (value: boolean) => isResizingPageViewerStore.set(value);
  const setIsResizingChatPanel = (value: boolean) => isResizingChatPanelStore.set(value);
  const setIsResizingBottomPanel = (value: boolean) => isResizingBottomPanelStore.set(value);
  const setCurrentChatId = (value: string | null) => currentChatIdStore.set(value);

  // Версия формата сохраненных состояний
  const UI_STATE_VERSION = 1;

  // Значения по умолчанию для сброса
  const DEFAULT_UI_STATE: Omit<UIStateConfig, 'version'> = {
    pageViewerWidth: UI_CONSTANTS.DEFAULT_PAGE_VIEWER_WIDTH,
    chatPanelWidth: UI_CONSTANTS.DEFAULT_CHAT_PANEL_WIDTH,
    bottomPanelHeight: UI_CONSTANTS.DEFAULT_BOTTOM_PANEL_HEIGHT,
    showPageViewer: false,
    showAIChat: false,
    showParserResults: false,
    showAISettings: false,
    showCodeEditor: false,
    activeBottomTab: null,
    currentUrl: '',
    aiModelType: 'ollama',
    aiModelName: 'llama3.2:3b',
    aiApiKey: '',
    aiOllamaUrl: 'http://localhost:11434',
    aiDescription: 'Извлеки информацию о модах: название, ссылка, описание, изображение',
    selectedSiteId: null,
    generatedCode: '',
    editedCode: '',
  };

  /**
   * Type guard для проверки UI состояния
   */
  function isUIStateObject(state: unknown): state is Record<string, unknown> {
    return typeof state === 'object' && state !== null;
  }

  /**
   * Функция миграции состояний между версиями
   */
  function migrateUIState(state: unknown, fromVersion: number, toVersion: number): Record<string, unknown> | null {
    // Проверяем что state является объектом
    if (!isUIStateObject(state)) {
      return null;
    }
    const migratedState = { ...state };

    // Миграция с версии 0 (без версии) на версию 1
    if (fromVersion < 1 && toVersion >= 1) {
      // Добавляем недостающие поля со значениями по умолчанию
      if (migratedState.pageViewerWidth === undefined) {
        migratedState.pageViewerWidth = DEFAULT_UI_STATE.pageViewerWidth;
      }
      // Миграция: если pageViewerWidth < 100, значит это старый формат (проценты)
      // Конвертируем в пиксели (предполагаем ширину контейнера ~1200px, 50% = 600px)
      if (typeof migratedState.pageViewerWidth === 'number' && migratedState.pageViewerWidth < 100) {
        migratedState.pageViewerWidth = Math.round(
          (migratedState.pageViewerWidth / 100) * UI_CONSTANTS.CONTAINER_WIDTH_ESTIMATE
        );
      }
      if (migratedState.chatPanelWidth === undefined) {
        migratedState.chatPanelWidth = DEFAULT_UI_STATE.chatPanelWidth;
      }
      // Аналогичная миграция для chatPanelWidth
      if (typeof migratedState.chatPanelWidth === 'number' && migratedState.chatPanelWidth < 100) {
        migratedState.chatPanelWidth = Math.round(
          (migratedState.chatPanelWidth / 100) * UI_CONSTANTS.CONTAINER_WIDTH_ESTIMATE
        );
      }
    }

    return migratedState;
  }

  /**
   * Функция экспорта настроек в JSON
   */
  function exportUIState(): string {
    try {
      const state: UIStateConfig = {
        version: UI_STATE_VERSION,
        pageViewerWidth: get(pageViewerWidthStore),
        chatPanelWidth: get(chatPanelWidthStore),
        bottomPanelHeight: get(bottomPanelHeightStore),
        showPageViewer: get(showPageViewerStore),
        showAIChat: get(showAIChatStore),
        showParserResults: get(showParserResultsStore),
        showAISettings: false, // Не сохраняем состояние модального окна
        showCodeEditor: get(showCodeEditorStore),
        activeBottomTab: get(activeBottomTabStore),
        currentUrl: '', // Будет заполнено вызывающим кодом
        aiModelType: 'ollama', // Будет заполнено вызывающим кодом
        aiModelName: 'llama3.2:3b', // Будет заполнено вызывающим кодом
        aiApiKey: '', // Будет заполнено вызывающим кодом
        aiOllamaUrl: 'http://localhost:11434', // Будет заполнено вызывающим кодом
        aiDescription: 'Извлеки информацию о модах: название, ссылка, описание, изображение', // Будет заполнено вызывающим кодом
        selectedSiteId: null, // Будет заполнено вызывающим кодом
        generatedCode: '', // Будет заполнено вызывающим кодом
        editedCode: '', // Будет заполнено вызывающим кодом
      };
      return JSON.stringify(state, null, 2);
    } catch (e) {
      console.error('Failed to export UI state:', e);
      return '';
    }
  }

  /**
   * Функция импорта настроек из JSON
   */
  function importUIState(jsonString: string): boolean {
    try {
      const state = JSON.parse(jsonString);

      // Проверяем версию и мигрируем при необходимости
      const stateVersion = state.version || 0;
      if (stateVersion !== UI_STATE_VERSION) {
        const migratedState = migrateUIState(state, stateVersion, UI_STATE_VERSION);
        applyUIState(migratedState);
      } else {
        applyUIState(state);
      }

      return true;
    } catch (e) {
      console.error('Failed to import UI state:', e);
      alert('Ошибка импорта настроек: ' + (e instanceof Error ? e.message : String(e)));
      return false;
    }
  }

  /**
   * Функция применения состояния (общая для restore и import)
   */
  function applyUIState(state: unknown) {
    // Проверяем что state является объектом
    if (!isUIStateObject(state)) {
      console.warn('Invalid UI state format for applyUIState');
      return;
    }
    const uiState = state;
    // Восстанавливаем размеры панелей
    if (typeof uiState.pageViewerWidth === 'number') {
      // Если значение < 100, это старый формат (проценты), конвертируем
      let width = uiState.pageViewerWidth;
      if (width < 100) {
        const containerWidth = 1200; // примерная ширина
        width = Math.round((width / 100) * containerWidth);
      }
      pageViewerWidthStore.set(Math.max(UI_CONSTANTS.PAGE_VIEWER_WIDTH_MIN, Math.min(UI_CONSTANTS.PAGE_VIEWER_WIDTH_MAX, width)));
    }
    if (typeof uiState.chatPanelWidth === 'number') {
      chatPanelWidthStore.set(Math.max(UI_CONSTANTS.CHAT_PANEL_WIDTH_MIN, Math.min(UI_CONSTANTS.CHAT_PANEL_WIDTH_MAX, uiState.chatPanelWidth)));
    }
    if (typeof uiState.bottomPanelHeight === 'number') {
      bottomPanelHeightStore.set(Math.max(
        UI_CONSTANTS.BOTTOM_PANEL_HEIGHT_MIN,
        Math.min(window.innerHeight * 0.7, uiState.bottomPanelHeight)
      ));
    }

    // Восстанавливаем видимость панелей
    if (typeof uiState.showPageViewer === 'boolean') {
      console.log('Restoring showPageViewer:', uiState.showPageViewer);
      showPageViewerStore.set(uiState.showPageViewer);
    }
    if (typeof uiState.showAIChat === 'boolean') {
      console.log('Restoring showAIChat:', uiState.showAIChat);
      showAIChatStore.set(uiState.showAIChat);
    }
    if (typeof uiState.showParserResults === 'boolean') {
      console.log('Restoring showParserResults:', uiState.showParserResults);
      showParserResultsStore.set(uiState.showParserResults);
    }
    if (typeof uiState.showCodeEditor === 'boolean') {
      console.log('Restoring showCodeEditor:', uiState.showCodeEditor);
      showCodeEditorStore.set(uiState.showCodeEditor);
    }

    // Восстанавливаем активную вкладку
    if (uiState.activeBottomTab && typeof uiState.activeBottomTab === 'string' &&
        ['code', 'results', 'review'].includes(uiState.activeBottomTab)) {
      activeBottomTabStore.set(uiState.activeBottomTab as BottomTabType);
    }
  }

  /**
   * Функция сброса настроек интерфейса к значениям по умолчанию
   */
  function resetUIState() {
    if (
      confirm('Вы уверены, что хотите сбросить все настройки интерфейса к значениям по умолчанию?')
    ) {
      pageViewerWidthStore.set(DEFAULT_UI_STATE.pageViewerWidth);
      chatPanelWidthStore.set(DEFAULT_UI_STATE.chatPanelWidth);
      bottomPanelHeightStore.set(DEFAULT_UI_STATE.bottomPanelHeight);
      showPageViewerStore.set(DEFAULT_UI_STATE.showPageViewer);
      showAIChatStore.set(DEFAULT_UI_STATE.showAIChat);
      showParserResultsStore.set(DEFAULT_UI_STATE.showParserResults);
      showCodeEditorStore.set(DEFAULT_UI_STATE.showCodeEditor);
      activeBottomTabStore.set(DEFAULT_UI_STATE.activeBottomTab);

      alert('Настройки интерфейса сброшены к значениям по умолчанию');
    }
  }

  /**
   * Функция экспорта настроек в файл
   */
  function exportUIStateToFile() {
    try {
      const json = exportUIState();
      if (!json) {
        alert('Ошибка экспорта настроек');
        return;
      }

      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `parser-builder-ui-settings-${new Date().toISOString().split('T')[0]}.json`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);

      alert('Настройки экспортированы в файл');
    } catch (e) {
      console.error('Failed to export UI state to file:', e);
      alert('Ошибка экспорта настроек: ' + (e instanceof Error ? e.message : String(e)));
    }
  }

  /**
   * Функция импорта настроек из файла
   */
  function importUIStateFromFile() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    input.onchange = e => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) {
        return;
      }

      const reader = new FileReader();
      reader.onload = event => {
        const content = event.target?.result as string;
        if (content) {
          if (importUIState(content)) {
            alert('Настройки успешно импортированы');
          }
        }
      };
      reader.onerror = () => {
        alert('Ошибка чтения файла');
      };
      reader.readAsText(file);
    };
    input.click();
  }

  /**
   * Автосохранение состояния интерфейса
   */
  function saveCurrentState(url: string, chatId: string | null, additionalState?: {
    nodes?: Node[];
    edges?: Edge[];
    generatedCode?: string;
    editedCode?: string;
  }) {
    // Всегда сохраняем состояние, даже если URL/chatId пустые
    {
      const uiState = {
        // Parser state
        nodes: (additionalState?.nodes as Node[]) || [],
        edges: (additionalState?.edges as Edge[]) || [],
        generatedCode: String(additionalState?.generatedCode || ''),
        editedCode: String(additionalState?.editedCode || ''),

        // UI state - берем текущие значения из stores
        pageViewerWidth: get(pageViewerWidthStore),
        chatPanelWidth: get(chatPanelWidthStore),
        bottomPanelHeight: get(bottomPanelHeightStore),
        showPageViewer: get(showPageViewerStore),
        showAIChat: get(showAIChatStore),
        showParserResults: get(showParserResultsStore),
        showCodeEditor: get(showCodeEditorStore),
        activeBottomTab: get(activeBottomTabStore),

        stateUrl: url,
        chatId: chatId,
      };

      console.log('Saving UI state:', {
        url,
        chatId,
        showPageViewer: uiState.showPageViewer,
        showAIChat: uiState.showAIChat,
        showParserResults: uiState.showParserResults,
        hasNodes: Array.isArray(additionalState?.nodes),
        nodesCount: Array.isArray(additionalState?.nodes) ? additionalState.nodes.length : 0,
        hasGeneratedCode: !!additionalState?.generatedCode,
        stack: new Error().stack?.split('\n')[2]?.trim() // Добавляем информацию о том, откуда вызвано сохранение
      });

      saveUIState(uiState);
    }
  }

  /**
   * Восстанавливает состояние интерфейса
   */
  function restoreUIState(url: string, chatId: string | null): ParserBuilderUIState | null {
    try {
      console.log('Attempting to restore UI state for URL:', url, 'chatId:', chatId);

      // Сначала пробуем восстановить состояние для текущего URL и чата
      const restoredState = restoreUIStateFromStorage(url, chatId);

      if (restoredState) {
        console.log('Found UI state, applying:', {
          hasNodes: Array.isArray(restoredState.nodes) && restoredState.nodes.length > 0,
          hasEdges: Array.isArray(restoredState.edges) && restoredState.edges.length > 0,
          hasGeneratedCode: !!restoredState.generatedCode,
          hasEditedCode: !!restoredState.editedCode
        });
        applyUIState(restoredState);
        console.log('Restored UI state from localStorage for URL:', url, 'chatId:', chatId);
        return restoredState;
      } else {
        console.log('No UI state found for URL, trying general state');
        // Если не найдено состояние для URL, пробуем восстановить общее состояние
        const generalState = restoreUIStateFromStorage('', null);
        if (generalState) {
          console.log('Found general UI state, applying');
          applyUIState(generalState);
          console.log('Restored general UI state from localStorage');
          return generalState;
        } else {
          console.log('No saved UI state found at all');
          return null;
        }
      }
    } catch (e) {
      console.error('Failed to restore UI state:', e);
      return null;
    }
  }

  /**
   * Диагностика состояния localStorage
   */
  function diagnoseState() {
    console.log('=== STATE DIAGNOSIS ===');
    console.log('Current state:', {
      showPageViewer: get(showPageViewerStore),
      showAIChat: get(showAIChatStore),
      showParserResults: get(showParserResultsStore),
      activeBottomTab: get(activeBottomTabStore),
      pageViewerWidth: get(pageViewerWidthStore),
      chatPanelWidth: get(chatPanelWidthStore),
      bottomPanelHeight: get(bottomPanelHeightStore)
    });

    // Проверяем все ключи в localStorage
    const keys = Object.keys(localStorage);
    console.log('All localStorage keys:', keys);

    // Ищем ключи, связанные с состоянием
    const stateKeys = keys.filter(key => key.includes('parser-builder-ui-state'));
    console.log('Parser state keys:', stateKeys);

    // Показываем содержимое каждого ключа
    stateKeys.forEach(key => {
      try {
        const state = JSON.parse(localStorage.getItem(key) || '{}');
        console.log(`State for key "${key}":`, {
          version: state.version,
          url: state.stateUrl,
          chatId: state.chatId,
          hasNodes: Array.isArray(state.nodes) && state.nodes.length > 0,
          hasEdges: Array.isArray(state.edges) && state.edges.length > 0,
          hasGeneratedCode: !!state.generatedCode,
          hasEditedCode: !!state.editedCode,
          nodesCount: Array.isArray(state.nodes) ? state.nodes.length : 0,
          edgesCount: Array.isArray(state.edges) ? state.edges.length : 0
        });
      } catch (e) {
        console.error(`Error parsing state for key "${key}":`, e);
      }
    });
  }

  return {
    // Stores
    showPageViewerStore,
    showAIChatStore,
    showParserResultsStore,
    showUISettingsMenuStore,
    showCodeEditorStore,
    activeBottomTabStore,
    bottomPanelHeightStore,
    pageViewerWidthStore,
    chatPanelWidthStore,
    isResizingPageViewerStore,
    isResizingChatPanelStore,
    isResizingBottomPanelStore,
    currentChatIdStore,

    // Сеттеры
    setShowPageViewer,
    setShowAIChat,
    setShowParserResults,
    setShowUISettingsMenu,
    setShowCodeEditor,
    setActiveBottomTab,
    setBottomPanelHeight,
    setPageViewerWidth,
    setChatPanelWidth,
    setIsResizingPageViewer,
    setIsResizingChatPanel,
    setIsResizingBottomPanel,
    setCurrentChatId,

    // Управление состоянием
    resetUIState,
    exportUIStateToFile,
    importUIStateFromFile,
    saveCurrentState,
    restoreUIState,
    applyUIState,
    diagnoseState,
    getCurrentValues,
  };
}

