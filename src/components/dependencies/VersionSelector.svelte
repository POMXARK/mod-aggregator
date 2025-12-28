<script lang="ts">
  import type { File } from '@/types/file';
  import { VersionResolver } from '@/lib/dependencies/version-resolver';

  const props = $props<{
    fileName: string;
    availableFiles: File[];
    selectedVersion?: string;
    onVersionChange: (version: string | undefined) => void;
  }>();

  const availableVersions = $derived(
    VersionResolver.getAvailableVersions(props.availableFiles, props.fileName)
  );

  const latestVersion = $derived(
    VersionResolver.getLatestVersion(props.availableFiles, props.fileName)
  );

  let selected = $state(props.selectedVersion ?? latestVersion ?? undefined);
  let allowAnyVersion = $state(props.selectedVersion === undefined);

  $effect(() => {
    if (allowAnyVersion) {
      props.onVersionChange(undefined);
    } else {
      props.onVersionChange(selected);
    }
  });
</script>

<div class="version-selector space-y-2">
  <label class="flex items-center space-x-2">
    <input
      type="checkbox"
      bind:checked={allowAnyVersion}
      class="rounded border-gray-300 dark:border-gray-600"
    />
    <span class="text-sm text-gray-700 dark:text-gray-300"> Любая версия </span>
  </label>

  {#if !allowAnyVersion}
    <select
      bind:value={selected}
      class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
      disabled={availableVersions.length === 0}
    >
      {#if availableVersions.length === 0}
        <option value="">Версии не найдены</option>
      {:else}
        {#each availableVersions as version (version)}
          <option value={version}>{version}</option>
        {/each}
      {/if}
    </select>

    {#if latestVersion && selected !== latestVersion}
      <p class="text-xs text-gray-500 dark:text-gray-400">
        Последняя версия: {latestVersion}
      </p>
    {/if}
  {/if}
</div>
