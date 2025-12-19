/**
 * Главный экспорт фреймворка конструктора
 * 
 * Единая точка входа для всех компонентов и утилит фреймворка
 */

// Конфигурация
export * from './node-config';
export * from './node-registry';

// Фабрика
export * from './node-factory';

// Типы полей
export * from './field-types';

// Управление состоянием
export * from './node-state';

// Инициализация фреймворка
import { registerStandardNodeTypes } from './node-registry';

/**
 * Инициализирует фреймворк конструктора
 * 
 * Регистрирует все стандартные типы нод
 */
export function initParserFramework(): void {
  registerStandardNodeTypes();
}

