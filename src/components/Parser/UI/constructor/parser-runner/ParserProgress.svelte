<script lang="ts">
  interface Props {
    progress: string;
    isRunning: boolean;
  }

  const { progress, isRunning }: Props = $props();
</script>

{#if progress && isRunning}
  <div class="progress-bar">
    <div class="progress-text">{progress}</div>
    <div class="progress-indicator">
      <div class="progress-bar-fill"></div>
    </div>
  </div>
{/if}

<style>
  .progress-bar {
    padding: 0.75rem 1rem;
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    border-radius: 0.5rem;
  }

  .progress-text {
    color: #0ea5e9;
    font-size: 0.875rem;
    margin-bottom: 0.5rem;
  }

  .progress-indicator {
    height: 4px;
    background: rgba(14, 165, 233, 0.2);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-bar-fill {
    height: 100%;
    background: #0ea5e9;
    animation: progress 1.5s ease-in-out infinite;
  }

  @keyframes progress {
    0%,
    100% {
      transform: translateX(-100%);
    }
    50% {
      transform: translateX(100%);
    }
  }
</style>
