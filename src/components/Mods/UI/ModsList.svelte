<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@/lib/tauri-wrapper';
  import type { Mod } from '@/lib/api';
  import ModCard from './ModCard.svelte';

  interface Props {
    selectedSiteId: number | null;
  }

  const { selectedSiteId }: Props = $props();

  let mods = $state<Mod[]>([]);
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
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  }

  async function handleCheckUpdates() {
    loading = true;
    try {
      await invoke('check_updates', { siteId: selectedSiteId });
      await loadMods();
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : String(e);
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
      {#each mods as mod (mod.id)}
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

  .loading,
  .empty {
    text-align: center;
    padding: clamp(2rem, 4vh, 3rem);
    color: #64748b;
    font-size: clamp(1rem, 1.5vw, 1.125rem);
  }

  .mods-grid {
    display: grid;
    gap: clamp(1rem, 1.5vw, 1.5rem);
    /* Используем auto-fill для плавной адаптации */
    grid-template-columns: repeat(auto-fill, minmax(clamp(280px, 25vw, 320px), 1fr));
  }

  /* Адаптивная сетка с четкими breakpoints */
  /* Очень маленькие экраны (до 480px) */
  @media (max-width: 480px) {
    .mods-grid {
      grid-template-columns: 1fr;
      gap: 0.875rem;
    }
  }

  /* Маленькие экраны (481px - 640px) */
  @media (min-width: 481px) and (max-width: 640px) {
    .mods-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
    }
  }

  /* Средние экраны (641px - 767px) */
  @media (min-width: 641px) and (max-width: 767px) {
    .mods-grid {
      grid-template-columns: repeat(2, 1fr);
      gap: clamp(0.875rem, 1.25vw, 1.25rem);
    }
  }

  /* Планшеты (768px - 1023px) */
  @media (min-width: 768px) and (max-width: 1023px) {
    .mods-grid {
      grid-template-columns: repeat(2, 1fr);
      gap: clamp(1rem, 1.5vw, 1.5rem);
    }
  }

  /* Десктоп (1024px - 1535px) */
  @media (min-width: 1024px) and (max-width: 1535px) {
    .mods-grid {
      grid-template-columns: repeat(3, 1fr);
      gap: clamp(1rem, 1.5vw, 1.5rem);
    }
  }

  /* Большие экраны (1536px+) */
  @media (min-width: 1536px) {
    .mods-grid {
      grid-template-columns: repeat(4, 1fr);
      gap: clamp(1.25rem, 2vw, 1.5rem);
    }
  }

  /* Улучшение для landscape ориентации на мобильных */
  @media (max-width: 767px) and (orientation: landscape) {
    .mods-grid {
      grid-template-columns: repeat(2, 1fr);
      gap: 0.875rem;
    }
  }
</style>
