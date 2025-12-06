<script lang="ts">
  import HomeIcon from './icons/HomeIcon.svelte';
  import CogIcon from './icons/CogIcon.svelte';
  import CodeIcon from './icons/CodeIcon.svelte';
  import BellIcon from './icons/BellIcon.svelte';
  
  interface Props {
    currentPage: string;
    sites: any[];
    selectedSiteId: number | null;
    onPageChange: (page: string) => void;
    onSiteSelect: (siteId: number | null) => void;
  }
  
  let { currentPage, sites, selectedSiteId, onPageChange, onSiteSelect }: Props = $props();

  function handlePageClick(page: string) {
    onPageChange(page);
  }

  function handleSiteClick(siteId: number | null) {
    onSiteSelect(siteId);
  }
</script>

<aside class="sidebar">
  <div class="sidebar-header">
    <h1 class="logo">Mod Aggregator</h1>
  </div>
  
  <nav class="sidebar-nav">
    <button 
      type="button"
      class="nav-item" 
      class:active={currentPage === 'mods'}
      onclick={() => handlePageClick('mods')}
    >
      <HomeIcon class="icon" />
      <span>Моды</span>
    </button>
    
    <button 
      type="button"
      class="nav-item" 
      class:active={currentPage === 'sites'}
      onclick={() => handlePageClick('sites')}
    >
      <CogIcon class="icon" />
      <span>Сайты</span>
    </button>
    
    <button 
      type="button"
      class="nav-item" 
      class:active={currentPage === 'parser'}
      onclick={() => handlePageClick('parser')}
    >
      <CodeIcon class="icon" />
      <span>Конструктор</span>
    </button>
    
    <button 
      type="button"
      class="nav-item" 
      class:active={currentPage === 'notifications'}
      onclick={() => handlePageClick('notifications')}
    >
      <BellIcon class="icon" />
      <span>Уведомления</span>
    </button>
  </nav>
  
  <div class="sites-list">
    <h3 class="sites-title">Сайты</h3>
    <button 
      type="button"
      class="site-item" 
      class:active={selectedSiteId === null}
      onclick={() => handleSiteClick(null)}
    >
      Все сайты
    </button>
    {#each sites as site}
      <button 
        type="button"
        class="site-item" 
        class:active={selectedSiteId === site.id}
        onclick={() => handleSiteClick(site.id)}
      >
        {site.name}
      </button>
    {/each}
  </div>
</aside>

<style>
  .sidebar {
    width: 280px;
    background: #0f172a;
    border-right: 1px solid #1e293b;
    display: flex;
    flex-direction: column;
    height: 100vh;
  }
  
  .sidebar-header {
    padding: 1.5rem;
    border-bottom: 1px solid #1e293b;
  }
  
  .logo {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 700;
    color: #0ea5e9;
  }
  
  .sidebar-nav {
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    border-radius: 0.5rem;
    transition: all 0.2s;
    text-align: left;
    font-size: 0.95rem;
  }
  
  .nav-item:hover {
    background: #1e293b;
    color: #e2e8f0;
  }
  
  .nav-item.active {
    background: #1e3a8a;
    color: #0ea5e9;
  }
  
  .sites-list {
    flex: 1;
    padding: 1rem;
    overflow-y: auto;
    border-top: 1px solid #1e293b;
  }
  
  .sites-title {
    font-size: 0.75rem;
    text-transform: uppercase;
    color: #64748b;
    margin: 0 0 0.75rem 0;
    font-weight: 600;
    letter-spacing: 0.05em;
  }
  
  .site-item {
    display: block;
    width: 100%;
    padding: 0.5rem 0.75rem;
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    border-radius: 0.375rem;
    text-align: left;
    font-size: 0.875rem;
    margin-bottom: 0.25rem;
    transition: all 0.2s;
  }
  
  .site-item:hover {
    background: #1e293b;
    color: #e2e8f0;
  }
  
  .site-item.active {
    background: #1e3a8a;
    color: #0ea5e9;
  }
  
  .icon {
    width: 20px;
    height: 20px;
  }
</style>

