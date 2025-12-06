<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { format } from 'date-fns';
  import { ru } from 'date-fns/locale';
  import ModCard from './ModCard.svelte';
  
  interface Props {
    selectedSiteId: number | null;
  }
  
  let { selectedSiteId }: Props = $props();
  
  let mods = $state<any[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);
  
  onMount(() => {
    loadMods();
  });
  
  $effect(() => {
    loadMods();
  });
  
  async function loadMods() {
    loading = true;
    error = null;
    try {
      mods = await invoke('get_mods', { siteId: selectedSiteId });
    } catch (e: any) {
      error = e.toString();
    } finally {
      loading = false;
    }
  }
  
  async function handleCheckUpdates() {
    loading = true;
    try {
      await invoke('check_updates', { siteId: selectedSiteId });
      await loadMods();
    } catch (e: any) {
      error = e.toString();
    } finally {
      loading = false;
    }
  }
</script>

<div class="mods-list-container">
  <div class="header">
    <h2>Моды</h2>
    <button type="button" class="btn-primary" onclick={handleCheckUpdates} disabled={loading}>
      {loading ? 'Проверка...' : 'Проверить обновления'}
    </button>
  </div>
  
  {#if error}
    <div class="error">{error}</div>
  {/if}
  
  {#if loading && mods.length === 0}
    <div class="loading">Загрузка...</div>
  {:else if mods.length === 0}
    <div class="empty">Моды не найдены</div>
  {:else}
    <div class="mods-grid">
      {#each mods as mod}
        <ModCard {mod} />
      {/each}
    </div>
  {/if}
</div>

<style>
  .mods-list-container {
    max-width: 1400px;
    margin: 0 auto;
  }
  
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }
  
  .header h2 {
    margin: 0;
    font-size: 2rem;
    font-weight: 700;
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
  
  .error {
    padding: 1rem;
    background: #7f1d1d;
    color: #fca5a5;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
  }
  
  .loading, .empty {
    text-align: center;
    padding: 3rem;
    color: #64748b;
    font-size: 1.125rem;
  }
  
  .mods-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 1.5rem;
  }
</style>

