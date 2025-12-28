<script lang="ts">
  import type { ParserResult } from '@/lib/composables';

  interface Props {
    results: ParserResult[];
    extractionStats: unknown | null;
    onToggleExpand: (index: number) => void;
    onUpdateResult?: (index: number, updater: (result: ParserResult) => ParserResult) => void;
  }

  const { results, extractionStats, onToggleExpand, onUpdateResult }: Props = $props();

  function toggleFieldExpanded(index: number, key: string) {
    if (onUpdateResult) {
      onUpdateResult(index, result => {
        const currentExpanded = result.data[`_${key}_expanded`] || false;
        result.data[`_${key}_expanded`] = !currentExpanded;
        return result;
      });
    }
  }

  function formatJson(index: number, key: string, stringValue: string) {
    try {
      const parsed = JSON.parse(stringValue);
      const formatted = JSON.stringify(parsed, null, 2);
      if (onUpdateResult) {
        onUpdateResult(index, result => {
          result.data[key] = formatted;
          result.data[`_${key}_expanded`] = true;
          return result;
        });
      }
    } catch (e) {
      console.error('Failed to format JSON:', e);
    }
  }
</script>

{#if results.length > 0}
  <div class="results-container">
    <div class="results-header">
      <strong>📊 Результаты парсинга ({results.length} элементов):</strong>
    </div>
    <div class="results-list">
      {#each results as result, index (index)}
        <div class="result-item">
          <div class="result-header">
            <span class="result-number">#{index + 1}</span>
            <button class="btn-expand" onclick={() => onToggleExpand(index)}>
              {result.expanded ? '▼' : '▶'}
            </button>
          </div>
          {#if result.expanded}
            <div class="result-content">
              {#each Object.keys(result.data || {}).filter(k => !k.startsWith('_') || !k.endsWith('_expanded')) as key (key)}
                {@const value = result.data[key]}
                {@const stringValue = String(value)}
                {@const isLong = stringValue.length > 200}
                {@const displayValue = isLong ? stringValue.substring(0, 200) + '...' : stringValue}
                {@const isJson =
                  stringValue.trim().startsWith('{') || stringValue.trim().startsWith('[')}
                <div class="result-field">
                  <div class="result-field-header">
                    <strong>{key}:</strong>
                    {#if isLong}
                      <button
                        class="btn-toggle-expand-value"
                        onclick={() => toggleFieldExpanded(index, key)}
                        title={result.data[`_${key}_expanded`] ? 'Свернуть' : 'Развернуть'}
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
                      onclick={() => formatJson(index, key, stringValue)}
                      title="Форматировать JSON"
                    >
                      📝 Форматировать JSON
                    </button>
                  {/if}
                </div>
              {/each}
              {#if extractionStats && extractionStats[`item_${index}`]}
                {@const stats = extractionStats[`item_${index}`]}
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
                        <span>
                          <strong>{diag.field}:</strong>
                          {diag.message || diag.value || diag.value_preview || 'OK'}
                        </span>
                      </div>
                    {/each}
                  </div>
                {/if}
              {/if}
            </div>
          {:else}
            {@const keys = Object.keys(result.data || {})}
            <div class="result-preview">
              {keys
                .slice(0, 3)
                .map(key => `${key}: ${result.data[key]}`)
                .join(', ')}
              {keys.length > 3 ? '...' : ''}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .results-container {
    padding: 1rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
  }

  .results-header {
    color: #0ea5e9;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .results-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-height: 600px;
    overflow-y: auto;
  }

  .result-item {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    padding: 0.75rem 1rem;
    transition: border-color 0.2s ease;
  }

  .result-item:hover {
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
    font-size: 0.875rem;
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
</style>
