<script lang="ts">
  import type { ParserSettings } from '@/lib/composables';

  interface Props {
    settings: ParserSettings;
    isRunning: boolean;
    onSettingsChange: (settings: ParserSettings) => void;
    onReset: () => void;
  }

  const { settings, isRunning, onSettingsChange, onReset }: Props = $props();

  function updateSetting<K extends keyof ParserSettings>(key: K, value: ParserSettings[K]) {
    onSettingsChange({ ...settings, [key]: value });
  }
</script>

<div class="settings-panel">
  <div class="settings-header">
    <strong>Настройки парсера</strong>
    <button class="btn-reset-settings" onclick={onReset} title="Сбросить настройки">
      🔄 Сбросить
    </button>
  </div>
  <div class="settings-content">
    <div class="setting-item">
      <label for="max-elements"> Максимум элементов для обработки: </label>
      <input
        type="number"
        id="max-elements"
        min="1"
        max="10000"
        value={settings.maxElements}
        oninput={e => updateSetting('maxElements', Number((e.target as HTMLInputElement).value))}
        disabled={isRunning}
      />
      <span class="setting-hint">(0 = без ограничений)</span>
    </div>
    <div class="setting-item">
      <label for="timeout-seconds"> Таймаут выполнения (секунды): </label>
      <input
        type="number"
        id="timeout-seconds"
        min="1"
        max="600"
        value={settings.timeoutSeconds}
        oninput={e => updateSetting('timeoutSeconds', Number((e.target as HTMLInputElement).value))}
        disabled={isRunning}
      />
      <span class="setting-hint">(от 1 до 600 секунд)</span>
    </div>
    <div class="setting-item">
      <label
        for="slow-mode"
        style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;"
      >
        <input
          type="checkbox"
          id="slow-mode"
          checked={settings.slowMode}
          onchange={e => updateSetting('slowMode', (e.target as HTMLInputElement).checked)}
          disabled={isRunning}
        />
        <span>Медленный режим (пошаговая обработка)</span>
      </label>
      <span class="setting-hint">(выводит каждый элемент по очереди)</span>
    </div>
    {#if settings.slowMode}
      <div class="setting-item indented">
        <label for="delay-per-element"> Задержка между элементами (мс): </label>
        <input
          type="number"
          id="delay-per-element"
          min="100"
          max="5000"
          step="100"
          value={settings.delayPerElement}
          oninput={e =>
            updateSetting('delayPerElement', Number((e.target as HTMLInputElement).value))}
          disabled={isRunning}
        />
        <span class="setting-hint">(100-5000 миллисекунд)</span>
      </div>
    {/if}
  </div>
</div>

<style>
  .settings-panel {
    padding: 1rem;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    margin-top: 0.5rem;
  }

  .settings-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    color: #0ea5e9;
    font-size: 0.875rem;
  }

  .btn-reset-settings {
    padding: 0.25rem 0.5rem;
    background: transparent;
    border: 1px solid #475569;
    border-radius: 0.25rem;
    color: #94a3b8;
    font-size: 0.75rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-reset-settings:hover {
    background: #334155;
    border-color: #64748b;
    color: #e2e8f0;
  }

  .settings-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .setting-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .setting-item.indented {
    margin-left: 1.5rem;
  }

  .setting-item label {
    color: #cbd5e1;
    font-size: 0.875rem;
    min-width: 250px;
  }

  .setting-item input {
    padding: 0.5rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    color: #e2e8f0;
    font-size: 0.875rem;
    width: 120px;
  }

  .setting-item input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .setting-item input:focus {
    outline: none;
    border-color: #0ea5e9;
  }

  .setting-hint {
    color: #64748b;
    font-size: 0.75rem;
    font-style: italic;
  }
</style>
