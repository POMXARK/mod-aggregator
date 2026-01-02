/**
 * Тип действия пользователя для истории
 */
export type RecentActionType =
  | 'file_added'
  | 'file_deleted'
  | 'collection_created'
  | 'dependency_added'
  | 'dependency_removed'
  | 'collection_updated';

/**
 * Запись о последнем действии пользователя
 */
export interface RecentAction {
  type: RecentActionType;
  targetId: number;
  targetName: string;
  timestamp: string;
}

/**
 * Настройки UI пользователя
 */
export interface UiPreferences {
  viewMode?: 'list' | 'tiles';
  windowWidth?: number;
  windowHeight?: number;
  sidebarCollapsed?: boolean;
  theme?: 'dark' | 'light';
  currentPage?: string;
  selectedSiteId?: number | null;
}

/**
 * Модель состояния сессии
 *
 * Сохраняет состояние приложения для восстановления при следующем запуске.
 * Использует singleton pattern (только одна запись с id=1).
 */
export interface SessionState {
  fileOrder: number[]; // file_ids в порядке отображения
  uiPreferences: UiPreferences;
  openCollections: number[]; // collection_ids
  selectedFiles: number[]; // file_ids
  recentActions: RecentAction[]; // Максимум 50 записей
}

/**
 * Результат восстановления сессии
 */
export interface SessionRestoreResult {
  restored: boolean;
  fileOrder: number[];
  uiPreferences: UiPreferences;
  openCollections: number[];
  selectedFiles: number[];
  warnings: string[]; // предупреждения о недоступных файлах/коллекциях
  currentPage?: string;
  selectedSiteId?: number | null;
}












