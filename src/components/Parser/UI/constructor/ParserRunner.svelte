<script lang="ts">
  import type { Node, Edge } from '@xyflow/svelte';
  import { useParserSettings, type ParserSettings, useParserRunner } from '@/lib/composables';
  import ParserRunnerControls from './parser-runner/ParserRunnerControls.svelte';
  import ParserSettingsPanel from './parser-runner/ParserSettingsPanel.svelte';
  import ParserProgress from './parser-runner/ParserProgress.svelte';
  import ParserError from './parser-runner/ParserError.svelte';
  import ParserDiagnostics from './parser-runner/ParserDiagnostics.svelte';
  import ParserResults from './parser-runner/ParserResults.svelte';

  interface Props {
    nodes: Node[];
    edges: Edge[];
    currentUrl: string;
    siteId?: number | null;
    onResults?: (results: unknown[], diagnostics: unknown[], stats: unknown) => void;
    onError?: (error: string) => void;
    // Настройки парсера (можно передать извне для синхронизации)
    initialMaxElements?: number;
    initialTimeoutSeconds?: number;
    initialSlowMode?: boolean;
    initialDelayPerElement?: number;
    onSettingsChange?: (settings: ParserSettings) => void;
  }

  const {
    nodes = [],
    edges = [],
    currentUrl = '',
    siteId = null,
    onResults,
    onError,
    initialMaxElements,
    initialTimeoutSeconds,
    initialSlowMode,
    initialDelayPerElement,
    onSettingsChange,
  }: Props = $props();

  // Используем composable для настроек
  const settings = useParserSettings({
    initialMaxElements,
    initialTimeoutSeconds,
    initialSlowMode,
    initialDelayPerElement,
    onSettingsChange,
  });

  // Используем composable для запуска парсера
  const runner = useParserRunner({
    nodes,
    edges,
    currentUrl,
    siteId,
    getSettings: () => ({
      maxElements: settings.maxElements,
      timeoutSeconds: settings.timeoutSeconds,
      slowMode: settings.slowMode,
      delayPerElement: settings.delayPerElement,
    }),
    onResults,
    onError,
  });

  let showSettings = $state(false);

  function handleToggleExpand(index: number) {
    runner.results = runner.results.map((r, i) =>
      i === index ? { ...r, expanded: !r.expanded } : r
    );
  }

  function handleUpdateResult(index: number, updater: (result: unknown) => unknown) {
    runner.results = runner.results.map((r, i) => (i === index ? updater(r) : r));
  }
</script>

<div class="parser-runner">
  <ParserRunnerControls
    isRunning={runner.isRunning}
    canRun={nodes.length > 0 && !!currentUrl}
    hasResults={runner.results.length > 0}
    hasError={!!runner.error}
    hasDiagnostics={runner.diagnostics.length > 0}
    {showSettings}
    onRun={runner.runParser}
    onStop={runner.stopParser}
    onClear={runner.clearResults}
    onToggleSettings={() => (showSettings = !showSettings)}
    runDisabledReason={nodes.length === 0
      ? 'Создайте ноды парсера'
      : !currentUrl
        ? 'Загрузите страницу'
        : ''}
  />

  {#if showSettings}
    <ParserSettingsPanel
      settings={{
        maxElements: settings.maxElements,
        timeoutSeconds: settings.timeoutSeconds,
        slowMode: settings.slowMode,
        delayPerElement: settings.delayPerElement,
      }}
      isRunning={runner.isRunning}
      onSettingsChange={newSettings => {
        settings.maxElements = newSettings.maxElements;
        settings.timeoutSeconds = newSettings.timeoutSeconds;
        settings.slowMode = newSettings.slowMode;
        settings.delayPerElement = newSettings.delayPerElement;
      }}
      onReset={settings.resetSettings}
    />
  {/if}

  <ParserProgress progress={runner.progress} isRunning={runner.isRunning} />

  <ParserError error={runner.error} isRunning={runner.isRunning} />

  <ParserDiagnostics diagnostics={runner.diagnostics} />

  {#if runner.results.length > 0 && !runner.isRunning}
    <ParserResults
      results={runner.results}
      extractionStats={runner.extractionStats}
      onToggleExpand={handleToggleExpand}
      onUpdateResult={handleUpdateResult}
    />
  {/if}

  {#if !runner.isRunning && !runner.error && runner.results.length === 0 && runner.diagnostics.length === 0}
    <div class="empty-state">
      Нажмите "Запустить парсер" для тестирования парсера на текущей странице
    </div>
  {/if}
</div>

<style>
  .parser-runner {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
  }

  .empty-state {
    padding: 2rem;
    text-align: center;
    color: #64748b;
    font-size: 0.875rem;
  }
</style>
