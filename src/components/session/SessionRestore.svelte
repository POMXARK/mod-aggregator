<script lang="ts">
  import { onMount } from 'svelte';
  import { SessionStateManager } from '@/lib/session/session-state';
  import type { SessionRestoreResult, UiPreferences } from '@/types/session';

  interface Props {
    onRestored?: (result: SessionRestoreResult) => void;
  }

  const { onRestored }: Props = $props();

  let restored = $state(false);
  let restoring = $state(false);
  let restoreResult = $state<SessionRestoreResult | null>(null);
  let error = $state<string | null>(null);

  // Состояние UI настроек
  let uiPreferences = $state<UiPreferences>({});
  let savePreferencesDebounced = $state<ReturnType<
    typeof SessionStateManager.createDebouncedPreferencesSaver
  > | null>(null);

  onMount(async () => {
    await restoreSession();
    // Инициализируем debounced saver для автосохранения
    savePreferencesDebounced = SessionStateManager.createDebouncedPreferencesSaver(500);
  });

  async function restoreSession() {
    restoring = true;
    error = null;

    try {
      restoreResult = await SessionStateManager.restoreSession();
      restored = restoreResult.restored;

      if (restoreResult.restored) {
        uiPreferences = restoreResult.uiPreferences;
      }

      if (onRestored && restoreResult) {
        onRestored(restoreResult);
      }
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при восстановлении сессии';
      console.error('Failed to restore session:', e);
    } finally {
      restoring = false;
    }
  }

  function updateViewMode(mode: 'list' | 'tiles') {
    uiPreferences = { ...uiPreferences, viewMode: mode };
    if (savePreferencesDebounced) {
      savePreferencesDebounced({ viewMode: mode });
    }
  }

  function updateSidebarCollapsed(collapsed: boolean) {
    uiPreferences = { ...uiPreferences, sidebarCollapsed: collapsed };
    if (savePreferencesDebounced) {
      savePreferencesDebounced({ sidebarCollapsed: collapsed });
    }
  }

  function updateTheme(theme: 'dark' | 'light') {
    uiPreferences = { ...uiPreferences, theme };
    if (savePreferencesDebounced) {
      savePreferencesDebounced({ theme });
    }
  }

  async function resetSession() {
    if (!confirm('Сбросить все настройки сессии? Это действие нельзя отменить.')) {
      return;
    }

    try {
      await SessionStateManager.resetSessionState();
      await restoreSession();
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при сбросе сессии';
    }
  }
</script>

<div class="session-restore">
  {#if restoring}
    <div class="restoring-indicator">
      <p>Восстановление сессии...</p>
    </div>
  {/if}

  {#if restoreResult && restoreResult.warnings.length > 0}
    <div class="warnings">
      <h3>Предупреждения при восстановлении:</h3>
      {#each restoreResult.warnings as warning, index (index)}
        <div class="warning-item">⚠️ {warning}</div>
      {/each}
    </div>
  {/if}

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if restored}
    <div class="session-controls">
      <h3>Настройки интерфейса</h3>

      <div class="preferences-group">
        <label>Режим отображения:</label>
        <div class="radio-group">
          <label>
            <input
              type="radio"
              name="view-mode"
              value="list"
              checked={uiPreferences.viewMode === 'list'}
              onchange={() => updateViewMode('list')}
            />
            Список
          </label>
          <label>
            <input
              type="radio"
              name="view-mode"
              value="tiles"
              checked={uiPreferences.viewMode === 'tiles'}
              onchange={() => updateViewMode('tiles')}
            />
            Плитки
          </label>
        </div>
      </div>

      <div class="preferences-group">
        <label>
          <input
            type="checkbox"
            checked={uiPreferences.sidebarCollapsed ?? false}
            onchange={e => updateSidebarCollapsed(e.currentTarget.checked)}
          />
          Свернуть боковую панель
        </label>
      </div>

      <div class="preferences-group">
        <label>Тема:</label>
        <select
          value={uiPreferences.theme || 'dark'}
          onchange={e => updateTheme(e.currentTarget.value as 'dark' | 'light')}
        >
          <option value="dark">Темная</option>
          <option value="light">Светлая</option>
        </select>
      </div>

      <div class="session-actions">
        <button type="button" class="btn-secondary btn-danger" onclick={resetSession}>
          Сбросить настройки
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  .session-restore {
    width: 100%;
    max-width: 800px;
    margin: 0 auto;
    padding: 1.5rem;
  }

  .restoring-indicator {
    text-align: center;
    padding: 2rem;
    color: #64748b;
  }

  .warnings {
    padding: 1rem;
    background: #78350f;
    border-radius: 0.375rem;
    margin-bottom: 1.5rem;
    border-left: 3px solid #f59e0b;
  }

  .warnings h3 {
    margin: 0 0 0.75rem 0;
    color: #fcd34d;
    font-size: 1rem;
    font-weight: 600;
  }

  .warning-item {
    color: #fcd34d;
    font-size: 0.875rem;
    margin-bottom: 0.5rem;
  }

  .warning-item:last-child {
    margin-bottom: 0;
  }

  .error {
    padding: 1rem;
    background: #7f1d1d;
    color: #fca5a5;
    border-radius: 0.375rem;
    margin-bottom: 1.5rem;
    border-left: 3px solid #ef4444;
  }

  .session-controls {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    padding: 1.5rem;
  }

  .session-controls h3 {
    margin: 0 0 1.5rem 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #e2e8f0;
  }

  .preferences-group {
    margin-bottom: 1.5rem;
  }

  .preferences-group label {
    display: block;
    color: #e2e8f0;
    font-size: 0.875rem;
    font-weight: 500;
    margin-bottom: 0.5rem;
  }

  .preferences-group input[type='checkbox'] {
    margin-right: 0.5rem;
    cursor: pointer;
  }

  .preferences-group select {
    width: 100%;
    max-width: 200px;
    padding: 0.5rem;
    background: #0f172a;
    color: #e2e8f0;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    font-size: 0.875rem;
  }

  .preferences-group select:focus {
    outline: none;
    border-color: #0ea5e9;
    box-shadow: 0 0 0 3px rgba(14, 165, 233, 0.1);
  }

  .radio-group {
    display: flex;
    gap: 1rem;
    margin-top: 0.5rem;
  }

  .radio-group label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    font-weight: normal;
  }

  .radio-group input[type='radio'] {
    cursor: pointer;
  }

  .session-actions {
    margin-top: 2rem;
    padding-top: 1.5rem;
    border-top: 1px solid #334155;
  }

  .btn-secondary {
    padding: 0.75rem 1.5rem;
    background: #334155;
    color: #e2e8f0;
    border: 1px solid #475569;
    border-radius: 0.375rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #475569;
    border-color: #64748b;
  }

  .btn-secondary.btn-danger {
    background: #7f1d1d;
    border-color: #991b1b;
    color: #fca5a5;
  }

  .btn-secondary.btn-danger:hover:not(:disabled) {
    background: #991b1b;
    border-color: #b91c1c;
  }
</style>
