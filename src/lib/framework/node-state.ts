/**
 * Утилиты для управления состоянием нод
 *
 * Централизованная логика синхронизации состояния между компонентами
 */

export interface NodeStateManager<T = any> {
  getValue(key: string): T;
  setValue(key: string, value: T): void;
  subscribe(key: string, callback: (value: T) => void): () => void;
}

/**
 * Создает менеджер состояния для ноды
 */
export function createNodeStateManager(data: Record<string, any>): NodeStateManager {
  const listeners = new Map<string, Set<(value: any) => void>>();

  return {
    getValue(key: string) {
      return data[key];
    },

    setValue(key: string, value: any) {
      const oldValue = data[key];
      if (oldValue !== value) {
        data[key] = value;
        // Уведомляем подписчиков
        const callbacks = listeners.get(key);
        if (callbacks) {
          callbacks.forEach(cb => cb(value));
        }
      }
    },

    subscribe(key: string, callback: (value: any) => void) {
      if (!listeners.has(key)) {
        listeners.set(key, new Set());
      }
      listeners.get(key)!.add(callback);

      // Возвращаем функцию отписки
      return () => {
        const callbacks = listeners.get(key);
        if (callbacks) {
          callbacks.delete(callback);
        }
      };
    },
  };
}

/**
 * Синхронизирует локальное состояние с данными ноды
 */
export function syncNodeState<T>(
  data: Record<string, any>,
  key: string,
  defaultValue: T
): {
  value: T;
  setValue: (value: T) => void;
} {
  // Используем простую переменную для хранения значения
  let currentValue = data[key] ?? defaultValue;

  return {
    get value() {
      return currentValue;
    },
    setValue(newValue: T) {
      currentValue = newValue;
      data[key] = newValue;
    }
  };
}
