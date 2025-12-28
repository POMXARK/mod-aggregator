<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@/lib/tauri-wrapper';
  import { CollectionLogic } from '@/lib/collections/collection-logic';
  import type { CollectionLogicRule, ConditionType, Action, Collection } from '@/types/collection';

  interface Props {
    collectionId: number;
    onRuleCreated?: (rule: CollectionLogicRule) => void;
    onRuleUpdated?: (rule: CollectionLogicRule) => void;
    onRuleDeleted?: (ruleId: number) => void;
  }

  let { collectionId = $bindable(), onRuleCreated, onRuleUpdated, onRuleDeleted }: Props = $props();

  let rules = $state<CollectionLogicRule[]>([]);
  let allCollections = $state<Collection[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let showCreateForm = $state(false);
  let editingRule = $state<CollectionLogicRule | null>(null);

  // Форма создания/редактирования правила
  let ruleName = $state('');
  let conditionType = $state<ConditionType>('boolean');
  let action = $state<Action>('enable');

  // Параметры условий
  let booleanValue = $state(true);
  let collectionCheckId = $state<number | null>(null);
  let fileCheckName = $state('');
  let fileCheckVersion = $state('');
  let andOrRuleIds = $state<number[]>([]);

  onMount(async () => {
    await Promise.all([loadRules(), loadCollections(), loadFiles()]);
  });

  $effect(() => {
    if (collectionId) {
      loadRules();
    }
  });

  async function loadRules() {
    if (!collectionId) {
      return;
    }

    loading = true;
    error = null;

    try {
      rules = await invoke<CollectionLogicRule[]>('get_collection_logic_rules', {
        collection_id: collectionId,
      });
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при загрузке правил';
      console.error('Failed to load rules:', e);
    } finally {
      loading = false;
    }
  }

  async function loadCollections() {
    try {
      allCollections = await invoke<Collection[]>('get_collections');
    } catch (e: unknown) {
      console.error('Failed to load collections:', e);
    }
  }

  async function loadFiles() {}

  function resetForm() {
    ruleName = '';
    conditionType = 'boolean';
    action = 'enable';
    booleanValue = true;
    collectionCheckId = null;
    fileCheckName = '';
    fileCheckVersion = '';
    andOrRuleIds = [];
    editingRule = null;
    showCreateForm = false;
  }

  function getConditionParams(): Record<string, unknown> {
    switch (conditionType) {
      case 'boolean':
        return { value: booleanValue };
      case 'collection_check':
        return { collection_id: collectionCheckId };
      case 'file_check':
        return {
          file_name: fileCheckName,
          file_version: fileCheckVersion,
        };
      case 'and':
      case 'or':
        return { rules: andOrRuleIds };
      default:
        return {};
    }
  }

  function canSave(): boolean {
    if (!ruleName.trim()) {
      return false;
    }

    const params = getConditionParams();
    return CollectionLogic.validateConditionParams(conditionType, params);
  }

  async function handleSave() {
    if (!canSave() || !collectionId) {
      return;
    }

    loading = true;
    error = null;

    try {
      const conditionParams = getConditionParams();

      if (editingRule) {
        // Обновление существующего правила
        const updated = await invoke<CollectionLogicRule>('update_collection_logic_rule', {
          id: editingRule.id,
          name: ruleName.trim(),
          condition_type: conditionType,
          condition_params: conditionParams,
          action,
        });

        if (onRuleUpdated) {
          onRuleUpdated(updated);
        }
      } else {
        // Создание нового правила
        const created = await invoke<CollectionLogicRule>('create_collection_logic_rule', {
          collection_id: collectionId,
          name: ruleName.trim(),
          condition_type: conditionType,
          condition_params: conditionParams,
          action,
        });

        if (onRuleCreated) {
          onRuleCreated(created);
        }
      }

      resetForm();
      await loadRules();
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при сохранении правила';
      console.error('Failed to save rule:', e);
    } finally {
      loading = false;
    }
  }

  function startEdit(rule: CollectionLogicRule) {
    editingRule = rule;
    ruleName = rule.name;
    conditionType = rule.conditionType;
    action = rule.action;

    // Заполняем параметры в зависимости от типа условия
    switch (rule.conditionType) {
      case 'boolean':
        booleanValue = rule.conditionParams.value as boolean;
        break;
      case 'collection_check':
        collectionCheckId = rule.conditionParams.collection_id as number;
        break;
      case 'file_check':
        fileCheckName = rule.conditionParams.file_name as string;
        fileCheckVersion = rule.conditionParams.file_version as string;
        break;
      case 'and':
      case 'or':
        andOrRuleIds = (rule.conditionParams.rules as number[]) || [];
        break;
    }

    showCreateForm = true;
  }

  async function handleDelete(ruleId: number) {
    if (!confirm('Удалить это правило?')) {
      return;
    }

    // Проверяем, можно ли удалить правило
    const canDelete = CollectionLogic.canDeleteRule(ruleId, rules);
    if (!canDelete) {
      const dependentRules = CollectionLogic.getDependentRules(ruleId, rules);
      error = `Нельзя удалить правило: оно используется в других правилах (${dependentRules.join(', ')})`;
      return;
    }

    loading = true;
    error = null;

    try {
      await invoke('delete_collection_logic_rule', { id: ruleId });

      if (onRuleDeleted) {
        onRuleDeleted(ruleId);
      }

      await loadRules();
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при удалении правила';
      console.error('Failed to delete rule:', e);
    } finally {
      loading = false;
    }
  }

  function toggleRuleSelection(ruleId: number) {
    if (andOrRuleIds.includes(ruleId)) {
      andOrRuleIds = andOrRuleIds.filter(id => id !== ruleId);
    } else {
      andOrRuleIds = [...andOrRuleIds, ruleId];
    }
  }
</script>

<div class="logic-builder">
  <div class="header">
    <h2>Правила логики коллекции</h2>
    <button
      type="button"
      class="btn-primary"
      onclick={() => {
        resetForm();
        showCreateForm = true;
      }}
      disabled={!collectionId}
    >
      + Создать правило
    </button>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if showCreateForm}
    <div class="form-container">
      <div class="form-header">
        <h3>{editingRule ? 'Редактировать правило' : 'Создать правило'}</h3>
        <button type="button" class="btn-close" onclick={resetForm}> × </button>
      </div>

      <div class="form-fields">
        <div class="form-field">
          <label for="rule-name">Название правила *</label>
          <input
            id="rule-name"
            type="text"
            bind:value={ruleName}
            placeholder="Например: Включить если базовый мод установлен"
            disabled={loading}
          />
        </div>

        <div class="form-field">
          <label for="action-select">Действие</label>
          <select id="action-select" bind:value={action} disabled={loading}>
            <option value="enable">Включить</option>
            <option value="disable">Выключить</option>
          </select>
        </div>

        <div class="form-field">
          <label for="condition-type">Тип условия</label>
          <select id="condition-type" bind:value={conditionType} disabled={loading}>
            <option value="boolean">Boolean (включено/выключено)</option>
            <option value="collection_check">Проверка коллекции</option>
            <option value="file_check">Проверка файла</option>
            <option value="and">AND (логическое И)</option>
            <option value="or">OR (логическое ИЛИ)</option>
          </select>
        </div>

        <!-- Boolean условие -->
        {#if conditionType === 'boolean'}
          <div class="form-field">
            <label>
              <input type="checkbox" bind:checked={booleanValue} disabled={loading} />
              Включено
            </label>
          </div>
        {/if}

        <!-- Collection check условие -->
        {#if conditionType === 'collection_check'}
          <div class="form-field">
            <label for="collection-select">Коллекция</label>
            <select id="collection-select" bind:value={collectionCheckId} disabled={loading}>
              <option value={null}>Выберите коллекцию</option>
              {#each allCollections as collection (collection.id)}
                {#if collection.id !== collectionId}
                  <option value={collection.id}>{collection.name}</option>
                {/if}
              {/each}
            </select>
          </div>
        {/if}

        <!-- File check условие -->
        {#if conditionType === 'file_check'}
          <div class="form-field">
            <label for="file-name">Имя файла *</label>
            <input
              id="file-name"
              type="text"
              bind:value={fileCheckName}
              placeholder="base-mod"
              disabled={loading}
            />
          </div>
          <div class="form-field">
            <label for="file-version">Версия файла *</label>
            <input
              id="file-version"
              type="text"
              bind:value={fileCheckVersion}
              placeholder="1.0.0"
              disabled={loading}
            />
          </div>
        {/if}

        <!-- AND/OR условие -->
        {#if conditionType === 'and' || conditionType === 'or'}
          <div class="form-field">
            <label>Выберите правила для комбинации</label>
            <div class="rules-list">
              {#each rules as rule (rule.id)}
                {#if !editingRule || rule.id !== editingRule.id}
                  <label class="rule-item">
                    <input
                      type="checkbox"
                      checked={andOrRuleIds.includes(rule.id)}
                      onchange={() => toggleRuleSelection(rule.id)}
                      disabled={loading}
                    />
                    <span>{rule.name}</span>
                    <span class="rule-type">({rule.conditionType})</span>
                  </label>
                {/if}
              {/each}
              {#if rules.length === 0 || (editingRule && rules.length === 1)}
                <p class="hint">Создайте другие правила для комбинации</p>
              {/if}
            </div>
          </div>
        {/if}
      </div>

      <div class="form-actions">
        <button
          type="button"
          class="btn-primary"
          onclick={handleSave}
          disabled={!canSave() || loading}
        >
          {loading ? 'Сохранение...' : 'Сохранить'}
        </button>
        <button type="button" class="btn-secondary" onclick={resetForm} disabled={loading}>
          Отмена
        </button>
      </div>
    </div>
  {/if}

  {#if loading && rules.length === 0 && !showCreateForm}
    <div class="loading">Загрузка правил...</div>
  {:else if rules.length === 0}
    <div class="empty">
      <p>Нет правил логики. Создайте первое правило для управления файлами коллекции.</p>
    </div>
  {:else}
    <div class="rules-list">
      {#each rules as rule (rule.id)}
        <div class="rule-card">
          <div class="rule-header">
            <div class="rule-info">
              <h4>{rule.name}</h4>
              <p class="rule-description">
                {CollectionLogic.getRuleDescription(rule)}
              </p>
            </div>
            <div class="rule-actions">
              <button
                type="button"
                class="btn-icon"
                onclick={() => startEdit(rule)}
                title="Редактировать"
              >
                ✏️
              </button>
              <button
                type="button"
                class="btn-icon btn-danger"
                onclick={() => handleDelete(rule.id)}
                title="Удалить"
                disabled={!CollectionLogic.canDeleteRule(rule.id, rules)}
              >
                🗑️
              </button>
            </div>
          </div>
          <div class="rule-meta">
            <span class="rule-type-badge">{rule.conditionType}</span>
            <span class="rule-action-badge" class:action-enable={rule.action === 'enable'}>
              {rule.action === 'enable' ? 'Включить' : 'Выключить'}
            </span>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .logic-builder {
    width: 100%;
    max-width: 1200px;
    margin: 0 auto;
    padding: 1.5rem;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .header h2 {
    margin: 0;
    font-size: 1.75rem;
    font-weight: 700;
    color: #e2e8f0;
  }

  .btn-primary {
    padding: 0.75rem 1.5rem;
    background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s;
    box-shadow: 0 2px 8px rgba(14, 165, 233, 0.3);
  }

  .btn-primary:hover:not(:disabled) {
    background: linear-gradient(135deg, #0284c7 0%, #0369a1 100%);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(14, 165, 233, 0.5);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none;
  }

  .error {
    padding: 1rem;
    background: #7f1d1d;
    color: #fca5a5;
    border-radius: 0.375rem;
    margin-bottom: 1rem;
    border-left: 3px solid #ef4444;
  }

  .form-container {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    padding: 1.5rem;
    margin-bottom: 2rem;
  }

  .form-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .form-header h3 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #e2e8f0;
  }

  .btn-close {
    background: none;
    border: none;
    color: #94a3b8;
    font-size: 1.5rem;
    cursor: pointer;
    padding: 0;
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.25rem;
    transition: all 0.2s;
  }

  .btn-close:hover {
    background: #334155;
    color: #e2e8f0;
  }

  .form-fields {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-field label {
    color: #e2e8f0;
    font-size: 0.875rem;
    font-weight: 500;
  }

  .form-field input,
  .form-field select,
  .form-field textarea {
    padding: 0.75rem;
    background: #0f172a;
    color: #e2e8f0;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-family: inherit;
  }

  .form-field input:focus,
  .form-field select:focus,
  .form-field textarea:focus {
    outline: none;
    border-color: #0ea5e9;
    box-shadow: 0 0 0 3px rgba(14, 165, 233, 0.1);
  }

  .form-field input:disabled,
  .form-field select:disabled,
  .form-field textarea:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .form-field input[type='checkbox'] {
    width: auto;
    margin-right: 0.5rem;
  }

  .rules-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-height: 300px;
    overflow-y: auto;
    padding: 0.75rem;
    background: #0f172a;
    border-radius: 0.375rem;
  }

  .rule-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    cursor: pointer;
    border-radius: 0.25rem;
    transition: background 0.2s;
  }

  .rule-item:hover {
    background: #1e293b;
  }

  .rule-item input[type='checkbox'] {
    cursor: pointer;
  }

  .rule-type {
    color: #94a3b8;
    font-size: 0.75rem;
  }

  .hint {
    color: #64748b;
    font-size: 0.875rem;
    font-style: italic;
    margin: 0;
    padding: 0.5rem;
  }

  .form-actions {
    display: flex;
    gap: 0.75rem;
    margin-top: 1.5rem;
  }

  .btn-secondary {
    padding: 0.75rem 1.5rem;
    background: #334155;
    color: #e2e8f0;
    border: 1px solid #475569;
    border-radius: 0.375rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #475569;
    border-color: #64748b;
  }

  .btn-secondary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .loading,
  .empty {
    text-align: center;
    padding: 3rem;
    color: #64748b;
  }

  .rules-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .rule-card {
    padding: 1.25rem;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    transition: all 0.2s;
  }

  .rule-card:hover {
    border-color: #475569;
    background: #0f172a;
  }

  .rule-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 1rem;
    margin-bottom: 0.75rem;
  }

  .rule-info {
    flex: 1;
  }

  .rule-info h4 {
    margin: 0 0 0.5rem 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: #e2e8f0;
  }

  .rule-description {
    margin: 0;
    color: #94a3b8;
    font-size: 0.875rem;
  }

  .rule-actions {
    display: flex;
    gap: 0.5rem;
  }

  .btn-icon {
    background: none;
    border: 1px solid #334155;
    color: #e2e8f0;
    padding: 0.5rem;
    border-radius: 0.25rem;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 1rem;
  }

  .btn-icon:hover:not(:disabled) {
    background: #334155;
    border-color: #475569;
  }

  .btn-icon:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .btn-danger:hover:not(:disabled) {
    background: #7f1d1d;
    border-color: #991b1b;
    color: #fca5a5;
  }

  .rule-meta {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .rule-type-badge,
  .rule-action-badge {
    padding: 0.25rem 0.75rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .rule-type-badge {
    background: #334155;
    color: #94a3b8;
  }

  .rule-action-badge {
    background: #7f1d1d;
    color: #fca5a5;
  }

  .rule-action-badge.action-enable {
    background: #065f46;
    color: #6ee7b7;
  }
</style>
