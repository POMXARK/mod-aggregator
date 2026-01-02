import type { Node, Edge } from '@xyflow/svelte';

/**
 * Типы для модуля ParserBuilder
 */

// Типы для состояния UI
export type BottomTabType = 'code' | 'results' | 'review' | 'runner' | null;
export type AIModelType = 'ollama' | 'openai' | 'anthropic' | 'google';
export type OllamaStatus = 'checking' | 'connected' | 'error' | null;

// Типы для элементов интерфейса
export interface ContextMenuState {
  x: number;
  y: number;
}

export interface SelectedElementInfo {
  selector: string;
  elementInfo: {
    tagName: string;
    text: string;
    attributes: Record<string, string>;
    similarElements?: number;
  };
}

// Типы для результатов парсинга
export interface ParserResult {
  data: Record<string, unknown>;
  expanded: boolean;
}

// Типы для настроек парсера
export interface ParserSettings {
  maxElements: number;
  timeoutSeconds: number;
  slowMode: boolean;
  delayPerElement: number;
}

// Типы для AI настроек
export interface AISettings {
  modelType: AIModelType;
  modelName: string;
  apiKey: string;
  ollamaUrl: string;
  description: string;
}

// Типы для состояния Ollama
export interface OllamaState {
  models: string[];
  status: OllamaStatus;
  error: string | null;
}

// Основной интерфейс состояния ParserBuilder (закомментирован - не используется)
// export interface ParserBuilderState {
//   // Состояние парсера
//   nodes: Node[];
//   edges: Edge[];
//   selectedSite: Site | null;
//   sites: Site[];
//   currentUrl: string;
//   generatedCode: string;
//   editedCode: string;
//   error: string | null;

//   // Состояние UI
//   showPageViewer: boolean;
//   showAIChat: boolean;
//   showParserResults: boolean;
//   showAISettings: boolean;
//   showCodeEditor: boolean;
//   showUISettingsMenu: boolean;
//   activeBottomTab: BottomTabType;
//   bottomPanelHeight: number;

//   // Размеры панелей
//   pageViewerWidth: number;
//   chatPanelWidth: number;
//   isResizingPageViewer: boolean;
//   isResizingChatPanel: boolean;
//   isResizingBottomPanel: boolean;

//   // Выбранные элементы
//   contextMenu: ContextMenuState | null;
//   selectedElementInfo: SelectedElementInfo | null;

//   // Результаты тестирования
//   parserResults: ParserResult[];
//   parserTestError: string | null;
//   parserDiagnostics: unknown[];
//   parserExtractionStats: unknown;
//   isCheckingCodeWithAI: boolean;
//   aiCodeReview: string | null;

//   // Настройки парсера
//   parserSettings: ParserSettings;

//   // AI настройки
//   aiSettings: AISettings;
//   ollamaState: OllamaState;
//   isAIGenerating: boolean;

//   // Callbacks
//   addAIMessageToChat: ((content: string) => void) | null;
//   onGenerateCode: (() => void) | null;
//   onTestParser: (() => Promise<void>) | null;
//   onCheckCodeWithAI: (() => Promise<void>) | null;

//   // Внутренние ID
//   savedSelectedSiteId: number | null;
//   currentChatId: string | null;
// }

// Типы для конфигурации UI состояния (для сохранения/восстановления)
export interface UIStateConfig {
  version: number;
  pageViewerWidth: number;
  chatPanelWidth: number;
  bottomPanelHeight: number;
  showPageViewer: boolean;
  showAIChat: boolean;
  showParserResults: boolean;
  showAISettings: boolean;
  showCodeEditor: boolean;
  activeBottomTab: BottomTabType;
  currentUrl: string;
  aiModelType: AIModelType;
  aiModelName: string;
  aiApiKey: string;
  aiOllamaUrl: string;
  aiDescription: string;
  selectedSiteId: number | null;
  generatedCode: string;
  editedCode: string;
  nodes?: Node[];
  edges?: Edge[];
  stateUrl?: string;
  chatId?: string;
}

// Типы для операций с парсером (закомментирован - не используется)
// export interface ParserOperations {
//   loadSites(): Promise<void>;
//   generateParserCode(): void;
//   generateParserConfig(): ParserConfig;
//   checkCodeWithAI(): Promise<void>;
//   handleTestParser(): Promise<void>;
//   handleAIGenerate(): Promise<void>;
//   handleLoadSite(): Promise<void>;
//   handleSaveParser(): void;
//   autoDetectElements(): Promise<void>;
//   detectWithAI(selector: string, elementInfo: unknown): Promise<unknown[]>;
//   confirmElementSelection(): void;
//   cancelElementSelection(): void;
//   diagnoseState(): void;
//   saveCurrentState(): void;
// }

// Типы для обработчиков событий (закомментирован - не используется)
// export interface EventHandlers {
//   handlePaneClick(event: MouseEvent): void;
//   handleContextMenu(event: MouseEvent): void;
//   handleAddNode(type: string): void;
//   handleConnect(connection: unknown): void;
//   handleNodesChange(changes: unknown[]): void;
//   handleDeleteSelected(): void;
//   handleElementSelect(selector: string, element: HTMLElement | null, elementData?: unknown): void;
//   handleMouseMove(event: MouseEvent | PointerEvent): void;
//   handleMouseUp(event: MouseEvent | PointerEvent): void;
//   handleAIModelTypeChange(): Promise<void>;
// }

// Типы для функций управления состоянием (закомментирован - не используется)
// export interface StateManagement {
//   exportUIState(): string;
//   importUIState(jsonString: string): boolean;
//   applyUIState(state: unknown): void;
//   resetUIState(): void;
//   exportUIStateToFile(): void;
//   importUIStateFromFile(): void;
//   migrateUIState(state: unknown, fromVersion: number, toVersion: number): unknown;
// }
