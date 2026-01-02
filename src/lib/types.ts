/**
 * Общие типы для всего приложения
 *
 * Централизованное определение всех типов интерфейсов и перечислений
 */

// API типы (расширяем существующие из api.ts)
export interface ParserConfig {
  list_selector: string;
  title_selector?: string;
  url_selector?: string;
  description_selector?: string;
  image_selector?: string;
  [key: string]: unknown;
}

// UI типы
export type PageType =
  | 'mods'
  | 'sites'
  | 'parser'
  | 'notifications'
  | 'files'
  | 'collections'
  | 'settings'
  | 'test';

// Компонентные типы
export interface ComponentConfig {
  enabled: boolean;
  priority?: number;
  label?: string;
  description?: string;
}

// Сессия
export interface SessionRestoreResult {
  restored: boolean;
  warnings: string[];
  uiPreferences?: {
    sidebarCollapsed?: boolean;
  };
  fileOrder?: string[];
  openCollections?: string[];
  selectedFiles?: string[];
}

// Чат типы
export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: Date;
  attachedElement?: {
    selector: string;
    tagName: string;
    text: string;
    attributes: Record<string, string>;
  };
}

export interface Chat {
  id: string;
  title: string;
  messages: ChatMessage[];
  createdAt: Date;
  updatedAt: Date;
}

// AI настройки
export interface AIModelConfig {
  type: 'ollama' | 'openai' | 'anthropic' | 'google';
  name: string;
  apiKey?: string;
  ollamaUrl?: string;
}

// События
export interface ComponentEvent<T = unknown> {
  type: string;
  payload: T;
  timestamp: Date;
}

// Навигация
export interface NavigationState {
  currentPage: PageType;
  sidebarOpen: boolean;
  selectedSiteId: number | null;
}

// Файлы
export interface FileInfo {
  id: string;
  name: string;
  path: string;
  size: number;
  type: string;
  lastModified: Date;
}

// Коллекции
export interface Collection {
  id: string;
  name: string;
  description?: string;
  files: FileInfo[];
  createdAt: Date;
  updatedAt: Date;
}

// Уведомления
export interface NotificationItem {
  id: string;
  type: 'info' | 'warning' | 'error' | 'success';
  title: string;
  message: string;
  read: boolean;
  timestamp: Date;
  actions?: NotificationAction[];
}

export interface NotificationAction {
  label: string;
  action: () => void;
  type?: 'primary' | 'secondary';
}

// Валидация форм
export interface ValidationError {
  field: string;
  message: string;
}

export interface FormState<T = Record<string, unknown>> {
  data: T;
  errors: ValidationError[];
  isValid: boolean;
  isSubmitting: boolean;
}

// Универсальные утилитарные типы
export type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

export type Optional<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;

export type RequiredFields<T, K extends keyof T> = T & Required<Pick<T, K>>;

// Асинхронные типы
export type AsyncResult<T> = Promise<Result<T>>;
export type Result<T, E = Error> = { success: true; data: T } | { success: false; error: E };

// Event handlers
export type EventHandler<T = unknown> = (event: T) => void;
export type AsyncEventHandler<T = unknown> = (event: T) => Promise<void>;

// Component props helpers
export type ComponentProps<T extends Record<string, unknown>> = {
  [K in keyof T]: T[K];
};

export type Bindable<T> = T & {
  bind?: (value: T) => void;
};

// Node types (для flow)
export interface NodeData {
  nodeType: string;
  label: string;
  [key: string]: unknown;
}

export interface FlowNode {
  id: string;
  type: string;
  position: { x: number; y: number };
  data: NodeData;
  selected?: boolean;
  dragging?: boolean;
}

export interface FlowEdge {
  id: string;
  source: string;
  target: string;
  sourceHandle?: string;
  targetHandle?: string;
}

// Parser runner
export interface ParserResult {
  success: boolean;
  data?: unknown[];
  errors?: string[];
  warnings?: string[];
  executionTime?: number;
}

export interface ParserExecutionState {
  isRunning: boolean;
  progress: number;
  currentStep?: string;
  results?: ParserResult;
}

// Settings
export interface AppSettings {
  theme: 'light' | 'dark' | 'auto';
  language: string;
  ai: AIModelConfig;
  notifications: {
    enabled: boolean;
    soundEnabled: boolean;
  };
  ui: {
    sidebarCollapsed: boolean;
    compactMode: boolean;
  };
}

// Export/import
export interface ExportData {
  version: string;
  timestamp: Date;
  sites: Array<{
    id: number;
    name: string;
    url: string;
    parserConfig: ParserConfig;
  }>;
  collections?: Collection[];
  settings?: DeepPartial<AppSettings>;
}

export interface ImportResult {
  success: boolean;
  imported: {
    sites: number;
    collections: number;
  };
  errors: string[];
  warnings: string[];
}












