<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '../lib/tauri-wrapper';
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
    max-width: min(1400px, 95vw);
    margin: 0 auto;
    width: 100%;
  }
  
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: clamp(1.5rem, 2.5vh, 2rem);
    flex-wrap: wrap;
    gap: clamp(1rem, 1.5vw, 1.5rem);
  }
  
  .header h2 {
    margin: 0;
    font-size: clamp(1.75rem, 3vw, 2rem);
    font-weight: 700;
    color: #e2e8f0;
    line-height: 1.2;
  }
  
  .btn-primary {
    padding: clamp(0.625rem, 1vw, 0.75rem) clamp(1.25rem, 2vw, 1.5rem);
    background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
    color: white;
    border: none;
    border-radius: clamp(0.375rem, 0.5vw, 0.5rem);
    font-weight: 600;
    font-size: clamp(0.875rem, 1vw, 0.95rem);
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 2px 8px rgba(14, 165, 233, 0.3);
    white-space: nowrap;
  }
  
  .btn-primary:hover:not(:disabled) {
    background: linear-gradient(135deg, #0284c7 0%, #0369a1 100%);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(14, 165, 233, 0.5);
  }

  .btn-primary:active:not(:disabled) {
    transform: translateY(0);
  }
  
  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none;
  }
  
  .error {
    padding: clamp(0.875rem, 1.25vw, 1rem);
    background: linear-gradient(135deg, #7f1d1d 0%, #991b1b 100%);
    color: #fca5a5;
    border-radius: clamp(0.375rem, 0.5vw, 0.5rem);
    margin-bottom: clamp(0.875rem, 1.25vw, 1rem);
    font-size: clamp(0.875rem, 1vw, 0.95rem);
    border-left: 3px solid #ef4444;
  }
  
  .loading, .empty {
    text-align: center;
    padding: clamp(2rem, 4vh, 3rem);
    color: #64748b;
    font-size: clamp(1rem, 1.5vw, 1.125rem);
  }
  
  .mods-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(clamp(280px, 25vw, 320px), 1fr));
    gap: clamp(1rem, 1.5vw, 1.5rem);
  }

  @media (max-width: 768px) {
    .mods-grid {
      grid-template-columns: 1fr;
    }
  }
</style>

