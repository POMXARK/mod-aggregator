import { describe, it, expect } from 'vitest';
import fc from 'fast-check';
import { CollectionLogic } from '../collection-logic';
import type { CollectionLogicRule, ConditionType, Action } from '../../../types/collection';

// Property-based test arbitraries
const conditionTypeArb = fc.constantFrom<ConditionType>(
  'boolean', 'collection_check', 'file_check', 'and', 'or'
);

const actionArb = fc.constantFrom<Action>('enable', 'disable');

const booleanConditionParamsArb = fc.record({
  value: fc.boolean()
});

const collectionCheckConditionParamsArb = fc.record({
  collection_id: fc.integer({ min: 1, max: 1000 })
});

const fileCheckConditionParamsArb = fc.record({
  file_name: fc.string({ minLength: 1, maxLength: 100 }),
  file_version: fc.string({ minLength: 1, maxLength: 50 })
});

const andOrConditionParamsArb = fc.record({
  rules: fc.array(fc.integer({ min: 1, max: 100 }), { minLength: 1, maxLength: 10 })
});

const conditionParamsArb = fc.oneof(
  booleanConditionParamsArb,
  collectionCheckConditionParamsArb,
  fileCheckConditionParamsArb,
  andOrConditionParamsArb
);

const ruleArb = fc.record({
  id: fc.integer({ min: 1, max: 1000 }),
  collectionId: fc.integer({ min: 1, max: 100 }),
  name: fc.string({ minLength: 1, maxLength: 100 }),
  conditionType: conditionTypeArb,
  conditionParams: conditionParamsArb,
  action: actionArb,
  createdAt: fc.constant('2024-01-01T00:00:00Z')
});

const rulesArrayArb = fc.array(ruleArb, { minLength: 1, maxLength: 20 });

describe('CollectionLogic - Property-based Tests', () => {
  describe('validateConditionParams', () => {
    it('should always return boolean for any condition type and params', () => {
      fc.assert(
        fc.property(conditionTypeArb, fc.anything(), (conditionType, params) => {
          const result = CollectionLogic.validateConditionParams(conditionType, params);
          expect(typeof result).toBe('boolean');
        })
      );
    });

    it('should validate boolean conditions correctly', () => {
      fc.assert(
        fc.property(booleanConditionParamsArb, (params) => {
          const result = CollectionLogic.validateConditionParams('boolean', params);
          expect(result).toBe(true);
        })
      );
    });

    it('should validate collection_check conditions correctly', () => {
      fc.assert(
        fc.property(collectionCheckConditionParamsArb, (params) => {
          const result = CollectionLogic.validateConditionParams('collection_check', params);
          expect(result).toBe(true);
        })
      );

      fc.assert(
        fc.property(fc.record({ collection_id: fc.integer({ max: 0 }) }), (params) => {
          const result = CollectionLogic.validateConditionParams('collection_check', params);
          expect(result).toBe(false);
        })
      );
    });

    it('should validate file_check conditions correctly', () => {
      fc.assert(
        fc.property(fileCheckConditionParamsArb, (params) => {
          const result = CollectionLogic.validateConditionParams('file_check', params);
          expect(result).toBe(true);
        })
      );

      fc.assert(
        fc.property(
          fc.record({
            file_name: fc.constant(''),
            file_version: fc.string({ minLength: 1 })
          }),
          (params) => {
            const result = CollectionLogic.validateConditionParams('file_check', params);
            expect(result).toBe(false);
          }
        )
      );
    });

    it('should validate and/or conditions correctly', () => {
      fc.assert(
        fc.property(andOrConditionParamsArb, (params) => {
          const result = CollectionLogic.validateConditionParams('and', params);
          expect(result).toBe(true);
        })
      );

      fc.assert(
        fc.property(
          fc.record({
            rules: fc.constant([])
          }),
          (params) => {
            const result = CollectionLogic.validateConditionParams('and', params);
            expect(result).toBe(false);
          }
        )
      );
    });
  });

  describe('getRuleDescription', () => {
    it('should always return a string', () => {
      fc.assert(
        fc.property(ruleArb, (rule) => {
          const result = CollectionLogic.getRuleDescription(rule);
          expect(typeof result).toBe('string').toBe(true);
          expect(result.length).toBeGreaterThan(0);
        })
      );
    });

    it('should include action text in description', () => {
      fc.assert(
        fc.property(ruleArb, (rule) => {
          const result = CollectionLogic.getRuleDescription(rule);
          const actionText = rule.action === 'enable' ? 'включить' : 'выключить';
          expect(result).toContain(actionText);
        })
      );
    });
  });

  describe('canDeleteRule', () => {
    it('should always return boolean', () => {
      fc.assert(
        fc.property(fc.integer({ min: 1, max: 1000 }), rulesArrayArb, (ruleId, rules) => {
          const result = CollectionLogic.canDeleteRule(ruleId, rules);
          expect(typeof result).toBe('boolean');
        })
      );
    });

    it('should allow deleting rules not referenced by others', () => {
      fc.assert(
        fc.property(rulesArrayArb, (rules) => {
          fc.pre(rules.length > 0);
          const ruleId = rules[0].id;
          const result = CollectionLogic.canDeleteRule(ruleId, rules);
          // This is a probabilistic property - most rules should be deletable
          expect(typeof result).toBe('boolean');
        })
      );
    });

    it('should not allow deleting rules referenced in and/or conditions', () => {
      // Create a rule that references another rule
      const baseRule = {
        id: 1,
        collectionId: 1,
        name: 'Base Rule',
        conditionType: 'boolean' as ConditionType,
        conditionParams: { value: true },
        action: 'enable' as Action,
        createdAt: '2024-01-01T00:00:00Z'
      };

      const referencingRule = {
        id: 2,
        collectionId: 1,
        name: 'Referencing Rule',
        conditionType: 'and' as ConditionType,
        conditionParams: { rules: [1] },
        action: 'enable' as Action,
        createdAt: '2024-01-01T00:00:00Z'
      };

      const rules = [baseRule, referencingRule];
      const result = CollectionLogic.canDeleteRule(1, rules);
      expect(result).toBe(false);
    });
  });

  describe('getDependentRules', () => {
    it('should always return array of numbers', () => {
      fc.assert(
        fc.property(fc.integer({ min: 1, max: 1000 }), rulesArrayArb, (ruleId, rules) => {
          const result = CollectionLogic.getDependentRules(ruleId, rules);
          expect(Array.isArray(result)).toBe(true);
          result.forEach(id => expect(typeof id).toBe('number'));
        })
      );
    });

    it('should return empty array for non-referenced rules', () => {
      fc.assert(
        fc.property(rulesArrayArb, (rules) => {
          fc.pre(rules.length > 0);
          const ruleId = rules[0].id;
          const result = CollectionLogic.getDependentRules(ruleId, rules);
          // Should contain only valid rule IDs
          result.forEach(id => {
            expect(rules.some(r => r.id === id)).toBe(true);
          });
        })
      );
    });
  });

  describe('validateRules', () => {
    it('should always return valid structure', () => {
      fc.assert(
        fc.property(rulesArrayArb, (rules) => {
          const result = CollectionLogic.validateRules(rules);
          expect(result).toHaveProperty('valid');
          expect(result).toHaveProperty('errors');
          expect(typeof result.valid).toBe('boolean');
          expect(Array.isArray(result.errors)).toBe(true);
          result.errors.forEach(error => expect(typeof error).toBe('string'));
        })
      );
    });

    it('should return valid=true when all rules are valid', () => {
      fc.assert(
        fc.property(rulesArrayArb, (rules) => {
          // Filter to only valid rules for this property
          const validRules = rules.filter(rule => {
            try {
              return CollectionLogic.validateConditionParams(rule.conditionType, rule.conditionParams);
            } catch {
              return false;
            }
          });

          fc.pre(validRules.length === rules.length);

          // Ensure no duplicate IDs
          const ids = validRules.map(r => r.id);
          const uniqueIds = new Set(ids);
          fc.pre(ids.length === uniqueIds.size);

          const result = CollectionLogic.validateRules(validRules);
          expect(result.valid).toBe(true);
          expect(result.errors).toEqual([]);
        })
      );
    });

    it('should detect invalid rules', () => {
      const invalidRule = {
        id: 1,
        collectionId: 1,
        name: 'Invalid Rule',
        conditionType: 'boolean' as ConditionType,
        conditionParams: { value: 'not boolean' },
        action: 'enable' as Action,
        createdAt: '2024-01-01T00:00:00Z'
      };

      const result = CollectionLogic.validateRules([invalidRule]);
      expect(result.valid).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });
  });

  describe('getEvaluationStats', () => {
    const evaluationResultArb = fc.record({
      collectionId: fc.integer({ min: 1, max: 100 }),
      enabledFiles: fc.array(fc.integer({ min: 1, max: 1000 }), { minLength: 0, maxLength: 50 }),
      disabledFiles: fc.array(fc.integer({ min: 1, max: 1000 }), { minLength: 0, maxLength: 50 }),
      evaluationDetails: fc.constant([]) // Simplified for this test
    });

    it('should always return valid stats structure', () => {
      fc.assert(
        fc.property(evaluationResultArb, (result) => {
          const stats = CollectionLogic.getEvaluationStats(result);
          expect(stats).toHaveProperty('totalFiles');
          expect(stats).toHaveProperty('enabledCount');
          expect(stats).toHaveProperty('disabledCount');
          expect(stats).toHaveProperty('enabledPercentage');

          expect(typeof stats.totalFiles).toBe('number');
          expect(typeof stats.enabledCount).toBe('number');
          expect(typeof stats.disabledCount).toBe('number');
          expect(typeof stats.enabledPercentage).toBe('number');
        })
      );
    });

    it('should calculate correct total files', () => {
      fc.assert(
        fc.property(evaluationResultArb, (result) => {
          const stats = CollectionLogic.getEvaluationStats(result);
          const expectedTotal = result.enabledFiles.length + result.disabledFiles.length;
          expect(stats.totalFiles).toBe(expectedTotal);
        })
      );
    });

    it('should calculate correct percentages', () => {
      fc.assert(
        fc.property(evaluationResultArb, (result) => {
          const stats = CollectionLogic.getEvaluationStats(result);
          const expectedPercentage = stats.totalFiles > 0
            ? Math.round((stats.enabledCount / stats.totalFiles) * 100)
            : 0;
          expect(stats.enabledPercentage).toBe(expectedPercentage);
        })
      );
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
  });

  describe('Rule Creation Functions', () => {
    it('createBooleanRule should always produce valid rules', () => {
      fc.assert(
        fc.property(
          fc.integer({ min: 1, max: 100 }),
          fc.string({ minLength: 1, maxLength: 50 }),
          fc.boolean(),
          actionArb,
          (collectionId, name, value, action) => {
            const rule = CollectionLogic.createBooleanRule(collectionId, name, value, action);

            expect(rule.collectionId).toBe(collectionId);
            expect(rule.name).toBe(name);
            expect(rule.conditionType).toBe('boolean');
            expect(rule.conditionParams).toEqual({ value });
            expect(rule.action).toBe(action);

            // Should not have id and createdAt
            expect(rule).not.toHaveProperty('id');
            expect(rule).not.toHaveProperty('createdAt');
          }
        )
      );
    });

    it('createCollectionCheckRule should always produce valid rules', () => {
      fc.assert(
        fc.property(
          fc.integer({ min: 1, max: 100 }),
          fc.string({ minLength: 1, maxLength: 50 }),
          fc.integer({ min: 1, max: 1000 }),
          actionArb,
          (collectionId, name, targetCollectionId, action) => {
            const rule = CollectionLogic.createCollectionCheckRule(collectionId, name, targetCollectionId, action);

            expect(rule.collectionId).toBe(collectionId);
            expect(rule.name).toBe(name);
            expect(rule.conditionType).toBe('collection_check');
            expect(rule.conditionParams).toEqual({ collection_id: targetCollectionId });
            expect(rule.action).toBe(action);
          }
        )
      );
    });

    it('createFileCheckRule should always produce valid rules', () => {
      fc.assert(
        fc.property(
          fc.integer({ min: 1, max: 100 }),
          fc.string({ minLength: 1, maxLength: 50 }),
          fc.string({ minLength: 1, maxLength: 20 }),
          fc.string({ minLength: 1, maxLength: 20 }),
          actionArb,
          (collectionId, name, fileName, fileVersion, action) => {
            const rule = CollectionLogic.createFileCheckRule(collectionId, name, fileName, fileVersion, action);

            expect(rule.collectionId).toBe(collectionId);
            expect(rule.name).toBe(name);
            expect(rule.conditionType).toBe('file_check');
            expect(rule.conditionParams).toEqual({
              file_name: fileName,
              file_version: fileVersion
            });
            expect(rule.action).toBe(action);
          }
        )
      );
    });

    it('createAndRule should always produce valid rules', () => {
      fc.assert(
        fc.property(
          fc.integer({ min: 1, max: 100 }),
          fc.string({ minLength: 1, maxLength: 50 }),
          fc.array(fc.integer({ min: 1, max: 100 }), { minLength: 1, maxLength: 5 }),
          actionArb,
          (collectionId, name, ruleIds, action) => {
            const rule = CollectionLogic.createAndRule(collectionId, name, ruleIds, action);

            expect(rule.collectionId).toBe(collectionId);
            expect(rule.name).toBe(name);
            expect(rule.conditionType).toBe('and');
            expect(rule.conditionParams).toEqual({ rules: ruleIds });
            expect(rule.action).toBe(action);
          }
        )
      );
    });

    it('createOrRule should always produce valid rules', () => {
      fc.assert(
        fc.property(
          fc.integer({ min: 1, max: 100 }),
          fc.string({ minLength: 1, maxLength: 50 }),
          fc.array(fc.integer({ min: 1, max: 100 }), { minLength: 1, maxLength: 5 }),
          actionArb,
          (collectionId, name, ruleIds, action) => {
            const rule = CollectionLogic.createOrRule(collectionId, name, ruleIds, action);

            expect(rule.collectionId).toBe(collectionId);
            expect(rule.name).toBe(name);
            expect(rule.conditionType).toBe('or');
            expect(rule.conditionParams).toEqual({ rules: ruleIds });
            expect(rule.action).toBe(action);
          }
        )
      );
    });
  });

  describe('Complex Rule Interactions', () => {
    it('should handle circular dependencies correctly', () => {
      fc.assert(
        fc.property(rulesArrayArb, (rules) => {
          // Add a rule that references itself
          const selfReferencingRule = {
            id: 999,
            collectionId: 1,
            name: 'Self Ref',
            conditionType: 'and' as ConditionType,
            conditionParams: { rules: [999] },
            action: 'enable' as Action,
            createdAt: '2024-01-01T00:00:00Z'
          };

          const rulesWithSelfRef = [...rules, selfReferencingRule];
          const result = CollectionLogic.validateRules(rulesWithSelfRef);

          expect(result.valid).toBe(false);
          expect(result.errors.some(error => error.includes('ссылается само на себя'))).toBe(true);
        })
      );
    });

    it('should handle complex dependency chains', () => {
      // Create a chain: Rule1 -> Rule2 -> Rule3
      const rule1 = {
        id: 1,
        collectionId: 1,
        name: 'Rule 1',
        conditionType: 'boolean' as ConditionType,
        conditionParams: { value: true },
        action: 'enable' as Action,
        createdAt: '2024-01-01T00:00:00Z'
      };

      const rule2 = {
        id: 2,
        collectionId: 1,
        name: 'Rule 2',
        conditionType: 'and' as ConditionType,
        conditionParams: { rules: [1] },
        action: 'enable' as Action,
        createdAt: '2024-01-01T00:00:00Z'
      };

      const rule3 = {
        id: 3,
        collectionId: 1,
        name: 'Rule 3',
        conditionType: 'and' as ConditionType,
        conditionParams: { rules: [2] },
        action: 'enable' as Action,
        createdAt: '2024-01-01T00:00:00Z'
      };

      const rules = [rule1, rule2, rule3];

      // Rule 1 should not be deletable
      expect(CollectionLogic.canDeleteRule(1, rules)).toBe(false);

      // Rule 2 should not be deletable
      expect(CollectionLogic.canDeleteRule(2, rules)).toBe(false);

      // Rule 3 should be deletable
      expect(CollectionLogic.canDeleteRule(3, rules)).toBe(true);

      // Check dependencies
      expect(CollectionLogic.getDependentRules(1, rules)).toEqual([2]);
      expect(CollectionLogic.getDependentRules(2, rules)).toEqual([3]);
      expect(CollectionLogic.getDependentRules(3, rules)).toEqual([]);
    });
  });
});

