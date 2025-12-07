<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '../lib/tauri-wrapper';
  import PlusIcon from './icons/PlusIcon.svelte';
  import TrashIcon from './icons/TrashIcon.svelte';
  import PencilIcon from './icons/PencilIcon.svelte';
  import SiteForm from './SiteForm.svelte';
  
  interface Props {
    onSiteAdded: () => void;
  }
  
  let { onSiteAdded }: Props = $props();
  
  let sites = $state<any[]>([]);
  let editingSite = $state<any | null>(null);
  let showForm = $state(false);
  let loading = $state(false);
  
  onMount(() => {
    loadSites();
  });
  
  async function loadSites() {
    loading = true;
    try {
      sites = await invoke('get_sites');
    } catch (error) {
      console.error('Failed to load sites:', error);
    } finally {
      loading = false;
    }
  }
  
  async function handleDelete(id: number) {
    if (!confirm('Удалить этот сайт?')) return;
    
    try {
      await invoke('delete_site', { id });
      await loadSites();
    } catch (error) {
      console.error('Failed to delete site:', error);
    }
  }
  
  function handleEdit(site: any) {
    editingSite = site;
    showForm = true;
  }
  
  function handleAdd() {
    editingSite = null;
    showForm = true;
  }
  
  function handleFormClose() {
    showForm = false;
    editingSite = null;
  }
  
  async function handleFormSubmit() {
    await loadSites();
    showForm = false;
    editingSite = null;
    onSiteAdded();
  }
</script>

<div class="sites-manager">
  <div class="header">
    <h2>Управление сайтами</h2>
    <button class="btn-primary" onclick={handleAdd}>
      <PlusIcon class="icon" />
      Добавить сайт
    </button>
  </div>
  
  {#if showForm}
    <SiteForm 
      site={editingSite} 
      onClose={handleFormClose}
      onSubmit={handleFormSubmit}
    />
  {/if}
  
  {#if loading}
    <div class="loading">Загрузка...</div>
  {:else if sites.length === 0}
    <div class="empty">Нет добавленных сайтов</div>
  {:else}
    <div class="sites-grid">
      {#each sites as site}
        <div class="site-card">
          <div class="site-info">
            <h3>{site.name}</h3>
            <p class="site-url">{site.url}</p>
          </div>
          <div class="site-actions">
            <button class="btn-icon" onclick={() => handleEdit(site)}>
              <PencilIcon class="icon-small" />
            </button>
            <button class="btn-icon btn-danger" onclick={() => handleDelete(site.id)}>
              <TrashIcon class="icon-small" />
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .sites-manager {
    max-width: min(1200px, 95vw);
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
    display: flex;
    align-items: center;
    gap: clamp(0.375rem, 0.5vw, 0.5rem);
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
  
  .btn-primary:hover {
    background: linear-gradient(135deg, #0284c7 0%, #0369a1 100%);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(14, 165, 233, 0.5);
  }

  .btn-primary:active {
    transform: translateY(0);
  }
  
  .sites-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(clamp(280px, 25vw, 300px), 1fr));
    gap: clamp(1rem, 1.5vw, 1.5rem);
  }

  @media (max-width: 768px) {
    .sites-grid {
      grid-template-columns: 1fr;
    }
  }
  
  .site-card {
    background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
    border: 1px solid rgba(51, 65, 85, 0.5);
    border-radius: clamp(0.5rem, 0.75vw, 0.75rem);
    padding: clamp(1.25rem, 2vw, 1.5rem);
    display: flex;
    justify-content: space-between;
    align-items: center;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    gap: clamp(1rem, 1.5vw, 1.5rem);
  }
  
  .site-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.4);
    border-color: rgba(14, 165, 233, 0.5);
  }
  
  .site-info {
    flex: 1;
    min-width: 0;
  }

  .site-info h3 {
    margin: 0 0 clamp(0.375rem, 0.5vw, 0.5rem) 0;
    font-size: clamp(1rem, 1.5vw, 1.125rem);
    font-weight: 600;
    color: #e2e8f0;
    line-height: 1.3;
    word-break: break-word;
  }
  
  .site-url {
    margin: 0;
    font-size: clamp(0.75rem, 1vw, 0.875rem);
    color: #94a3b8;
    word-break: break-all;
    line-height: 1.4;
  }
  
  .site-actions {
    display: flex;
    gap: clamp(0.375rem, 0.5vw, 0.5rem);
    flex-shrink: 0;
  }
  
  .btn-icon {
    padding: clamp(0.375rem, 0.5vw, 0.5rem);
    background: transparent;
    border: 1px solid rgba(51, 65, 85, 0.5);
    border-radius: clamp(0.25rem, 0.375vw, 0.375rem);
    color: #94a3b8;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: clamp(2rem, 2.5vw, 2.5rem);
    min-height: clamp(2rem, 2.5vw, 2.5rem);
  }
  
  .btn-icon:hover {
    background: rgba(51, 65, 85, 0.8);
    color: #e2e8f0;
    transform: scale(1.1);
    border-color: rgba(148, 163, 184, 0.5);
  }

  .btn-icon:active {
    transform: scale(0.95);
  }
  
  .btn-danger:hover {
    background: linear-gradient(135deg, #7f1d1d 0%, #991b1b 100%);
    border-color: #ef4444;
    color: #fca5a5;
  }
  
  .loading, .empty {
    text-align: center;
    padding: clamp(2rem, 4vh, 3rem);
    color: #64748b;
    font-size: clamp(1rem, 1.5vw, 1.125rem);
  }
  
  .icon {
    width: clamp(1.125rem, 1.5vw, 1.25rem);
    height: clamp(1.125rem, 1.5vw, 1.25rem);
  }
  
  .icon-small {
    width: clamp(1rem, 1.25vw, 1.125rem);
    height: clamp(1rem, 1.25vw, 1.125rem);
  }
</style>

