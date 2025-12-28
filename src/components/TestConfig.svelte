<script lang="ts">
  import { componentsConfig, updateComponentConfig } from '@/config/components';

  function toggleComponent(component: keyof typeof componentsConfig) {
    updateComponentConfig({
      [component]: {
        ...componentsConfig[component],
        enabled: !componentsConfig[component].enabled,
      },
    });
  }
</script>

<div class="test-config">
  <h2>Тест конфигурации компонентов</h2>

  <div class="config-status">
    <h3>Текущая конфигурация:</h3>
    <pre>{JSON.stringify(componentsConfig, null, 2)}</pre>
  </div>

  <div class="component-toggles">
    <h3>Переключатели компонентов:</h3>
    {#each Object.entries(componentsConfig) as [key, config] (key)}
      {@const componentKey = key as keyof typeof componentsConfig}
      <label class="toggle-item">
        <input
          type="checkbox"
          checked={config.enabled}
          onchange={() => toggleComponent(componentKey)}
        />
        <span>{componentKey}</span>
        {#if config.priority}
          <small>(приоритет: {config.priority})</small>
        {/if}
      </label>
    {/each}
  </div>

  <div class="info">
    <p>Эта страница тестирует систему конфигурации компонентов гибридной архитектуры.</p>
    <p>Изменения сохраняются автоматически в localStorage.</p>
  </div>
</div>

<style>
  .test-config {
    padding: 2rem;
    max-width: 800px;
    margin: 0 auto;
  }

  .config-status {
    margin: 2rem 0;
    padding: 1rem;
    background: #1e293b;
    border-radius: 0.5rem;
  }

  .config-status pre {
    background: #0f172a;
    padding: 1rem;
    border-radius: 0.25rem;
    overflow-x: auto;
    font-size: 0.875rem;
    color: #e2e8f0;
  }

  .component-toggles {
    margin: 2rem 0;
  }

  .toggle-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem;
    margin: 0.25rem 0;
    background: #1e293b;
    border-radius: 0.25rem;
    cursor: pointer;
  }

  .toggle-item:hover {
    background: #334155;
  }

  .toggle-item input[type='checkbox'] {
    width: 1.25rem;
    height: 1.25rem;
    accent-color: #0ea5e9;
  }

  .toggle-item span {
    flex: 1;
    font-weight: 500;
    color: #e2e8f0;
  }

  .toggle-item small {
    color: #64748b;
    font-size: 0.75rem;
  }

  .info {
    margin-top: 2rem;
    padding: 1rem;
    background: #0f172a;
    border-radius: 0.5rem;
    color: #cbd5e1;
  }

  .info p {
    margin: 0.5rem 0;
  }
</style>
