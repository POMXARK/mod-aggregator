<script lang="ts">
  import { PlayIcon, TrashIcon } from '@/components/icons';
  import type { Site } from '@/lib/api';
  import type { Node } from '@xyflow/svelte';

  // Props от родительского компонента
  interface Props {
    // Состояние
    selectedSite: Site | null;
    sites: Site[];
    error: string | null;
    showPageViewer: boolean;
    showAIChat: boolean;
    showAISettings: boolean;
    isAIGenerating: boolean;
    currentUrl: string;
    nodes: Node[];
    showUISettingsMenu: boolean;

    // Callbacks
    onSiteChange: (site: Site | null) => void;
    onTogglePageViewer: () => void;
    onAIGenerate: () => void;
    onToggleAISettings: () => void;
    onToggleAIChat: () => void;
    onDiagnoseState: () => void;
    onGenerateCode: () => void;
    onTestParser: () => void;
    onSaveParser: () => void;
    onDeleteSelected: () => void;
    onToggleUISettingsMenu: () => void;
    onExportSettings: () => void;
    onImportSettings: () => void;
    onResetSettings: () => void;
  }

  let { ...props }: Props = $props();
</script>

<div class="builder-header">
  <h2>Конструктор парсера</h2>
  <div class="header-actions">
    <select
      bind:value={props.selectedSite}
      onchange={() => props.onSiteChange(props.selectedSite)}
      class="site-select"
    >
      <option value="null">Выберите сайт</option>
      {#each props.sites as site (site.id)}
        <option value={site.id}>{site.name} ({site.url})</option>
      {/each}
    </select>
    {#if props.error}
      <div class="error-message" style="color: #ef4444; font-size: 0.875rem; padding: 0.5rem;">
        {props.error}
      </div>
    {/if}
    <button class="btn-secondary" onclick={props.onTogglePageViewer}>
      {props.showPageViewer ? 'Скрыть' : 'Открыть'} страницу
    </button>
    {#if props.showPageViewer}
      <div class="selection-hint">
        💡 Нажмите "Выделить элемент" в панели страницы, затем кликните на элемент для создания
        ноды
      </div>
    {/if}
    <button
      class="btn-primary"
      onclick={props.onAIGenerate}
      disabled={props.isAIGenerating}
      title="Сгенерировать парсер с помощью AI"
    >
      {#if props.isAIGenerating}
        <span>🤖 Генерация...</span>
      {:else}
        <span>🤖 AI Генерация</span>
      {/if}
    </button>
    <button
      class="btn-secondary"
      onclick={props.onToggleAISettings}
      title="Настройки AI"
    >
      ⚙️ AI
    </button>
    <button
      class="btn-secondary"
      onclick={props.onToggleAIChat}
      title="Открыть чат с AI"
    >
      💬 Чат
    </button>
    <button
      class="btn-secondary"
      onclick={props.onDiagnoseState}
      title="Диагностика состояния localStorage"
    >
      🔍 Диагностика
    </button>
    <button class="btn-primary" onclick={props.onGenerateCode}>
      <PlayIcon class="icon-small" />
      Генерировать код
    </button>
    <button
      class="btn-primary"
      onclick={props.onTestParser}
      disabled={props.nodes.length === 0 || !props.currentUrl}
      title={props.nodes.length === 0
        ? 'Создайте ноды парсера'
        : !props.currentUrl
          ? 'Загрузите страницу'
          : 'Запустить парсер на текущей странице'}
    >
      ▶️ Запустить парсер
    </button>
    <button class="btn-primary" onclick={props.onSaveParser}> Сохранить парсер </button>
    {#if props.nodes && Array.isArray(props.nodes) && props.nodes.some((n: Node) => n.selected)}
      <button class="btn-danger" onclick={props.onDeleteSelected}>
        <TrashIcon class="icon-small" />
        Удалить
      </button>
    {/if}
    <div class="ui-settings-menu">
      <button
        class="btn-secondary"
        onclick={props.onToggleUISettingsMenu}
        title="Настройки интерфейса"
      >
        ⚙️ Настройки
      </button>
      {#if props.showUISettingsMenu}
        <div class="ui-settings-dropdown">
          <button
            class="ui-settings-item"
            onclick={() => {
              props.onExportSettings();
              props.onToggleUISettingsMenu();
            }}
            title="Экспортировать настройки интерфейса в файл"
          >
            📤 Экспорт настроек
          </button>
          <button
            class="ui-settings-item"
            onclick={() => {
              props.onImportSettings();
              props.onToggleUISettingsMenu();
            }}
            title="Импортировать настройки интерфейса из файла"
          >
            📥 Импорт настроек
          </button>
          <div class="ui-settings-divider"></div>
          <button
            class="ui-settings-item ui-settings-danger"
            onclick={() => {
              props.onResetSettings();
              props.onToggleUISettingsMenu();
            }}
            title="Сбросить все настройки интерфейса к значениям по умолчанию"
          >
            🔄 Сбросить настройки
          </button>
        </div>
      {/if}
    </div>
  </div>
</div>

