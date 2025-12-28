<script lang="ts">
  import {
    HomeIcon,
    CodeIcon,
    BellIcon,
    FolderIcon,
    CollectionIcon,
    CogIcon,
    XMarkIcon,
    MenuIcon,
  } from './icons/index';
  import type { Site } from '@/lib/api';
  // import Tooltip from './Tooltip.svelte'; // Временно отключен для тестирования
  import type { AppComponents } from '../config/components';

  interface Props {
    currentPage: string;
    sites: Site[];
    selectedSiteId: number | null;
    isOpen: boolean;
    componentsConfig: AppComponents;
    onPageChange: (page: string) => void;
    onSiteSelect: (siteId: number | null) => void;
    onToggle: () => void;
  }

  const {
    currentPage,
    sites,
    selectedSiteId,
    isOpen,
    componentsConfig,
    onPageChange,
    onSiteSelect,
    onToggle,
  }: Props = $props();

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
    <button
      type="button"
      class="toggle-button"
      onclick={onToggle}
      title={isOpen ? 'Скрыть меню' : 'Показать меню'}
      aria-label={isOpen ? 'Скрыть меню' : 'Показать меню'}
    >
      {#if isOpen}
        <XMarkIcon class="icon-small" />
      {:else}
        <MenuIcon class="icon-small" />
      {/if}
    </button>
  </div>

  <nav class="sidebar-nav">
    {#if componentsConfig.mods.enabled}
      <button
        type="button"
        class="nav-item"
        class:active={currentPage === 'mods'}
        onclick={() => handlePageClick('mods')}
        title={!isOpen ? 'Моды' : ''}
        aria-label="Моды"
      >
        <HomeIcon class="icon" />
        {#if isOpen}
          <span>Моды</span>
        {/if}
      </button>
    {/if}

    {#if componentsConfig.sites.enabled}
      <button
        type="button"
        class="nav-item"
        class:active={currentPage === 'sites'}
        onclick={() => handlePageClick('sites')}
        title={!isOpen ? 'Сайты' : ''}
        aria-label="Сайты"
      >
        <CogIcon class="icon" />
        {#if isOpen}
          <span>Сайты</span>
        {/if}
      </button>
    {/if}

    {#if componentsConfig.parser.enabled}
      <button
        type="button"
        class="nav-item"
        class:active={currentPage === 'parser'}
        onclick={() => handlePageClick('parser')}
        title={!isOpen ? 'Конструктор парсеров' : ''}
        aria-label="Конструктор парсеров"
      >
        <CodeIcon class="icon" />
        {#if isOpen}
          <span>Конструктор</span>
        {/if}
      </button>
    {/if}

    {#if componentsConfig.notifications.enabled}
      <button
        type="button"
        class="nav-item"
        class:active={currentPage === 'notifications'}
        onclick={() => handlePageClick('notifications')}
        title={!isOpen ? 'Уведомления' : ''}
        aria-label="Уведомления"
      >
        <BellIcon class="icon" />
        {#if isOpen}
          <span>Уведомления</span>
        {/if}
      </button>
    {/if}

    {#if componentsConfig.files.enabled}
      <button
        type="button"
        class="nav-item"
        class:active={currentPage === 'files'}
        title={!isOpen ? 'Файлы' : ''}
        aria-label="Файлы"
      >
        <FolderIcon class="icon" />
        {#if isOpen}
          <span>Файлы</span>
        {/if}
      </button>
    {/if}

    {#if componentsConfig.collections.enabled}
      <button
        type="button"
        class="nav-item"
        class:active={currentPage === 'collections'}
        title={!isOpen ? 'Коллекции' : ''}
        aria-label="Коллекции"
      >
        <CollectionIcon class="icon" />
        {#if isOpen}
          <span>Коллекции</span>
        {/if}
      </button>
    {/if}

    <button
      type="button"
      class="nav-item"
      class:active={currentPage === 'settings'}
      onclick={() => handlePageClick('settings')}
      title={!isOpen ? 'Настройки компонентов' : ''}
      aria-label="Настройки компонентов"
    >
      <CogIcon class="icon" />
      {#if isOpen}
        <span>Настройки</span>
      {/if}
    </button>

    <button
      type="button"
      class="nav-item"
      class:active={currentPage === 'test'}
      onclick={() => handlePageClick('test')}
      title={!isOpen ? 'Тест конфигурации' : ''}
      aria-label="Тест конфигурации"
    >
      <CodeIcon class="icon" />
      {#if isOpen}
        <span>Тест</span>
      {/if}
    </button>
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
      {#each sites as site (site.id)}
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
    --sidebar-width: clamp(240px, 18vw, 320px);
    --sidebar-collapsed-width: clamp(56px, 5vw, 72px);
    --header-padding: clamp(0.75rem, 1.2vw, 1.25rem);
    --nav-padding: clamp(0.5rem, 0.8vw, 1rem);
    --font-size-base: clamp(0.8125rem, 0.9vw, 0.9375rem);
    --font-size-title: clamp(1rem, 1.5vw, 1.375rem);
    --font-size-small: clamp(0.6875rem, 0.8vw, 0.8125rem);
    --icon-size: clamp(1.125rem, 1.3vw, 1.375rem);
    --icon-small-size: clamp(0.875rem, 1vw, 1.125rem);
    --border-radius: clamp(0.375rem, 0.5vw, 0.5rem);
    --transition-speed: 0.3s cubic-bezier(0.4, 0, 0.2, 1);

    width: var(--sidebar-width);
    min-width: 0;
    max-width: 100vw;
    background: linear-gradient(180deg, #0f172a 0%, #1e293b 100%);
    border-right: 1px solid rgba(30, 41, 59, 0.8);
    display: flex;
    flex-direction: column;
    height: 100vh;
    position: relative;
    /* Оптимизация: только width для transition, transform для анимации */
    transition: width var(--transition-speed);
    box-shadow: 2px 0 8px rgba(0, 0, 0, 0.3);
    z-index: 100;
    overflow: hidden;
    /* Оптимизация производительности */
    will-change: width;
    contain: layout style paint;
  }

  .sidebar.collapsed {
    width: var(--sidebar-collapsed-width);
  }

  /* Адаптация для маленьких окон (до 1024px) */
  @media (max-width: 1024px) {
    .sidebar {
      --sidebar-width: clamp(200px, 20vw, 280px);
      --sidebar-collapsed-width: clamp(52px, 4.5vw, 68px);
      --header-padding: clamp(0.625rem, 1vw, 1rem);
      --nav-padding: clamp(0.5rem, 0.75vw, 0.875rem);
      --font-size-title: clamp(0.9375rem, 1.3vw, 1.25rem);
      --font-size-base: clamp(0.75rem, 0.85vw, 0.875rem);
      --icon-size: clamp(1rem, 1.2vw, 1.25rem);
      --icon-small-size: clamp(0.8125rem, 0.95vw, 1rem);
    }
  }

  /* Адаптация для средних окон (до 768px) */
  @media (max-width: 768px) {
    .sidebar {
      --sidebar-width: clamp(180px, 25vw, 260px);
      --sidebar-collapsed-width: clamp(48px, 4vw, 64px);
      --header-padding: clamp(0.5rem, 0.875vw, 0.875rem);
      --nav-padding: clamp(0.375rem, 0.6vw, 0.75rem);
      --font-size-title: clamp(0.875rem, 1.2vw, 1.125rem);
      --font-size-base: clamp(0.6875rem, 0.8vw, 0.8125rem);
      --icon-size: clamp(0.9375rem, 1.1vw, 1.125rem);
      --icon-small-size: clamp(0.75rem, 0.9vw, 0.9375rem);
    }

    .nav-item {
      padding: clamp(0.5rem, 0.7vw, 0.625rem) clamp(0.5rem, 0.8vw, 0.75rem);
      min-height: clamp(1.875rem, 2.5vh, 2.5rem);
      max-height: clamp(2.25rem, 3vh, 2.75rem);
      gap: clamp(0.25rem, 0.5vw, 0.5rem);
    }
  }

  /* Мобильная адаптация */
  @media (max-width: 767px) {
    .sidebar {
      position: fixed;
      left: 0;
      top: 0;
      width: min(280px, 85vw);
      max-width: 85vw;
      transform: translateX(-100%);
      z-index: 1000;
      box-shadow: 4px 0 20px rgba(0, 0, 0, 0.6);
      --sidebar-width: min(280px, 85vw);
      --sidebar-collapsed-width: min(280px, 85vw);
      --header-padding: clamp(0.5rem, 0.875vw, 0.875rem);
      --nav-padding: clamp(0.375rem, 0.6vw, 0.75rem);
      --font-size-title: clamp(0.875rem, 1.2vw, 1.125rem);
      --font-size-base: clamp(0.6875rem, 0.8vw, 0.8125rem);
      --font-size-small: clamp(0.625rem, 0.75vw, 0.75rem);
      --icon-size: clamp(1rem, 4vw, 1.25rem);
      --icon-small-size: clamp(0.875rem, 3.5vw, 1rem);
      transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .sidebar:not(.collapsed) {
      transform: translateX(0);
    }

    .sidebar.collapsed {
      width: min(280px, 85vw);
      transform: translateX(-100%);
    }

    .nav-item {
      padding: clamp(0.5rem, 0.65vw, 0.625rem) clamp(0.5rem, 0.75vw, 0.75rem);
      min-height: clamp(1.75rem, 2.25vh, 2.25rem);
      max-height: clamp(2.125rem, 2.75vh, 2.625rem);
      gap: clamp(0.25rem, 0.45vw, 0.5rem);
      font-size: clamp(0.625rem, 0.75vw, 0.75rem);
    }

    .sidebar-nav {
      gap: clamp(0.25rem, 0.4vw, 0.5rem);
    }
  }

  /* Очень маленькие экраны */
  @media (max-width: 480px) {
    .sidebar {
      width: min(260px, 90vw);
      max-width: 90vw;
      --sidebar-width: min(260px, 90vw);
      --sidebar-collapsed-width: min(260px, 90vw);
      --header-padding: clamp(0.5rem, 0.75vw, 0.75rem);
      --nav-padding: clamp(0.375rem, 0.55vw, 0.625rem);
      --icon-size: clamp(0.875rem, 4.5vw, 1.125rem);
      --icon-small-size: clamp(0.75rem, 4vw, 0.9375rem);
      --font-size-title: clamp(0.8125rem, 1.1vw, 1rem);
      --font-size-base: clamp(0.625rem, 0.75vw, 0.75rem);
      --font-size-small: clamp(0.5625rem, 0.7vw, 0.6875rem);
    }

    .nav-item {
      padding: clamp(0.4375rem, 0.6vw, 0.5625rem) clamp(0.4375rem, 0.7vw, 0.625rem);
      min-height: clamp(1.625rem, 2vh, 2rem);
      max-height: clamp(2rem, 2.5vh, 2.5rem);
      gap: clamp(0.25rem, 0.4vw, 0.4375rem);
      font-size: clamp(0.5625rem, 0.7vw, 0.6875rem);
    }

    .sidebar-nav {
      gap: clamp(0.25rem, 0.35vw, 0.4375rem);
      padding: clamp(0.375rem, 0.55vw, 0.625rem);
    }

    .sidebar-header {
      padding: clamp(0.5rem, 0.75vw, 0.75rem);
      min-height: clamp(2.5rem, 3.5vh, 3rem);
      max-height: clamp(2.5rem, 3.5vh, 3rem);
    }
  }

  /* Экстремально маленькие экраны (до 360px) */
  @media (max-width: 360px) {
    .sidebar {
      width: min(240px, 95vw);
      max-width: 95vw;
      --sidebar-width: min(240px, 95vw);
      --sidebar-collapsed-width: min(240px, 95vw);
      --header-padding: clamp(0.4375rem, 0.7vw, 0.625rem);
      --nav-padding: clamp(0.3125rem, 0.5vw, 0.5rem);
    }

    .nav-item {
      padding: clamp(0.375rem, 0.55vw, 0.5rem) clamp(0.375rem, 0.6vw, 0.5625rem);
      min-height: clamp(1.5rem, 1.875vh, 1.875rem);
      max-height: clamp(1.875rem, 2.25vh, 2.25rem);
      gap: clamp(0.1875rem, 0.35vw, 0.375rem);
    }

    .sidebar-nav {
      gap: clamp(0.1875rem, 0.3vw, 0.375rem);
    }
  }

  .sidebar-header {
    padding: var(--header-padding);
    border-bottom: 1px solid rgba(30, 41, 59, 0.8);
    display: flex;
    justify-content: space-between;
    align-items: center;
    min-height: clamp(3rem, 4.5vh, 4rem);
    max-height: clamp(3rem, 4.5vh, 4rem);
    height: clamp(3rem, 4.5vh, 4rem);
    background: rgba(15, 23, 42, 0.95);
    /* Оптимизация: убрали backdrop-filter - очень дорогая операция */
    gap: clamp(0.375rem, 0.5vw, 0.625rem);
    overflow: hidden;
    flex-shrink: 0;
    box-sizing: border-box;
    position: relative;
    /* Оптимизация производительности */
    contain: layout style paint;
  }

  .logo-container {
    display: flex;
    align-items: center;
    gap: clamp(0.375rem, 0.6vw, 0.625rem);
    flex: 1;
    min-width: 0;
    overflow: hidden;
  }

  .logo-icon {
    width: var(--icon-size);
    height: var(--icon-size);
    min-width: var(--icon-size);
    min-height: var(--icon-size);
    max-width: var(--icon-size);
    max-height: var(--icon-size);
    flex-shrink: 0;
    border-radius: var(--border-radius);
    transition: transform 0.2s;
    object-fit: contain;
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
    flex: 1;
    min-width: 0;
    max-width: 100%;
  }

  .sidebar.collapsed .logo {
    opacity: 0;
    width: 0;
  }

  /* Ограничение для Tooltip контейнера */
  .sidebar-header :global(.tooltip) {
    display: flex !important;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    height: auto !important;
    max-height: 1rem !important;
    max-width: 1rem !important;
    overflow: hidden !important;
    box-sizing: border-box !important;
  }

  .toggle-button {
    /* Фиксированный маленький размер */
    width: 1rem !important;
    height: 1rem !important;
    min-width: 1rem !important;
    min-height: 1rem !important;
    max-width: 1rem !important;
    max-height: 1rem !important;
    padding: 0 !important;
    margin: 0 !important;
    background: transparent;
    border: 1px solid rgba(148, 163, 184, 0.2);
    border-radius: var(--border-radius);
    color: #94a3b8;
    cursor: pointer;
    display: flex !important;
    align-items: center;
    justify-content: center;
    /* Оптимизация: только нужные свойства */
    transition:
      background-color 0.2s ease,
      border-color 0.2s ease,
      color 0.2s ease,
      transform 0.2s ease;
    flex-shrink: 0;
    box-sizing: border-box !important;
    align-self: center;
    overflow: hidden !important;
    position: relative;
    /* Оптимизация производительности */
    will-change: transform;
  }

  /* Строгое ограничение иконки внутри toggle-button */
  .toggle-button .icon-small {
    width: 0.75rem !important;
    height: 0.75rem !important;
    min-width: 0.75rem !important;
    min-height: 0.75rem !important;
    max-width: 0.75rem !important;
    max-height: 0.75rem !important;
  }

  .toggle-button .icon-small :global(svg),
  .toggle-button :global(svg) {
    width: 0.75rem !important;
    height: 0.75rem !important;
    min-width: 0.75rem !important;
    min-height: 0.75rem !important;
    max-width: 0.75rem !important;
    max-height: 0.75rem !important;
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
    gap: clamp(0.25rem, 0.4vw, 0.5rem);
    flex-shrink: 0;
    overflow-y: auto;
    overflow-x: hidden;
    min-height: 0;
  }

  .nav-item {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    gap: clamp(0.5rem, 0.75vw, 0.75rem);
    padding: clamp(0.5rem, 0.75vw, 0.75rem) clamp(0.5rem, 0.875vw, 0.875rem);
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    border-radius: var(--border-radius);
    /* Оптимизация: только нужные свойства для transition */
    transition:
      background-color 0.2s ease,
      color 0.2s ease,
      transform 0.2s ease;
    text-align: left;
    font-size: var(--font-size-base);
    font-weight: 500;
    position: relative;
    overflow: hidden;
    min-height: clamp(2rem, 2.5vh, 2.75rem);
    max-height: clamp(2.5rem, 3.5vh, 3rem);
    width: 100%;
    box-sizing: border-box;
    flex-shrink: 0;
    /* Оптимизация производительности */
    will-change: transform;
  }

  .sidebar.collapsed .nav-item {
    justify-content: center;
    padding: clamp(0.625rem, 0.875vw, 0.75rem);
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
    overflow: hidden;
    text-overflow: ellipsis;
    transition: opacity var(--transition-speed);
    flex: 1;
    min-width: 0;
    /* Обеспечиваем достаточно места для текста */
    margin-left: 0;
    padding-right: 0.25rem;
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
    padding: clamp(0.4375rem, 0.7vw, 0.625rem) clamp(0.5rem, 0.8vw, 0.75rem);
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
    box-sizing: border-box;
    min-height: clamp(1.75rem, 2.25vh, 2.25rem);
    max-height: clamp(2.125rem, 2.75vh, 2.625rem);
  }

  @media (max-width: 768px) {
    .site-item {
      padding: clamp(0.375rem, 0.6vw, 0.5625rem) clamp(0.4375rem, 0.7vw, 0.625rem);
      min-height: clamp(1.625rem, 2vh, 2rem);
      max-height: clamp(2rem, 2.5vh, 2.5rem);
      font-size: clamp(0.5625rem, 0.7vw, 0.6875rem);
    }
  }

  @media (max-width: 480px) {
    .site-item {
      padding: clamp(0.3125rem, 0.55vw, 0.5rem) clamp(0.375rem, 0.6vw, 0.5625rem);
      min-height: clamp(1.5rem, 1.875vh, 1.875rem);
      max-height: clamp(1.875rem, 2.25vh, 2.25rem);
      font-size: clamp(0.5rem, 0.65vw, 0.625rem);
    }
  }

  @media (max-width: 360px) {
    .site-item {
      padding: clamp(0.25rem, 0.5vw, 0.4375rem) clamp(0.3125rem, 0.55vw, 0.5rem);
      min-height: clamp(1.375rem, 1.75vh, 1.75rem);
      max-height: clamp(1.75rem, 2vh, 2rem);
    }
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

  /* Ограничение размера иконок в навигации - маленький размер */
  .nav-item .icon {
    width: 1rem !important;
    height: 1rem !important;
    min-width: 1rem !important;
    min-height: 1rem !important;
    max-width: 1rem !important;
    max-height: 1rem !important;
    flex-shrink: 0;
    transition: transform 0.2s;
    box-sizing: border-box;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  /* Принудительное ограничение размера SVG внутри иконок навигации - очень агрессивно */
  .nav-item .icon :global(svg),
  .nav-item :global(svg.icon),
  .nav-item :global(svg),
  .sidebar-nav button :global(svg) {
    width: 1rem !important;
    height: 1rem !important;
    min-width: 1rem !important;
    min-height: 1rem !important;
    max-width: 1rem !important;
    max-height: 1rem !important;
    flex-shrink: 0;
    display: block;
    box-sizing: border-box;
    overflow: hidden;
  }

  /* Общие стили для иконок (для других мест использования) */
  .icon {
    width: var(--icon-size);
    height: var(--icon-size);
    min-width: var(--icon-size);
    min-height: var(--icon-size);
    max-width: var(--icon-size);
    max-height: var(--icon-size);
    flex-shrink: 0;
    transition: transform 0.2s;
    box-sizing: border-box;
  }

  /* Адаптация для маленьких экранов */
  @media (max-width: 768px) {
    .nav-item .icon {
      width: 0.75rem !important;
      height: 0.75rem !important;
      min-width: 0.75rem !important;
      min-height: 0.75rem !important;
      max-width: 0.75rem !important;
      max-height: 0.75rem !important;
    }

    .nav-item .icon :global(svg),
    .nav-item :global(svg.icon) {
      width: 0.75rem !important;
      height: 0.75rem !important;
      min-width: 0.75rem !important;
      min-height: 0.75rem !important;
      max-width: 0.75rem !important;
      max-height: 0.75rem !important;
    }
  }

  @media (max-width: 480px) {
    .nav-item .icon {
      width: 0.625rem !important;
      height: 0.625rem !important;
      min-width: 0.625rem !important;
      min-height: 0.625rem !important;
      max-width: 0.625rem !important;
      max-height: 0.625rem !important;
    }

    .nav-item .icon :global(svg),
    .nav-item :global(svg.icon) {
      width: 0.625rem !important;
      height: 0.625rem !important;
      min-width: 0.625rem !important;
      min-height: 0.625rem !important;
      max-width: 0.625rem !important;
      max-height: 0.625rem !important;
    }
  }

  .nav-item:hover .icon {
    transform: scale(1.1);
  }

  .nav-item.active .icon {
    color: #0ea5e9;
  }

  .icon-small {
    width: 0.75rem !important;
    height: 0.75rem !important;
    min-width: 0.75rem !important;
    min-height: 0.75rem !important;
    max-width: 0.75rem !important;
    max-height: 0.75rem !important;
    flex-shrink: 0;
  }

  /* Ограничение размера SVG внутри icon-small */
  .icon-small :global(svg),
  .toggle-button :global(svg) {
    width: 0.75rem !important;
    height: 0.75rem !important;
    min-width: 0.75rem !important;
    min-height: 0.75rem !important;
    max-width: 0.75rem !important;
    max-height: 0.75rem !important;
  }
</style>
