/**
 * Фабрика для создания нод
 * 
 * Упрощает создание нод с использованием конфигурации
 */

import type { Node } from '@xyflow/svelte';
import { getNodeConfig, createNodeData } from './node-config';

export interface CreateNodeOptions {
  /** Позиция ноды */
  position?: { x: number; y: number };
  /** Кастомные данные */
  data?: Record<string, any>;
  /** ID ноды (если не указан, генерируется автоматически) */
  id?: string;
}

/**
 * Создает новую ноду на основе типа
 */
export function createNode(
  type: string,
  options: CreateNodeOptions = {}
): Node | null {
  const config = getNodeConfig(type);
  if (!config) {
    console.error(`Unknown node type: ${type}`);
    return null;
  }

  const nodeId = options.id || `node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  const position = options.position || { 
    x: Math.random() * 400 + 100, 
    y: Math.random() * 400 + 100 
  };

  const data = createNodeData(type, options.data);

  return {
    id: nodeId,
    type: 'universal', // Используем универсальный компонент
    position,
    data,
  };
}

/**
 * Создает ноду из сохраненной конфигурации
 */
export function createNodeFromConfig(config: {
  id?: string;
  type: string;
  position: { x: number; y: number };
  data: Record<string, any>;
}): Node {
  const nodeId = config.id || `node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  
  return {
    id: nodeId,
    type: 'universal',
    position: config.position,
    data: {
      nodeType: config.type,
      ...config.data,
    },
  };
}

/**
 * Клонирует ноду
 */
export function cloneNode(node: Node, offset: { x: number; y: number } = { x: 20, y: 20 }): Node {
  const newId = `node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  
  return {
    ...node,
    id: newId,
    position: {
      x: node.position.x + offset.x,
      y: node.position.y + offset.y,
    },
    data: {
      ...node.data,
      label: `${node.data.label} (копия)`,
    },
  };
}

