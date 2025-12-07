<script lang="ts">
  import HomeIcon from './icons/HomeIcon.svelte';
  import CogIcon from './icons/CogIcon.svelte';
  import CodeIcon from './icons/CodeIcon.svelte';
  import BellIcon from './icons/BellIcon.svelte';
  import XMarkIcon from './icons/XMarkIcon.svelte';
  import MenuIcon from './icons/MenuIcon.svelte';
  import Tooltip from './Tooltip.svelte';
  
  interface Props {
    currentPage: string;
    sites: any[];
    selectedSiteId: number | null;
    isOpen: boolean;
    onPageChange: (page: string) => void;
    onSiteSelect: (siteId: number | null) => void;
    onToggle: () => void;
  }
  
  let { currentPage, sites, selectedSiteId, isOpen, onPageChange, onSiteSelect, onToggle }: Props = $props();

  function handlePageClick(page: string) {
    onPageChange(page);
  }

  function handleSiteClick(siteId: number | null) {
    onSiteSelect(siteId);
  }
</script>

<aside class="sidebar" class:collapsed={!isOpen}>
  <div class="sidebar-header">
    <div class="logo-container">
      <img src="/app-icon.svg" alt="Mod Aggregator" class="logo-icon" />
      {#if isOpen}
        <h1 class="logo">Mod Aggregator</h1>
      {/if}
    </div>
    <Tooltip text={isOpen ? 'Скрыть меню' : 'Показать меню'} position="right">
      <button 
        type="button"
        class="toggle-button"
        onclick={onToggle}
        aria-label={isOpen ? 'Скрыть меню' : 'Показать меню'}
      >
        {#if isOpen}
          <XMarkIcon class="icon-small" />
        {:else}
          <MenuIcon class="icon-small" />
        {/if}
      </button>
    </Tooltip>
  </div>
  
  <nav class="sidebar-nav">
    <Tooltip text="Моды" position="right" show={!isOpen}>
      <button 
        type="button"
        class="nav-item" 
        class:active={currentPage === 'mods'}
        onclick={() => handlePageClick('mods')}
        aria-label="Моды"
      >
        <HomeIcon class="icon" />
        {#if isOpen}
          <span>Моды</span>
        {/if}
      </button>
    </Tooltip>
    
    <Tooltip text="Сайты" position="right" show={!isOpen}>
      <button 
        type="button"
        class="nav-item" 
        class:active={currentPage === 'sites'}
        onclick={() => handlePageClick('sites')}
        aria-label="Сайты"
      >
        <CogIcon class="icon" />
        {#if isOpen}
          <span>Сайты</span>
        {/if}
      </button>
    </Tooltip>
    
    <Tooltip text="Конструктор парсеров" position="right" show={!isOpen}>
      <button 
        type="button"
        class="nav-item" 
        class:active={currentPage === 'parser'}
        onclick={() => handlePageClick('parser')}
        aria-label="Конструктор парсеров"
      >
        <CodeIcon class="icon" />
        {#if isOpen}
          <span>Конструктор</span>
        {/if}
      </button>
    </Tooltip>
    
    <Tooltip text="Уведомления" position="right" show={!isOpen}>
      <button 
        type="button"
        class="nav-item" 
        class:active={currentPage === 'notifications'}
        onclick={() => handlePageClick('notifications')}
        aria-label="Уведомления"
      >
        <BellIcon class="icon" />
        {#if isOpen}
          <span>Уведомления</span>
        {/if}
      </button>
    </Tooltip>
  </nav>
  
  {#if isOpen}
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
  {/if}
</aside>

<style>
  .sidebar {
    --sidebar-width: clamp(16vw, 280px, 20vw);
    --sidebar-collapsed-width: clamp(4vw, 64px, 6vw);
    --header-padding: clamp(1rem, 1.5vw, 1.5rem);
    --nav-padding: clamp(0.75rem, 1vw, 1rem);
    --font-size-base: clamp(0.875rem, 1vw, 1rem);
    --font-size-title: clamp(1.25rem, 2vw, 1.5rem);
    --font-size-small: clamp(0.75rem, 0.9vw, 0.875rem);
    --icon-size: clamp(1.25rem, 1.5vw, 1.5rem);
    --icon-small-size: clamp(1rem, 1.2vw, 1.25rem);
    --border-radius: clamp(0.375rem, 0.5vw, 0.5rem);
    --transition-speed: 0.3s cubic-bezier(0.4, 0, 0.2, 1);

    width: var(--sidebar-width);
    background: linear-gradient(180deg, #0f172a 0%, #1e293b 100%);
    border-right: 1px solid rgba(30, 41, 59, 0.8);
    display: flex;
    flex-direction: column;
    height: 100vh;
    position: relative;
    transition: width var(--transition-speed);
    box-shadow: 2px 0 8px rgba(0, 0, 0, 0.3);
  }

  .sidebar.collapsed {
    width: var(--sidebar-collapsed-width);
  }
  
  .sidebar-header {
    padding: var(--header-padding);
    border-bottom: 1px solid rgba(30, 41, 59, 0.8);
    display: flex;
    justify-content: space-between;
    align-items: center;
    min-height: clamp(3.5rem, 5vh, 4.5rem);
    background: rgba(15, 23, 42, 0.95);
    backdrop-filter: blur(10px);
  }
  
  .logo-container {
    display: flex;
    align-items: center;
    gap: clamp(0.5rem, 0.75vw, 0.75rem);
    flex: 1;
    min-width: 0;
  }
  
  .logo-icon {
    width: var(--icon-size);
    height: var(--icon-size);
    flex-shrink: 0;
    border-radius: var(--border-radius);
    transition: transform 0.2s;
  }

  .logo-icon:hover {
    transform: scale(1.05);
  }
  
  .logo {
    margin: 0;
    font-size: var(--font-size-title);
    font-weight: 700;
    color: #0ea5e9;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    transition: opacity var(--transition-speed);
  }

  .sidebar.collapsed .logo {
    opacity: 0;
    width: 0;
  }
  
  .toggle-button {
    width: var(--icon-size);
    height: var(--icon-size);
    padding: 0;
    background: transparent;
    border: 1px solid rgba(148, 163, 184, 0.2);
    border-radius: var(--border-radius);
    color: #94a3b8;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    flex-shrink: 0;
  }

  .toggle-button:hover {
    background: rgba(14, 165, 233, 0.15);
    border-color: #0ea5e9;
    color: #0ea5e9;
    transform: rotate(90deg) scale(1.1);
    box-shadow: 0 2px 8px rgba(14, 165, 233, 0.3);
  }

  .toggle-button:active {
    transform: rotate(90deg) scale(0.95);
  }
  
  .sidebar-nav {
    padding: var(--nav-padding);
    display: flex;
    flex-direction: column;
    gap: clamp(0.375rem, 0.5vw, 0.5rem);
    flex-shrink: 0;
  }
  
  .nav-item {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: clamp(0.5rem, 0.75vw, 0.75rem);
    padding: clamp(0.75rem, 1vw, 0.875rem) clamp(0.875rem, 1.25vw, 1rem);
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    border-radius: var(--border-radius);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    text-align: left;
    font-size: var(--font-size-base);
    font-weight: 500;
    position: relative;
    overflow: visible;
    min-height: clamp(2.5rem, 3.5vh, 3rem);
    width: 100%;
  }

  .sidebar.collapsed .nav-item {
    justify-content: center;
    padding: clamp(0.75rem, 1vw, 0.875rem);
  }

  .nav-item::before {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 3px;
    background: #0ea5e9;
    transform: scaleY(0);
    transition: transform 0.2s;
  }

  .nav-item:hover {
    background: rgba(30, 41, 59, 0.6);
    color: #e2e8f0;
    transform: translateX(2px);
  }

  .sidebar.collapsed .nav-item:hover {
    transform: scale(1.1);
    background: rgba(14, 165, 233, 0.15);
  }

  .nav-item:hover::before {
    transform: scaleY(1);
  }
  
  .nav-item.active {
    background: linear-gradient(90deg, rgba(14, 165, 233, 0.15) 0%, rgba(14, 165, 233, 0.05) 100%);
    color: #0ea5e9;
    font-weight: 600;
  }

  .nav-item.active::before {
    transform: scaleY(1);
  }

  .nav-item span {
    white-space: nowrap;
    transition: opacity var(--transition-speed);
  }

  .sidebar.collapsed .nav-item span {
    opacity: 0;
    width: 0;
    overflow: hidden;
  }
  
  .sites-list {
    flex: 1;
    padding: var(--nav-padding);
    overflow-y: auto;
    overflow-x: hidden;
    border-top: 1px solid rgba(30, 41, 59, 0.8);
    min-height: 0;
  }

  .sites-list::-webkit-scrollbar {
    width: 4px;
  }

  .sites-list::-webkit-scrollbar-track {
    background: transparent;
  }

  .sites-list::-webkit-scrollbar-thumb {
    background: rgba(148, 163, 184, 0.3);
    border-radius: 2px;
  }

  .sites-list::-webkit-scrollbar-thumb:hover {
    background: rgba(148, 163, 184, 0.5);
  }
  
  .sites-title {
    font-size: var(--font-size-small);
    text-transform: uppercase;
    color: #64748b;
    margin: 0 0 clamp(0.5rem, 0.75vw, 0.75rem) 0;
    font-weight: 600;
    letter-spacing: 0.05em;
  }
  
  .site-item {
    display: block;
    width: 100%;
    padding: clamp(0.5rem, 0.75vw, 0.625rem) clamp(0.625rem, 0.875vw, 0.75rem);
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    border-radius: var(--border-radius);
    text-align: left;
    font-size: var(--font-size-small);
    margin-bottom: clamp(0.125rem, 0.25vw, 0.25rem);
    transition: all 0.2s;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  
  .site-item:hover {
    background: rgba(30, 41, 59, 0.6);
    color: #e2e8f0;
    transform: translateX(2px);
  }
  
  .site-item.active {
    background: linear-gradient(90deg, rgba(14, 165, 233, 0.15) 0%, rgba(14, 165, 233, 0.05) 100%);
    color: #0ea5e9;
    font-weight: 600;
  }
  
  .icon {
    width: var(--icon-size);
    height: var(--icon-size);
    flex-shrink: 0;
    transition: transform 0.2s;
  }

  .nav-item:hover .icon {
    transform: scale(1.1);
  }

  .nav-item.active .icon {
    color: #0ea5e9;
  }

  .icon-small {
    width: var(--icon-small-size);
    height: var(--icon-small-size);
    flex-shrink: 0;
  }
</style>

