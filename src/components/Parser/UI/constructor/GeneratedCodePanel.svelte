<script lang="ts">
  import { ResizeHandle } from '@/components';
  import ParserRunner from './ParserRunner.svelte';
  import type { Node, Edge } from '@xyflow/svelte';

  interface Props {
    visible: boolean;
    height: number;
    generatedCode: string;
    editedCode?: string;
    showCodeEditor?: boolean;
    showParserResults?: boolean;
    parserResults?: Array<{ data: Record<string, unknown>; expanded: boolean }>;
    parserTestError?: string | null;
    parserDiagnostics?: unknown[];
    parserExtractionStats?: unknown;
    aiCodeReview?: string | null;
    activeBottomTab?: 'code' | 'results' | 'review' | 'runner' | null;
    isCheckingCodeWithAI?: boolean;
    aiApiKey?: string;
    aiModelType?: 'ollama' | 'openai' | 'anthropic' | 'google';
    showAIChat?: boolean;
    nodes?: Node[];
    edges?: Edge[];
    currentUrl?: string;
    siteId?: number | null;
    onHeightChange?: (newHeight: number) => void;
    onHeightChangeStart?: () => void;
    onHeightChangeEnd?: () => void;
    onClose?: () => void;
    onTabChange?: (tab: 'code' | 'results' | 'review' | 'runner' | null) => void;
    onCodeApply?: (code: string) => void;
    onCodeCancel?: () => void;
    onToggleEditor?: () => void;
    onCopyCode?: () => void;
    onCheckCodeWithAI?: () => void;
    onToggleResultExpansion?: (index: number) => void;
    onClearReview?: () => void;
    onRunnerResults?: (results: unknown[], diagnostics: unknown[], stats: unknown) => void;
    onRunnerError?: (error: string) => void;
    // Настройки парсера для синхронизации
    parserMaxElements?: number;
    parserTimeoutSeconds?: number;
    parserSlowMode?: boolean;
    parserDelayPerElement?: number;
    onParserSettingsChange?: (settings: {
      maxElements: number;
      timeoutSeconds: number;
      slowMode: boolean;
      delayPerElement: number;
    }) => void;
  }

  let {
    visible = $bindable(),
    height = $bindable(),
    generatedCode = '',
    editedCode = $bindable(''),
    showCodeEditor = $bindable(false),
    showParserResults = false,
    parserResults = [],
    parserTestError = null,
    parserDiagnostics = [],
    parserExtractionStats = null,
    aiCodeReview = null,
    activeBottomTab = $bindable(null),
    isCheckingCodeWithAI = false,
    aiApiKey = '',
    aiModelType = 'ollama',
    showAIChat = false,
    nodes = [],
    edges = [],
    currentUrl = '',
    siteId = null,
    onHeightChange,
    onHeightChangeStart,
    onHeightChangeEnd,
    onClose,
    onTabChange,
    onCodeApply,
    onCodeCancel,
    onToggleEditor,
    onCopyCode,
    onCheckCodeWithAI,
    onToggleResultExpansion,
    onClearReview,
    onRunnerResults,
    onRunnerError,
    parserMaxElements,
    parserTimeoutSeconds,
    parserSlowMode,
    parserDelayPerElement,
    onParserSettingsChange,
  }: Props = $props();

  function handleHeightChange(newHeight: number) {
    height = newHeight;
    if (onHeightChange) {
      onHeightChange(newHeight);
    }
  }

  function handleClose() {
    if (onClose) {
      onClose();
    } else {
      activeBottomTab = null;
      generatedCode = '';
      showParserResults = false;
      aiCodeReview = null;
    }
  }

  function handleTabChange(tab: 'code' | 'results' | 'review' | 'runner') {
    if (onTabChange) {
      onTabChange(tab);
    } else {
      activeBottomTab = tab;
    }
  }

  function handleCodeApply() {
    if (onCodeApply && editedCode) {
      onCodeApply(editedCode);
    } else if (editedCode) {
      generatedCode = editedCode;
      showCodeEditor = false;
    }
  }

  function handleCodeCancel() {
    if (onCodeCancel) {
      onCodeCancel();
    } else {
      editedCode = generatedCode;
      showCodeEditor = false;
    }
  }
</script>

{#if visible}
  <div class="generated-code-panel" style="height: {height}px">
    <div class="bottom-panel-tabs">
      {#if generatedCode}
        <button
          class="bottom-tab"
          class:active={activeBottomTab === 'code' || (activeBottomTab === null && generatedCode)}
          onclick={() => handleTabChange('code')}
        >
          📄 Код
        </button>
      {/if}
      {#if showParserResults && parserResults.length > 0}
        <button
          class="bottom-tab"
          class:active={activeBottomTab === 'results'}
          onclick={() => handleTabChange('results')}
        >
          📊 Результаты ({parserResults.length})
        </button>
      {/if}
      {#if aiCodeReview && !showAIChat}
        <button
          class="bottom-tab"
          class:active={activeBottomTab === 'review'}
          onclick={() => handleTabChange('review')}
        >
          🤖 AI Отзыв
        </button>
      {/if}
      <button
        class="bottom-tab"
        class:active={activeBottomTab === 'runner'}
        onclick={() => handleTabChange('runner')}
        title="Запустить парсер и просмотреть результаты"
      >
        ▶️ Запуск парсера
      </button>
      <div class="bottom-tabs-spacer"></div>
      <button class="bottom-tab-close" onclick={handleClose} title="Закрыть панель"> × </button>
    </div>

    <div class="bottom-panel-content">
      {#if (activeBottomTab === 'code' || (activeBottomTab === null && generatedCode)) && generatedCode}
        <div class="bottom-tab-content">
          <div class="code-toolbar">
            <button
              class="btn-toolbar"
              onclick={onCheckCodeWithAI}
              disabled={isCheckingCodeWithAI || (!aiApiKey && aiModelType !== 'ollama')}
              title="Проверить код с помощью AI"
            >
              {#if isCheckingCodeWithAI}
                ⏳ Проверка...
              {:else}
                🤖 Проверить AI
              {/if}
            </button>
            <button class="btn-toolbar" onclick={onToggleEditor}>
              {showCodeEditor ? '👁️ Просмотр' : '✏️ Редактировать'}
            </button>
            <button class="btn-toolbar" onclick={onCopyCode} title="Копировать код">
              📋 Копировать
            </button>
          </div>
          {#if showCodeEditor}
            <textarea
              class="code-editor-full"
              bind:value={editedCode}
              placeholder="Введите или отредактируйте код парсера..."
              spellcheck="false"
            ></textarea>
            <div class="code-editor-actions">
              <button class="btn-primary btn-small" onclick={handleCodeApply}>
                ✅ Применить
              </button>
              <button class="btn-secondary btn-small" onclick={handleCodeCancel}>
                ❌ Отмена
              </button>
            </div>
          {:else}
            <div class="code-content-wrapper">
              <pre class="code-content-full"><code>{generatedCode}</code></pre>
            </div>
          {/if}
        </div>
      {/if}

      {#if activeBottomTab === 'results' && showParserResults}
        <div class="bottom-tab-content">
          <div class="parser-results-content-full">
            {#if parserTestError}
              <div class="parser-error">
                ❌ Ошибка: {parserTestError}
              </div>
            {:else if parserResults.length === 0}
              <div class="parser-empty">Нет результатов. Проверьте конфигурацию парсера.</div>
            {:else}
              <div class="parser-stats">
                Найдено элементов: <strong>{parserResults.length}</strong>
              </div>

              {#if parserDiagnostics.length > 0}
                <div class="parser-diagnostics">
                  <div class="diagnostics-header">
                    <strong>🔍 Диагностика:</strong>
                  </div>
                  <div class="diagnostics-list">
                    {#each parserDiagnostics as diag, index (index)}
                      <div
                        class="diagnostic-item"
                        class:diagnostic-info={diag.type === 'info'}
                        class:diagnostic-success={diag.type === 'success'}
                        class:diagnostic-warning={diag.type === 'warning'}
                        class:diagnostic-error={diag.type === 'error'}
                      >
                        <span class="diagnostic-icon">
                          {#if diag.type === 'success'}✅
                          {:else if diag.type === 'warning'}⚠️
                          {:else if diag.type === 'error'}❌
                          {:else}ℹ️
                          {/if}
                        </span>
                        <span class="diagnostic-message">{diag.message}</span>
                      </div>
                    {/each}
                  </div>
                </div>
              {/if}

              <div class="parser-results-list-full">
                {#each parserResults as result, index (index)}
                  <div class="parser-result-item">
                    <div class="result-header">
                      <span class="result-number">#{index + 1}</span>
                      <button class="btn-expand" onclick={() => onToggleResultExpansion?.(index)}>
                        {result.expanded ? '▼' : '▶'}
                      </button>
                    </div>
                    {#if result.expanded}
                      <div class="result-content">
                        {#each Object.keys(result.data || {}) as key (key)}
                          {@const value = result.data[key]}
                          {@const stringValue = String(value)}
                          {@const isLong = stringValue.length > 200}
                          {@const displayValue = isLong
                            ? stringValue.substring(0, 200) + '...'
                            : stringValue}
                          {@const isJson =
                            stringValue.trim().startsWith('{') ||
                            stringValue.trim().startsWith('[')}
                          {@const isInternalKey = key.startsWith('_') && key.endsWith('_expanded')}
                          {#if !isInternalKey}
                            <div class="result-field">
                              <div class="result-field-header">
                                <strong>{key}:</strong>
                                {#if isLong}
                                  <button
                                    class="btn-toggle-expand-value"
                                    onclick={() => {
                                      const currentExpanded =
                                        result.data[`_${key}_expanded`] || false;
                                      result.data[`_${key}_expanded`] = !currentExpanded;
                                      parserResults = [...parserResults];
                                    }}
                                    title={result.data[`_${key}_expanded`]
                                      ? 'Свернуть'
                                      : 'Развернуть'}
                                  >
                                    {result.data[`_${key}_expanded`] ? '▲' : '▼'}
                                  </button>
                                {/if}
                              </div>
                              <span
                                class="result-value"
                                class:result-value-long={isLong && !result.data[`_${key}_expanded`]}
                                class:result-value-json={isJson}
                              >
                                {result.data[`_${key}_expanded`] ? stringValue : displayValue}
                              </span>
                              {#if isJson && !result.data[`_${key}_expanded`]}
                                <button
                                  class="btn-format-json"
                                  onclick={() => {
                                    try {
                                      const parsed = JSON.parse(stringValue);
                                      result.data[key] = JSON.stringify(parsed, null, 2);
                                      result.data[`_${key}_expanded`] = true;
                                      parserResults = [...parserResults];
                                    } catch (e) {
                                      console.error('Failed to format JSON:', e);
                                    }
                                  }}
                                  title="Форматировать JSON"
                                >
                                  📝 Форматировать JSON
                                </button>
                              {/if}
                            </div>
                          {/if}
                        {/each}
                        {#if parserExtractionStats && parserExtractionStats[`item_${index}`]}
                          {@const stats = parserExtractionStats[`item_${index}`]}
                          {#if stats.diagnostics && stats.diagnostics.length > 0}
                            <div class="result-diagnostics">
                              <strong>Детали извлечения:</strong>
                              {#each stats.diagnostics as diag, diagIndex (diagIndex)}
                                <div
                                  class="result-diagnostic"
                                  class:diagnostic-success={diag.status === 'success'}
                                  class:diagnostic-warning={diag.status === 'warning'}
                                  class:diagnostic-error={diag.status === 'error'}
                                >
                                  <span class="diagnostic-icon">
                                    {#if diag.status === 'success'}✅
                                    {:else if diag.status === 'warning'}⚠️
                                    {:else if diag.status === 'error'}❌
                                    {:else}ℹ️
                                    {/if}
                                  </span>
                                  <span
                                    ><strong>{diag.field}:</strong>
                                    {diag.message || diag.value || diag.value_preview || 'OK'}</span
                                  >
                                </div>
                              {/each}
                            </div>
                          {/if}
                        {/if}
                      </div>
                    {:else}
                      {@const keys = Object.keys(result.data || {}).filter(
                        k => !k.startsWith('_') || !k.endsWith('_expanded')
                      )}
                      <div class="result-preview">
                        {keys
                          .slice(0, 3)
                          .map(key => {
                            const value = String(result.data[key]);
                            const preview =
                              value.length > 50 ? value.substring(0, 50) + '...' : value;
                            return `${key}: ${preview}`;
                          })
                          .join(', ')}
                        {keys.length > 3 ? '...' : ''}
                      </div>
                    {/if}
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      {/if}

      {#if activeBottomTab === 'review' && aiCodeReview}
        <div class="bottom-tab-content">
          <div class="review-content-full">
            <div class="review-header-full">
              <strong>🤖 Отзыв AI:</strong>
              <button class="btn-close-small" onclick={onClearReview}>×</button>
            </div>
            <div class="review-text">{aiCodeReview}</div>
          </div>
        </div>
      {/if}

      {#if activeBottomTab === 'runner'}
        <div class="bottom-tab-content">
          <ParserRunner
            {nodes}
            {edges}
            {currentUrl}
            {siteId}
            initialMaxElements={parserMaxElements}
            initialTimeoutSeconds={parserTimeoutSeconds}
            initialSlowMode={parserSlowMode}
            initialDelayPerElement={parserDelayPerElement}
            onSettingsChange={settings => {
              if (onParserSettingsChange) {
                onParserSettingsChange(settings);
              }
            }}
            onResults={(res, diag, stats) => {
              if (onRunnerResults) {
                onRunnerResults(res, diag, stats);
              }
            }}
            onError={err => {
              if (onRunnerError) {
                onRunnerError(err);
              }
            }}
          />
        </div>
      {/if}
    </div>

    <ResizeHandle
      direction="horizontal"
      mode="bottom"
      minValue={150}
      maxValue={window.innerHeight * 0.7}
      getCurrentValue={() => height}
      onResize={handleHeightChange}
      onResizeStart={onHeightChangeStart}
      onResizeEnd={onHeightChangeEnd}
      className="resize-handle"
    />
  </div>
{/if}

<style>
  .generated-code-panel {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    background: #1e293b;
    border-top: 1px solid #334155;
    display: flex;
    flex-direction: column;
    z-index: 10;
  }

  .bottom-panel-tabs {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid #334155;
    background: #0f172a;
    flex-shrink: 0;
  }

  .bottom-tab {
    padding: 0.5rem 1rem;
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    transition: all 0.2s ease;
  }

  .bottom-tab:hover {
    background: #334155;
    color: #e2e8f0;
  }

  .bottom-tab.active {
    background: #0ea5e9;
    color: white;
  }

  .bottom-tabs-spacer {
    flex: 1;
  }

  .bottom-tab-close {
    padding: 0.5rem;
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    border-radius: 0.375rem;
    font-size: 1.25rem;
    line-height: 1;
    transition: all 0.2s ease;
  }

  .bottom-tab-close:hover {
    background: #334155;
    color: #e2e8f0;
  }

  .bottom-panel-content {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
  }

  .bottom-tab-content {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .code-toolbar {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
    flex-shrink: 0;
  }

  .btn-toolbar {
    padding: 0.5rem 1rem;
    background: #334155;
    border: 1px solid #475569;
    color: #e2e8f0;
    cursor: pointer;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    transition: all 0.2s ease;
  }

  .btn-toolbar:hover:not(:disabled) {
    background: #475569;
    border-color: #64748b;
  }

  .btn-toolbar:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .code-editor-full {
    flex: 1;
    padding: 1rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    color: #e2e8f0;
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
    line-height: 1.5;
    resize: none;
    margin-bottom: 1rem;
  }

  .code-editor-actions {
    display: flex;
    gap: 0.5rem;
    flex-shrink: 0;
  }

  .btn-primary,
  .btn-secondary {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 0.375rem;
    cursor: pointer;
    font-size: 0.875rem;
    transition: all 0.2s ease;
  }

  .btn-primary {
    background: #0ea5e9;
    color: white;
  }

  .btn-primary:hover {
    background: #0284c7;
  }

  .btn-secondary {
    background: #334155;
    color: #e2e8f0;
  }

  .btn-secondary:hover {
    background: #475569;
  }

  .btn-small {
    padding: 0.375rem 0.75rem;
    font-size: 0.8125rem;
  }

  .code-content-wrapper {
    flex: 1;
    overflow: auto;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    padding: 1rem;
  }

  .code-content-full {
    margin: 0;
    color: #e2e8f0;
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
    line-height: 1.5;
    white-space: pre-wrap;
    word-wrap: break-word;
  }

  .parser-results-content-full {
    height: 100%;
    overflow-y: auto;
  }

  .parser-error {
    padding: 1rem;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 0.5rem;
    color: #ef4444;
    margin-bottom: 1rem;
  }

  .parser-empty {
    padding: 2rem;
    text-align: center;
    color: #64748b;
  }

  .parser-stats {
    padding: 0.75rem 1rem;
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    color: #0ea5e9;
  }

  .parser-stats strong {
    color: #e2e8f0;
  }

  .parser-diagnostics {
    margin-bottom: 1rem;
    padding: 0.75rem 1rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
  }

  .diagnostics-header {
    color: #0ea5e9;
    margin-bottom: 0.5rem;
  }

  .diagnostics-list {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .diagnostic-item {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    padding: 0.5rem;
    border-radius: 0.375rem;
    font-size: 0.875rem;
  }

  .diagnostic-item.diagnostic-info {
    background: rgba(14, 165, 233, 0.1);
    color: #0ea5e9;
  }

  .diagnostic-item.diagnostic-success {
    background: rgba(16, 185, 129, 0.1);
    color: #10b981;
  }

  .diagnostic-item.diagnostic-warning {
    background: rgba(245, 158, 11, 0.1);
    color: #f59e0b;
  }

  .diagnostic-item.diagnostic-error {
    background: rgba(239, 68, 68, 0.1);
    color: #ef4444;
  }

  .parser-results-list-full {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .parser-result-item {
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    padding: 0.75rem 1rem;
    transition: border-color 0.2s ease;
  }

  .parser-result-item:hover {
    border-color: #0ea5e9;
  }

  .result-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .result-number {
    font-weight: 600;
    color: #0ea5e9;
  }

  .btn-expand {
    background: transparent;
    border: none;
    color: #cbd5e1;
    cursor: pointer;
    font-size: 0.875rem;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    transition: background-color 0.2s ease;
  }

  .btn-expand:hover {
    background: #334155;
  }

  .result-preview {
    color: #94a3b8;
    font-size: 0.875rem;
    font-style: italic;
  }

  .result-content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-top: 0.5rem;
    padding-top: 0.5rem;
    border-top: 1px solid #334155;
  }

  .result-field {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .result-field-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .result-field strong {
    color: #0ea5e9;
    font-size: 0.875rem;
    flex-shrink: 0;
  }

  .btn-toggle-expand-value {
    background: transparent;
    border: 1px solid #334155;
    color: #94a3b8;
    cursor: pointer;
    font-size: 0.75rem;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    transition: all 0.2s ease;
    flex-shrink: 0;
  }

  .btn-toggle-expand-value:hover {
    background: #334155;
    color: #e2e8f0;
    border-color: #475569;
  }

  .result-value {
    color: #e2e8f0;
    font-size: 0.875rem;
    word-break: break-word;
    padding: 0.5rem;
    background: #0f172a;
    border: 1px solid #1e293b;
    border-radius: 0.375rem;
    white-space: pre-wrap;
    font-family: 'Courier New', monospace;
    line-height: 1.5;
    max-height: 300px;
    overflow-y: auto;
  }

  .result-value-long {
    max-height: 100px;
    overflow: hidden;
    position: relative;
  }

  .result-value-long::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 30px;
    background: linear-gradient(to bottom, transparent, #0f172a);
    pointer-events: none;
  }

  .result-value-json {
    font-family: 'Courier New', monospace;
  }

  .btn-format-json {
    background: #334155;
    border: 1px solid #475569;
    color: #e2e8f0;
    cursor: pointer;
    font-size: 0.75rem;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    transition: all 0.2s ease;
    align-self: flex-start;
  }

  .btn-format-json:hover {
    background: #475569;
    border-color: #64748b;
  }

  .result-diagnostics {
    margin-top: 0.75rem;
    padding-top: 0.75rem;
    border-top: 1px solid #334155;
  }

  .result-diagnostic {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    padding: 0.375rem 0.5rem;
    margin-bottom: 0.25rem;
    border-radius: 0.25rem;
    font-size: 0.875rem;
  }

  .result-diagnostic.diagnostic-success {
    background: rgba(16, 185, 129, 0.1);
    color: #10b981;
  }

  .result-diagnostic.diagnostic-warning {
    background: rgba(245, 158, 11, 0.1);
    color: #f59e0b;
  }

  .result-diagnostic.diagnostic-error {
    background: rgba(239, 68, 68, 0.1);
    color: #ef4444;
  }

  .review-content-full {
    height: 100%;
    overflow-y: auto;
  }

  .review-header-full {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    color: #0ea5e9;
  }

  .btn-close-small {
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    font-size: 1.25rem;
    line-height: 1;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    transition: all 0.2s ease;
  }

  .btn-close-small:hover {
    background: #334155;
    color: #e2e8f0;
  }

  .review-text {
    color: #e2e8f0;
    line-height: 1.6;
    white-space: pre-wrap;
  }

  .resize-handle {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
  }
</style>
