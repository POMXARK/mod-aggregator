import type {
  CollectionLogicRule,
  ConditionType,
  Action,
  CollectionEvaluationResult,
} from '../../types/collection.js';

/**
 * Утилита для работы с логикой коллекций на frontend
 *
 * Предоставляет функции для валидации, построения и работы с правилами логики коллекций.
 */
export class CollectionLogic {
  /**
   * Валидировать параметры условия
   *
   * Проверяет, что параметры условия соответствуют ожидаемому формату для данного типа условия.
   *
   * @param conditionType - Тип условия
   * @param conditionParams - Параметры условия
   * @returns true если параметры валидны, иначе false
   */
  static validateConditionParams(
    conditionType: ConditionType,
    conditionParams: Record<string, unknown>
  ): boolean {
    switch (conditionType) {
      case 'boolean':
        return typeof conditionParams.value === 'boolean';

      case 'collection_check':
        return (
          typeof conditionParams.collection_id === 'number' && conditionParams.collection_id > 0
        );

      case 'file_check':
        return (
          typeof conditionParams.file_name === 'string' &&
          conditionParams.file_name.length > 0 &&
          typeof conditionParams.file_version === 'string' &&
          conditionParams.file_version.length > 0
        );

      case 'and':
      case 'or':
        return (
          Array.isArray(conditionParams.rules) &&
          conditionParams.rules.length > 0 &&
          conditionParams.rules.every((ruleId: unknown) => typeof ruleId === 'number')
        );

      default:
        return false;
    }
  }

  /**
   * Создать правило с boolean условием
   *
   * @param collectionId - ID коллекции
   * @param name - Название правила
   * @param value - Boolean значение
   * @param action - Действие (enable/disable)
   * @returns Правило логики
   */
  static createBooleanRule(
    collectionId: number,
    name: string,
    value: boolean,
    action: Action = 'enable'
  ): Omit<CollectionLogicRule, 'id' | 'createdAt'> {
    return {
      collectionId,
      name,
      conditionType: 'boolean',
      conditionParams: { value },
      action,
    };
  }

  /**
   * Создать правило с проверкой коллекции
   *
   * @param collectionId - ID коллекции
   * @param name - Название правила
   * @param targetCollectionId - ID проверяемой коллекции
   * @param action - Действие (enable/disable)
   * @returns Правило логики
   */
  static createCollectionCheckRule(
    collectionId: number,
    name: string,
    targetCollectionId: number,
    action: Action = 'enable'
  ): Omit<CollectionLogicRule, 'id' | 'createdAt'> {
    return {
      collectionId,
      name,
      conditionType: 'collection_check',
      conditionParams: { collection_id: targetCollectionId },
      action,
    };
  }

  /**
   * Создать правило с проверкой файла
   *
   * @param collectionId - ID коллекции
   * @param name - Название правила
   * @param fileName - Имя файла
   * @param fileVersion - Версия файла
   * @param action - Действие (enable/disable)
   * @returns Правило логики
   */
  static createFileCheckRule(
    collectionId: number,
    name: string,
    fileName: string,
    fileVersion: string,
    action: Action = 'enable'
  ): Omit<CollectionLogicRule, 'id' | 'createdAt'> {
    return {
      collectionId,
      name,
      conditionType: 'file_check',
      conditionParams: {
        file_name: fileName,
        file_version: fileVersion,
      },
      action,
    };
  }

  /**
   * Создать правило AND (логическое И)
   *
   * @param collectionId - ID коллекции
   * @param name - Название правила
   * @param ruleIds - Массив ID правил для комбинации
   * @param action - Действие (enable/disable)
   * @returns Правило логики
   */
  static createAndRule(
    collectionId: number,
    name: string,
    ruleIds: number[],
    action: Action = 'enable'
  ): Omit<CollectionLogicRule, 'id' | 'createdAt'> {
    return {
      collectionId,
      name,
      conditionType: 'and',
      conditionParams: { rules: ruleIds },
      action,
    };
  }

  /**
   * Создать правило OR (логическое ИЛИ)
   *
   * @param collectionId - ID коллекции
   * @param name - Название правила
   * @param ruleIds - Массив ID правил для комбинации
   * @param action - Действие (enable/disable)
   * @returns Правило логики
   */
  static createOrRule(
    collectionId: number,
    name: string,
    ruleIds: number[],
    action: Action = 'enable'
  ): Omit<CollectionLogicRule, 'id' | 'createdAt'> {
    return {
      collectionId,
      name,
      conditionType: 'or',
      conditionParams: { rules: ruleIds },
      action,
    };
  }

  /**
   * Получить человекочитаемое описание правила
   *
   * @param rule - Правило логики
   * @returns Описание правила
   */
  static getRuleDescription(rule: CollectionLogicRule): string {
    const actionText = rule.action === 'enable' ? 'включить' : 'выключить';

    switch (rule.conditionType) {
      case 'boolean': {
        const value = rule.conditionParams.value as boolean;
        return `${actionText} если ${value ? 'включено' : 'выключено'}`;
      }

      case 'collection_check': {
        const collectionId = rule.conditionParams.collection_id as number;
        return `${actionText} если коллекция #${collectionId} активна`;
      }

      case 'file_check': {
        const fileName = rule.conditionParams.file_name as string;
        const fileVersion = rule.conditionParams.file_version as string;
        return `${actionText} если файл ${fileName}@${fileVersion} установлен`;
      }

      case 'and': {
        const ruleIds = rule.conditionParams.rules as number[];
        return `${actionText} если все правила [${ruleIds.join(', ')}] выполнены`;
      }

      case 'or': {
        const ruleIds = rule.conditionParams.rules as number[];
        return `${actionText} если хотя бы одно правило [${ruleIds.join(', ')}] выполнено`;
      }

      default:
        return `Неизвестное правило: ${rule.name}`;
    }
  }

  /**
   * Проверить, может ли правило быть удалено
   *
   * Правило не может быть удалено, если на него ссылаются другие правила (в and/or условиях).
   *
   * @param ruleId - ID правила для проверки
   * @param allRules - Все правила коллекции
   * @returns true если правило может быть удалено
   */
  static canDeleteRule(ruleId: number, allRules: CollectionLogicRule[]): boolean {
    // Проверяем, не используется ли это правило в других правилах
    for (const rule of allRules) {
      if (rule.id === ruleId) {
        continue;
      }

      if (rule.conditionType === 'and' || rule.conditionType === 'or') {
        const ruleIds = rule.conditionParams.rules as number[];
        if (ruleIds.includes(ruleId)) {
          return false; // Правило используется в другом правиле
        }
      }
    }

    return true;
  }

  /**
   * Получить все правила, которые зависят от указанного правила
   *
   * @param ruleId - ID правила
   * @param allRules - Все правила коллекции
   * @returns Массив ID правил, которые зависят от указанного
   */
  static getDependentRules(ruleId: number, allRules: CollectionLogicRule[]): number[] {
    const dependentIds: number[] = [];

    for (const rule of allRules) {
      if (rule.id === ruleId) {
        continue;
      }

      if (rule.conditionType === 'and' || rule.conditionType === 'or') {
        const ruleIds = rule.conditionParams.rules as number[];
        if (ruleIds.includes(ruleId)) {
          dependentIds.push(rule.id);
        }
      }
    }

    return dependentIds;
  }

  /**
   * Валидировать все правила коллекции
   *
   * Проверяет валидность всех правил и их взаимосвязей.
   *
   * @param rules - Массив правил для валидации
   * @returns Объект с результатами валидации
   */
  static validateRules(rules: CollectionLogicRule[]): {
    valid: boolean;
    errors: string[];
  } {
    const errors: string[] = [];
    const ruleIdSet = new Set(rules.map(r => r.id));

    for (const rule of rules) {
      // Проверяем валидность параметров
      if (!this.validateConditionParams(rule.conditionType, rule.conditionParams)) {
        errors.push(`Правило "${rule.name}" (ID: ${rule.id}) имеет невалидные параметры`);
      }

      // Проверяем ссылки на другие правила в and/or условиях
      if (rule.conditionType === 'and' || rule.conditionType === 'or') {
        const ruleIds = rule.conditionParams.rules as number[];
        for (const refRuleId of ruleIds) {
          if (!ruleIdSet.has(refRuleId)) {
            errors.push(
              `Правило "${rule.name}" (ID: ${rule.id}) ссылается на несуществующее правило ${refRuleId}`
            );
          }
        }

        // Проверяем на циклические зависимости
        if (ruleIds.includes(rule.id)) {
          errors.push(`Правило "${rule.name}" (ID: ${rule.id}) ссылается само на себя`);
        }
      }
    }

    return {
      valid: errors.length === 0,
      errors,
    };
  }

  /**
   * Получить статистику по результату вычисления
   *
   * @param result - Результат вычисления логики коллекции
   * @returns Статистика
   */
  static getEvaluationStats(result: CollectionEvaluationResult): {
    totalFiles: number;
    enabledCount: number;
    disabledCount: number;
    enabledPercentage: number;
  } {
    const totalFiles = result.enabledFiles.length + result.disabledFiles.length;
    const enabledCount = result.enabledFiles.length;
    const disabledCount = result.disabledFiles.length;
    const enabledPercentage = totalFiles > 0 ? Math.round((enabledCount / totalFiles) * 100) : 0;

    return {
      totalFiles,
      enabledCount,
      disabledCount,
      enabledPercentage,
    };
  }
}
