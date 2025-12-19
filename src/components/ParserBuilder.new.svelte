<!--
  Обновленный ParserBuilder с использованием фреймворка конструктора
  Максимальное переиспользование логики, код как коллекция/фреймворк
-->
<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '../lib/tauri-wrapper';
  import { SvelteFlow, Background, Controls, MiniMap } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import type { Node, Edge, Connection, NodeTypes } from '@xyflow/svelte';
  
  // Фреймворк конструктора
  import { initParserFramework, createNode, getRegisteredNodeTypes } from '../lib/framework';
  import UniversalNode from './nodes/UniversalNode.svelte';
  
  // Остальные компоненты
  import PageViewer from './PageViewer.svelte';
  import ContextMenu from './ContextMenu.svelte';
  import ElementSelector from './ElementSelector.svelte';
  
  import PlusIcon from './icons/PlusIcon.svelte';
  import PlayIcon from './icons/PlayIcon.svelte';
  import TrashIcon from './icons/TrashIcon.svelte';
  import RefreshIcon from './icons/RefreshIcon.svelte';

  // Инициализация фреймворка
  initParserFramework();

  // Один компонент для всех типов нод
  const nodeTypes: NodeTypes = {
    universal: UniversalNode,
  };

  // Состояние
  let nodes = $state<Node[]>([]);
  let edges = $state<Edge[]>([]);
  let selectedSite = $state<any | null>(null);
  let sites = $state<any[]>([]);
  let currentUrl = $state('');
  let showPageViewer = $state(false);
  let contextMenu = $state<{ x: number; y: number } | null>(null);
  let generatedCode = $state('');
  let selectedElementInfo = $state<{
    selector: string;
    elementInfo: {
      tagName: string;
      text: string;
      attributes: Record<string, string>;
      similarElements?: number;
    };
  } | null>(null);
  let error = $state<string | null>(null);

  // Доступные типы нод из фреймворка
  const availableNodeTypes = $derived(getRegisteredNodeTypes());

  onMount(async () => {
    await loadSites();
  });

  /**
   * Загружает список всех сайтов из базы данных
   */
  async function loadSites() {
    try {
      sites = await invoke('get_sites');
    } catch (error) {
      console.error('Failed to load sites:', error);
    }
  }

  /**
   * Упрощенная функция добавления ноды через фреймворк
   */
  function handleAddNode(type: string) {
    const position = { 
      x: Math.random() * 400 + 100, 
      y: Math.random() * 400 + 100 
    };
    
    const newNode = createNode(type, { position });
    
    if (newNode) {
      nodes = [...nodes, newNode];
      contextMenu = null;
    }
  }

  /**
   * Обрабатывает клик по области графа
   */
  function handlePaneClick(event: MouseEvent) {
    if (event.button === 2) {
      contextMenu = {
        x: event.clientX,
        y: event.clientY,
      };
    } else {
      contextMenu = null;
    }
  }

  /**
   * Обрабатывает контекстное меню
   */
  function handleContextMenu(event: MouseEvent) {
    event.preventDefault();
    contextMenu = {
      x: event.clientX,
      y: event.clientY,
    };
  }

  /**
   * Обрабатывает соединение нод
   */
  function handleConnect(connection: Connection) {
    if (connection.source && connection.target) {
      edges = [...edges, {
        id: `edge-${connection.source}-${connection.target}`,
        source: connection.source,
        target: connection.target,
      }];
    }
  }

  /**
   * Удаляет выбранные ноды
   */
  function handleDeleteSelected() {
    const selectedNodeIds = nodes
      .filter(n => n.selected)
      .map(n => n.id);
    
    nodes = nodes.filter(n => !n.selected);
    edges = edges.filter(e => 
      !selectedNodeIds.includes(e.source) && 
      !selectedNodeIds.includes(e.target)
    );
  }

  /**
   * Обрабатывает загрузку сайта
   */
  async function handleLoadSite() {
    try {
      error = null;
      
      if (!selectedSite) {
        currentUrl = '';
        showPageViewer = false;
        nodes = [];
        edges = [];
        return;
      }
      
      if (!selectedSite.url) {
        error = 'Выбранный сайт не имеет URL';
        return;
      }
      
      try {
        new URL(selectedSite.url);
      } catch (e) {
        error = 'Неверный формат URL';
        return;
      }
      
      currentUrl = selectedSite.url;
      showPageViewer = true;
      
      // Загружаем конфигурацию парсера из сайта
      const config = selectedSite.parser_config || {};
      nodes = [];
      edges = [];
      
      // TODO: Восстановление нод из конфигурации
      // Это можно сделать через createNodeFromConfig
      
    } catch (err) {
      console.error('Error loading site:', err);
      error = 'Ошибка загрузки сайта';
    }
  }

  /**
   * Генерирует код парсера из нод
   */
  function generateParserCode() {
    // TODO: Реализовать генерацию кода через фреймворк
    let code = "// Generated parser code\n\n";
    code += "use scraper::{Html, Selector};\n\n";
    code += "pub fn parse_page(html: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {\n";
    code += "    let document = Html::parse_document(html);\n";
    code += "    let mut results = Vec::new();\n\n";
    
    // Находим корневую ноду selector
    const rootNode = nodes.find(n => n.data.nodeType === 'selector' && !edges.some(e => e.target === n.id));
    
    if (rootNode) {
      const selector = rootNode.data.selector || '';
      code += `    let selector = Selector::parse("${selector}")?;\n`;
      code += "    for element in document.select(&selector) {\n";
      code += "        let mut item = serde_json::json!({});\n";
      
      // Обрабатываем связанные ноды extract
      const extractEdges = edges.filter(e => e.source === rootNode.id);
      for (const edge of extractEdges) {
        const extractNode = nodes.find(n => n.id === edge.target);
        if (extractNode && extractNode.data.nodeType === 'extract') {
          const attribute = extractNode.data.attribute || 'text';
          if (attribute === 'text') {
            code += `        item["${attribute}"] = serde_json::json!(element.text().collect::<String>().trim());\n`;
          } else if (attribute === 'href') {
            code += `        if let Some(href) = element.value().attr("href") {\n`;
            code += `            item["url"] = serde_json::json!(href);\n`;
            code += `        }\n`;
          }
        }
      }
      
      code += "        results.push(item);\n";
      code += "    }\n\n";
    }
    
    code += "    Ok(results)\n";
    code += "}\n";
    
    generatedCode = code;
  }

  /**
   * Сохраняет конфигурацию парсера
   */
  async function handleSaveParser() {
    if (!selectedSite) {
      error = 'Выберите сайт для сохранения';
      return;
    }
    
    try {
      const parserConfig = {
        nodes: nodes.map(n => ({
          id: n.id,
          type: n.data.nodeType,
          position: n.position,
          data: n.data,
        })),
        connections: edges.map(e => ({
          from: e.source,
          to: e.target,
        })),
      };
      
      await invoke('update_site', {
        id: selectedSite.id,
        name: selectedSite.name,
        url: selectedSite.url,
        parser_config: parserConfig,
      });
      
      // Обновляем список сайтов
      await loadSites();
      error = null;
    } catch (err) {
      console.error('Error saving parser:', err);
      error = 'Ошибка сохранения парсера';
    }
  }
</script>

<div class="parser-builder">
  <div class="toolbar">
    <div class="toolbar-section">
      <select bind:value={selectedSite} onchange={handleLoadSite}>
        <option value={null}>Выберите сайт</option>
        {#each sites as site}
          <option value={site}>{site.name}</option>
        {/each}
      </select>
      
      <button on:click={handleLoadSite} title="Загрузить сайт">
        <RefreshIcon />
      </button>
    </div>
    
    <div class="toolbar-section">
      <button on:click={generateParserCode} title="Сгенерировать код">
        <PlayIcon />
      </button>
      <button on:click={handleDeleteSelected} title="Удалить выбранные">
        <TrashIcon />
      </button>
      <button on:click={handleSaveParser} title="Сохранить парсер">
        Сохранить
      </button>
    </div>
  </div>

  {#if error}
    <div class="error-banner">{error}</div>
  {/if}

  <div class="builder-content">
    <SvelteFlow
      {nodes}
      {edges}
      {nodeTypes}
      on:connect={handleConnect}
      on:paneclick={handlePaneClick}
      on:panecontextmenu={handleContextMenu}
    >
      <Background />
      <Controls />
      <MiniMap />
    </SvelteFlow>

    {#if contextMenu}
      <ContextMenu
        x={contextMenu.x}
        y={contextMenu.y}
        nodeTypes={availableNodeTypes}
        on:add-node={(e) => handleAddNode(e.detail)}
        on:close={() => contextMenu = null}
      />
    {/if}

    {#if showPageViewer}
      <div class="page-viewer-container">
        <PageViewer url={currentUrl} />
      </div>
    {/if}
  </div>

  {#if generatedCode}
    <div class="code-panel">
      <h3>Сгенерированный код</h3>
      <pre><code>{generatedCode}</code></pre>
    </div>
  {/if}
</div>

<style>
  .parser-builder {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: #0f172a;
  }

  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background: #1e293b;
    border-bottom: 1px solid #334155;
  }

  .toolbar-section {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .toolbar select {
    padding: 0.5rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    color: #e2e8f0;
  }

  .toolbar button {
    padding: 0.5rem 1rem;
    background: #0ea5e9;
    border: none;
    border-radius: 0.375rem;
    color: white;
    cursor: pointer;
  }

  .error-banner {
    padding: 0.75rem;
    background: rgba(239, 68, 68, 0.1);
    color: #ef4444;
    border-bottom: 1px solid #ef4444;
  }

  .builder-content {
    flex: 1;
    position: relative;
  }

  .page-viewer-container {
    position: absolute;
    top: 1rem;
    right: 1rem;
    width: 400px;
    max-height: 600px;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    z-index: 10;
  }

  .code-panel {
    padding: 1rem;
    background: #1e293b;
    border-top: 1px solid #334155;
    max-height: 300px;
    overflow-y: auto;
  }

  .code-panel pre {
    margin: 0;
    padding: 1rem;
    background: #0f172a;
    border-radius: 0.375rem;
    overflow-x: auto;
  }

  .code-panel code {
    color: #e2e8f0;
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
  }
</style>

