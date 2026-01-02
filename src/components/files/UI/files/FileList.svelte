<script lang="ts">
  import { onMount } from 'svelte';
  import { SvelteSet } from 'svelte/reactivity';
  import { invoke } from '@/lib/tauri-wrapper';
  import type { File } from '@/types/file';
  import FileForm from './FileForm.svelte';
  import { PlusIcon } from '@/components/icons';

  interface Props {
    selectedFiles?: number[];
    onSelectionChange?: (selectedIds: number[]) => void;
    onFileClick?: (file: File) => void;
  }

  let {
    selectedFiles: externalSelected = $bindable([]),
    onSelectionChange,
    onFileClick,
  }: Props = $props();

  let files = $state<File[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let selectedFiles = $state<Set<number>>(new Set(externalSelected));
  let dragStartIndex = $state<number | null>(null);
  let dragEndIndex = $state<number | null>(null);
  let showFileForm = $state(false);
  let editingFile = $state<File | null>(null);

  // Инициализация selectedFiles из externalSelected при монтировании
  onMount(() => {
    selectedFiles = new Set(externalSelected);
  });

  // Функция для обновления выбора (используется вместо прямого изменения selectedFiles)
  function updateSelection(newSelection: Set<number>) {
    selectedFiles = newSelection;
    const selectionArray = Array.from(newSelection);
    if (onSelectionChange) {
      onSelectionChange(selectionArray);
    }
    externalSelected = selectionArray;
  }

  onMount(async () => {
    // Инициализируем selectedFiles из externalSelected
    selectedFiles = new Set(externalSelected);
    await loadFiles();
  });

  async function loadFiles() {
    loading = true;
    error = null;

    try {
      // Используем команду для получения всех файлов
      files = await invoke<File[]>('get_all_files');
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при загрузке файлов';
      console.error('Failed to load files:', e);
    } finally {
      loading = false;
    }
  }

  function handleFileClick(file: File, event: MouseEvent) {
    if (onFileClick) {
      onFileClick(file);
    }

    // Обработка выбора
    if (event.ctrlKey || event.metaKey) {
      // Ctrl/Cmd + Click: переключение выбора
      toggleSelection(file.id);
    } else if (event.shiftKey && selectedFiles.size > 0) {
      // Shift + Click: выбор диапазона
      selectRange(file.id);
    } else {
      // Обычный клик: одиночный выбор
      selectSingle(file.id);
    }
  }

  function toggleSelection(fileId: number) {
    const newSelected = new SvelteSet(selectedFiles);
    if (newSelected.has(fileId)) {
      newSelected.delete(fileId);
    } else {
      newSelected.add(fileId);
    }
    updateSelection(newSelected);
  }

  function selectSingle(fileId: number) {
    updateSelection(new Set([fileId]));
  }

  function selectRange(fileId: number) {
    const fileIds = files.map(f => f.id);
    const currentIndex = fileIds.indexOf(fileId);
    const lastSelected = Array.from(selectedFiles).pop();

    if (lastSelected === undefined) {
      selectSingle(fileId);
      return;
    }

    const lastIndex = fileIds.indexOf(lastSelected);
    const start = Math.min(currentIndex, lastIndex);
    const end = Math.max(currentIndex, lastIndex);

    const rangeIds = fileIds.slice(start, end + 1);
    updateSelection(new Set([...selectedFiles, ...rangeIds]));
  }

  function handleSelectAll() {
    if (selectedFiles.size === files.length) {
      updateSelection(new Set());
    } else {
      updateSelection(new Set(files.map(f => f.id)));
    }
  }

  function handleAddFile() {
    editingFile = null;
    showFileForm = true;
  }

  function handleCloseForm() {
    showFileForm = false;
    editingFile = null;
  }

  async function handleFormSubmit() {
    await loadFiles();
    handleCloseForm();
  }

  function handleDragStart(fileId: number, event: DragEvent) {
    if (!selectedFiles.has(fileId)) {
      selectSingle(fileId);
    }
    dragStartIndex = files.findIndex(f => f.id === fileId);
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = 'move';
      event.dataTransfer.setData('text/plain', fileId.toString());
    }
  }

  function handleDragOver(fileId: number, event: DragEvent) {
    event.preventDefault();
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = 'move';
    }
    dragEndIndex = files.findIndex(f => f.id === fileId);
  }

  function handleDrop(fileId: number, event: DragEvent) {
    event.preventDefault();
    if (dragStartIndex !== null && dragEndIndex !== null && dragStartIndex !== dragEndIndex) {
      // Здесь можно добавить логику переупорядочивания файлов
      // Пока просто сбрасываем индексы
    }
    dragStartIndex = null;
    dragEndIndex = null;
  }

  function handleDragEnd() {
    dragStartIndex = null;
    dragEndIndex = null;
  }

  function isSelected(fileId: number): boolean {
    return selectedFiles.has(fileId);
  }

  function getFileDisplayName(file: File): string {
    return `${file.name}@${file.version}`;
  }
</script>

<div class="file-list">
  <div class="header">
    <div class="header-left">
      <h2>Файлы</h2>
      {#if files.length > 0}
        <span class="file-count">
          {files.length}
          {files.length === 1 ? 'файл' : 'файлов'}
          {selectedFiles.size > 0 && `(${selectedFiles.size} выбрано)`}
        </span>
      {/if}
    </div>
    <div class="header-actions">
      <button class="btn-primary" onclick={handleAddFile}>
        <PlusIcon class="icon-small" />
        Добавить файл
      </button>
      {#if files.length > 0}
        <button type="button" class="btn-secondary" onclick={handleSelectAll}>
          {selectedFiles.size === files.length ? 'Снять выбор' : 'Выбрать все'}
        </button>
      {/if}
      <button type="button" class="btn-secondary" onclick={loadFiles} disabled={loading}>
        {loading ? 'Загрузка...' : 'Обновить'}
      </button>
    </div>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if loading && files.length === 0}
    <div class="loading">Загрузка файлов...</div>
  {:else if files.length === 0}
    <div class="empty">
      <p>Нет файлов.</p>
      <button class="btn-primary" onclick={handleAddFile}>
        <PlusIcon class="icon-small" />
        Добавить первый файл
      </button>
    </div>
  {:else}
    <div class="files-container">
      {#each files as file, index (file.id)}
        <div
          class="file-item"
          class:selected={isSelected(file.id)}
          class:dragging={dragStartIndex === index}
          class:drag-over={dragEndIndex === index}
          draggable="true"
          ondragstart={e => handleDragStart(file.id, e)}
          ondragover={e => handleDragOver(file.id, e)}
          ondrop={e => handleDrop(file.id, e)}
          ondragend={handleDragEnd}
          onclick={e => handleFileClick(file, e)}
          onkeydown={e => {
            if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault();
              handleFileClick(file, e);
            }
          }}
          role="button"
          tabindex="0"
        >
          <div class="file-checkbox">
            <input
              type="checkbox"
              checked={isSelected(file.id)}
              onchange={() => toggleSelection(file.id)}
              onclick={e => e.stopPropagation()}
            />
          </div>
          <div class="file-info">
            <div class="file-name">{getFileDisplayName(file)}</div>
            {#if file.path}
              <div class="file-path">{file.path}</div>
            {/if}
            <div class="file-meta">
              Создан: {new Date(file.createdAt).toLocaleDateString('ru-RU')}
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}

  {#if showFileForm}
    <FileForm file={editingFile} onClose={handleCloseForm} onSubmit={handleFormSubmit} />
  {/if}
</div>

{#if showFileForm}
  <FileForm file={editingFile} onClose={handleCloseForm} onSubmit={handleFormSubmit} />
{/if}

<style>
  .file-list {
    width: 100%;
    max-width: min(1200px, 95vw);
    margin: 0 auto;
    padding: clamp(1rem, 2vw, 1.5rem);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: clamp(1rem, 2vh, 1.5rem);
    flex-wrap: wrap;
    gap: clamp(0.75rem, 1.5vw, 1rem);
  }

  /* Планшетная адаптация заголовка */
  @media (max-width: 1023px) and (min-width: 768px) {
    .header {
      gap: clamp(0.5rem, 1vw, 0.75rem);
    }
  }

  /* Мобильная адаптация заголовка */
  @media (max-width: 767px) {
    .file-list {
      padding: clamp(0.75rem, 1.5vw, 1rem);
    }

    .header {
      flex-direction: column;
      align-items: stretch;
      gap: 0.75rem;
    }

    .header-left {
      width: 100%;
    }

    .header-actions {
      width: 100%;
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
    }

    .header-actions button {
      width: 100%;
      justify-content: center;
    }
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .header h2 {
    margin: 0;
    font-size: clamp(1.5rem, 2.5vw, 1.75rem);
    font-weight: 700;
    color: #e2e8f0;
    line-height: 1.2;
  }

  .file-count {
    padding: clamp(0.25rem, 0.5vw, 0.375rem) clamp(0.5rem, 1vw, 0.75rem);
    background: #1e293b;
    border-radius: clamp(0.25rem, 0.5vw, 0.375rem);
    color: #94a3b8;
    font-size: clamp(0.75rem, 1vw, 0.875rem);
  }

  .header-actions {
    display: flex;
    gap: 0.5rem;
  }

  .btn-secondary {
    padding: clamp(0.5rem, 0.75vw, 0.625rem) clamp(0.875rem, 1.5vw, 1rem);
    background: #334155;
    color: #e2e8f0;
    border: 1px solid #475569;
    border-radius: clamp(0.375rem, 0.5vw, 0.5rem);
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    font-size: clamp(0.8125rem, 1vw, 0.875rem);
    white-space: nowrap;
  }

  .btn-primary {
    padding: clamp(0.5rem, 0.75vw, 0.625rem) clamp(0.875rem, 1.5vw, 1rem);
    background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
    color: white;
    border: none;
    border-radius: clamp(0.375rem, 0.5vw, 0.5rem);
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    font-size: clamp(0.8125rem, 1vw, 0.875rem);
    display: flex;
    align-items: center;
    gap: clamp(0.375rem, 0.5vw, 0.5rem);
    white-space: nowrap;
  }

  .btn-primary:hover {
    background: linear-gradient(135deg, #0284c7 0%, #0369a1 100%);
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(14, 165, 233, 0.3);
  }

  .btn-primary .icon-small {
    width: 1rem;
    height: 1rem;
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

  .loading,
  .empty {
    text-align: center;
    padding: 3rem;
    color: #64748b;
  }

  .files-container {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .file-item {
    display: flex;
    align-items: flex-start;
    gap: clamp(0.75rem, 1.5vw, 1rem);
    padding: clamp(0.75rem, 1.5vw, 1rem);
    background: #1e293b;
    border: 2px solid #334155;
    border-radius: clamp(0.375rem, 0.75vw, 0.5rem);
    cursor: pointer;
    transition: all 0.2s;
    user-select: none;
  }

  /* Планшетная адаптация элементов файлов */
  @media (max-width: 1023px) and (min-width: 768px) {
    .file-item {
      padding: clamp(0.75rem, 1.25vw, 0.875rem);
      gap: clamp(0.625rem, 1vw, 0.875rem);
    }
  }

  /* Мобильная адаптация элементов файлов */
  @media (max-width: 767px) {
    .file-item {
      padding: clamp(0.625rem, 1.5vw, 0.75rem);
      gap: clamp(0.5rem, 1vw, 0.75rem);
      flex-direction: column;
    }

    .file-info {
      width: 100%;
    }

    .file-name {
      font-size: clamp(0.8125rem, 1.1vw, 0.875rem);
      word-break: break-word;
    }

    .file-path {
      font-size: clamp(0.6875rem, 0.95vw, 0.75rem);
      word-break: break-all;
    }
  }

  .file-item:hover {
    border-color: #475569;
    background: #0f172a;
  }

  .file-item.selected {
    border-color: #0ea5e9;
    background: #0c4a6e;
  }

  .file-item.dragging {
    opacity: 0.5;
  }

  .file-item.drag-over {
    border-color: #10b981;
    background: #064e3b;
  }

  .file-checkbox {
    flex-shrink: 0;
    margin-top: 0.25rem;
  }

  .file-checkbox input[type='checkbox'] {
    width: 1.25rem;
    height: 1.25rem;
    cursor: pointer;
    accent-color: #0ea5e9;
  }

  .file-info {
    flex: 1;
    min-width: 0;
  }

  .file-name {
    color: #e2e8f0;
    font-weight: 600;
    font-size: clamp(0.9375rem, 1.2vw, 1rem);
    margin-bottom: clamp(0.375rem, 0.5vw, 0.5rem);
    word-break: break-word;
    line-height: 1.4;
  }

  .file-path {
    color: #94a3b8;
    font-size: clamp(0.75rem, 1vw, 0.875rem);
    font-family: monospace;
    margin-bottom: clamp(0.125rem, 0.25vw, 0.25rem);
    word-break: break-all;
    line-height: 1.4;
  }

  .file-meta {
    color: #64748b;
    font-size: clamp(0.6875rem, 0.9vw, 0.75rem);
    margin-top: clamp(0.375rem, 0.5vw, 0.5rem);
    line-height: 1.4;
  }

  .icon-small {
    width: 1rem;
    height: 1rem;
  }
</style>
