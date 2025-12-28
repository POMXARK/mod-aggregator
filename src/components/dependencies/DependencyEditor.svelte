<script lang="ts">
  import { invoke } from '@/lib/tauri-wrapper';
  import type { File, FileDependency } from '@/types/file';
  import type { DependencyType } from '@/types/dependency';
  import { DependencyValidator } from '@/lib/dependencies/dependency-validator';
  import VersionSelector from './VersionSelector.svelte';

  const props = $props<{
    file: File;
    dependencies: FileDependency[];
    allFiles: File[];
    onDependencyAdded: () => void;
    onDependencyRemoved: () => void;
  }>();

  let showAddForm = $state(false);
  let targetFileName = $state('');
  let targetFileVersion = $state<string | undefined>(undefined);
  let dependencyType = $state<DependencyType>('required');
  let error = $state<string | null>(null);
  let loading = $state(false);

  const validation = $derived(() => {
    if (!targetFileName.trim()) {
      return { valid: false, error: 'Имя файла обязательно' };
    }
    return DependencyValidator.validateDependencyParams(
      props.file,
      targetFileName,
      targetFileVersion
    );
  });

  async function addDependency() {
    const validationResult = validation();
    if (!validationResult.valid) {
      error = validationResult.error ?? 'Ошибка валидации';
      return;
    }

    loading = true;
    error = null;

    try {
      await invoke('add_file_dependency', {
        params: {
          source_file_id: props.file.id,
          target_file_name: targetFileName,
          target_file_version: targetFileVersion,
          dependency_type: dependencyType,
        },
      });

      // Сброс формы
      targetFileName = '';
      targetFileVersion = undefined;
      dependencyType = 'required';
      showAddForm = false;
      props.onDependencyAdded();
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при добавлении зависимости';
    } finally {
      loading = false;
    }
  }

  async function removeDependency(dependencyId: number) {
    if (!confirm('Удалить зависимость?')) {
      return;
    }

    try {
      await invoke('remove_file_dependency', { dependency_id: dependencyId });
      props.onDependencyRemoved();
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при удалении зависимости';
    }
  }

  const availableFileNames = $derived(Array.from(new Set(props.allFiles.map(f => f.name))).sort());
</script>

<div class="dependency-editor space-y-4">
  <div class="flex items-center justify-between">
    <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">Зависимости</h3>
    <button
      onclick={() => (showAddForm = !showAddForm)}
      class="px-3 py-1.5 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm transition-colors"
    >
      {showAddForm ? 'Отмена' : '+ Добавить'}
    </button>
  </div>

  {#if error}
    <div
      class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-3 text-sm text-red-700 dark:text-red-300"
    >
      {error}
    </div>
  {/if}

  {#if showAddForm}
    <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4 space-y-3">
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          Имя файла
        </label>
        <input
          type="text"
          list="file-names"
          bind:value={targetFileName}
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100"
          placeholder="Введите имя файла"
        />
        <datalist id="file-names">
          {#each availableFileNames as name (name)}
            <option value={name}></option>
          {/each}
        </datalist>
      </div>

      {#if targetFileName}
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            Версия
          </label>
          <VersionSelector
            fileName={targetFileName}
            availableFiles={props.allFiles}
            selectedVersion={targetFileVersion}
            onVersionChange={v => (targetFileVersion = v)}
          />
        </div>
      {/if}

      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          Тип зависимости
        </label>
        <select
          bind:value={dependencyType}
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100"
        >
          <option value="required">Обязательная</option>
          <option value="optional">Опциональная</option>
          <option value="peer">Peer</option>
        </select>
      </div>

      <button
        onclick={addDependency}
        disabled={loading || !validation().valid}
        class="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white rounded-lg transition-colors"
      >
        {loading ? 'Добавление...' : 'Добавить'}
      </button>
    </div>
  {/if}

  <div class="space-y-2">
    {#if props.dependencies.length === 0}
      <p class="text-sm text-gray-500 dark:text-gray-400 text-center py-4">Нет зависимостей</p>
    {:else}
      {#each props.dependencies as dep (dep.id)}
        <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg">
          <div>
            <span class="font-medium text-gray-900 dark:text-gray-100">
              {dep.targetFileName}
            </span>
            {#if dep.targetFileVersion}
              <span class="text-sm text-gray-500 dark:text-gray-400">
                @{dep.targetFileVersion}
              </span>
            {/if}
            <span
              class="ml-2 text-xs px-2 py-0.5 rounded bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300"
            >
              {dep.dependencyType}
            </span>
          </div>
          <button
            onclick={() => removeDependency(dep.id)}
            class="px-2 py-1 text-red-600 hover:text-red-700 dark:text-red-400 dark:hover:text-red-300 text-sm"
          >
            Удалить
          </button>
        </div>
      {/each}
    {/if}
  </div>
</div>
