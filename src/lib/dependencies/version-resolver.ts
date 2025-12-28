import type { FileDependency } from '@/types/file';

/**
 * Утилита для разрешения версий зависимостей на frontend
 */
export class VersionResolver {
  /**
   * Получить доступные версии файла
   */
  static getAvailableVersions(files: File[], fileName: string): string[] {
    return files
      .filter(file => file.name === fileName)
      .map(file => file.version)
      .sort((a, b) => {
        // Простое сравнение версий (можно улучшить с помощью semver)
        return a.localeCompare(b, undefined, { numeric: true });
      });
  }

  /**
   * Проверить, удовлетворяет ли файл требованию зависимости
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
   * Получить последнюю версию файла
   */
  static getLatestVersion(files: File[], fileName: string): string | null {
    const versions = this.getAvailableVersions(files, fileName);
    return versions.length > 0 ? versions[versions.length - 1] : null;
  }

  /**
   * Найти подходящую версию для зависимости
   *
   * Если версия указана, возвращает её, если доступна.
   * Если версия не указана, возвращает последнюю доступную версию.
   */
  static resolveVersion(
    files: File[],
    dependency: FileDependency
  ): { version: string | null; exact: boolean } {
    const availableVersions = this.getAvailableVersions(files, dependency.targetFileName);

    if (availableVersions.length === 0) {
      return { version: null, exact: false };
    }

    if (dependency.targetFileVersion !== undefined) {
      // Ищем точное совпадение
      if (availableVersions.includes(dependency.targetFileVersion)) {
        return { version: dependency.targetFileVersion, exact: true };
      }
      // Возвращаем последнюю доступную версию
      return { version: availableVersions[availableVersions.length - 1], exact: false };
    }

    // Версия не указана, возвращаем последнюю
    return { version: availableVersions[availableVersions.length - 1], exact: false };
  }

  /**
   * Проверить совместимость версий
   *
   * Упрощенная проверка - можно улучшить с помощью semver
   */
  static isVersionCompatible(
    requiredVersion: string | undefined,
    availableVersion: string
  ): boolean {
    if (requiredVersion === undefined) {
      return true; // Любая версия подходит
    }

    return requiredVersion === availableVersion;
  }

  /**
   * Получить рекомендацию по версии для зависимости
   */
  static getVersionRecommendation(
    files: File[],
    dependency: FileDependency
  ): {
    recommended: string | null;
    available: string[];
    canSatisfy: boolean;
  } {
    const available = this.getAvailableVersions(files, dependency.targetFileName);
    const resolved = this.resolveVersion(files, dependency);

    return {
      recommended: resolved.version,
      available,
      canSatisfy: resolved.version !== null,
    };
  }
}
