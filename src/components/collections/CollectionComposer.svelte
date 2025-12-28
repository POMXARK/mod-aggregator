<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@/lib/tauri-wrapper';
  import { CollectionComposer as CollectionComposerUtil } from '@/lib/collections/collection-composer';
  import type { Collection } from '@/types/collection';

  interface Props {
    onCombined?: (collection: Collection) => void;
  }

  const { onCombined }: Props = $props();

  let allCollections = $state<Collection[]>([]);
  let selectedCollectionIds = $state<number[]>([]);
  let newCollectionName = $state('');
  let newCollectionDescription = $state('');
  let loading = $state(false);
  let error = $state<string | null>(null);
  let showForm = $state(false);
  let stats = $state<{
    totalFiles: number;
    uniqueFiles: number;
    filesPerCollection: Map<number, number>;
  } | null>(null);

  onMount(async () => {
    await loadCollections();
  });

  async function loadCollections() {
    loading = true;
    error = null;

    try {
      allCollections = await invoke<Collection[]>('get_collections');
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : 'Ошибка при загрузке коллекций';
      console.error('Failed to load collections:', e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    updateStats();
  });

  async function updateStats() {
    if (selectedCollectionIds.length === 0) {
      stats = null;
      return;
    }

    try {
      stats = await CollectionComposerUtil.getCollectionsStats(selectedCollectionIds);
    } catch (e: unknown) {
      console.warn('Failed to get stats:', e);
      stats = null;
    }
  }

  function toggleCollection(collectionId: number) {
    if (selectedCollectionIds.includes(collectionId)) {
      selectedCollectionIds = selectedCollectionIds.filter(id => id !== collectionId);
    } else {
      selectedCollectionIds = [...selectedCollectionIds, collectionId];
    }
  }

  function canCombine(): boolean {
    return selectedCollectionIds.length >= 2 && newCollectionName.trim().length > 0;
  }

  async function handleCombine() {
    if (!canCombine()) {
      return;
    }

    loading = true;
    error = null;

    try {
      const validation = await CollectionComposerUtil.canCombineCollections(selectedCollectionIds);

      if (!validation.canCombine) {
        error = validation.errors.join(', ');
        return;
      }

      const newCollection = await CollectionComposerUtil.combineCollections(
        newCollectionName.trim(),
        selectedCollectionIds,
        newCollectionDescription.trim() || undefined
      );

      // Сброс формы
      selectedCollectionIds = [];
      newCollectionName = '';
      newCollectionDescription = '';
      showForm = false;

      // Уведомление родительского компонента
      if (onCombined) {
        onCombined(newCollection);
      }

      // Перезагрузка коллекций
      await loadCollections();
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при объединении коллекций';
      console.error('Failed to combine collections:', e);
    } finally {
      loading = false;
    }
  }
</script>

<div class="collection-composer">
  <div class="header">
    <h2>Объединение коллекций</h2>
    <button type="button" class="btn-secondary" onclick={() => (showForm = !showForm)}>
      {showForm ? 'Скрыть форму' : 'Объединить коллекции'}
    </button>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if showForm}
    <div class="form-container">
      <div class="form-section">
        <h3>Выберите коллекции для объединения</h3>
        {#if loading && allCollections.length === 0}
          <div class="loading">Загрузка коллекций...</div>
        {:else if allCollections.length === 0}
          <div class="empty">Нет доступных коллекций</div>
        {:else}
          <div class="collections-list">
            {#each allCollections as collection (collection.id)}
              <label class="collection-item">
                <input
                  type="checkbox"
                  checked={selectedCollectionIds.includes(collection.id)}
                  onchange={() => toggleCollection(collection.id)}
                />
                <div class="collection-info">
                  <div class="collection-name">{collection.name}</div>
                  {#if collection.description}
                    <div class="collection-description">{collection.description}</div>
                  {/if}
                </div>
              </label>
            {/each}
          </div>
        {/if}
      </div>

      {#if selectedCollectionIds.length > 0}
        <div class="form-section">
          <h3>Статистика</h3>
          {#if stats}
            <div class="stats">
              <div class="stat-item">
                <span class="stat-label">Выбрано коллекций:</span>
                <span class="stat-value">{selectedCollectionIds.length}</span>
              </div>
              <div class="stat-item">
                <span class="stat-label">Всего файлов:</span>
                <span class="stat-value">{stats.totalFiles}</span>
              </div>
              <div class="stat-item">
                <span class="stat-label">Уникальных файлов:</span>
                <span class="stat-value">{stats.uniqueFiles}</span>
              </div>
            </div>
          {/if}
        </div>

        <div class="form-section">
          <h3>Новая коллекция</h3>
          <div class="form-fields">
            <div class="form-field">
              <label for="collection-name">Название *</label>
              <input
                id="collection-name"
                type="text"
                bind:value={newCollectionName}
                placeholder="Название объединенной коллекции"
                disabled={loading}
              />
            </div>
            <div class="form-field">
              <label for="collection-description">Описание</label>
              <textarea
                id="collection-description"
                bind:value={newCollectionDescription}
                placeholder="Описание коллекции (необязательно)"
                rows="3"
                disabled={loading}
              ></textarea>
            </div>
          </div>
        </div>

        <div class="form-actions">
          <button
            type="button"
            class="btn-primary"
            onclick={handleCombine}
            disabled={!canCombine() || loading}
          >
            {loading ? 'Объединение...' : 'Объединить коллекции'}
          </button>
          <button
            type="button"
            class="btn-secondary"
            onclick={() => {
              selectedCollectionIds = [];
              newCollectionName = '';
              newCollectionDescription = '';
            }}
            disabled={loading}
          >
            Очистить
          </button>
        </div>
      {/if}
    </div>
  {/if}

  {#if selectedCollectionIds.length > 0 && !showForm}
    <div class="selected-info">
      <p>
        Выбрано коллекций: <strong>{selectedCollectionIds.length}</strong>
        {#if selectedCollectionIds.length >= 2}
          <span class="ready-badge">Готово к объединению</span>
        {/if}
      </p>
    </div>
  {/if}
</div>

<style>
  .collection-composer {
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
  }

  .form-section {
    margin-bottom: 2rem;
  }

  .form-section:last-child {
    margin-bottom: 0;
  }

  .form-section h3 {
    margin: 0 0 1rem 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #e2e8f0;
  }

  .collections-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-height: 400px;
    overflow-y: auto;
  }

  .collection-item {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 1rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .collection-item:hover {
    border-color: #475569;
    background: #1e293b;
  }

  .collection-item input[type='checkbox'] {
    margin-top: 0.25rem;
    cursor: pointer;
  }

  .collection-info {
    flex: 1;
  }

  .collection-name {
    color: #e2e8f0;
    font-weight: 500;
    margin-bottom: 0.25rem;
  }

  .collection-description {
    color: #94a3b8;
    font-size: 0.875rem;
  }

  .stats {
    display: flex;
    gap: 1.5rem;
    padding: 1rem;
    background: #0f172a;
    border-radius: 0.375rem;
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
  .form-field textarea:focus {
    outline: none;
    border-color: #0ea5e9;
    box-shadow: 0 0 0 3px rgba(14, 165, 233, 0.1);
  }

  .form-field input:disabled,
  .form-field textarea:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .form-actions {
    display: flex;
    gap: 0.75rem;
    margin-top: 1.5rem;
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

  .loading,
  .empty {
    text-align: center;
    padding: 2rem;
    color: #64748b;
  }

  .selected-info {
    padding: 1rem;
    background: #1e293b;
    border-radius: 0.375rem;
    margin-top: 1rem;
  }

  .selected-info p {
    margin: 0;
    color: #e2e8f0;
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .ready-badge {
    padding: 0.25rem 0.75rem;
    background: #065f46;
    color: #6ee7b7;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    font-weight: 500;
  }
</style>
