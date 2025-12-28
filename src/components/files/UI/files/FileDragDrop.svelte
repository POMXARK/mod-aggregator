<script lang="ts">
  import { SvelteMap } from 'svelte/reactivity';
  import { SessionStateManager } from '@/lib/session/session-state';
  import type { File } from '@/types/file';

  interface Props {
    files: File[];
    fileOrder?: number[];
    onOrderChange?: (newOrder: number[]) => void;
  }

  let { files = $bindable([]), fileOrder = $bindable([]), onOrderChange }: Props = $props();

  let draggedFileId = $state<number | null>(null);
  let dragOverFileId = $state<number | null>(null);
  const saveFileOrderDebounced = $derived(SessionStateManager.createDebouncedFileOrderSaver(500));

  // Синхронизируем порядок файлов
  const orderedFiles = $derived(() => {
    if (fileOrder.length === 0) {
      return [...files];
    }

    // Создаем карту для быстрого поиска
    const fileMap = new SvelteMap(files.map(f => [f.id, f]));
    const ordered: File[] = [];

    // Добавляем файлы в порядке fileOrder
    for (const fileId of fileOrder) {
      const file = fileMap.get(fileId);
      if (file) {
        ordered.push(file);
        fileMap.delete(fileId);
      }
    }

    // Добавляем оставшиеся файлы (которых нет в fileOrder)
    for (const file of files) {
      if (fileMap.has(file.id)) {
        ordered.push(file);
      }
    }

    return ordered;
  });

  function handleDragStart(fileId: number, event: DragEvent) {
    draggedFileId = fileId;
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
    dragOverFileId = fileId;
  }

  function handleDragLeave() {
    dragOverFileId = null;
  }

  function handleDrop(targetFileId: number, event: DragEvent) {
    event.preventDefault();
    dragOverFileId = null;

    if (draggedFileId === null || draggedFileId === targetFileId) {
      draggedFileId = null;
      return;
    }

    // Переупорядочиваем файлы
    const currentOrder = fileOrder.length > 0 ? [...fileOrder] : files.map(f => f.id);
    const draggedIndex = currentOrder.indexOf(draggedFileId);
    const targetIndex = currentOrder.indexOf(targetFileId);

    if (draggedIndex === -1 || targetIndex === -1) {
      draggedFileId = null;
      return;
    }

    // Удаляем из старой позиции и вставляем в новую
    currentOrder.splice(draggedIndex, 1);
    currentOrder.splice(targetIndex, 0, draggedFileId);

    // Обновляем порядок
    fileOrder = currentOrder;

    // Сохраняем с debounce
    if (saveFileOrderDebounced) {
      saveFileOrderDebounced(currentOrder);
    }

    if (onOrderChange) {
      onOrderChange(currentOrder);
    }

    draggedFileId = null;
  }

  function handleDragEnd() {
    draggedFileId = null;
    dragOverFileId = null;
  }
</script>

<div class="file-drag-drop">
  {#each orderedFiles as file (file.id)}
    <div
      class="file-item"
      class:dragging={draggedFileId === file.id}
      class:drag-over={dragOverFileId === file.id}
      draggable="true"
      ondragstart={e => handleDragStart(file.id, e)}
      ondragover={e => handleDragOver(file.id, e)}
      ondragleave={handleDragLeave}
      ondrop={e => handleDrop(file.id, e)}
      ondragend={handleDragEnd}
    >
      <div class="drag-handle">⋮⋮</div>
      <div class="file-info">
        <div class="file-name">{file.name}@{file.version}</div>
        {#if file.path}
          <div class="file-path">{file.path}</div>
        {/if}
      </div>
    </div>
  {/each}
</div>

<style>
  .file-drag-drop {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .file-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background: #1e293b;
    border: 2px solid #334155;
    border-radius: 0.375rem;
    cursor: move;
    transition: all 0.2s;
    user-select: none;
  }

  .file-item:hover {
    border-color: #475569;
    background: #0f172a;
  }

  .file-item.dragging {
    opacity: 0.5;
    border-color: #0ea5e9;
  }

  .file-item.drag-over {
    border-color: #10b981;
    background: #064e3b;
    transform: translateY(-2px);
  }

  .drag-handle {
    color: #64748b;
    font-size: 1.25rem;
    cursor: grab;
    user-select: none;
    flex-shrink: 0;
  }

  .drag-handle:active {
    cursor: grabbing;
  }

  .file-item.dragging .drag-handle {
    cursor: grabbing;
  }

  .file-info {
    flex: 1;
    min-width: 0;
  }

  .file-name {
    color: #e2e8f0;
    font-weight: 500;
    font-size: 0.875rem;
    margin-bottom: 0.25rem;
    word-break: break-word;
  }

  .file-path {
    color: #94a3b8;
    font-size: 0.75rem;
    font-family: monospace;
    word-break: break-all;
  }
</style>
