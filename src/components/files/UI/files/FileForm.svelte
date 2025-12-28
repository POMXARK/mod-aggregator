<script lang="ts">
  import { invoke } from '@/lib/tauri-wrapper';
  import { XMarkIcon, PlusIcon, TrashIcon } from '@/components/icons';
  import type { File } from '@/types/file';
  import type { DependencyType } from '@/types/dependency';

  interface Props {
    file: File | null;
    onClose: () => void;
    onSubmit: () => void;
  }

  const { file, onClose, onSubmit }: Props = $props();

  // File fields
  const initialName = $derived(file?.name || '');
  const initialVersion = $derived(file?.version || '');
  const initialPath = $derived(file?.path || '');
  const initialMetadata = $derived(file?.metadata || {});

  let name = $state(initialName);
  let version = $state(initialVersion);
  let path = $state(initialPath);
  let metadataJson = $state(JSON.stringify(initialMetadata, null, 2));

  // Dependencies (only for new files, existing files manage dependencies separately)
  let dependencies = $state<
    Array<{
      targetFileName: string;
      targetFileVersion?: string;
      dependencyType: DependencyType;
    }>
  >([]);

  // Dependency form
  let showDependencyForm = $state(false);
  let depTargetName = $state('');
  let depTargetVersion = $state<string | undefined>(undefined);
  let depType = $state<DependencyType>('required');

  // State
  let saving = $state(false);
  let error = $state<string | null>(null);
  let validationErrors = $state<Record<string, string>>({});

  // Load all files for dependency selection
  let allFiles = $state<File[]>([]);

  $effect(() => {
    loadAllFiles();
  });

  // Update state when file prop changes
  $effect(() => {
    if (file) {
      name = file.name || '';
      version = file.version || '';
      path = file.path || '';
      metadataJson = JSON.stringify(file.metadata || {}, null, 2);
      // For existing files, don't show dependencies in form (they're managed separately)
      dependencies = [];
    } else {
      name = '';
      version = '';
      path = '';
      metadataJson = '{}';
      dependencies = [];
    }
  });

  async function loadAllFiles() {
    try {
      allFiles = await invoke('get_file_versions', { name: '' });
    } catch {
      console.error('Failed to load files');
    }
  }

  function validateForm(): boolean {
    validationErrors = {};

    if (!name.trim()) {
      validationErrors.name = 'Имя файла обязательно';
    }

    if (!version.trim()) {
      validationErrors.version = 'Версия файла обязательна';
    }

    // Validate metadata JSON
    try {
      if (metadataJson.trim()) {
        JSON.parse(metadataJson);
      }
    } catch {
      validationErrors.metadata = 'Некорректный JSON в метаданных';
    }

    return Object.keys(validationErrors).length === 0;
  }

  async function handleSubmit() {
    if (!validateForm()) {
      error = 'Исправьте ошибки в форме';
      return;
    }

    saving = true;
    error = null;

    try {
      let metadata = {};
      if (metadataJson.trim()) {
        metadata = JSON.parse(metadataJson);
      }

      if (file) {
        // Update existing file
        await invoke('update_file', {
          params: {
            id: file.id,
            name: name.trim() || undefined,
            version: version.trim() || undefined,
            path: path.trim() || undefined,
            metadata: Object.keys(metadata).length > 0 ? metadata : undefined,
          },
        });
      } else {
        // Create new file with dependencies
        await invoke('create_file', {
          params: {
            name: name.trim(),
            version: version.trim(),
            path: path.trim() || undefined,
            metadata: Object.keys(metadata).length > 0 ? metadata : undefined,
            dependencies:
              dependencies.length > 0
                ? dependencies.map(dep => ({
                    target_file_name: dep.targetFileName,
                    target_file_version: dep.targetFileVersion,
                    dependency_type: dep.dependencyType,
                  }))
                : undefined,
          },
        });
      }
      onSubmit();
    } catch (e: unknown) {
      error = e?.toString() || 'Ошибка при сохранении файла';
    } finally {
      saving = false;
    }
  }

  function handleClose() {
    onClose();
  }

  function addDependency() {
    if (!depTargetName.trim()) {
      error = 'Укажите имя файла зависимости';
      return;
    }

    dependencies = [
      ...dependencies,
      {
        targetFileName: depTargetName.trim(),
        targetFileVersion: depTargetVersion?.trim() || undefined,
        dependencyType: depType,
      },
    ];

    // Reset form
    depTargetName = '';
    depTargetVersion = undefined;
    depType = 'required';
    showDependencyForm = false;
  }

  function removeDependency(index: number) {
    dependencies = dependencies.filter((_, i) => i !== index);
  }

  const availableFileNames = $derived(Array.from(new Set(allFiles.map(f => f.name))).sort());

  function getAvailableVersions(fileName: string): string[] {
    return allFiles
      .filter(f => f.name === fileName)
      .map(f => f.version)
      .sort();
  }
</script>

<div class="modal-overlay" onclick={handleClose}>
  <div class="modal-content" onclick={e => e.stopPropagation()}>
    <div class="modal-header">
      <h2>{file ? 'Редактировать файл' : 'Добавить файл'}</h2>
      <button class="btn-close" onclick={handleClose}>
        <XMarkIcon class="icon" />
      </button>
    </div>

    <form
      class="form"
      onsubmit={e => {
        e.preventDefault();
        handleSubmit();
      }}
    >
      {#if error}
        <div class="error">{error}</div>
      {/if}

      <div class="form-group">
        <label>Имя файла *</label>
        <input
          type="text"
          bind:value={name}
          required
          placeholder="my-mod"
          class={validationErrors.name ? 'error-input' : ''}
        />
        {#if validationErrors.name}
          <span class="field-error">{validationErrors.name}</span>
        {/if}
      </div>

      <div class="form-group">
        <label>Версия *</label>
        <input
          type="text"
          bind:value={version}
          required
          placeholder="1.0.0"
          class={validationErrors.version ? 'error-input' : ''}
        />
        {#if validationErrors.version}
          <span class="field-error">{validationErrors.version}</span>
        {/if}
        <small class="form-hint">Формат: name@version (например: my-mod@1.0.0)</small>
      </div>

      <div class="form-group">
        <label>Путь к файлу</label>
        <input type="text" bind:value={path} placeholder="/path/to/file" />
      </div>

      <div class="form-group">
        <label>Метаданные (JSON)</label>
        <textarea
          bind:value={metadataJson}
          rows="6"
          placeholder={`{"author": "Author Name", "description": "..."}`}
          class={validationErrors.metadata ? 'error-input' : ''}
        ></textarea>
        {#if validationErrors.metadata}
          <span class="field-error">{validationErrors.metadata}</span>
        {/if}
        <small class="form-hint">Дополнительные метаданные в формате JSON</small>
      </div>

      {#if !file}
        <!-- Dependencies section (only for new files) -->
        <h3 class="section-title">Зависимости</h3>

        {#if dependencies.length > 0}
          <div class="dependencies-list">
            {#each dependencies as dep, index (dep.targetFileName + dep.targetFileVersion)}
              <div class="dependency-item">
                <div class="dependency-info">
                  <span class="dependency-name">{dep.targetFileName}</span>
                  {#if dep.targetFileVersion}
                    <span class="dependency-version">@{dep.targetFileVersion}</span>
                  {:else}
                    <span class="dependency-version">@любая</span>
                  {/if}
                  <span class="dependency-type">{dep.dependencyType}</span>
                </div>
                <button
                  type="button"
                  class="btn-icon-small"
                  onclick={() => removeDependency(index)}
                >
                  <TrashIcon class="icon-small" />
                </button>
              </div>
            {/each}
          </div>
        {/if}

        {#if showDependencyForm}
          <div class="dependency-form">
            <div class="form-group">
              <label>Имя файла зависимости *</label>
              <input
                type="text"
                bind:value={depTargetName}
                list="file-names"
                placeholder="Выберите или введите имя"
                required
              />
              <datalist id="file-names">
                {#each availableFileNames as fileName (fileName)}
                  <option value={fileName}></option>
                {/each}
              </datalist>
            </div>

            <div class="form-group">
              <label>Версия (опционально)</label>
              {#if depTargetName}
                <select bind:value={depTargetVersion}>
                  <option value="">Любая версия</option>
                  {#each getAvailableVersions(depTargetName) as ver (ver)}
                    <option value={ver}>{ver}</option>
                  {/each}
                </select>
              {:else}
                <input
                  type="text"
                  bind:value={depTargetVersion}
                  placeholder="1.0.0 или оставьте пустым"
                />
              {/if}
            </div>

            <div class="form-group">
              <label>Тип зависимости</label>
              <select bind:value={depType}>
                <option value="required">Обязательная</option>
                <option value="optional">Опциональная</option>
                <option value="peer">Peer</option>
              </select>
            </div>

            <div class="dependency-form-actions">
              <button
                type="button"
                class="btn-secondary"
                onclick={() => {
                  showDependencyForm = false;
                }}
              >
                Отмена
              </button>
              <button type="button" class="btn-primary" onclick={addDependency}> Добавить </button>
            </div>
          </div>
        {:else}
          <button
            type="button"
            class="btn-secondary btn-add-dependency"
            onclick={() => {
              showDependencyForm = true;
            }}
          >
            <PlusIcon class="icon-small" />
            Добавить зависимость
          </button>
        {/if}
      {:else}
        <div class="info-box">
          <p>
            Для редактирования зависимостей существующего файла используйте редактор зависимостей на
            странице файла.
          </p>
        </div>
      {/if}

      <div class="form-actions">
        <button type="button" class="btn-secondary" onclick={handleClose}> Отмена </button>
        <button type="submit" class="btn-primary" disabled={saving}>
          {saving ? 'Сохранение...' : 'Сохранить'}
        </button>
      </div>
    </form>
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

  .form {
    padding: 1.5rem;
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

  .form-group input,
  .form-group textarea,
  .form-group select {
    width: 100%;
    padding: 0.75rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    color: #e2e8f0;
    font-size: 0.95rem;
    transition: border-color 0.2s;
    font-family: inherit;
  }

  .form-group input:focus,
  .form-group textarea:focus,
  .form-group select:focus {
    outline: none;
    border-color: #0ea5e9;
  }

  .form-group textarea {
    resize: vertical;
    min-height: 100px;
  }

  .error-input {
    border-color: #ef4444 !important;
  }

  .field-error {
    display: block;
    margin-top: 0.25rem;
    color: #ef4444;
    font-size: 0.75rem;
  }

  .form-hint {
    display: block;
    margin-top: 0.25rem;
    color: #64748b;
    font-size: 0.75rem;
  }

  .section-title {
    margin: 2rem 0 1rem 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: #0ea5e9;
    padding-top: 1.5rem;
    border-top: 1px solid #334155;
  }

  .error {
    padding: 0.75rem;
    background: #7f1d1d;
    color: #fca5a5;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .info-box {
    padding: 0.75rem;
    background: #1e3a8a;
    color: #93c5fd;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .dependencies-list {
    margin-bottom: 1rem;
  }

  .dependency-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .dependency-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
  }

  .dependency-name {
    font-weight: 600;
    color: #e2e8f0;
  }

  .dependency-version {
    color: #94a3b8;
    font-size: 0.875rem;
  }

  .dependency-type {
    padding: 0.25rem 0.5rem;
    background: #334155;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    color: #cbd5e1;
    text-transform: uppercase;
  }

  .btn-icon-small {
    background: transparent;
    border: none;
    color: #ef4444;
    cursor: pointer;
    padding: 0.25rem;
    border-radius: 0.25rem;
    transition: all 0.2s;
  }

  .btn-icon-small:hover {
    background: #7f1d1d;
    color: #fca5a5;
  }

  .dependency-form {
    padding: 1rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
  }

  .dependency-form-actions {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
    margin-top: 1rem;
  }

  .btn-add-dependency {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    justify-content: center;
  }

  .form-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 2rem;
    padding-top: 1.5rem;
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

  .icon-small {
    width: 16px;
    height: 16px;
  }
</style>
