<script lang="ts">
  // Пример простого импорта через path mapping
  import { invoke } from '@/lib/tauri-wrapper';
  import type { File } from '@/types/file';
  import { someUtility } from '@/utils/helpers';

  interface Props {
    title: string;
    data?: File[];
  }

  const { title, data = [] }: Props = $props();

  async function handleAction() {
    try {
      const result = await invoke('some_command', { param: 'value' });
      someUtility(result);
    } catch (error) {
      console.error('Action failed:', error);
    }
  }
</script>

<div class="simple-component">
  <h3>{title}</h3>
  <button onclick={handleAction}>Выполнить действие</button>

  {#if data.length > 0}
    <ul>
      {#each data as item (item.name)}
        <li>{item.name}</li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .simple-component {
    padding: 1rem;
    border: 1px solid #ccc;
    border-radius: 0.5rem;
  }
</style>
