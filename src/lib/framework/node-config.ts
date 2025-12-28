/**
 * Конфигурация нод
 *
 * Централизованное определение всех типов нод и их полей
 */

import type { FieldConfig } from './field-types';

export interface NodeTypeConfig {
  /** Тип ноды */
  type: string;
  /** Лейбл ноды */
  label: string;
  /** Описание ноды */
  description?: string;
  /** Цвет ноды */
  color: string;
  /** Иконка ноды (опционально) */
  icon?: string;
  /** Конфигурация полей */
  fields: FieldConfig[];
  /** Начальные данные */
  defaultData?: Record<string, any>;
  /** Позиция handles */
  handles?: {
    source?: boolean;
    target?: boolean;
  };
}

/**
 * Реестр всех конфигураций нод
 */
export const nodeConfigRegistry = new Map<string, NodeTypeConfig>();

/**
 * Регистрирует конфигурацию ноды
 */
export function registerNodeType(config: NodeTypeConfig): void {
  nodeConfigRegistry.set(config.type, config);
}

/**
 * Получает конфигурацию ноды
 */
export function getNodeConfig(type: string): NodeTypeConfig | undefined {
  return nodeConfigRegistry.get(type);
}

/**
 * Создает начальные данные для ноды
 */
export function createNodeData(
  type: string,
  customData?: Record<string, any>
): Record<string, any> {
  const config = getNodeConfig(type);
  if (!config) {
    console.warn(`Unknown node type: ${type}`);
    return { label: type, ...customData };
  }

  const data: Record<string, any> = {
    nodeType: type,
    label: config.label,
    ...config.defaultData,
    ...customData,
  };

  // Устанавливаем значения по умолчанию для всех полей
  config.fields.forEach(field => {
    if (field.defaultValue !== undefined && data[field.key] === undefined) {
      data[field.key] = field.defaultValue;
    }
  });

  return data;
}

/**
 * Получает список всех зарегистрированных типов нод
 */
export function getRegisteredNodeTypes(): string[] {
  return Array.from(nodeConfigRegistry.keys());
}

