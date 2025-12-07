<script lang="ts">
  import RecentUrlsDropdown from './RecentUrlsDropdown.svelte';

  interface Props {
    url: string;
    loading: boolean;
    selectionMode: boolean;
    recentUrls: string[];
    showRecentUrls: boolean;
    panelCollapsed: boolean;
    onUrlChange: (url: string) => void;
    onLoad: (forceRefresh: boolean) => void;
    onSelectRecent: (url: string) => void;
    onRefreshRecent: (url: string) => void;
    onRemoveRecent: (url: string) => void;
    onToggleSelection: () => void;
    onToggleCollapse: () => void;
    onShowRecentUrls: (show: boolean) => void;
  }

  let {
    url,
    loading,
    selectionMode,
    recentUrls,
    showRecentUrls,
    panelCollapsed,
    onUrlChange,
    onLoad,
    onSelectRecent,
    onRefreshRecent,
    onRemoveRecent,
    onToggleSelection,
    onToggleCollapse,
    onShowRecentUrls,
  }: Props = $props();
</script>

<div class="viewer-header" class:collapsed={panelCollapsed}>
  <button 
    class="btn-collapse" 
    onclick={onToggleCollapse}
    aria-label={panelCollapsed ? 'Развернуть панель' : 'Свернуть панель'}
    title={panelCollapsed ? 'Развернуть панель' : 'Свернуть панель'}
  >
    {panelCollapsed ? '▼' : '▲'}
  </button>
  <div class="header-content">
    <div class="url-input-group">
      <div class="url-input-wrapper">
        <input 
          type="url" 
          value={url}
          oninput={(e) => onUrlChange(e.currentTarget.value)}
          onfocus={() => onShowRecentUrls(recentUrls.length > 0)}
          onkeydown={(e) => {
            if (e.key === 'Enter') {
              onLoad(false);
            }
          }}
          placeholder="https://example.com"
          class="url-input"
        />
        <RecentUrlsDropdown
          recentUrls={recentUrls}
          show={showRecentUrls}
          onSelect={onSelectRecent}
          onRefresh={onRefreshRecent}
          onRemove={onRemoveRecent}
        />
      </div>
      <button 
        class="btn-load" 
        onclick={() => onLoad(false)} 
        disabled={loading} 
        title="Загрузить привязанную страницу"
      >
        {loading ? '⏳' : '📥'}
      </button>
      <button 
        class="btn-refresh" 
        onclick={() => onLoad(true)} 
        disabled={loading} 
        title="Обновить страницу с сервера"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99" />
        </svg>
      </button>
    </div>
    <button 
      class="btn-select" 
      class:active={selectionMode}
      onclick={onToggleSelection}
      title={selectionMode ? 'Отключить выделение' : 'Включить выделение элементов'}
    >
      {selectionMode ? '✓' : '🎯'}
    </button>
    {#if selectionMode}
      <div class="selection-mode-hint">
        🎯 Режим выделения активен
      </div>
    {/if}
  </div>
</div>

<style>
  .viewer-header {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem;
    border-bottom: 1px solid #334155;
    align-items: center;
    transition: all 0.3s ease;
    min-height: 3rem;
  }
  
  .viewer-header.collapsed {
    min-height: 2.5rem;
  }
  
  .viewer-header.collapsed .header-content {
    display: none;
  }
  
  .btn-collapse {
    padding: 0.25rem 0.5rem;
    background: #334155;
    color: #e2e8f0;
    border: none;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    cursor: pointer;
    transition: all 0.2s;
    min-width: 1.5rem;
    height: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .btn-collapse:hover {
    background: #475569;
  }
  
  .header-content {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    flex: 1;
    transition: opacity 0.3s ease;
  }

  .url-input-group {
    display: flex;
    gap: 0.375rem;
    flex: 1;
  }

  .url-input-wrapper {
    position: relative;
    flex: 1;
  }

  .url-input {
    width: 100%;
    padding: 0.375rem 0.5rem;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    color: #e2e8f0;
    font-size: 0.8125rem;
  }

  .url-input:focus {
    outline: none;
    border-color: #0ea5e9;
  }

  .btn-load,
  .btn-select,
  .btn-refresh {
    padding: 0.375rem 0.75rem;
    background: #0ea5e9;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-weight: 500;
    font-size: 0.8125rem;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
    min-width: 2.5rem;
    height: 1.875rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .btn-load:hover:not(:disabled),
  .btn-select:hover:not(:disabled),
  .btn-refresh:hover:not(:disabled) {
    background: #0284c7;
  }
  
  .btn-load:disabled,
  .btn-refresh:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .btn-refresh {
    margin-left: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.375rem;
  }
  
  .btn-refresh svg {
    width: 16px;
    height: 16px;
  }

  .btn-select.active {
    background: #10b981;
  }

  .selection-mode-hint {
    padding: 0.25rem 0.5rem;
    background: rgba(16, 185, 129, 0.15);
    border: 1px solid rgba(16, 185, 129, 0.3);
    border-radius: 0.375rem;
    color: #10b981;
    font-size: 0.75rem;
    font-weight: 500;
    animation: pulse-hint 2s ease-in-out infinite;
    white-space: nowrap;
  }

  @keyframes pulse-hint {
    0%, 100% {
      opacity: 1;
      box-shadow: 0 0 0 0 rgba(16, 185, 129, 0.4);
    }
    50% {
      opacity: 0.8;
      box-shadow: 0 0 0 4px rgba(16, 185, 129, 0);
    }
  }
</style>



