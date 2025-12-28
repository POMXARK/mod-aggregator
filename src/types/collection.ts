import type { File } from './file';

/**
 * Модель коллекции файлов
 *
 * Коллекция представляет группу файлов с опциональной логикой включения/выключения.
 */
export interface Collection {
  id: number;
  name: string;
  description?: string;
  createdAt: string;
  updatedAt: string;
}

/**
 * Тип условия для логики коллекции
 */
export type ConditionType = 'boolean' | 'collection_check' | 'file_check' | 'and' | 'or';

/**
 * Действие для правила логики
 */
export type Action = 'enable' | 'disable';

/**
 * Модель правила логики коллекции
 *
 * Представляет условие для включения/выключения файла в коллекции.
 * Правила настраиваются через UI-конструктор с предопределенными условиями.
 */
export interface CollectionLogicRule {
  id: number;
  collectionId: number;
  name: string;
  conditionType: ConditionType;
  /**
   * Параметры условия в формате JSON
   * - Для boolean: {"value": true/false}
   * - Для collection_check: {"collection_id": 123}
   * - Для file_check: {"file_name": "mod", "file_version": "1.0"}
   * - Для and/or: {"rules": [rule_id1, rule_id2]}
   */
  conditionParams: Record<string, unknown>;
  action: Action;
  createdAt: string;
}

/**
 * Файл в коллекции с его логикой
 */
export interface CollectionFile {
  id: number;
  collectionId: number;
  fileId: number;
  file: File;
  logicRuleId?: number;
  logicRule?: CollectionLogicRule;
  orderIndex: number;
  createdAt: string;
}

/**
 * Результат вычисления логики коллекции
 */
export interface CollectionEvaluationResult {
  collectionId: number;
  enabledFiles: number[]; // file_ids
  disabledFiles: number[]; // file_ids
  evaluationDetails: FileEvaluationDetail[];
}

/**
 * Детали вычисления для файла
 */
export interface FileEvaluationDetail {
  fileId: number;
  enabled: boolean;
  appliedRules: number[]; // rule_ids, которые повлияли на решение
}

/**
 * Представление файла из нескольких коллекций
 */
export interface CollectionFileView {
  fileId: number;
  file: File;
  collections: Array<{
    collectionId: number;
    collectionName: string;
    orderIndex: number;
    logicRuleId?: number;
  }>;
}

