/**
 * Фабрика для создания нод с использованием конфигурации
 * 
 * Уменьшает бойлерплейт код при создании новых нод
 */

import type { Node } from '@xyflow/svelte';
import { getNodeConfig, createNodeData } from './node-configs';

/**
 * Создать новую ноду на основе типа
 */
export function createNode(
  type: string,
  position: { x: number; y: number },
  customData?: Record<string, any>
): Node | null {
  const config = getNodeConfig(type);
  if (!config) {
    console.error(`Unknown node type: ${type}`);
    return null;
  }

  const nodeId = `node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  const defaultData = createNodeData(type);

  return {
    id: nodeId,
    type: 'universal', // Используем универсальный компонент
    position,
    data: {
      nodeType: type, // Сохраняем тип для конфигурации
      label: config.label,
      ...defaultData,
      ...customData, // Позволяем переопределить данные
    },
  };
}

/**
 * Создать ноду из сохраненной конфигурации
 */
export function createNodeFromConfig(
  nodeConfig: {
    id?: string;
    type: string;
    position: { x: number; y: number };
    data: Record<string, any>;
  }
): Node {
  const nodeId = nodeConfig.id || `node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  
  return {
    id: nodeId,
    type: 'universal',
    position: nodeConfig.position,
    data: {
      nodeType: nodeConfig.type,
      ...nodeConfig.data,
    },
  };
}

