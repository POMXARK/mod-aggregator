import { invoke } from '@/lib/tauri-wrapper';
import type {
  SessionState,
  SessionRestoreResult,
  UiPreferences,
  RecentAction,
} from '@/types/session';

// Type declarations for browser APIs are handled by TypeScript

/**
 * Утилита для работы с состоянием сессии
 *
 * Предоставляет функции для сохранения и восстановления состояния приложения.
 */
export class SessionStateManager {
  /**
   * Получить текущее состояние сессии
   */
  static async getSessionState(): Promise<SessionState> {
    try {
      return await invoke<SessionState>('get_session_state');
    } catch (e: any) {
      console.error('Failed to get session state:', e);
      // Возвращаем состояние по умолчанию
      return {
        fileOrder: [],
        uiPreferences: {},
        openCollections: [],
        selectedFiles: [],
        recentActions: [],
      };
    }
  }

  /**
   * Обновить порядок файлов
   */
  static async updateFileOrder(fileOrder: number[]): Promise<void> {
    try {
      await invoke('update_file_order', { file_order: fileOrder });
    } catch (e: any) {
      console.error('Failed to update file order:', e);
      throw new Error(
        `Не удалось обновить порядок файлов: ${e?.toString() ?? 'Неизвестная ошибка'}`
      );
    }
  }

  /**
   * Обновить настройки UI
   */
  static async updateUiPreferences(preferences: Partial<UiPreferences>): Promise<void> {
    try {
      await invoke('update_ui_preferences', { preferences });
    } catch (e: any) {
      console.error('Failed to update UI preferences:', e);
      throw new Error(`Не удалось обновить настройки UI: ${e?.toString() ?? 'Неизвестная ошибка'}`);
    }
  }

  /**
   * Обновить открытые коллекции
   */
  static async updateOpenCollections(collectionIds: number[]): Promise<void> {
    try {
      await invoke('update_open_collections', { collection_ids: collectionIds });
    } catch (e: any) {
      console.error('Failed to update open collections:', e);
      throw new Error(
        `Не удалось обновить открытые коллекции: ${e?.toString() ?? 'Неизвестная ошибка'}`
      );
    }
  }

  /**
   * Обновить выбранные файлы
   */
  static async updateSelectedFiles(fileIds: number[]): Promise<void> {
    try {
      await invoke('update_selected_files', { file_ids: fileIds });
    } catch (e: any) {
      console.error('Failed to update selected files:', e);
      throw new Error(
        `Не удалось обновить выбранные файлы: ${e?.toString() ?? 'Неизвестная ошибка'}`
      );
    }
  }

  /**
   * Добавить действие в историю
   */
  static async addRecentAction(
    type: RecentAction['type'],
    targetId: number,
    targetName: string
  ): Promise<void> {
    try {
      await invoke('add_recent_action', {
        action_type: type,
        target_id: targetId,
        target_name: targetName,
      });
    } catch (e: any) {
      console.error('Failed to add recent action:', e);
      // Не бросаем ошибку, так как это не критично
    }
  }

  /**
   * Получить последние действия
   */
  static async getRecentActions(limit?: number): Promise<RecentAction[]> {
    try {
      return await invoke<RecentAction[]>('get_recent_actions', { limit });
    } catch (e: any) {
      console.error('Failed to get recent actions:', e);
      return [];
    }
  }

  /**
   * Очистить историю действий
   */
  static async clearRecentActions(): Promise<void> {
    try {
      await invoke('clear_recent_actions');
    } catch (e: any) {
      console.error('Failed to clear recent actions:', e);
      throw new Error(`Не удалось очистить историю: ${e?.toString() ?? 'Неизвестная ошибка'}`);
    }
  }

  /**
   * Восстановить сессию при запуске
   */
  static async restoreSession(): Promise<SessionRestoreResult> {
    try {
      return await invoke<SessionRestoreResult>('restore_session');
    } catch (e: any) {
      console.error('Failed to restore session:', e);
      // Возвращаем пустой результат
      return {
        restored: false,
        fileOrder: [],
        uiPreferences: {},
        openCollections: [],
        selectedFiles: [],
        warnings: [`Не удалось восстановить сессию: ${e?.toString() ?? 'Неизвестная ошибка'}`],
      };
    }
  }

  /**
   * Сбросить состояние сессии
   */
  static async resetSessionState(): Promise<void> {
    try {
      await invoke('reset_session_state');
    } catch (e: any) {
      console.error('Failed to reset session state:', e);
      throw new Error(`Не удалось сбросить состояние: ${e?.toString() ?? 'Неизвестная ошибка'}`);
    }
  }

  /**
   * Сохранить порядок файлов с debounce
   */
  static createDebouncedFileOrderSaver(delay: number = 500): (fileOrder: number[]) => void {
    let timeoutId: ReturnType<typeof setTimeout> | null = null;

    return (fileOrder: number[]) => {
      if (timeoutId) {
        clearTimeout(timeoutId);
      }

      timeoutId = setTimeout(async () => {
        try {
          await this.updateFileOrder(fileOrder);
        } catch (e) {
          console.error('Failed to save file order:', e);
        }
      }, delay);
    };
  }

  /**
   * Сохранить настройки UI с debounce
   */
  static createDebouncedPreferencesSaver(
    delay: number = 500
  ): (preferences: Partial<UiPreferences>) => void {
    let timeoutId: ReturnType<typeof setTimeout> | null = null;
    let currentPreferences: Partial<UiPreferences> = {};

    return (preferences: Partial<UiPreferences>) => {
      currentPreferences = { ...currentPreferences, ...preferences };

      if (timeoutId) {
        clearTimeout(timeoutId);
      }

      timeoutId = setTimeout(async () => {
        try {
          await this.updateUiPreferences(currentPreferences);
          currentPreferences = {};
        } catch (e) {
          console.error('Failed to save UI preferences:', e);
        }
      }, delay);
    };
  }

  /**
   * Получить человекочитаемое описание действия
   */
  static getActionDescription(action: RecentAction): string {
    const actionNames: Record<RecentAction['type'], string> = {
      file_added: 'Добавлен файл',
      file_deleted: 'Удален файл',
      collection_created: 'Создана коллекция',
      dependency_added: 'Добавлена зависимость',
      dependency_removed: 'Удалена зависимость',
      collection_updated: 'Обновлена коллекция',
    };

    const actionName = actionNames[action.type] || action.type;
    const date = new Date(action.timestamp).toLocaleString('ru-RU');

    return `${actionName}: ${action.targetName} (${date})`;
  }
}
