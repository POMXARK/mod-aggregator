import { persisted } from 'svelte-persisted-store';
import { get, writable, type Writable } from 'svelte/store';

/**
 * Универсальная система управления состоянием с использованием svelte-persisted-store
 */
export class StatePersistenceManager {
  private static instance: StatePersistenceManager;
  private debounceMs = 1000;

  private constructor() {}

  static getInstance(): StatePersistenceManager {
    if (!StatePersistenceManager.instance) {
      StatePersistenceManager.instance = new StatePersistenceManager();
    }
    return StatePersistenceManager.instance;
  }

  /**
   * Создает реактивное состояние с автоматическим сохранением
   */
  createPersistentState<T>(
    key: string,
    defaultValue: T,
    options: {
      debounceMs?: number;
      serialize?: (value: T) => string;
      deserialize?: (value: string) => T;
      version?: number;
    } = {}
  ): Writable<T> & {
    reset: () => void;
    save: () => void;
    load: () => void;
  } {
    const {
      debounceMs = this.debounceMs,
      serialize = JSON.stringify,
      deserialize = JSON.parse,
      version = 1
    } = options;

    // Создаем persisted store с svelte-persisted-store
    const store = persisted(key, defaultValue, {
      serializer: {
        parse: (value: string) => {
          try {
            const data = deserialize(value);
            // Проверяем версию если есть
            if (data.version !== undefined && data.version !== version) {
              console.log(`State version mismatch for key "${key}", using default`);
              return defaultValue;
            }
            return data.value !== undefined ? data.value : data;
          } catch (error) {
            console.warn(`Failed to deserialize state for key "${key}":`, error);
            return defaultValue;
          }
        },
        stringify: (value: T) => {
          const data = {
            value,
            timestamp: Date.now(),
            version
          };
          return serialize(data);
        }
      }
    });

    // Расширяем store дополнительными методами для совместимости
    const extendedStore = store as Writable<T> & {
      reset: () => void;
      save: () => void;
      load: () => void;
    };

    extendedStore.reset = () => {
      store.set(defaultValue);
      if (typeof window !== 'undefined') {
        try {
          localStorage.removeItem(key);
        } catch (error) {
          console.warn(`Failed to remove state for key "${key}":`, error);
        }
      }
    };

    extendedStore.save = () => {
      // svelte-persisted-store сохраняет автоматически, но можем форсировать
      store.update(current => current);
    };

    extendedStore.load = () => {
      // svelte-persisted-store загружает автоматически при создании
      // Можно принудительно перезагрузить значение
      const stored = typeof window !== 'undefined' ? localStorage.getItem(key) : null;
      if (stored) {
        try {
          const data = deserialize(stored);
          if (data.version !== undefined && data.version !== version) {
            console.log(`State version mismatch for key "${key}", using default`);
            store.set(defaultValue);
            return;
          }
          store.set(data.value !== undefined ? data.value : data);
        } catch (error) {
          console.warn(`Failed to load state for key "${key}":`, error);
          store.set(defaultValue);
        }
      } else {
        store.set(defaultValue);
      }
    };

    return extendedStore;
  }

  /**
   * Создает группу связанных состояний с общим ключом
   */
  createPersistentStateGroup<T extends Record<string, any>>(
    baseKey: string,
    defaultState: T,
    options: {
      debounceMs?: number;
      version?: number;
    } = {}
  ): {
    stores: { [K in keyof T]: Writable<T[K]> & { reset: () => void; save: () => void; load: () => void } };
    saveAll: () => void;
    loadAll: () => void;
    resetAll: () => void;
    getState: () => T;
    setState: (state: Partial<T>) => void;
  } {
    const { debounceMs = this.debounceMs, version = 1 } = options;

    // Создаем stores для каждого поля
    const stores = {} as any;

    for (const [key, defaultValue] of Object.entries(defaultState)) {
      const store = this.createPersistentState(
        `${baseKey}.${key}`,
        defaultValue,
        { debounceMs, version }
      );
      stores[key] = store;
    }

    const saveAll = () => {
      for (const key of Object.keys(stores)) {
        stores[key].save();
      }
    };

    const loadAll = () => {
      for (const key of Object.keys(stores)) {
        stores[key].load();
      }
    };

    const resetAll = () => {
      for (const key of Object.keys(stores)) {
        stores[key].reset();
      }
    };

    const getState = (): T => {
      const state = {} as T;
      for (const [key, store] of Object.entries(stores)) {
        state[key as keyof T] = get(store);
      }
      return state;
    };

    const setState = (partialState: Partial<T>) => {
      for (const [key, value] of Object.entries(partialState)) {
        if (key in stores) {
          stores[key].set(value);
        }
      }
    };

    return {
      stores,
      saveAll,
      loadAll,
      resetAll,
      getState,
      setState
    };
  }

  /**
   * Получает все ключи состояний из localStorage
   */
  getAllStoredKeys(): string[] {
    if (typeof window === 'undefined') return [];

    const keys: string[] = [];
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key && (key.includes('state') || key.includes('persisted'))) {
        keys.push(key);
      }
    }
    return keys;
  }

  /**
   * Экспортирует все состояния в JSON
   */
  exportAllStates(): string {
    if (typeof window === 'undefined') return '{}';

    const exportData: Record<string, any> = {};

    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key && (key.includes('state') || key.includes('persisted'))) {
        try {
          const value = localStorage.getItem(key);
          if (value) {
            exportData[key] = JSON.parse(value);
          }
        } catch (error) {
          console.warn(`Failed to export state for key "${key}":`, error);
        }
      }
    }

    return JSON.stringify(exportData, null, 2);
  }

  /**
   * Импортирует состояния из JSON
   */
  importStates(jsonString: string): boolean {
    if (typeof window === 'undefined') return false;

    try {
      const importData = JSON.parse(jsonString);

      for (const [key, data] of Object.entries(importData)) {
        try {
          localStorage.setItem(key, JSON.stringify(data));
        } catch (error) {
          console.warn(`Failed to import state for key "${key}":`, error);
        }
      }

      return true;
    } catch (error) {
      console.error('Failed to import states:', error);
      return false;
    }
  }
}

// Экспортируем singleton instance
export const statePersistence = StatePersistenceManager.getInstance();

/**
 * Composable для создания персистентного состояния
 */
export function usePersistentState<T>(
  key: string,
  defaultValue: T,
  options: {
    debounceMs?: number;
    serialize?: (value: T) => string;
    deserialize?: (value: string) => T;
    version?: number;
  } = {}
) {
  return statePersistence.createPersistentState(key, defaultValue, options);
}

/**
 * Composable для создания группы связанных персистентных состояний
 */
export function usePersistentStateGroup<T extends Record<string, any>>(
  baseKey: string,
  defaultState: T,
  options: {
    debounceMs?: number;
    version?: number;
  } = {}
) {
  return statePersistence.createPersistentStateGroup(baseKey, defaultState, options);
}

/**
 * Типы для работы с персистентными состояниями
 */
export type PersistentStore<T> = Writable<T> & {
  reset: () => void;
  save: () => void;
  load: () => void;
};

export type PersistentStateGroup<T extends Record<string, any>> = {
  stores: { [K in keyof T]: PersistentStore<T[K]> };
  saveAll: () => void;
  loadAll: () => void;
  resetAll: () => void;
  getState: () => T;
  setState: (state: Partial<T>) => void;
};