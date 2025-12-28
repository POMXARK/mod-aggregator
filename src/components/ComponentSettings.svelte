<script lang="ts">
  import {
    componentsConfig,
    updateComponentConfig,
    defaultComponents,
    developmentProfiles,
    applyDevelopmentProfile,
    resetToFullConfig,
  } from '@/config/components';

  function handleToggle(component: keyof typeof componentsConfig, enabled: boolean) {
    const updatedConfig = { ...componentsConfig };
    updatedConfig[component] = { ...updatedConfig[component], enabled };
    updateComponentConfig(updatedConfig);
  }

  function handleReset() {
    if (confirm('Сбросить настройки компонентов к значениям по умолчанию?')) {
      updateComponentConfig(defaultComponents);
    }
  }

  function exportConfig() {
    const dataStr = JSON.stringify(componentsConfig, null, 2);
    const dataBlob = new Blob([dataStr], { type: 'application/json' });
    const url = URL.createObjectURL(dataBlob);
    const link = document.createElement('a');
    link.href = url;
    link.download = 'component-config.json';
    link.click();
    URL.revokeObjectURL(url);
  }

  function importConfig(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) {
      return;
    }

    const reader = new FileReader();
    reader.onload = e => {
      try {
        const config = JSON.parse(e.target?.result as string);
        updateComponentConfig(config);
        alert('Настройки успешно импортированы');
      } catch (error) {
        alert('Ошибка при импорте файла: ' + error);
      }
    };
    reader.readAsText(file);
  }

  function applyProfile(profileName: string) {
    applyDevelopmentProfile(profileName as keyof typeof developmentProfiles);
  }

  // Компоненты, отключенные в коде App.svelte
  const codeDisabledComponents: string[] = [];

  function getComponentStatus(componentKey: string) {
    if (codeDisabledComponents.includes(componentKey)) {
      return 'code-disabled';
    }
    return componentsConfig[componentKey as keyof typeof componentsConfig].enabled
      ? 'enabled'
      : 'disabled';
  }

  function getComponentStatusText(componentKey: string) {
    if (codeDisabledComponents.includes(componentKey)) {
      return 'Отключен в коде';
    }
    return componentsConfig[componentKey as keyof typeof componentsConfig].enabled
      ? 'Включен'
      : 'Отключен';
  }
</script>

<div class="component-settings">
  <div class="settings-header">
    <h3>Настройки компонентов</h3>
    <div class="settings-actions">
      <button class="btn-secondary" onclick={exportConfig}>Экспорт</button>
      <label class="btn-secondary">
        Импорт
        <input type="file" accept=".json" style="display: none" onchange={importConfig} />
      </label>
      <button class="btn-danger" onclick={handleReset}>Сброс</button>
    </div>
  </div>

  <div class="development-profiles">
    <h3>🎯 Режимы разработки</h3>
    <p>Быстрое переключение между режимами для фокусированной разработки:</p>
    <div class="help-text">
      💡 <strong>Как использовать:</strong>
      <ol>
        <li>Выберите режим для разработки конкретной системы</li>
        <li>Перезагрузите страницу для применения изменений</li>
        <li>Разрабатывайте в изолированном окружении</li>
        <li>Вернитесь к "Все компоненты" для полной проверки</li>
      </ol>
    </div>
    <div class="profile-buttons">
      {#each Object.keys(developmentProfiles) as profileName (profileName)}
        <button class="btn-primary profile-btn" onclick={() => applyProfile(profileName)}>
          {profileName === 'parserOnly'
            ? '🔧 Только Parser'
            : profileName === 'sitesOnly'
              ? '🌐 Только Sites'
              : profileName === 'filesOnly'
                ? '📁 Только Files'
                : profileName}
        </button>
      {/each}
      <button class="btn-success profile-btn" onclick={resetToFullConfig}>
        ✅ Все компоненты
      </button>
    </div>
  </div>

  <div class="settings-grid">
    {#each Object.keys(componentsConfig) as componentKey (componentKey)}
      {@const config = componentsConfig[componentKey as keyof typeof componentsConfig]}
      <div class="setting-item">
        <label class="setting-label">
          <input
            type="checkbox"
            checked={config.enabled}
            onchange={e =>
              handleToggle(
                componentKey as keyof typeof componentsConfig,
                (e.target as HTMLInputElement).checked
              )}
          />
          <span class="component-name">
            {componentKey.charAt(0).toUpperCase() + componentKey.slice(1)}
          </span>
          <span class={getComponentStatus(componentKey)}>
            {getComponentStatusText(componentKey)}
          </span>
          {#if config.priority !== undefined}
            <span class="priority">Приоритет: {config.priority}</span>
          {/if}
        </label>
      </div>
    {/each}
  </div>
</div>

<style>
  .component-settings {
    padding: 1rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
  }

  .settings-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid #334155;
  }

  .settings-header h3 {
    margin: 0;
    color: #e2e8f0;
  }

  .settings-actions {
    display: flex;
    gap: 0.5rem;
  }

  .btn-secondary,
  .btn-danger {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 0.375rem;
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 500;
    transition: background-color 0.2s ease;
  }

  .btn-secondary {
    background: #334155;
    color: #e2e8f0;
  }

  .btn-secondary:hover {
    background: #475569;
  }

  .btn-danger {
    background: #dc2626;
    color: white;
  }

  .btn-danger:hover {
    background: #b91c1c;
  }

  .btn-success {
    background: #059669;
    color: white;
  }

  .btn-success:hover {
    background: #047857;
  }

  .development-profiles {
    margin-bottom: 2rem;
    padding: 1rem;
    background: #1e293b;
    border-radius: 0.5rem;
    border: 1px solid #334155;
  }

  .development-profiles h3 {
    color: #e2e8f0;
    margin-bottom: 0.5rem;
  }

  .development-profiles p {
    color: #cbd5e1;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .help-text {
    background: #0f172a;
    padding: 1rem;
    border-radius: 0.375rem;
    margin-bottom: 1rem;
    border: 1px solid #334155;
  }

  .help-text ol {
    margin: 0.5rem 0 0 0;
    padding-left: 1.5rem;
  }

  .help-text li {
    margin-bottom: 0.25rem;
    color: #e2e8f0;
    font-size: 0.875rem;
  }

  .profile-buttons {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
  }

  .profile-btn {
    flex: 1;
    min-width: 120px;
    padding: 0.75rem 1rem;
    font-size: 0.875rem;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
  }

  .setting-item {
    padding: 0.75rem;
    background: #1e293b;
    border-radius: 0.375rem;
    border: 1px solid #334155;
  }

  .setting-label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    cursor: pointer;
    color: #e2e8f0;
  }

  .setting-label input[type='checkbox'] {
    width: 1rem;
    height: 1rem;
    accent-color: #0ea5e9;
  }

  .component-name {
    flex: 1;
    font-weight: 500;
  }

  .priority {
    font-size: 0.75rem;
    color: #64748b;
    background: #334155;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
  }

  .enabled {
    color: #10b981;
    font-weight: bold;
  }

  .disabled {
    color: #ef4444;
    font-weight: bold;
  }

  .code-disabled {
    color: #f59e0b;
    font-weight: bold;
    font-style: italic;
  }
</style>
