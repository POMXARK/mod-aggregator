<script lang="ts">
  interface Props {
    recentUrls: string[];
    show: boolean;
    onSelect: (url: string) => void;
    onRefresh: (url: string) => void;
    onRemove: (url: string) => void;
  }

  let { recentUrls, show, onSelect, onRefresh, onRemove }: Props = $props();
</script>

{#if show && recentUrls.length > 0}
  <div class="recent-urls-dropdown">
    <div class="recent-urls-header">Последние открытые:</div>
    {#each recentUrls as recentUrl}
      <div class="recent-url-item">
        <button 
          class="recent-url-link"
          onclick={() => onSelect(recentUrl)}
        >
          {recentUrl}
        </button>
        <div class="recent-url-actions">
          <button 
            class="recent-url-refresh"
            onclick={() => onRefresh(recentUrl)}
            aria-label="Обновить"
            title="Обновить страницу с сервера"
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99" />
            </svg>
          </button>
          <button 
            class="recent-url-remove"
            onclick={() => onRemove(recentUrl)}
            aria-label="Удалить"
            title="Удалить из списка"
          >
            ×
          </button>
        </div>
      </div>
    {/each}
  </div>
{/if}

<style>
  .recent-urls-dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    margin-top: 0.25rem;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    max-height: 300px;
    overflow-y: auto;
    z-index: 1000;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .recent-urls-header {
    padding: 0.375rem 0.5rem;
    font-size: 0.6875rem;
    color: #94a3b8;
    border-bottom: 1px solid #334155;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .recent-url-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.375rem 0.5rem;
    border-bottom: 1px solid #334155;
    transition: background 0.2s;
  }

  .recent-url-item:last-child {
    border-bottom: none;
  }

  .recent-url-item:hover {
    background: #0f172a;
  }

  .recent-url-link {
    flex: 1;
    text-align: left;
    background: none;
    border: none;
    color: #e2e8f0;
    font-size: 0.8125rem;
    cursor: pointer;
    padding: 0.125rem 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .recent-url-link:hover {
    color: #0ea5e9;
  }

  .recent-url-actions {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    flex-shrink: 0;
  }
  
  .recent-url-refresh,
  .recent-url-remove {
    background: none;
    border: none;
    color: #94a3b8;
    font-size: 1rem;
    cursor: pointer;
    padding: 0.125rem 0.375rem;
    line-height: 1;
    border-radius: 0.25rem;
    transition: all 0.2s;
    min-width: 1.5rem;
    height: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }
  
  .recent-url-refresh {
    font-size: 0.875rem;
  }
  
  .recent-url-refresh svg {
    width: 14px;
    height: 14px;
    display: block;
    flex-shrink: 0;
  }
  
  .recent-url-refresh:hover {
    color: #0ea5e9;
    background: rgba(14, 165, 233, 0.1);
  }
  
  .recent-url-remove:hover {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.1);
  }
</style>



