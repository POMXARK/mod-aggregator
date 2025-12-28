<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@/lib/tauri-wrapper';
  import type { CollectionFileView, CollectionEvaluationResult } from '@/types/collection';
  import { CollectionComposer } from '@/lib/collections/collection-composer';
  import { CollectionLogic } from '@/lib/collections/collection-logic';

  interface Props {
    collectionIds: number[];
    showEvaluation?: boolean;
  }

  let { collectionIds = $bindable(), showEvaluation = $bindable(true) }: Props = $props();

  let files = $state<CollectionFileView[]>([]);
  let evaluationResult = $state<CollectionEvaluationResult | null>(null);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let sortBy = $state<'name' | 'order' | 'collection'>('order');
  let showDisabled = $state(true);

  // Статистика
  const stats = $derived(() => {
    if (evaluationResult) {
      return CollectionLogic.getEvaluationStats(evaluationResult);
    }
    return {
      totalFiles: files.length,
      enabledCount: files.length,
      disabledCount: 0,
      enabledPercentage: 100,
    };
  });

  // Отфильтрованные файлы
  const filteredFiles = $derived(() => {
    let result = [...files];

    // Сортировка
    if (sortBy === 'name') {
      result.sort((a, b) => {
        const nameA = a.file.name.toLowerCase();
        const nameB = b.file.name.toLowerCase();
        if (nameA !== nameB) {
          return nameA.localeCompare(nameB);
        }
        return a.file.version.localeCompare(b.file.version);
      });
    } else if (sortBy === 'collection') {
      result.sort((a, b) => {
        const collA = a.collections[0]?.collectionName || '';
        const collB = b.collections[0]?.collectionName || '';
        return collA.localeCompare(collB);
      });
    } else {
      // sortBy === 'order'
      result = CollectionComposer.sortFilesByOrder(result);
    }

    // Фильтрация по статусу (если есть оценка)
    if (evaluationResult && !showDisabled) {
      result = result.filter(f => evaluationResult.enabledFiles.includes(f.fileId));
    }

    return result;
  });

  onMount(async () => {
    await loadFiles();
    if (showEvaluation && collectionIds.length > 0) {
      await evaluateCollection();
    }
  });

  $effect(() => {
    loadFiles();
    if (showEvaluation && collectionIds.length > 0) {
      evaluateCollection();
    }
  });

  async function loadFiles() {
    if (collectionIds.length === 0) {
      files = [];
      return;
    }

    loading = true;
    error = null;

    try {
      files = await CollectionComposer.getFilesFromMultipleCollections(collectionIds);
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при загрузке файлов';
      console.error('Failed to load collection files:', e);
    } finally {
      loading = false;
    }
  }

  async function evaluateCollection() {
    if (collectionIds.length === 0) {
      evaluationResult = null;
      return;
    }

    // Оцениваем первую коллекцию (можно расширить для нескольких)
    const collectionId = collectionIds[0];

    try {
      evaluationResult = await invoke<CollectionEvaluationResult>('evaluate_collection_logic', {
        collection_id: collectionId,
      });
    } catch (e: unknown) {
      console.warn('Failed to evaluate collection logic:', e);
      evaluationResult = null;
    }
  }

  function isFileEnabled(fileId: number): boolean {
    if (!evaluationResult) {
      return true;
    }
    return evaluationResult.enabledFiles.includes(fileId);
  }

  function getFileCollections(file: CollectionFileView): string {
    return file.collections.map(c => c.collectionName).join(', ');
  }
</script>

<div class="collection-viewer">
  <div class="header">
    <div class="header-left">
      <h2>Файлы коллекций</h2>
      {#if collectionIds.length > 0}
        <span class="collection-count">
          {collectionIds.length}
          {collectionIds.length === 1 ? 'коллекция' : 'коллекций'}
        </span>
      {/if}
    </div>
    <div class="header-actions">
      <button type="button" class="btn-secondary" onclick={loadFiles} disabled={loading}>
        {loading ? 'Загрузка...' : 'Обновить'}
      </button>
    </div>
  </div>

  {#if stats().totalFiles > 0 && showEvaluation && evaluationResult}
    <div class="stats">
      <div class="stat-item">
        <span class="stat-label">Всего файлов:</span>
        <span class="stat-value">{stats().totalFiles}</span>
      </div>
      <div class="stat-item stat-enabled">
        <span class="stat-label">Включено:</span>
        <span class="stat-value">{stats().enabledCount}</span>
        <span class="stat-percentage">({stats().enabledPercentage}%)</span>
      </div>
      <div class="stat-item stat-disabled">
        <span class="stat-label">Выключено:</span>
        <span class="stat-value">{stats().disabledCount}</span>
      </div>
    </div>
  {/if}

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if collectionIds.length === 0}
    <div class="empty">
      <p>Выберите коллекции для просмотра</p>
    </div>
  {:else if loading && files.length === 0}
    <div class="loading">Загрузка файлов...</div>
  {:else if files.length === 0}
    <div class="empty">
      <p>В выбранных коллекциях нет файлов</p>
    </div>
  {:else}
    <div class="controls">
      <div class="control-group">
        <label for="sort-select">Сортировка:</label>
        <select id="sort-select" bind:value={sortBy}>
          <option value="order">По порядку</option>
          <option value="name">По имени</option>
          <option value="collection">По коллекции</option>
        </select>
      </div>
      {#if showEvaluation && evaluationResult}
        <div class="control-group">
          <label>
            <input type="checkbox" bind:checked={showDisabled} />
            Показывать выключенные
          </label>
        </div>
      {/if}
    </div>

    <div class="files-list">
      {#each filteredFiles as file (file.fileId)}
        <div
          class="file-item"
          class:disabled={showEvaluation && evaluationResult && !isFileEnabled(file.fileId)}
        >
          <div class="file-header">
            <div class="file-name">
              <strong>{file.file.name}@{file.file.version}</strong>
              {#if showEvaluation && evaluationResult}
                {#if isFileEnabled(file.fileId)}
                  <span class="status-badge status-enabled">Включен</span>
                {:else}
                  <span class="status-badge status-disabled">Выключен</span>
                {/if}
              {/if}
            </div>
            <div class="file-collections">
              {getFileCollections(file)}
            </div>
          </div>
          {#if file.file.path}
            <div class="file-path">{file.file.path}</div>
          {/if}
          {#if file.collections.length > 1}
            <div class="file-meta">
              Присутствует в {file.collections.length} коллекциях
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .collection-viewer {
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
    flex-wrap: wrap;
    gap: 1rem;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .header h2 {
    margin: 0;
    font-size: 1.75rem;
    font-weight: 700;
    color: #e2e8f0;
  }

  .collection-count {
    padding: 0.25rem 0.75rem;
    background: #1e293b;
    border-radius: 0.375rem;
    color: #94a3b8;
    font-size: 0.875rem;
  }

  .header-actions {
    display: flex;
    gap: 0.5rem;
  }

  .btn-secondary {
    padding: 0.5rem 1rem;
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

  .stats {
    display: flex;
    gap: 1.5rem;
    padding: 1rem;
    background: #1e293b;
    border-radius: 0.5rem;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .stat-label {
    color: #94a3b8;
    font-size: 0.875rem;
  }

  .stat-value {
    color: #e2e8f0;
    font-weight: 600;
    font-size: 1rem;
  }

  .stat-percentage {
    color: #64748b;
    font-size: 0.875rem;
  }

  .stat-enabled .stat-value {
    color: #10b981;
  }

  .stat-disabled .stat-value {
    color: #ef4444;
  }

  .error {
    padding: 1rem;
    background: #7f1d1d;
    color: #fca5a5;
    border-radius: 0.375rem;
    margin-bottom: 1rem;
    border-left: 3px solid #ef4444;
  }

  .empty,
  .loading {
    text-align: center;
    padding: 3rem;
    color: #64748b;
  }

  .controls {
    display: flex;
    gap: 1.5rem;
    margin-bottom: 1.5rem;
    padding: 1rem;
    background: #1e293b;
    border-radius: 0.5rem;
    flex-wrap: wrap;
  }

  .control-group {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .control-group label {
    color: #e2e8f0;
    font-size: 0.875rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }

  .control-group select {
    padding: 0.5rem;
    background: #0f172a;
    color: #e2e8f0;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    font-size: 0.875rem;
  }

  .control-group input[type='checkbox'] {
    cursor: pointer;
  }

  .files-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .file-item {
    padding: 1rem;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    transition: all 0.2s;
  }

  .file-item:hover {
    border-color: #475569;
    background: #0f172a;
  }

  .file-item.disabled {
    opacity: 0.6;
    background: #0f172a;
  }

  .file-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 1rem;
    margin-bottom: 0.5rem;
    flex-wrap: wrap;
  }

  .file-name {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex: 1;
  }

  .file-name strong {
    color: #e2e8f0;
    font-size: 1rem;
  }

  .status-badge {
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .status-enabled {
    background: #065f46;
    color: #6ee7b7;
  }

  .status-disabled {
    background: #7f1d1d;
    color: #fca5a5;
  }

  .file-collections {
    color: #94a3b8;
    font-size: 0.875rem;
  }

  .file-path {
    color: #64748b;
    font-size: 0.875rem;
    margin-top: 0.5rem;
    font-family: monospace;
  }

  .file-meta {
    color: #94a3b8;
    font-size: 0.875rem;
    margin-top: 0.5rem;
  }
</style>
