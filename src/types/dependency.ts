/**
 * Тип зависимости между файлами
 */
export type DependencyType = 'required' | 'optional' | 'peer';

/**
 * Модель зависимости файла
 *
 * Представляет зависимость одного файла от другого.
 * Зависимость идентифицируется по имени и опционально версии.
 */
export interface FileDependency {
  id: number;
  sourceFileId: number;
  targetFileName: string;
  targetFileVersion?: string;
  dependencyType: DependencyType;
  createdAt: string;
}

/**
 * Результат проверки зависимостей файла
 */
export interface DependencyCheckResult {
  fileId: number;
  missingDependencies: MissingDependency[];
  satisfiedDependencies: number[]; // dependency IDs
  versionConflicts: VersionConflict[];
}

/**
 * Отсутствующая зависимость
 */
export interface MissingDependency {
  dependencyId: number;
  targetFileName: string;
  targetFileVersion?: string;
  dependencyType: DependencyType;
  availableVersions: string[]; // доступные версии, если есть
}

/**
 * Конфликт версий зависимости
 */
export interface VersionConflict {
  dependencyId: number;
  targetFileName: string;
  requiredVersion?: string;
  availableVersion: string; // версия, которая есть, но не подходит
}

/**
 * Узел графа зависимостей
 */
export interface GraphNode {
  fileId: number;
  name: string;
  version: string;
  hasMissingDependencies: boolean;
}

/**
 * Ребро графа зависимостей
 */
export interface GraphEdge {
  fromFileId: number;
  toFileId: number;
  dependencyId: number;
  dependencyType: DependencyType;
  satisfied: boolean;
}

/**
 * Граф зависимостей
 */
export interface DependencyGraph {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

