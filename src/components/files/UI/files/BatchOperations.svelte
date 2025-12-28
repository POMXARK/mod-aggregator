<script lang="ts">
  import { invoke } from '@/lib/tauri-wrapper';
  import type {
    BatchOperationResult,
    BatchDependencyCheckResult,
    BatchValidationResult,
  } from '@/types/batch-operations';
  import type { Collection } from '@/types/collection';

  interface Props {
    selectedFileIds: number[];
    onOperationComplete?: () => void;
  }

  let { selectedFileIds = $bindable([]), onOperationComplete }: Props = $props();

  let loading = $state(false);
  let error = $state<string | null>(null);
  let warnings = $state<string[]>([]);
  let validationResult = $state<BatchValidationResult | null>(null);
  let dependencyCheckResult = $state<BatchDependencyCheckResult | null>(null);
  let allCollections = $state<Collection[]>([]);
  let showDeleteDialog = $state(false);
  let showMoveDialog = $state(false);
  let selectedCollectionId = $state<number | null>(null);
  let forceDelete = $state(false);

  $effect(() => {
    if (selectedFileIds.length > 0) {
      loadCollections();
      checkDependencies();
    } else {
      validationResult = null;
      dependencyCheckResult = null;
    }
  });

  async function loadCollections() {
    try {
      allCollections = await invoke<Collection[]>('get_collections');
    } catch (e: unknown) {
      console.error('Failed to load collections:', e);
    }
  }

  async function checkDependencies() {
    if (selectedFileIds.length === 0) {
      return;
    }

    try {
      dependencyCheckResult = await invoke<BatchDependencyCheckResult>('batch_check_dependencies', {
        file_ids: selectedFileIds,
      });
    } catch (e: unknown) {
      console.warn('Failed to check dependencies:', e);
    }
  }

  async function validateOperation(operationType: string) {
    if (selectedFileIds.length === 0) {
      return;
    }

    try {
      validationResult = await invoke<BatchValidationResult>('validate_batch_operation', {
        operation_type: operationType,
        file_ids: selectedFileIds,
      });
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при валидации операции';
    }
  }

  async function handleDelete() {
    if (selectedFileIds.length === 0) {
      return;
    }

    loading = true;
    error = null;
    warnings = [];

    try {
      const result = await invoke<BatchOperationResult>('batch_delete_files', {
        file_ids: selectedFileIds,
        force: forceDelete,
      });

      if (result.blocked_files.length > 0) {
        warnings.push(
          `Не удалось удалить ${result.blocked_files.length} файл(ов) из-за зависимостей`
        );
      }

      if (result.failed_count > 0) {
        warnings.push(`Ошибки при удалении ${result.failed_count} файл(ов)`);
      }

      if (result.success_count > 0) {
        if (onOperationComplete) {
          onOperationComplete();
        }
        selectedFileIds = [];
        showDeleteDialog = false;
      }
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при удалении файлов';
    } finally {
      loading = false;
    }
  }

  async function handleMoveToCollection() {
    if (selectedFileIds.length === 0 || !selectedCollectionId) {
      return;
    }

    loading = true;
    error = null;
    warnings = [];

    try {
      const result = await invoke<BatchOperationResult>('batch_move_files_to_collection', {
        file_ids: selectedFileIds,
        collection_id: selectedCollectionId,
      });

      if (result.warnings.length > 0) {
        warnings.push(...result.warnings);
      }

      if (result.success_count > 0) {
        if (onOperationComplete) {
          onOperationComplete();
        }
        selectedFileIds = [];
        showMoveDialog = false;
        selectedCollectionId = null;
      }
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при перемещении файлов';
    } finally {
      loading = false;
    }
  }

  async function handleExport() {
    if (selectedFileIds.length === 0) {
      return;
    }

    loading = true;
    error = null;

    try {
      const json = await invoke<string>('batch_export_files', { file_ids: selectedFileIds });

      // Создаем blob и скачиваем
      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `files-export-${Date.now()}.json`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);

      if (onOperationComplete) {
        onOperationComplete();
      }
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при экспорте файлов';
    } finally {
      loading = false;
    }
  }

  function openDeleteDialog() {
    validateOperation('delete');
    showDeleteDialog = true;
  }

  function openMoveDialog() {
    showMoveDialog = true;
  }

  function getDependencyWarnings(): string[] {
    const warnings: string[] = [];

    if (dependencyCheckResult) {
      if (dependencyCheckResult.summary.files_with_missing_deps > 0) {
        warnings.push(
          `${dependencyCheckResult.summary.files_with_missing_deps} файл(ов) имеют отсутствующие зависимости`
        );
      }
      if (dependencyCheckResult.summary.files_with_conflicts > 0) {
        warnings.push(
          `${dependencyCheckResult.summary.files_with_conflicts} файл(ов) имеют конфликты версий`
        );
      }
    }

    if (validationResult) {
      warnings.push(...validationResult.warnings);
    }

    return warnings;
  }
</script>

{#if selectedFileIds.length > 0}
  <div class="batch-operations">
    <div class="operations-bar">
      <div class="selection-info">
        <strong>Выбрано: {selectedFileIds.length} файл(ов)</strong>
      </div>
      <div class="operations-menu">
        <button
          type="button"
          class="btn-operation btn-danger"
          onclick={openDeleteDialog}
          disabled={loading}
        >
          🗑️ Удалить
        </button>
        <button type="button" class="btn-operation" onclick={openMoveDialog} disabled={loading}>
          📁 Переместить в коллекцию
        </button>
        <button type="button" class="btn-operation" onclick={handleExport} disabled={loading}>
          💾 Экспортировать
        </button>
        <button
          type="button"
          class="btn-operation"
          onclick={() => (selectedFileIds = [])}
          disabled={loading}
        >
          ✖️ Снять выбор
        </button>
      </div>
    </div>

    {#if getDependencyWarnings().length > 0 || warnings.length > 0}
      <div class="warnings">
        {#each [...getDependencyWarnings(), ...warnings] as warning, index (index)}
          <div class="warning-item">
            ⚠️ {warning}
          </div>
        {/each}
      </div>
    {/if}

    {#if error}
      <div class="error">{error}</div>
    {/if}
  </div>
{/if}

<!-- Диалог удаления -->
{#if showDeleteDialog}
  <div class="dialog-overlay" onclick={() => (showDeleteDialog = false)}>
    <div class="dialog" onclick={e => e.stopPropagation()}>
      <div class="dialog-header">
        <h3>Удалить файлы</h3>
        <button class="btn-close" onclick={() => (showDeleteDialog = false)}>×</button>
      </div>
      <div class="dialog-content">
        <p>Вы уверены, что хотите удалить {selectedFileIds.length} файл(ов)?</p>

        {#if validationResult && validationResult.affected_dependencies.length > 0}
          <div class="dependency-warning">
            <strong>Внимание:</strong>
            {validationResult.affected_dependencies.length} файл(ов) имеют зависимые файлы. Они не будут
            удалены, если не включить принудительное удаление.
          </div>
          <label class="checkbox-label">
            <input type="checkbox" bind:checked={forceDelete} />
            Принудительное удаление (игнорировать зависимости)
          </label>
        {/if}
      </div>
      <div class="dialog-actions">
        <button
          type="button"
          class="btn-primary btn-danger"
          onclick={handleDelete}
          disabled={loading}
        >
          {loading ? 'Удаление...' : 'Удалить'}
        </button>
        <button
          type="button"
          class="btn-secondary"
          onclick={() => (showDeleteDialog = false)}
          disabled={loading}
        >
          Отмена
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Диалог перемещения -->
{#if showMoveDialog}
  <div class="dialog-overlay" onclick={() => (showMoveDialog = false)}>
    <div class="dialog" onclick={e => e.stopPropagation()}>
      <div class="dialog-header">
        <h3>Переместить в коллекцию</h3>
        <button class="btn-close" onclick={() => (showMoveDialog = false)}>×</button>
      </div>
      <div class="dialog-content">
        <p>Выберите коллекцию для перемещения {selectedFileIds.length} файл(ов):</p>
        <select bind:value={selectedCollectionId} class="collection-select" disabled={loading}>
          <option value={null}>Выберите коллекцию</option>
          {#each allCollections as collection (collection.id)}
            <option value={collection.id}>{collection.name}</option>
          {/each}
        </select>
      </div>
      <div class="dialog-actions">
        <button
          type="button"
          class="btn-primary"
          onclick={handleMoveToCollection}
          disabled={loading || !selectedCollectionId}
        >
          {loading ? 'Перемещение...' : 'Переместить'}
        </button>
        <button
          type="button"
          class="btn-secondary"
          onclick={() => (showMoveDialog = false)}
          disabled={loading}
        >
          Отмена
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .batch-operations {
    position: sticky;
    top: 0;
    z-index: 100;
    background: #1e293b;
    border-bottom: 2px solid #334155;
    padding: 1rem;
    margin-bottom: 1.5rem;
  }

  .operations-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .selection-info {
    color: #e2e8f0;
    font-size: 0.875rem;
  }

  .operations-menu {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .btn-operation {
    padding: 0.5rem 1rem;
    background: #334155;
    color: #e2e8f0;
    border: 1px solid #475569;
    border-radius: 0.375rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.875rem;
  }

  .btn-operation:hover:not(:disabled) {
    background: #475569;
    border-color: #64748b;
  }

  .btn-operation:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-operation.btn-danger {
    background: #7f1d1d;
    border-color: #991b1b;
    color: #fca5a5;
  }

  .btn-operation.btn-danger:hover:not(:disabled) {
    background: #991b1b;
    border-color: #b91c1c;
  }

  .warnings {
    margin-top: 1rem;
    padding: 0.75rem;
    background: #78350f;
    border-radius: 0.375rem;
    border-left: 3px solid #f59e0b;
  }

  .warning-item {
    color: #fcd34d;
    font-size: 0.875rem;
    margin-bottom: 0.25rem;
  }

  .warning-item:last-child {
    margin-bottom: 0;
  }

  .error {
    margin-top: 1rem;
    padding: 1rem;
    background: #7f1d1d;
    color: #fca5a5;
    border-radius: 0.375rem;
    border-left: 3px solid #ef4444;
  }

  .dialog-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .dialog {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    min-width: 400px;
    max-width: 600px;
    max-height: 80vh;
    overflow-y: auto;
  }

  .dialog-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #334155;
  }

  .dialog-header h3 {
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

  .dialog-content {
    padding: 1.5rem;
  }

  .dialog-content p {
    margin: 0 0 1rem 0;
    color: #e2e8f0;
  }

  .dependency-warning {
    padding: 0.75rem;
    background: #78350f;
    border-radius: 0.375rem;
    margin-bottom: 1rem;
    color: #fcd34d;
    font-size: 0.875rem;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #e2e8f0;
    cursor: pointer;
    font-size: 0.875rem;
  }

  .checkbox-label input[type='checkbox'] {
    cursor: pointer;
  }

  .collection-select {
    width: 100%;
    padding: 0.75rem;
    background: #0f172a;
    color: #e2e8f0;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    margin-top: 0.5rem;
  }

  .collection-select:focus {
    outline: none;
    border-color: #0ea5e9;
    box-shadow: 0 0 0 3px rgba(14, 165, 233, 0.1);
  }

  .dialog-actions {
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
    padding: 1.5rem;
    border-top: 1px solid #334155;
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
  }

  .btn-primary:hover:not(:disabled) {
    background: linear-gradient(135deg, #0284c7 0%, #0369a1 100%);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary.btn-danger {
    background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
  }

  .btn-primary.btn-danger:hover:not(:disabled) {
    background: linear-gradient(135deg, #b91c1c 0%, #991b1b 100%);
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
</style>
