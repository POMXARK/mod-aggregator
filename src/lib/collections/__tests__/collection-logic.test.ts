import { describe, it, expect, beforeEach } from 'vitest';
import { CollectionLogic } from '../collection-logic';
import type { CollectionLogicRule, ConditionType, Action } from '../../../types/collection';

describe('CollectionLogic', () => {
  describe('validateConditionParams', () => {
    it('should validate boolean condition', () => {
      expect(CollectionLogic.validateConditionParams('boolean', { value: true })).toBe(true);
      expect(CollectionLogic.validateConditionParams('boolean', { value: false })).toBe(true);
      expect(CollectionLogic.validateConditionParams('boolean', { value: 'true' })).toBe(false);
      expect(CollectionLogic.validateConditionParams('boolean', {})).toBe(false);
    });

    it('should validate collection_check condition', () => {
      expect(CollectionLogic.validateConditionParams('collection_check', { collection_id: 1 })).toBe(true);
      expect(CollectionLogic.validateConditionParams('collection_check', { collection_id: 0 })).toBe(false);
      expect(CollectionLogic.validateConditionParams('collection_check', { collection_id: -1 })).toBe(false);
      expect(CollectionLogic.validateConditionParams('collection_check', { collection_id: '1' })).toBe(false);
    });

    it('should validate file_check condition', () => {
      expect(CollectionLogic.validateConditionParams('file_check', {
        file_name: 'mod.dll',
        file_version: '1.0.0'
      })).toBe(true);

      expect(CollectionLogic.validateConditionParams('file_check', {
        file_name: '',
        file_version: '1.0.0'
      })).toBe(false);

      expect(CollectionLogic.validateConditionParams('file_check', {
        file_name: 'mod.dll',
        file_version: ''
      })).toBe(false);

      expect(CollectionLogic.validateConditionParams('file_check', {
        file_name: 123,
        file_version: '1.0.0'
      })).toBe(false);
    });

    it('should validate and/or conditions', () => {
      expect(CollectionLogic.validateConditionParams('and', { rules: [1, 2, 3] })).toBe(true);
      expect(CollectionLogic.validateConditionParams('or', { rules: [1] })).toBe(true);
      expect(CollectionLogic.validateConditionParams('and', { rules: [] })).toBe(false);
      expect(CollectionLogic.validateConditionParams('and', { rules: 'not-array' })).toBe(false);
      expect(CollectionLogic.validateConditionParams('and', { rules: [1, '2', 3] })).toBe(false);
    });

    it('should return false for unknown condition types', () => {
      expect(CollectionLogic.validateConditionParams('unknown' as ConditionType, {})).toBe(false);
    });
  });

  describe('createBooleanRule', () => {
    it('should create boolean rule with default action', () => {
      const rule = CollectionLogic.createBooleanRule(1, 'Test Rule', true);

      expect(rule).toEqual({
        collectionId: 1,
        name: 'Test Rule',
        conditionType: 'boolean',
        conditionParams: { value: true },
        action: 'enable'
      });
    });

    it('should create boolean rule with custom action', () => {
      const rule = CollectionLogic.createBooleanRule(1, 'Test Rule', false, 'disable');

      expect(rule).toEqual({
        collectionId: 1,
        name: 'Test Rule',
        conditionType: 'boolean',
        conditionParams: { value: false },
        action: 'disable'
      });
    });
  });

  describe('createCollectionCheckRule', () => {
    it('should create collection check rule', () => {
      const rule = CollectionLogic.createCollectionCheckRule(1, 'Check Collection', 2);

      expect(rule).toEqual({
        collectionId: 1,
        name: 'Check Collection',
        conditionType: 'collection_check',
        conditionParams: { collection_id: 2 },
        action: 'enable'
      });
    });
  });

  describe('createFileCheckRule', () => {
    it('should create file check rule', () => {
      const rule = CollectionLogic.createFileCheckRule(1, 'Check File', 'mod.dll', '1.0.0');

      expect(rule).toEqual({
        collectionId: 1,
        name: 'Check File',
        conditionType: 'file_check',
        conditionParams: {
          file_name: 'mod.dll',
          file_version: '1.0.0'
        },
        action: 'enable'
      });
    });
  });

  describe('createAndRule and createOrRule', () => {
    it('should create AND rule', () => {
      const rule = CollectionLogic.createAndRule(1, 'AND Rule', [1, 2, 3]);

      expect(rule).toEqual({
        collectionId: 1,
        name: 'AND Rule',
        conditionType: 'and',
        conditionParams: { rules: [1, 2, 3] },
        action: 'enable'
      });
    });

    it('should create OR rule', () => {
      const rule = CollectionLogic.createOrRule(1, 'OR Rule', [4, 5]);

      expect(rule).toEqual({
        collectionId: 1,
        name: 'OR Rule',
        conditionType: 'or',
        conditionParams: { rules: [4, 5] },
        action: 'enable'
      });
    });
  });

  describe('getRuleDescription', () => {
    const mockRule = (conditionType: ConditionType, conditionParams: Record<string, unknown>, action: Action = 'enable'): CollectionLogicRule => ({
      id: 1,
      collectionId: 1,
      name: 'Test Rule',
      conditionType,
      conditionParams,
      action,
      createdAt: '2024-01-01T00:00:00Z'
    });

    it('should describe boolean rules', () => {
      expect(CollectionLogic.getRuleDescription(mockRule('boolean', { value: true }))).toBe('включить если включено');
      expect(CollectionLogic.getRuleDescription(mockRule('boolean', { value: false }, 'disable'))).toBe('выключить если выключено');
    });

    it('should describe collection check rules', () => {
      expect(CollectionLogic.getRuleDescription(mockRule('collection_check', { collection_id: 5 }))).toBe('включить если коллекция #5 активна');
    });

    it('should describe file check rules', () => {
      expect(CollectionLogic.getRuleDescription(mockRule('file_check', {
        file_name: 'mod.dll',
        file_version: '1.0.0'
      }))).toBe('включить если файл mod.dll@1.0.0 установлен');
    });

    it('should describe AND/OR rules', () => {
      expect(CollectionLogic.getRuleDescription(mockRule('and', { rules: [1, 2, 3] }))).toBe('включить если все правила [1, 2, 3] выполнены');
      expect(CollectionLogic.getRuleDescription(mockRule('or', { rules: [4, 5] }, 'disable'))).toBe('выключить если хотя бы одно правило [4, 5] выполнено');
    });

    it('should handle unknown rule types', () => {
      expect(CollectionLogic.getRuleDescription(mockRule('unknown' as ConditionType, {}))).toBe('Неизвестное правило: Test Rule');
    });
  });

  describe('canDeleteRule', () => {
    const rules: CollectionLogicRule[] = [
      { id: 1, collectionId: 1, name: 'Rule 1', conditionType: 'boolean', conditionParams: { value: true }, action: 'enable', createdAt: '2024-01-01' },
      { id: 2, collectionId: 1, name: 'Rule 2', conditionType: 'and', conditionParams: { rules: [1] }, action: 'enable', createdAt: '2024-01-01' },
      { id: 3, collectionId: 1, name: 'Rule 3', conditionType: 'or', conditionParams: { rules: [1, 2] }, action: 'enable', createdAt: '2024-01-01' },
      { id: 4, collectionId: 1, name: 'Rule 4', conditionType: 'boolean', conditionParams: { value: false }, action: 'enable', createdAt: '2024-01-01' }
    ];

    it('should allow deleting rule not referenced by others', () => {
      expect(CollectionLogic.canDeleteRule(4, rules)).toBe(true);
    });

    it('should prevent deleting rule referenced by AND rule', () => {
      expect(CollectionLogic.canDeleteRule(1, rules)).toBe(false);
    });

    it('should prevent deleting rule referenced by OR rule', () => {
      expect(CollectionLogic.canDeleteRule(2, rules)).toBe(false);
    });

    it('should allow deleting rule that only references itself in dependencies', () => {
      const selfReferencingRules = [
        { id: 1, collectionId: 1, name: 'Rule 1', conditionType: 'boolean', conditionParams: { value: true }, action: 'enable', createdAt: '2024-01-01' },
        { id: 2, collectionId: 1, name: 'Rule 2', conditionType: 'and', conditionParams: { rules: [2] }, action: 'enable', createdAt: '2024-01-01' }
      ];
      expect(CollectionLogic.canDeleteRule(2, selfReferencingRules)).toBe(true);
    });
  });

  describe('getDependentRules', () => {
    const rules: CollectionLogicRule[] = [
      { id: 1, collectionId: 1, name: 'Rule 1', conditionType: 'boolean', conditionParams: { value: true }, action: 'enable', createdAt: '2024-01-01' },
      { id: 2, collectionId: 1, name: 'Rule 2', conditionType: 'and', conditionParams: { rules: [1] }, action: 'enable', createdAt: '2024-01-01' },
      { id: 3, collectionId: 1, name: 'Rule 3', conditionType: 'or', conditionParams: { rules: [1, 2] }, action: 'enable', createdAt: '2024-01-01' },
      { id: 4, collectionId: 1, name: 'Rule 4', conditionType: 'boolean', conditionParams: { value: false }, action: 'enable', createdAt: '2024-01-01' }
    ];

    it('should return empty array for rule not referenced by others', () => {
      expect(CollectionLogic.getDependentRules(4, rules)).toEqual([]);
    });

    it('should return rules that reference the given rule', () => {
      expect(CollectionLogic.getDependentRules(1, rules)).toEqual([2, 3]);
      expect(CollectionLogic.getDependentRules(2, rules)).toEqual([3]);
    });

    it('should not include self-references', () => {
      const selfReferencingRules = [
        { id: 1, collectionId: 1, name: 'Rule 1', conditionType: 'and', conditionParams: { rules: [1] }, action: 'enable', createdAt: '2024-01-01' }
      ];
      expect(CollectionLogic.getDependentRules(1, selfReferencingRules)).toEqual([]);
    });
  });

  describe('validateRules', () => {
    it('should validate valid rules', () => {
      const validRules: CollectionLogicRule[] = [
        { id: 1, collectionId: 1, name: 'Rule 1', conditionType: 'boolean', conditionParams: { value: true }, action: 'enable', createdAt: '2024-01-01' },
        { id: 2, collectionId: 1, name: 'Rule 2', conditionType: 'and', conditionParams: { rules: [1] }, action: 'enable', createdAt: '2024-01-01' }
      ];

      const result = CollectionLogic.validateRules(validRules);
      expect(result.valid).toBe(true);
      expect(result.errors).toEqual([]);
    });

    it('should detect invalid condition parameters', () => {
      const invalidRules: CollectionLogicRule[] = [
        { id: 1, collectionId: 1, name: 'Invalid Boolean', conditionType: 'boolean', conditionParams: { value: 'true' }, action: 'enable', createdAt: '2024-01-01' },
        { id: 2, collectionId: 1, name: 'Valid Rule', conditionType: 'boolean', conditionParams: { value: true }, action: 'enable', createdAt: '2024-01-01' }
      ];

      const result = CollectionLogic.validateRules(invalidRules);
      expect(result.valid).toBe(false);
      expect(result.errors).toContain('Правило "Invalid Boolean" (ID: 1) имеет невалидные параметры');
    });

    it('should detect missing rule references', () => {
      const rulesWithMissingRef: CollectionLogicRule[] = [
        { id: 1, collectionId: 1, name: 'Rule 1', conditionType: 'boolean', conditionParams: { value: true }, action: 'enable', createdAt: '2024-01-01' },
        { id: 2, collectionId: 1, name: 'Rule 2', conditionType: 'and', conditionParams: { rules: [1, 999] }, action: 'enable', createdAt: '2024-01-01' }
      ];

      const result = CollectionLogic.validateRules(rulesWithMissingRef);
      expect(result.valid).toBe(false);
      expect(result.errors).toContain('Правило "Rule 2" (ID: 2) ссылается на несуществующее правило 999');
    });

    it('should detect self-referencing rules', () => {
      const selfReferencingRules: CollectionLogicRule[] = [
        { id: 1, collectionId: 1, name: 'Self Ref', conditionType: 'and', conditionParams: { rules: [1] }, action: 'enable', createdAt: '2024-01-01' }
      ];

      const result = CollectionLogic.validateRules(selfReferencingRules);
      expect(result.valid).toBe(false);
      expect(result.errors).toContain('Правило "Self Ref" (ID: 1) ссылается само на себя');
    });

    it('should handle multiple validation errors', () => {
      const multipleErrorsRules: CollectionLogicRule[] = [
        { id: 1, collectionId: 1, name: 'Invalid', conditionType: 'boolean', conditionParams: {}, action: 'enable', createdAt: '2024-01-01' },
        { id: 2, collectionId: 1, name: 'Self Ref', conditionType: 'and', conditionParams: { rules: [2] }, action: 'enable', createdAt: '2024-01-01' },
        { id: 3, collectionId: 1, name: 'Missing Ref', conditionType: 'or', conditionParams: { rules: [1, 999] }, action: 'enable', createdAt: '2024-01-01' }
      ];

      const result = CollectionLogic.validateRules(multipleErrorsRules);
      expect(result.valid).toBe(false);
      expect(result.errors).toHaveLength(3);
    });
  });

  describe('getEvaluationStats', () => {
    it('should calculate correct statistics', () => {
      const result = {
        collectionId: 1,
        enabledFiles: [1, 2, 3],
        disabledFiles: [4, 5],
        evaluationDetails: []
      };

      const stats = CollectionLogic.getEvaluationStats(result);
      expect(stats.totalFiles).toBe(5);
      expect(stats.enabledCount).toBe(3);
      expect(stats.disabledCount).toBe(2);
      expect(stats.enabledPercentage).toBe(60);
    });

    it('should handle zero files', () => {
      const result = {
        collectionId: 1,
        enabledFiles: [],
        disabledFiles: [],
        evaluationDetails: []
      };

      const stats = CollectionLogic.getEvaluationStats(result);
      expect(stats.totalFiles).toBe(0);
      expect(stats.enabledCount).toBe(0);
      expect(stats.disabledCount).toBe(0);
      expect(stats.enabledPercentage).toBe(0);
    });

    it('should handle only enabled files', () => {
      const result = {
        collectionId: 1,
        enabledFiles: [1, 2],
        disabledFiles: [],
        evaluationDetails: []
      };

      const stats = CollectionLogic.getEvaluationStats(result);
      expect(stats.totalFiles).toBe(2);
      expect(stats.enabledCount).toBe(2);
      expect(stats.disabledCount).toBe(0);
      expect(stats.enabledPercentage).toBe(100);
    });
  });
});
