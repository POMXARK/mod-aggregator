/**
 * Типы для пакетных операций с файлами
 */

/**
 * Результат операции для одного файла
 */
export interface FileOperationResult {
  fileId: number;
  success: boolean;
  error?: string;
}

/**
 * Заблокированный файл (не может быть удален из-за зависимостей)
 */
export interface BlockedFile {
  fileId: number;
  fileName: string;
  dependentFiles: number[];
}

/**
 * Результат пакетной операции
 */
export interface BatchOperationResult {
  successCount: number;
  failedCount: number;
  results: FileOperationResult[];
  warnings: string[];
  blockedFiles: BlockedFile[];
}

/**
 * Отсутствующая зависимость
 */
export interface MissingDependency {
  targetFileName: string;
  targetFileVersion?: string;
  dependencyType: string;
}

/**
 * Конфликт версий
 */
export interface VersionConflict {
  targetFileName: string;
  requiredVersion?: string;
  foundVersion: string;
}

/**
 * Результат проверки зависимостей для файла
 */
export interface FileDependencyCheck {
  fileId: number;
  fileName: string;
  missingDependencies: MissingDependency[];
  satisfiedDependenciesCount: number;
  versionConflicts: VersionConflict[];
}

/**
 * Сводка проверки зависимостей
 */
export interface DependencyCheckSummary {
  totalFiles: number;
  filesWithMissingDeps: number;
  filesWithConflicts: number;
  totalMissingDeps: number;
}

/**
 * Результат проверки зависимостей для нескольких файлов
 */
export interface BatchDependencyCheckResult {
  results: FileDependencyCheck[];
  summary: DependencyCheckSummary;
}

/**
 * Затронутая зависимость
 */
export interface AffectedDependency {
  fileId: number;
  dependentFiles: number[];
}

/**
 * Результат валидации пакетной операции
 */
export interface BatchValidationResult {
  valid: boolean;
  canProceed: boolean;
  warnings: string[];
  errors: string[];
  affectedDependencies: AffectedDependency[];
}

