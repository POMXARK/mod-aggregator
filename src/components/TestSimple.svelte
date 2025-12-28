<script lang="ts">
  import { componentsConfig } from '@/config/components';

  console.log('TestSimple component loaded');
  console.log('Components config:', componentsConfig);
</script>

<div class="test-simple">
  <h1>Hello World from TestSimple!</h1>
  <p>This is a simple test component that uses the new path mapping system.</p>

  <div class="config-display">
    <h2>Current Components Configuration:</h2>
    <ul>
      {#each Object.entries(componentsConfig) as [key, config] (key)}
        <li>
          <strong>{key}:</strong>
          <span class={config.enabled ? 'enabled' : 'disabled'}>
            {config.enabled ? 'Enabled' : 'Disabled'}
          </span>
          {#if config.priority}
            <small>(priority: {config.priority})</small>
          {/if}
        </li>
      {/each}
    </ul>
  </div>
</div>

<style>
  .test-simple {
    padding: 2rem;
    max-width: 800px;
    margin: 0 auto;
    font-family: Arial, sans-serif;
  }

  .test-simple h1 {
    color: #0ea5e9;
    margin-bottom: 1rem;
  }

  .test-simple p {
    color: #64748b;
    margin-bottom: 2rem;
  }

  .config-display {
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    padding: 1.5rem;
  }

  .config-display h2 {
    color: #e2e8f0;
    margin-bottom: 1rem;
  }

  .config-display ul {
    list-style: none;
    padding: 0;
  }

  .config-display li {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0;
    border-bottom: 1px solid #334155;
  }

  .config-display li:last-child {
    border-bottom: none;
  }

  .config-display strong {
    color: #0ea5e9;
    min-width: 120px;
  }

  .enabled {
    color: #10b981;
    font-weight: bold;
  }

  .disabled {
    color: #ef4444;
    font-weight: bold;
  }

  .config-display small {
    color: #64748b;
    margin-left: auto;
  }
</style>
