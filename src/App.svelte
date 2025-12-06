<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import Sidebar from './components/Sidebar.svelte';
  import ModsList from './components/ModsList.svelte';
  import SitesManager from './components/SitesManager.svelte';
  import ParserBuilder from './components/ParserBuilder.svelte';
  import NotificationsPanel from './components/NotificationsPanel.svelte';
  
  type Page = 'mods' | 'sites' | 'parser' | 'notifications';
  
  let currentPage: Page = $state('mods');
  let sites = $state<any[]>([]);
  let selectedSiteId = $state<number | null>(null);
  
  onMount(async () => {
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

<style>
  .app-container {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }
  
  .main-content {
    flex: 1;
    overflow-y: auto;
    padding: 2rem;
    background: #1e293b;
  }
</style>

