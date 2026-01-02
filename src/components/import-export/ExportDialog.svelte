<script lang="ts">
  import { invoke } from '@/lib/tauri-wrapper';
  import XMarkIcon from '@/components/icons/XMarkIcon.svelte';
  import { downloadJson } from '@/lib/import-export/json-serializer';
  import type { File } from '@/types/file';

  interface Props {
    file: File | null;
    onClose: () => void;
    onExported: () => void;
  }

  const { file, onClose, onExported }: Props = $props();

  let exporting = $state(false);
  let error = $state<string | null>(null);
  let exportData = $state<string | null>(null);

  // Обработчик клавиатуры для доступности
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      onClose();
    }
  }

  async function handleExport() {
    if (!file) {
      error = 'Файл не выбран';
      return;
    }

    exporting = true;
    error = null;
    exportData = null;

    try {
      const json = await invoke<string>('export_file', { file_id: file.id });
      exportData = json;

      // Скачиваем файл
      const filename = `${file.name}@${file.version}.json`;
      downloadJson(json, filename);

      onExported();
    } catch (e: unknown) {
      error = e?.toString() || 'Ошибка при экспорте файла';
    } finally {
      exporting = false;
    }
  }

  function handleClose() {
    onClose();
  }

  function copyToClipboard() {
    if (exportData) {
      navigator.clipboard.writeText(exportData);
      // Можно показать уведомление об успешном копировании
    }
  }
</script>

<div
  class="modal-overlay"
  onclick={handleClose}
  onkeydown={handleKeydown}
  role="presentation"
  tabindex="-1"
>
  <div
    class="modal-content"
    onclick={e => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
    aria-labelledby="export-dialog-title"
    tabindex="-1"
  >
    <div class="modal-header">
      <h2 id="export-dialog-title">Экспорт файла</h2>
      <button class="btn-close" onclick={handleClose}>
        <XMarkIcon class="icon" />
      </button>
    </div>

    <div class="modal-body">
      {#if error}
        <div class="error">{error}</div>
      {/if}

      {#if file}
        <div class="file-info">
          <p><strong>Файл:</strong> {file.name}@{file.version}</p>
          {#if file.path}
            <p><strong>Путь:</strong> {file.path}</p>
          {/if}
        </div>

        <div class="info-box">
          <p>
            Экспорт создаст JSON файл со всеми параметрами файла и его зависимостями. Файл можно
            использовать для импорта на другом устройстве или для резервного копирования.
          </p>
        </div>

        {#if exportData}
          <div class="success-box">
            <p>✅ Файл успешно экспортирован и скачан!</p>
            <button class="btn-secondary" onclick={copyToClipboard}>
              Копировать JSON в буфер обмена
            </button>
          </div>
        {/if}
      {:else}
        <div class="error">Файл не выбран</div>
      {/if}
    </div>

    <div class="modal-actions">
      <button type="button" class="btn-secondary" onclick={handleClose}> Закрыть </button>
      {#if file && !exportData}
        <button type="button" class="btn-primary" onclick={handleExport} disabled={exporting}>
          {exporting ? 'Экспорт...' : 'Экспортировать'}
        </button>
      {/if}
    </div>
  </div>
</div>

<style>
  .modal-overlay {
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
    padding: 2rem;
  }

  .modal-content {
    background: #1e293b;
    border-radius: 1rem;
    width: 100%;
    max-width: 600px;
    border: 1px solid #334155;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #334155;
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 700;
    color: #e2e8f0;
  }

  .btn-close {
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    padding: 0.5rem;
    border-radius: 0.375rem;
    transition: all 0.2s;
  }

  .btn-close:hover {
    background: #334155;
    color: #e2e8f0;
  }

  .modal-body {
    padding: 1.5rem;
  }

  .file-info {
    margin-bottom: 1rem;
    padding: 1rem;
    background: #0f172a;
    border-radius: 0.5rem;
    border: 1px solid #334155;
  }

  .file-info p {
    margin: 0.5rem 0;
    color: #cbd5e1;
  }

  .info-box {
    padding: 0.75rem;
    background: #1e3a8a;
    color: #93c5fd;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .success-box {
    padding: 1rem;
    background: #14532d;
    color: #86efac;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
  }

  .success-box p {
    margin: 0 0 0.5rem 0;
  }

  .error {
    padding: 0.75rem;
    background: #7f1d1d;
    color: #fca5a5;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .modal-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    padding: 1.5rem;
    border-top: 1px solid #334155;
  }

  .btn-secondary {
    padding: 0.75rem 1.5rem;
    background: transparent;
    border: 1px solid #334155;
    color: #94a3b8;
    border-radius: 0.5rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-secondary:hover {
    background: #334155;
    color: #e2e8f0;
  }

  .btn-primary {
    padding: 0.75rem 1.5rem;
    background: #0ea5e9;
    color: white;
    border: none;
    border-radius: 0.5rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-primary:hover:not(:disabled) {
    background: #0284c7;
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .icon {
    width: 20px;
    height: 20px;
  }
</style>
