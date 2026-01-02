<script lang="ts">
  import { invoke } from '@/lib/tauri-wrapper';
  import XMarkIcon from '@/components/icons/XMarkIcon.svelte';
  import {
    readJsonFromFile,
    type ImportValidationResult,
    type ImportResult,
  } from '@/lib/import-export/json-serializer';

  interface Props {
    onClose: () => void;
    onImported: () => void;
  }

  const { onClose, onImported }: Props = $props();

  let importing = $state(false);
  let validating = $state(false);
  let error = $state<string | null>(null);
  let validationResult = $state<ImportValidationResult | null>(null);
  let importResult = $state<ImportResult | null>(null);
  let jsonData = $state<string>('');

  let fileInput: HTMLInputElement;

  // Обработчик клавиатуры для доступности
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      onClose();
    }
  }

  async function handleFileSelect(event: Event) {
    const target = event.target as HTMLInputElement;
    const file = target.files?.[0];
    if (!file) {
      return;
    }

    try {
      const data = await readJsonFromFile(file);
      jsonData = data;
      await validateJson();
    } catch (e: unknown) {
      error = e?.toString() || 'Ошибка при чтении файла';
    }
  }

  async function validateJson() {
    if (!jsonData.trim()) {
      validationResult = null;
      return;
    }

    validating = true;
    error = null;

    try {
      const result = await invoke<ImportValidationResult>('validate_import_json', {
        json_data: jsonData,
      });
      validationResult = result;
    } catch (e: unknown) {
      error = e?.toString() || 'Ошибка при валидации JSON';
      validationResult = null;
    } finally {
      validating = false;
    }
  }

  async function handleImport() {
    if (!jsonData.trim()) {
      error = 'Выберите файл или вставьте JSON';
      return;
    }

    if (!validationResult?.valid) {
      error = 'Исправьте ошибки перед импортом';
      return;
    }

    importing = true;
    error = null;
    importResult = null;

    try {
      const result = await invoke<ImportResult>('import_file', {
        json_data: jsonData,
      });
      importResult = result;
      onImported();
    } catch (e: unknown) {
      error = e?.toString() || 'Ошибка при импорте файла';
    } finally {
      importing = false;
    }
  }

  function handleClose() {
    onClose();
  }

  function handlePaste(event: ClipboardEvent) {
    const text = event.clipboardData?.getData('text');
    if (text) {
      jsonData = text;
      validateJson();
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
    aria-labelledby="import-dialog-title"
    tabindex="-1"
  >
    <div class="modal-header">
      <h2 id="import-dialog-title">Импорт файла</h2>
      <button class="btn-close" onclick={handleClose}>
        <XMarkIcon class="icon" />
      </button>
    </div>

    <div class="modal-body">
      {#if error}
        <div class="error">{error}</div>
      {/if}

      <div class="info-box">
        <p>
          Импортируйте файл из JSON формата. Файл будет создан или обновлен, если уже существует с
          таким же name@version.
        </p>
      </div>

      <div class="form-group">
        <label>Выберите JSON файл</label>
        <input
          type="file"
          accept=".json,application/json"
          onchange={handleFileSelect}
          bind:this={fileInput}
          class="file-input"
        />
      </div>

      <div class="form-group">
        <label>Или вставьте JSON вручную</label>
        <textarea
          bind:value={jsonData}
          onpaste={handlePaste}
          placeholder={`{"version": "1.0", "type": "file", "data": "..."}`}
          rows="10"
          class="json-input"
        ></textarea>
        <button
          type="button"
          class="btn-secondary btn-small"
          onclick={validateJson}
          disabled={validating || !jsonData.trim()}
        >
          {validating ? 'Валидация...' : 'Проверить JSON'}
        </button>
      </div>

      {#if validationResult}
        <div class="validation-result">
          <h3>Результат валидации</h3>

          {#if validationResult.valid}
            <div class="validation-success">
              <p>✅ JSON валиден</p>
              {#if validationResult.type}
                <p><strong>Тип:</strong> {validationResult.type}</p>
              {/if}
              {#if validationResult.version}
                <p><strong>Версия схемы:</strong> {validationResult.version}</p>
              {/if}
              {#if validationResult.preview}
                <div class="preview">
                  <p><strong>Превью:</strong></p>
                  <ul>
                    <li>Файлов: {validationResult.preview.files_count}</li>
                    <li>Зависимостей: {validationResult.preview.dependencies_count}</li>
                  </ul>
                </div>
              {/if}
            </div>
          {:else}
            <div class="validation-error">
              <p>❌ JSON содержит ошибки:</p>
              <ul>
                {#each validationResult.errors as err, index (index)}
                  <li>{err}</li>
                {/each}
              </ul>
            </div>
          {/if}

          {#if validationResult.warnings.length > 0}
            <div class="validation-warnings">
              <p>⚠️ Предупреждения:</p>
              <ul>
                {#each validationResult.warnings as warning, index (index)}
                  <li>{warning}</li>
                {/each}
              </ul>
            </div>
          {/if}
        </div>
      {/if}

      {#if importResult}
        <div class="import-result">
          <h3>Результат импорта</h3>

          {#if importResult.file_id}
            <div class="import-success">
              <p>
                ✅ Файл {importResult.created ? 'создан' : 'обновлен'} (ID: {importResult.file_id})
              </p>
            </div>
          {/if}

          {#if importResult.missing_dependencies.length > 0}
            <div class="missing-deps">
              <p>⚠️ Отсутствующие зависимости ({importResult.missing_dependencies.length}):</p>
              <ul>
                {#each importResult.missing_dependencies as dep (dep.name)}
                  <li>
                    {dep.target_file_name}
                    {#if dep.target_file_version}@{dep.target_file_version}{/if}
                    {#if dep.available_versions.length > 0}
                      <span class="available-versions">
                        (доступны: {dep.available_versions.join(', ')})
                      </span>
                    {/if}
                  </li>
                {/each}
              </ul>
            </div>
          {/if}

          {#if importResult.warnings.length > 0}
            <div class="import-warnings">
              <p>⚠️ Предупреждения:</p>
              <ul>
                {#each importResult.warnings as warning, index (index)}
                  <li>{warning}</li>
                {/each}
              </ul>
            </div>
          {/if}
        </div>
      {/if}
    </div>

    <div class="modal-actions">
      <button type="button" class="btn-secondary" onclick={handleClose}> Закрыть </button>
      {#if validationResult?.valid && !importResult}
        <button type="button" class="btn-primary" onclick={handleImport} disabled={importing}>
          {importing ? 'Импорт...' : 'Импортировать'}
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
    max-width: 700px;
    max-height: 90vh;
    overflow-y: auto;
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

  .info-box {
    padding: 0.75rem;
    background: #1e3a8a;
    color: #93c5fd;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .form-group {
    margin-bottom: 1.25rem;
  }

  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 600;
    color: #cbd5e1;
    font-size: 0.875rem;
  }

  .file-input {
    width: 100%;
    padding: 0.75rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    color: #e2e8f0;
    font-size: 0.95rem;
  }

  .json-input {
    width: 100%;
    padding: 0.75rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    color: #e2e8f0;
    font-size: 0.85rem;
    font-family: 'Courier New', monospace;
    resize: vertical;
    min-height: 150px;
  }

  .json-input:focus {
    outline: none;
    border-color: #0ea5e9;
  }

  .btn-small {
    padding: 0.5rem 1rem;
    margin-top: 0.5rem;
    font-size: 0.875rem;
  }

  .validation-result,
  .import-result {
    margin-top: 1.5rem;
    padding: 1rem;
    background: #0f172a;
    border-radius: 0.5rem;
    border: 1px solid #334155;
  }

  .validation-result h3,
  .import-result h3 {
    margin: 0 0 0.75rem 0;
    font-size: 1.125rem;
    color: #e2e8f0;
  }

  .validation-success {
    color: #86efac;
  }

  .validation-error {
    color: #fca5a5;
  }

  .validation-warnings,
  .import-warnings {
    margin-top: 0.75rem;
    padding: 0.75rem;
    background: #78350f;
    color: #fcd34d;
    border-radius: 0.5rem;
  }

  .import-success {
    color: #86efac;
    margin-bottom: 0.75rem;
  }

  .missing-deps {
    margin-top: 0.75rem;
    padding: 0.75rem;
    background: #78350f;
    color: #fcd34d;
    border-radius: 0.5rem;
  }

  .available-versions {
    font-size: 0.875rem;
    color: #94a3b8;
  }

  .preview ul,
  .validation-error ul,
  .validation-warnings ul,
  .missing-deps ul,
  .import-warnings ul {
    margin: 0.5rem 0 0 1.5rem;
    padding: 0;
  }

  .preview li,
  .validation-error li,
  .validation-warnings li,
  .missing-deps li,
  .import-warnings li {
    margin: 0.25rem 0;
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

  .btn-secondary:hover:not(:disabled) {
    background: #334155;
    color: #e2e8f0;
  }

  .btn-secondary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
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
