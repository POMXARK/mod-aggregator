<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from './lib/tauri-wrapper';
  import { isTauri } from './lib/tauri-mock';
  import Sidebar from './components/Sidebar.svelte';
  import ModsList from './components/ModsList.svelte';
  import SitesManager from './components/SitesManager.svelte';
  import ParserBuilder from './components/ParserBuilder.svelte';
  import NotificationsPanel from './components/NotificationsPanel.svelte';
  import MenuIcon from './components/icons/MenuIcon.svelte';
  
  type Page = 'mods' | 'sites' | 'parser' | 'notifications';
  
  let currentPage: Page = $state('mods');
  let sites = $state<any[]>([]);
  let selectedSiteId = $state<number | null>(null);
  let sidebarOpen = $state(true);
  
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

  function toggleSidebar() {
    sidebarOpen = !sidebarOpen;
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
      isOpen={sidebarOpen}
      onPageChange={handlePageChange}
      onSiteSelect={handleSiteSelect}
      onToggle={toggleSidebar}
    />
    
    <main class="main-content">
      {#if !sidebarOpen}
        <button 
          type="button"
          class="sidebar-toggle-button"
          onclick={toggleSidebar}
          aria-label="Показать меню"
        >
          <MenuIcon class="icon" />
        </button>
      {/if}
      
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
    width: 100vw;
    overflow: hidden;
    font-size: clamp(0.875rem, 1vw, 1rem);
  }
  
  .browser-mode-banner {
    background: linear-gradient(90deg, #f59e0b 0%, #d97706 100%);
    color: #1e293b;
    padding: clamp(0.5rem, 0.75vh, 0.625rem) clamp(1rem, 1.5vw, 1.25rem);
    text-align: center;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
    font-weight: 600;
    border-bottom: 2px solid #d97706;
    flex-shrink: 0;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
  
  .app-container {
    display: flex;
    flex-direction: row;
    flex: 1;
    overflow: hidden;
    position: relative;
  }
  
  .main-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: clamp(1.5rem, 2.5vw, 2rem);
    background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
    min-width: 0;
    position: relative;
  }

  .main-content::-webkit-scrollbar {
    width: clamp(0.5rem, 0.75vw, 0.75rem);
  }

  .main-content::-webkit-scrollbar-track {
    background: rgba(15, 23, 42, 0.5);
  }

  .main-content::-webkit-scrollbar-thumb {
    background: rgba(148, 163, 184, 0.3);
    border-radius: clamp(0.25rem, 0.375vw, 0.375rem);
  }

  .main-content::-webkit-scrollbar-thumb:hover {
    background: rgba(148, 163, 184, 0.5);
  }

  .sidebar-toggle-button {
    position: fixed;
    top: clamp(1rem, 1.5vh, 1.5rem);
    left: clamp(1rem, 1.5vw, 1.5rem);
    z-index: 100;
    width: clamp(2.5rem, 3.5vw, 3rem);
    height: clamp(2.5rem, 3.5vw, 3rem);
    padding: 0;
    background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
    border: none;
    border-radius: clamp(0.5rem, 0.75vw, 0.75rem);
    color: white;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 4px 12px rgba(14, 165, 233, 0.4);
  }

  .sidebar-toggle-button:hover {
    transform: scale(1.1) rotate(90deg);
    box-shadow: 0 6px 16px rgba(14, 165, 233, 0.6);
  }

  .sidebar-toggle-button:active {
    transform: scale(0.95);
  }

  .sidebar-toggle-button .icon {
    width: clamp(1.25rem, 1.75vw, 1.5rem);
    height: clamp(1.25rem, 1.75vw, 1.5rem);
  }
</style>

