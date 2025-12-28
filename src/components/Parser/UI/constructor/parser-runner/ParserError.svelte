<script lang="ts">
  interface Props {
    error: string | null;
    isRunning: boolean;
  }

  const { error, isRunning }: Props = $props();
</script>

{#if error && !isRunning}
  <div class="error-container">
    <div class="error-header">
      <span class="error-icon">❌</span>
      <strong>Ошибка выполнения парсера:</strong>
    </div>
    <div class="error-message">{error}</div>
  </div>
{/if}

<style>
  .error-container {
    padding: 1rem;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 0.5rem;
  }

  .error-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
    color: #ef4444;
  }

  .error-icon {
    font-size: 1.25rem;
  }

  .error-message {
    color: #fca5a5;
    font-size: 0.875rem;
    line-height: 1.5;
  }
</style>
