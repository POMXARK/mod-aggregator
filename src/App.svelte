<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from './lib/tauri-wrapper';
  import { isTauri } from './lib/tauri-mock';

  // Простые импорты через path mapping
  import ModsList from './components/ModsList.svelte';
  import SitesManager from './components/SitesManager.svelte';
  import ParserBuilder from './components/Parser/UI/ParserBuilder.svelte';
  import FileList from './components/files/FileList.svelte';
  import CollectionViewer from './components/collections/CollectionViewer.svelte';
  import Sidebar from './components/Sidebar.svelte';
  import NotificationsPanel from './components/NotificationsPanel.svelte';
  import ComponentSettings from './components/ComponentSettings.svelte';
  import TestSimple from './components/TestSimple.svelte';
  import MenuIcon from './components/icons/MenuIcon.svelte';

  // Конфигурация компонентов
  import { componentsConfig, loadComponentConfig } from '@/config/components';
  import type { Site } from '@/lib/api';
  import { SessionStateManager } from '@/lib/session/session-state';
  import type { SessionRestoreResult, PageType } from '@/lib/types';

  let currentPage: PageType = $state('mods');
  let sites = $state<Site[]>([]);
  let selectedSiteId = $state<number | null>(null);
  let sidebarOpen = $state(true);

  // Show mode indicator - check in onMount to ensure Tauri is initialized
  let isTauriMode = $state(false);
  let sessionWarnings = $state<string[]>([]);

  onMount(async () => {
    // Загружаем конфигурацию компонентов
    loadComponentConfig();

    // Check Tauri after mount to ensure it's initialized
    // Wait a bit for Tauri to fully initialize
    await new Promise(resolve => setTimeout(resolve, 100));

    // Try multiple checks
    isTauriMode = isTauri();

    // Also try to detect by checking if we can actually call Tauri API
    if (!isTauriMode && typeof window !== 'undefined') {
      try {
        // @ts-expect-error: Checking for Tauri internal properties that may not exist in browser mode
        if (window.__TAURI__ || window.__TAURI_INTERNALS__) {
          isTauriMode = true;
        }
      } catch {
        // Not in Tauri
      }
    }

    console.log('Running in Tauri mode:', isTauriMode);
    console.log(
      'Window Tauri check:',
      typeof window !== 'undefined'
        ? {
            // @ts-expect-error: Checking for Tauri internal properties that may not exist in browser mode
            __TAURI__: !!window.__TAURI__,
            // @ts-expect-error: Checking for Tauri internal properties that may not exist in browser mode
            __TAURI_INTERNALS__: !!window.__TAURI_INTERNALS__,
          }
        : 'window undefined'
    );

    // Восстанавливаем сессию при запуске
    if (isTauriMode) {
      await restoreSession();
    }

    await loadSites();
  });

  async function restoreSession() {
    try {
      const result: SessionRestoreResult = await SessionStateManager.restoreSession();
      sessionWarnings = result.warnings;

      // Применяем восстановленные настройки
      if (result.restored) {
        // Восстанавливаем состояние боковой панели
        if (result.uiPreferences && result.uiPreferences.sidebarCollapsed !== undefined) {
          sidebarOpen = !result.uiPreferences.sidebarCollapsed;
        }

        // Можно добавить восстановление других настроек
        console.log('Session restored:', {
          fileOrder: result.fileOrder?.length || 0,
          openCollections: result.openCollections?.length || 0,
          selectedFiles: result.selectedFiles?.length || 0,
          warnings: result.warnings?.length || 0,
        });
      }
    } catch (e) {
      console.error('Failed to restore session:', e);
    }
  }

  async function loadSites() {
    try {
      sites = await invoke('get_sites');
    } catch (error) {
      console.error('Failed to load sites:', error);
    }
  }

  function handlePageChange(page: PageType) {
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

  {#if sessionWarnings.length > 0}
    <div class="session-warnings">
      {#each sessionWarnings as warning, index (index)}
        <div class="warning-item">⚠️ {warning}</div>
      {/each}
    </div>
  {/if}

  <div class="app-container">
    {#if sidebarOpen}
      <div
        class="sidebar-overlay"
        onclick={toggleSidebar}
        role="button"
        aria-label="Закрыть меню"
        tabindex="0"
      ></div>
    {/if}

    <Sidebar
      {currentPage}
      {sites}
      {selectedSiteId}
      isOpen={sidebarOpen}
      {componentsConfig}
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

      {#if currentPage === 'mods' && componentsConfig.mods.enabled}
        <ModsList {selectedSiteId} />
      {:else if currentPage === 'sites' && componentsConfig.sites.enabled}
        <SitesManager onSiteAdded={handleSiteAdded} />
      {:else if currentPage === 'parser' && componentsConfig.parser.enabled}
        <ParserBuilder />
      {:else if currentPage === 'notifications' && componentsConfig.notifications.enabled}
        <NotificationsPanel />
      {:else if currentPage === 'files' && componentsConfig.files.enabled}
        <FileList />
      {:else if currentPage === 'collections' && componentsConfig.collections.enabled}
        <CollectionViewer collectionIds={[]} />
      {:else if currentPage === 'settings'}
        <ComponentSettings />
      {:else if currentPage === 'test'}
        <TestSimple />
      {:else}
        <div class="empty-state">Компонент отключен в настройках или страница не найдена</div>
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

  .session-warnings {
    background: #78350f;
    border-bottom: 2px solid #f59e0b;
    padding: clamp(0.5rem, 0.75vh, 0.625rem) clamp(1rem, 1.5vw, 1.25rem);
    flex-shrink: 0;
  }

  .session-warnings .warning-item {
    color: #fcd34d;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
    margin-bottom: 0.25rem;
  }

  .session-warnings .warning-item:last-child {
    margin-bottom: 0;
  }

  .app-container {
    display: flex;
    flex-direction: row;
    flex: 1;
    overflow: hidden;
    position: relative;
  }

  /* Планшеты и маленькие экраны */
  @media (max-width: 1024px) {
    .app-container {
      flex-direction: row;
    }

    .main-content {
      padding: clamp(0.875rem, 1.5vw, 1.25rem) clamp(0.875rem, 2vw, 1.5rem);
    }
  }

  /* Мобильная адаптация */
  @media (max-width: 767px) {
    .app-container {
      flex-direction: column;
    }

    .main-content {
      padding: clamp(0.75rem, 2vw, 1rem);
      width: 100%;
      min-height: calc(100vh - 3rem);
    }
  }

  /* Очень маленькие экраны */
  @media (max-width: 480px) {
    .main-content {
      padding: 0.75rem;
    }
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: clamp(1rem, 2vw, 1.5rem) clamp(1rem, 2.5vw, 2rem);
    background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
    min-width: 0;
    position: relative;
    scroll-behavior: smooth;
    -webkit-overflow-scrolling: touch;
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
    touch-action: manipulation; /* Улучшение для touch устройств */
  }

  /* Планшеты */
  @media (max-width: 1024px) {
    .sidebar-toggle-button {
      width: clamp(2.25rem, 3vw, 2.75rem);
      height: clamp(2.25rem, 3vw, 2.75rem);
    }
  }

  /* Мобильная адаптация кнопки */
  @media (max-width: 767px) {
    .sidebar-toggle-button {
      z-index: 1001;
      width: 3rem;
      height: 3rem;
      top: 1rem;
      left: 1rem;
      box-shadow: 0 6px 20px rgba(14, 165, 233, 0.5);
    }
  }

  /* Очень маленькие экраны */
  @media (max-width: 480px) {
    .sidebar-toggle-button {
      width: 2.75rem;
      height: 2.75rem;
      top: 0.75rem;
      left: 0.75rem;
    }
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

  /* Overlay для мобильного меню */
  .sidebar-overlay {
    display: none;
  }

  @media (max-width: 767px) {
    .sidebar-overlay {
      display: block;
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: rgba(0, 0, 0, 0.6);
      z-index: 999;
      /* Оптимизация: убрали backdrop-filter для лучшей производительности */
      animation: fadeIn 0.3s ease-in-out;
      touch-action: none;
      /* Оптимизация производительности */
      will-change: opacity;
    }

    .sidebar-overlay:active {
      background: rgba(0, 0, 0, 0.7);
    }
  }

  /* Планшеты - легкий overlay */
  @media (min-width: 768px) and (max-width: 1024px) {
    .sidebar-overlay {
      display: none;
    }
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
</style>
