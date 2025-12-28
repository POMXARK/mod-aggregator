/**
 * Composable для управления состоянием ParserBuilder
 *
 * Управляет сохранением и восстановлением состояния UI,
 * включая размеры панелей, видимость, активные вкладки и настройки.
 */

export interface ParserBuilderUIState {
  version: number;
  pageViewerWidth: number;
  chatPanelWidth: number;
  bottomPanelHeight: number;
  showPageViewer: boolean;
  showAIChat: boolean;
  showParserResults: boolean;
  showAISettings: boolean;
  showCodeEditor: boolean;
  activeBottomTab: 'code' | 'results' | 'review' | 'runner' | null;
  currentUrl: string;
  aiModelType: 'ollama' | 'openai' | 'anthropic' | 'google';
  aiModelName: string;
  aiApiKey: string;
  aiOllamaUrl: string;
  aiDescription: string;
  selectedSiteId: number | null;
  generatedCode: string;
  editedCode: string;
}

const UI_STATE_KEY = 'parser-builder-ui-state';
const UI_STATE_VERSION = 1;

const DEFAULT_UI_STATE: ParserBuilderUIState = {
  version: UI_STATE_VERSION,
  pageViewerWidth: 600,
  chatPanelWidth: 400,
  bottomPanelHeight: 300,
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
 * Мигрирует состояние UI между версиями
 */
export function migrateUIState(
  state: any,
  fromVersion: number,
  toVersion: number
): ParserBuilderUIState {
  const migratedState = { ...state };

  // Миграция с версии 0 (без версии) на версию 1
  if (fromVersion < 1 && toVersion >= 1) {
    // Добавляем недостающие поля со значениями по умолчанию
    if (migratedState.pageViewerWidth === undefined) {
      migratedState.pageViewerWidth = DEFAULT_UI_STATE.pageViewerWidth;
    }
    // Миграция: если pageViewerWidth < 100, значит это старый формат (проценты)
    // Конвертируем в пиксели (предполагаем ширину контейнера ~1200px, 50% = 600px)
    if (migratedState.pageViewerWidth < 100) {
      const containerWidth = 1200; // примерная ширина
      migratedState.pageViewerWidth = Math.round(
        (migratedState.pageViewerWidth / 100) * containerWidth
      );
    }
    if (migratedState.chatPanelWidth === undefined) {
      migratedState.chatPanelWidth = DEFAULT_UI_STATE.chatPanelWidth;
    }
  }

  return migratedState as ParserBuilderUIState;
}

/**
 * Сохраняет состояние UI в localStorage
 */
export function saveUIState(state: Partial<ParserBuilderUIState>): void {
  try {
    const fullState: ParserBuilderUIState = {
      version: UI_STATE_VERSION,
      ...DEFAULT_UI_STATE,
      ...state,
    };
    localStorage.setItem(UI_STATE_KEY, JSON.stringify(fullState));
  } catch (e) {
    console.error('Failed to save UI state:', e);
  }
}

/**
 * Восстанавливает состояние UI из localStorage
 */
export function restoreUIState(): ParserBuilderUIState | null {
  try {
    const stored = localStorage.getItem(UI_STATE_KEY);
    if (stored) {
      const state = JSON.parse(stored);

      // Проверяем версию и мигрируем при необходимости
      const stateVersion = state.version || 0;
      if (stateVersion !== UI_STATE_VERSION) {
        const migratedState = migrateUIState(state, stateVersion, UI_STATE_VERSION);
        return migratedState;
      }

      return state as ParserBuilderUIState;
    }
  } catch (e) {
    console.error('Failed to restore UI state:', e);
  }
  return null;
}

/**
 * Экспортирует состояние UI в JSON
 */
export function exportUIState(state: Partial<ParserBuilderUIState>): string {
  try {
    const fullState: ParserBuilderUIState = {
      version: UI_STATE_VERSION,
      ...DEFAULT_UI_STATE,
      ...state,
    };
    return JSON.stringify(fullState, null, 2);
  } catch (e) {
    console.error('Failed to export UI state:', e);
    return '';
  }
}

/**
 * Импортирует состояние UI из JSON
 */
export function importUIState(jsonString: string): ParserBuilderUIState | null {
  try {
    const state = JSON.parse(jsonString);

    // Проверяем версию и мигрируем при необходимости
    const stateVersion = state.version || 0;
    if (stateVersion !== UI_STATE_VERSION) {
      const migratedState = migrateUIState(state, stateVersion, UI_STATE_VERSION);
      return migratedState;
    }

    return state as ParserBuilderUIState;
  } catch (e) {
    console.error('Failed to import UI state:', e);
    return null;
  }
}

/**
 * Применяет состояние UI к компоненту
 */
export function applyUIState(
  state: Partial<ParserBuilderUIState>,
  updaters: {
    setPageViewerWidth: (value: number) => void;
    setChatPanelWidth: (value: number) => void;
    setBottomPanelHeight: (value: number) => void;
    setShowPageViewer: (value: boolean) => void;
    setShowAIChat: (value: boolean) => void;
    setShowParserResults: (value: boolean) => void;
    setShowAISettings: (value: boolean) => void;
    setShowCodeEditor: (value: boolean) => void;
    setActiveBottomTab: (value: 'code' | 'results' | 'review' | 'runner' | null) => void;
    setCurrentUrl: (value: string) => void;
    setAIModelType: (value: 'ollama' | 'openai' | 'anthropic' | 'google') => void;
    setAIModelName: (value: string) => void;
    setAIApiKey: (value: string) => void;
    setAIOllamaUrl: (value: string) => void;
    setAIDescription: (value: string) => void;
    setSelectedSiteId: (value: number | null) => void;
    setGeneratedCode: (value: string) => void;
    setEditedCode: (value: string) => void;
  }
): void {
  // Восстанавливаем размеры панелей
  if (typeof state.pageViewerWidth === 'number') {
    let width = state.pageViewerWidth;
    if (width < 100) {
      const containerWidth = 1200;
      width = Math.round((width / 100) * containerWidth);
    }
    updaters.setPageViewerWidth(Math.max(300, Math.min(1000, width)));
  }
  if (typeof state.chatPanelWidth === 'number') {
    updaters.setChatPanelWidth(Math.max(250, Math.min(800, state.chatPanelWidth)));
  }
  if (typeof state.bottomPanelHeight === 'number') {
    updaters.setBottomPanelHeight(
      Math.max(150, Math.min(window.innerHeight * 0.7, state.bottomPanelHeight))
    );
  }

  // Восстанавливаем видимость панелей
  if (typeof state.showPageViewer === 'boolean') {
    updaters.setShowPageViewer(state.showPageViewer);
  }
  if (typeof state.showAIChat === 'boolean') {
    updaters.setShowAIChat(state.showAIChat);
  }
  if (typeof state.showParserResults === 'boolean') {
    updaters.setShowParserResults(state.showParserResults);
  }
  if (typeof state.showAISettings === 'boolean') {
    updaters.setShowAISettings(state.showAISettings);
  }
  if (typeof state.showCodeEditor === 'boolean') {
    updaters.setShowCodeEditor(state.showCodeEditor);
  }

  // Восстанавливаем активную вкладку
  if (
    state.activeBottomTab &&
    ['code', 'results', 'review', 'runner'].includes(state.activeBottomTab)
  ) {
    updaters.setActiveBottomTab(state.activeBottomTab);
  }

  // Восстанавливаем текущий URL
  if (typeof state.currentUrl === 'string') {
    updaters.setCurrentUrl(state.currentUrl);
  }

  // Восстанавливаем настройки AI
  if (
    state.aiModelType &&
    ['ollama', 'openai', 'anthropic', 'google'].includes(state.aiModelType)
  ) {
    updaters.setAIModelType(state.aiModelType);
  }
  if (typeof state.aiModelName === 'string') {
    updaters.setAIModelName(state.aiModelName);
  }
  if (typeof state.aiApiKey === 'string') {
    updaters.setAIApiKey(state.aiApiKey);
  }
  if (typeof state.aiOllamaUrl === 'string') {
    updaters.setAIOllamaUrl(state.aiOllamaUrl);
  }
  if (typeof state.aiDescription === 'string') {
    updaters.setAIDescription(state.aiDescription);
  }

  // Сохраняем ID выбранного сайта
  if (typeof state.selectedSiteId === 'number') {
    updaters.setSelectedSiteId(state.selectedSiteId);
  }

  // Восстанавливаем код
  if (typeof state.generatedCode === 'string') {
    updaters.setGeneratedCode(state.generatedCode);
  }
  if (typeof state.editedCode === 'string') {
    updaters.setEditedCode(state.editedCode);
  }
}

