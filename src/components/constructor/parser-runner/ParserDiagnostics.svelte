<script lang="ts">
  interface Diagnostic {
    type: 'info' | 'success' | 'warning' | 'error';
    message: string;
  }

  interface Props {
    diagnostics: Diagnostic[];
  }

  const { diagnostics }: Props = $props();
</script>

{#if diagnostics.length > 0}
  <div class="diagnostics-container">
    <div class="diagnostics-header">
      <strong>🔍 Диагностика выполнения:</strong>
    </div>
    <div class="diagnostics-list">
      {#each diagnostics as diag, index (index)}
        <div
          class="diagnostic-item"
          class:diagnostic-info={diag.type === 'info'}
          class:diagnostic-success={diag.type === 'success'}
          class:diagnostic-warning={diag.type === 'warning'}
          class:diagnostic-error={diag.type === 'error'}
        >
          <span class="diagnostic-icon">
            {#if diag.type === 'success'}✅
            {:else if diag.type === 'warning'}⚠️
            {:else if diag.type === 'error'}❌
            {:else}ℹ️
            {/if}
          </span>
          <span class="diagnostic-message">{diag.message}</span>
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .diagnostics-container {
    padding: 1rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
  }

  .diagnostics-header {
    color: #0ea5e9;
    margin-bottom: 0.75rem;
    font-size: 0.875rem;
  }

  .diagnostics-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .diagnostic-item {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    padding: 0.5rem;
    border-radius: 0.375rem;
    font-size: 0.875rem;
  }

  .diagnostic-item.diagnostic-info {
    background: rgba(14, 165, 233, 0.1);
    color: #0ea5e9;
  }

  .diagnostic-item.diagnostic-success {
    background: rgba(16, 185, 129, 0.1);
    color: #10b981;
  }

  .diagnostic-item.diagnostic-warning {
    background: rgba(245, 158, 11, 0.1);
    color: #f59e0b;
  }

  .diagnostic-item.diagnostic-error {
    background: rgba(239, 68, 68, 0.1);
    color: #ef4444;
  }

  .diagnostic-icon {
    font-size: 1rem;
    flex-shrink: 0;
  }

  .diagnostic-message {
    flex: 1;
  }
</style>
