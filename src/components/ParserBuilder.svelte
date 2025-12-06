<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  // Note: @xyflow/svelte may need to be replaced with a compatible library
  // For now, using a simplified version
  import PlusIcon from './icons/PlusIcon.svelte';
  import PlayIcon from './icons/PlayIcon.svelte';
  import ParserNode from './ParserNode.svelte';
  
  let nodes = $state<any[]>([]);
  let edges = $state<any[]>([]);
  let selectedSite = $state<any | null>(null);
  let sites = $state<any[]>([]);
  let htmlContent = $state('');
  let testUrl = $state('');
  let testSelector = $state('');
  let testResults = $state<any>(null);
  let loading = $state(false);
  
  onMount(async () => {
    await loadSites();
  });
  
  async function loadSites() {
    try {
      sites = await invoke('get_sites');
    } catch (error) {
      console.error('Failed to load sites:', error);
    }
  }
  
  function handleAddNode(type: string) {
    const newNode = {
      id: `node-${Date.now()}`,
      type: 'custom',
      position: { x: Math.random() * 400, y: Math.random() * 400 },
      data: {
        label: type,
        nodeType: type,
        config: {},
      },
    };
    nodes = [...nodes, newNode];
  }
  
  function handleNodeChange(changedNodes: any[]) {
    nodes = changedNodes;
  }
  
  function handleEdgeChange(changedEdges: any[]) {
    edges = changedEdges;
  }
  
  async function handleTestSelector() {
    if (!testUrl || !testSelector) {
      alert('Введите URL и селектор');
      return;
    }
    
    loading = true;
    try {
      // Fetch HTML
      const response = await fetch(testUrl);
      const html = await response.text();
      htmlContent = html;
      
      // Test selector
      testResults = await invoke('build_parser', {
        html,
        selector: testSelector,
      });
    } catch (error: any) {
      alert('Ошибка: ' + error.toString());
    } finally {
      loading = false;
    }
  }
  
  async function handleLoadSite() {
    if (!selectedSite) return;
    
    const config = selectedSite.parser_config;
    nodes = [];
    edges = [];
    
    // Create nodes from config
    if (config.list_selector) {
      nodes.push({
        id: 'list-selector',
        type: 'custom',
        position: { x: 100, y: 100 },
        data: {
          label: 'List Selector',
          nodeType: 'selector',
          config: { selector: config.list_selector },
        },
      });
    }
    
    if (config.title_selector) {
      nodes.push({
        id: 'title-selector',
        type: 'custom',
        position: { x: 100, y: 200 },
        data: {
          label: 'Title Selector',
          nodeType: 'selector',
          config: { selector: config.title_selector },
        },
      });
    }
  }
  
</script>

<div class="parser-builder">
  <div class="builder-header">
    <h2>Конструктор парсеров</h2>
    <div class="header-actions">
      <select bind:value={selectedSite} onchange={handleLoadSite} class="site-select">
        <option value={null}>Выберите сайт для редактирования</option>
        {#each sites as site}
          <option value={site}>{site.name}</option>
        {/each}
      </select>
      <button class="btn-primary" onclick={() => handleAddNode('selector')}>
        <PlusIcon class="icon-small" />
        Добавить ноду
      </button>
    </div>
  </div>
  
  <div class="builder-content">
    <div class="flow-container">
      <div class="flow-canvas">
        {#each nodes as node}
          <div 
            class="flow-node" 
            style="left: {node.position.x}px; top: {node.position.y}px;"
          >
            <ParserNode data={node.data} />
          </div>
        {/each}
        {#if nodes.length === 0}
          <div class="empty-flow">
            <p>Добавьте ноды для создания парсера</p>
            <p class="hint">Используйте кнопку "Добавить ноду" или выберите сайт для загрузки конфигурации</p>
          </div>
        {/if}
      </div>
    </div>
    
    <div class="test-panel">
      <h3>Тестирование селекторов</h3>
      
      <div class="form-group">
        <label>URL для теста</label>
        <input type="url" bind:value={testUrl} placeholder="https://example.com" />
      </div>
      
      <div class="form-group">
        <label>CSS Селектор</label>
        <input type="text" bind:value={testSelector} placeholder=".mod-item" />
      </div>
      
      <button class="btn-primary" onclick={handleTestSelector} disabled={loading}>
        <PlayIcon class="icon-small" />
        {loading ? 'Тестирование...' : 'Тестировать'}
      </button>
      
      {#if testResults}
        <div class="test-results">
          <h4>Результаты ({testResults.matches} совпадений)</h4>
          <div class="results-list">
            {#each testResults.results.slice(0, 10) as result}
              <div class="result-item">
                <strong>Текст:</strong> {result.text.substring(0, 100)}...
                <br />
                <strong>Атрибуты:</strong> {JSON.stringify(result.attributes)}
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .parser-builder {
    height: 100%;
    display: flex;
    flex-direction: column;
  }
  
  .builder-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #334155;
  }
  
  .builder-header h2 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 700;
    color: #e2e8f0;
  }
  
  .header-actions {
    display: flex;
    gap: 1rem;
    align-items: center;
  }
  
  .site-select {
    padding: 0.5rem 1rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    color: #e2e8f0;
    font-size: 0.9rem;
  }
  
  .btn-primary {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    background: #0ea5e9;
    color: white;
    border: none;
    border-radius: 0.5rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }
  
  .btn-primary:hover:not(:disabled) {
    background: #0284c7;
  }
  
  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .builder-content {
    flex: 1;
    display: flex;
    overflow: hidden;
  }
  
  .flow-container {
    flex: 1;
    background: #0f172a;
    position: relative;
    overflow: auto;
  }
  
  .flow-canvas {
    position: relative;
    width: 100%;
    height: 100%;
    min-height: 500px;
  }
  
  .flow-node {
    position: absolute;
    cursor: move;
  }
  
  .empty-flow {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #64748b;
    text-align: center;
  }
  
  .empty-flow .hint {
    font-size: 0.875rem;
    margin-top: 0.5rem;
    color: #475569;
  }
  
  .test-panel {
    width: 400px;
    padding: 1.5rem;
    background: #1e293b;
    border-left: 1px solid #334155;
    overflow-y: auto;
  }
  
  .test-panel h3 {
    margin: 0 0 1.5rem 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #e2e8f0;
  }
  
  .form-group {
    margin-bottom: 1.25rem;
  }
  
  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 600;
    color: #cbd5e1;
    font-size: 0.875rem;
  }
  
  .form-group input {
    width: 100%;
    padding: 0.75rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    color: #e2e8f0;
    font-size: 0.95rem;
  }
  
  .test-results {
    margin-top: 2rem;
    padding-top: 1.5rem;
    border-top: 1px solid #334155;
  }
  
  .test-results h4 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: #0ea5e9;
  }
  
  .results-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-height: 400px;
    overflow-y: auto;
  }
  
  .result-item {
    padding: 0.75rem;
    background: #0f172a;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    color: #cbd5e1;
    line-height: 1.6;
  }
  
  .result-item strong {
    color: #0ea5e9;
  }
  
  .icon-small {
    width: 18px;
    height: 18px;
  }
</style>

