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
    max-width: 1200px;
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
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    background: #0ea5e9;
    color: white;
    border: none;
    border-radius: 0.5rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }
  
  .btn-primary:hover {
    background: #0284c7;
  }
  
  .sites-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
  }
  
  .site-card {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.75rem;
    padding: 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    transition: transform 0.2s, box-shadow 0.2s;
  }
  
  .site-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
  }
  
  .site-info h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: #e2e8f0;
  }
  
  .site-url {
    margin: 0;
    font-size: 0.875rem;
    color: #94a3b8;
    word-break: break-all;
  }
  
  .site-actions {
    display: flex;
    gap: 0.5rem;
  }
  
  .btn-icon {
    padding: 0.5rem;
    background: transparent;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    color: #94a3b8;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .btn-icon:hover {
    background: #334155;
    color: #e2e8f0;
  }
  
  .btn-danger:hover {
    background: #7f1d1d;
    border-color: #991b1b;
    color: #fca5a5;
  }
  
  .loading, .empty {
    text-align: center;
    padding: 3rem;
    color: #64748b;
    font-size: 1.125rem;
  }
  
  .icon {
    width: 20px;
    height: 20px;
  }
  
  .icon-small {
    width: 18px;
    height: 18px;
  }
</style>

