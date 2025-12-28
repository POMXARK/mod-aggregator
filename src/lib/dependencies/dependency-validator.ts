import type { FileDependency } from '@/types/file';
import type { DependencyCheckResult } from '@/types/dependency';

/**
 * Утилита для валидации зависимостей на frontend
 */
export class DependencyValidator {
  /**
   * Проверить, является ли зависимость самозависимостью
   */
  static isSelfDependency(
    sourceFile: File,
    targetFileName: string,
    targetFileVersion?: string
  ): boolean {
    if (sourceFile.name !== targetFileName) {
      return false;
    }

    if (targetFileVersion !== undefined) {
      return sourceFile.version === targetFileVersion;
    }

    return true; // Если версия не указана, считаем самозависимостью
  }

  /**
   * Валидировать параметры зависимости перед добавлением
   */
  static validateDependencyParams(
    sourceFile: File,
    targetFileName: string,
    targetFileVersion?: string
  ): { valid: boolean; error?: string } {
    // Проверка самозависимости
    if (this.isSelfDependency(sourceFile, targetFileName, targetFileVersion)) {
      return {
        valid: false,
        error: 'Self-dependency not allowed',
      };
    }

    // Проверка пустых значений
    if (!targetFileName.trim()) {
      return {
        valid: false,
        error: 'Target file name is required',
      };
    }

    return { valid: true };
  }

  /**
   * Проверить, удовлетворяет ли файл зависимости
   */
  static satisfiesDependency(file: File, dependency: FileDependency): boolean {
    if (file.name !== dependency.targetFileName) {
      return false;
    }

    if (dependency.targetFileVersion !== undefined) {
      return file.version === dependency.targetFileVersion;
    }

    return true; // Любая версия подходит, если версия не указана
  }

  /**
   * Найти файлы, удовлетворяющие зависимости
   */
  static findSatisfyingFiles(files: File[], dependency: FileDependency): File[] {
    return files.filter(file => this.satisfiesDependency(file, dependency));
  }

  /**
   * Проверить, разрешена ли зависимость
   */
  static isDependencySatisfied(files: File[], dependency: FileDependency): boolean {
    return this.findSatisfyingFiles(files, dependency).length > 0;
  }

  /**
   * Получить доступные версии файла
   */
  static getAvailableVersions(files: File[], fileName: string): string[] {
    return files
      .filter(file => file.name === fileName)
      .map(file => file.version)
      .sort();
  }

  /**
   * Анализировать результат проверки зависимостей
   */
  static analyzeCheckResult(
    result: DependencyCheckResult,
    _allFiles: File[]
  ): {
    canInstall: boolean;
    warnings: string[];
    errors: string[];
    recommendations: string[];
  } {
    const warnings: string[] = [];
    const errors: string[] = [];
    const recommendations: string[] = [];

    // Анализ отсутствующих зависимостей
    for (const missing of result.missingDependencies) {
      if (missing.dependencyType === 'required') {
        errors.push(
          `Missing required dependency: ${missing.targetFileName}${missing.targetFileVersion ? `@${missing.targetFileVersion}` : ''}`
        );
      } else {
        warnings.push(
          `Missing optional dependency: ${missing.targetFileName}${missing.targetFileVersion ? `@${missing.targetFileVersion}` : ''}`
        );
      }

      if (missing.availableVersions.length > 0) {
        recommendations.push(
          `Available versions of ${missing.targetFileName}: ${missing.availableVersions.join(', ')}`
        );
      }
    }

    // Анализ конфликтов версий
    for (const conflict of result.versionConflicts) {
      errors.push(
        `Version conflict: ${conflict.targetFileName} requires ${conflict.requiredVersion ?? 'any version'}, but ${conflict.availableVersion} is available`
      );
      recommendations.push(
        `Consider updating dependency to version ${conflict.availableVersion} or install required version`
      );
    }

    const canInstall = errors.length === 0;

    return {
      canInstall,
      warnings,
      errors,
      recommendations,
    };
  }
}
