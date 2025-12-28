<script lang="ts">
  interface Props {
    isRunning: boolean;
    canRun: boolean;
    hasResults: boolean;
    hasError: boolean;
    hasDiagnostics: boolean;
    onRun: () => void;
    onStop: () => void;
    onClear: () => void;
    onToggleSettings: () => void;
    runDisabledReason?: string;
  }

  const {
    isRunning,
    canRun,
    hasResults,
    hasError,
    hasDiagnostics,
    onRun,
    onStop,
    onClear,
    onToggleSettings,
    runDisabledReason = '',
  }: Props = $props();
</script>

<div class="runner-controls">
  <button
    class="btn-run"
    onclick={onRun}
    disabled={isRunning || !canRun}
    title={runDisabledReason || 'Запустить парсер на текущей странице'}
  >
    {#if isRunning}
      <span class="spinner"></span>
      Запуск...
    {:else}
      ▶️ Запустить парсер
    {/if}
  </button>

  {#if isRunning}
    <button class="btn-stop" onclick={onStop} title="Остановить парсер"> ⏹️ Остановить </button>
  {/if}

  {#if (hasError || hasResults || hasDiagnostics) && !isRunning}
    <button class="btn-clear" onclick={onClear} title="Очистить результаты"> 🗑️ Очистить </button>
  {/if}

  <button class="btn-settings" onclick={onToggleSettings} title="Настройки парсера">
    ⚙️ Настройки
  </button>
</div>

<style>
  .runner-controls {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .btn-run {
    padding: 0.75rem 1.5rem;
    background: #0ea5e9;
    border: none;
    border-radius: 0.5rem;
    color: white;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: all 0.2s ease;
  }

  .btn-run:hover:not(:disabled) {
    background: #0284c7;
  }

  .btn-run:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-clear {
    padding: 0.75rem 1rem;
    background: #334155;
    border: 1px solid #475569;
    border-radius: 0.5rem;
    color: #e2e8f0;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-clear:hover {
    background: #475569;
    border-color: #64748b;
  }

  .btn-stop {
    padding: 0.75rem 1rem;
    background: #ef4444;
    border: none;
    border-radius: 0.5rem;
    color: white;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-stop:hover {
    background: #dc2626;
  }

  .btn-settings {
    padding: 0.75rem 1rem;
    background: #475569;
    border: 1px solid #64748b;
    border-radius: 0.5rem;
    color: #e2e8f0;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s ease;
    margin-left: auto;
  }

  .btn-settings:hover {
    background: #64748b;
    border-color: #94a3b8;
  }

  .spinner {
    width: 1rem;
    height: 1rem;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
