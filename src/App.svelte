<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from './lib/tauri-wrapper';
  import { isTauri } from './lib/tauri-mock';
  import Sidebar from './components/Sidebar.svelte';
  import ModsList from './components/ModsList.svelte';
  import SitesManager from './components/SitesManager.svelte';
  import ParserBuilder from './components/ParserBuilder.svelte';
  import NotificationsPanel from './components/NotificationsPanel.svelte';
  
  type Page = 'mods' | 'sites' | 'parser' | 'notifications';
  
  let currentPage: Page = $state('mods');
  let sites = $state<any[]>([]);
  let selectedSiteId = $state<number | null>(null);
  
  // Show mode indicator - check in onMount to ensure Tauri is initialized
  let isTauriMode = $state(false);
  
  onMount(async () => {
    // Check Tauri after mount to ensure it's initialized
    // Wait a bit for Tauri to fully initialize
    await new Promise(resolve => setTimeout(resolve, 100));
    
    // Try multiple checks
    isTauriMode = isTauri();
    
    // Also try to detect by checking if we can actually call Tauri API
    if (!isTauriMode && typeof window !== 'undefined') {
      try {
        // @ts-ignore
        if (window.__TAURI__ || window.__TAURI_INTERNALS__) {
          isTauriMode = true;
        }
      } catch (e) {
        // Not in Tauri
      }
    }
    
    console.log('Running in Tauri mode:', isTauriMode);
    console.log('Window Tauri check:', typeof window !== 'undefined' ? {
      // @ts-ignore
      __TAURI__: !!window.__TAURI__,
      // @ts-ignore
      __TAURI_INTERNALS__: !!window.__TAURI_INTERNALS__,
    } : 'window undefined');
    
    await loadSites();
  });
  
  async function loadSites() {
    try {
      sites = await invoke('get_sites');
    } catch (error) {
      console.error('Failed to load sites:', error);
    }
  }
  
  function handlePageChange(page: Page) {
    console.log('App: handlePageChange', page, 'current:', currentPage);
    currentPage = page;
    console.log('App: after update', currentPage);
  }

  function handleSiteSelect(siteId: number | null) {
    console.log('App: handleSiteSelect', siteId);
    selectedSiteId = siteId;
  }
  
  function handleSiteAdded() {
    loadSites();
  }
</script>

<div class="app-wrapper">
  {#if !isTauriMode}
    <div class="browser-mode-banner">
      🌐 Режим браузера (Mock данные) - для полного функционала используйте Tauri
    </div>
  {/if}
  
  <div class="app-container">
    <Sidebar 
      {currentPage}
      {sites}
      {selectedSiteId}
      onPageChange={handlePageChange}
      onSiteSelect={handleSiteSelect}
    />
    
    <main class="main-content">
      {#if currentPage === 'mods'}
        <ModsList {selectedSiteId} />
      {:else if currentPage === 'sites'}
        <SitesManager onSiteAdded={handleSiteAdded} />
      {:else if currentPage === 'parser'}
        <ParserBuilder />
      {:else if currentPage === 'notifications'}
        <NotificationsPanel />
      {/if}
    </main>
  </div>
</div>

<style>
  .app-wrapper {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }
  
  .browser-mode-banner {
    background: #f59e0b;
    color: #1e293b;
    padding: 0.5rem 1rem;
    text-align: center;
    font-size: 0.875rem;
    font-weight: 600;
    border-bottom: 2px solid #d97706;
    flex-shrink: 0;
  }
  
  .app-container {
    display: flex;
    flex-direction: row;
    flex: 1;
    overflow: hidden;
  }
  
  .main-content {
    flex: 1;
    overflow-y: auto;
    padding: 2rem;
    background: #1e293b;
  }
</style>

