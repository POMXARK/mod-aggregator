<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '../lib/tauri-wrapper';
  import { SvelteFlow, Background, Controls, MiniMap } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import type { Node, Edge, Connection, NodeTypes } from '@xyflow/svelte';
  
  import SelectorNode from './nodes/SelectorNode.svelte';
  import ExtractNode from './nodes/ExtractNode.svelte';
  import FilterNode from './nodes/FilterNode.svelte';
  import TransformNode from './nodes/TransformNode.svelte';
  import OutputNode from './nodes/OutputNode.svelte';
  import PageViewer from './PageViewer.svelte';
  import ContextMenu from './ContextMenu.svelte';
  import ElementSelector from './ElementSelector.svelte';
  
  import PlusIcon from './icons/PlusIcon.svelte';
  import PlayIcon from './icons/PlayIcon.svelte';
  import TrashIcon from './icons/TrashIcon.svelte';

  const nodeTypes: NodeTypes = {
    selector: SelectorNode,
    extract: ExtractNode,
    filter: FilterNode,
    transform: TransformNode,
    output: OutputNode,
  };

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

  function handleNodeClick(event: any) {
    // Handle node click if needed
  }

  function handlePaneClick(event: MouseEvent) {
    if (event.button === 2) { // Right click
      contextMenu = {
        x: event.clientX,
        y: event.clientY,
      };
    } else {
      contextMenu = null;
    }
  }

  function handleContextMenu(event: MouseEvent) {
    event.preventDefault();
    contextMenu = {
      x: event.clientX,
      y: event.clientY,
    };
  }

  function handleAddNode(type: string) {
    const nodeId = `node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    let newNode: Node;

    switch (type) {
      case 'selector':
        newNode = {
          id: nodeId,
          type: 'selector',
          position: { x: Math.random() * 400 + 100, y: Math.random() * 400 + 100 },
          data: {
            label: 'Selector',
            selector: '',
          },
        };
        break;
      case 'extract':
        newNode = {
          id: nodeId,
          type: 'extract',
          position: { x: Math.random() * 400 + 100, y: Math.random() * 400 + 100 },
          data: {
            label: 'Extract',
            attribute: 'text',
            selector: '',
          },
        };
        break;
      case 'filter':
        newNode = {
          id: nodeId,
          type: 'filter',
          position: { x: Math.random() * 400 + 100, y: Math.random() * 400 + 100 },
          data: {
            label: 'Filter',
            condition: '',
            operator: 'contains',
          },
        };
        break;
      case 'transform':
        newNode = {
          id: nodeId,
          type: 'transform',
          position: { x: Math.random() * 400 + 100, y: Math.random() * 400 + 100 },
          data: {
            label: 'Transform',
            function: 'trim',
          },
        };
        break;
      case 'output':
        newNode = {
          id: nodeId,
          type: 'output',
          position: { x: Math.random() * 400 + 100, y: Math.random() * 400 + 100 },
          data: {
            label: 'Output',
            fields: ['title', 'url'],
          },
        };
        break;
      default:
        return;
    }

    nodes = [...nodes, newNode];
    contextMenu = null;
  }

  function getNodeLabel(type: string): string {
    const labels: Record<string, string> = {
      selector: 'Selector',
      extract: 'Extract',
      filter: 'Filter',
      transform: 'Transform',
      output: 'Output',
    };
    return labels[type] || type;
  }

  function handleConnect(connection: Connection) {
    const newEdge: Edge = {
      id: `edge-${Date.now()}`,
      source: connection.source!,
      target: connection.target!,
      type: 'smoothstep',
    };
    edges = [...edges, newEdge];
  }

  function handleNodesChange(changes: any[]) {
    // Update nodes based on changes
    for (const change of changes) {
      if (change.type === 'position' && change.dragging === false) {
        const node = nodes.find(n => n.id === change.id);
        if (node) {
          node.position = change.position;
        }
      }
    }
  }

  function handleEdgesChange(changes: any[]) {
    // Handle edge changes
  }

  function handleDeleteSelected() {
    const selectedNodeIds = nodes.filter(n => n.selected).map(n => n.id);
    nodes = nodes.filter(n => !n.selected);
    edges = edges.filter(e => !selectedNodeIds.includes(e.source) && !selectedNodeIds.includes(e.target));
  }

  function handleElementSelect(selector: string, element: HTMLElement, elementData?: any) {
    console.log('handleElementSelect called:', selector, element, elementData);
    
    // Show selection panel instead of creating node immediately
    selectedElementInfo = {
      selector,
      elementInfo: {
        tagName: elementData?.tagName || element.tagName || 'UNKNOWN',
        text: elementData?.text || element.textContent?.trim().substring(0, 100) || '',
        attributes: elementData?.attributes || {},
        similarElements: elementData?.similarElements,
      },
    };
  }

  function confirmElementSelection() {
    if (!selectedElementInfo) return;
    
    // Create a selector node from the selected element
    const newNode: Node = {
      id: `node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
      type: 'selector',
      position: { x: Math.random() * 300 + 100, y: Math.random() * 300 + 100 },
      data: {
        label: 'Selector',
        selector: selectedElementInfo.selector,
      },
    };
    
    nodes = [...nodes, newNode];
    selectedElementInfo = null;
    
    // Automatically generate code after creating node
    setTimeout(() => {
      generateParserCode();
    }, 100);
  }

  function cancelElementSelection() {
    selectedElementInfo = null;
  }

  async function autoDetectElements() {
    if (!selectedElementInfo) return;
    
    // Try to find similar elements and suggest extraction nodes
    const iframe = document.querySelector('iframe');
    if (iframe?.contentWindow) {
      iframe.contentWindow.postMessage({
        type: 'find-similar',
        selector: selectedElementInfo.selector,
      }, '*');
    }
    
    // Also try AI detection if available
    try {
      const suggestions = await detectWithAI(selectedElementInfo.selector, selectedElementInfo.elementInfo);
      if (suggestions.length > 0) {
        // Create nodes from AI suggestions
        suggestions.forEach((suggestion, index) => {
          const newNode: Node = {
            id: `node-ai-${Date.now()}-${index}`,
            type: suggestion.type,
            position: { x: 200 + index * 250, y: 200 },
            data: suggestion.data,
          };
          nodes = [...nodes, newNode];
        });
        selectedElementInfo = null;
      }
    } catch (error) {
      console.error('AI detection failed:', error);
      // Fallback to simple detection
      simpleAutoDetect();
    }
  }

  async function detectWithAI(selector: string, elementInfo: any): Promise<any[]> {
    try {
      const { detectElementsWithAI } = await import('../lib/ai-detector');
      
      // Get HTML from iframe if possible
      const iframe = document.querySelector('iframe');
      let html = '';
      if (iframe?.contentWindow?.document) {
        html = iframe.contentWindow.document.documentElement.outerHTML;
      }
      
      const suggestions = await detectElementsWithAI(html, {
        tagName: elementInfo.tagName,
        text: elementInfo.text,
        attributes: elementInfo.attributes,
        selector,
      });
      
      return suggestions.map(s => ({
        type: s.type,
        data: s.data,
      }));
    } catch (error) {
      console.error('AI detection error:', error);
      return [];
    }
  }

  function simpleAutoDetect() {
    if (!selectedElementInfo) return;
    
    // Simple heuristic-based detection
    const elementInfo = selectedElementInfo.elementInfo;
    const suggestions: any[] = [];
    
    // If element has href, suggest extract node for URL
    if (elementInfo.attributes.href) {
      suggestions.push({
        type: 'extract',
        data: {
          label: 'Extract URL',
          attribute: 'href',
          selector: selectedElementInfo.selector + ' a',
        },
      });
    }
    
    // If element has src, suggest extract node for image
    if (elementInfo.attributes.src) {
      suggestions.push({
        type: 'extract',
        data: {
          label: 'Extract Image',
          attribute: 'src',
          selector: selectedElementInfo.selector + ' img',
        },
      });
    }
    
    // Always suggest text extraction
    suggestions.push({
      type: 'extract',
      data: {
        label: 'Extract Text',
        attribute: 'text',
        selector: selectedElementInfo.selector,
      },
    });
    
    // Create selector node first if it doesn't exist
    let selectorNode = nodes.find(n => n.type === 'selector' && n.data.selector === selectedElementInfo.selector);
    if (!selectorNode) {
      selectorNode = {
        id: `node-selector-${Date.now()}`,
        type: 'selector',
        position: { x: 100, y: 100 },
        data: {
          label: 'Selector',
          selector: selectedElementInfo.selector,
        },
      };
      nodes = [...nodes, selectorNode];
    }
    
    // Create nodes from suggestions
    suggestions.forEach((suggestion, index) => {
      const newNode: Node = {
        id: `node-auto-${Date.now()}-${index}`,
        type: suggestion.type,
        position: { x: 200 + index * 250, y: 200 },
        data: suggestion.data,
      };
      nodes = [...nodes, newNode];
      
      // Connect to selector node
      edges = [...edges, {
        id: `edge-auto-${Date.now()}-${index}`,
        source: selectorNode.id,
        target: newNode.id,
        type: 'smoothstep',
      }];
    });
    
    selectedElementInfo = null;
    
    // Automatically generate code after creating nodes
    setTimeout(() => {
      generateParserCode();
    }, 100);
  }

  function generateParserCode() {
    // Generate Rust parser code from nodes
    let code = "// Generated parser code\n\n";
    code += "use scraper::{Html, Selector};\n\n";
    code += "pub fn parse_page(html: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {\n";
    code += "    let document = Html::parse_document(html);\n";
    code += "    let mut results = Vec::new();\n\n";

    // Find root selector node
    const rootNode = nodes.find(n => n.type === 'selector' && !edges.some(e => e.target === n.id));
    
    if (rootNode) {
      const selector = rootNode.data.selector || '';
      code += `    let selector = Selector::parse("${selector}")?;\n`;
      code += "    for element in document.select(&selector) {\n";
      code += "        let mut item = serde_json::json!({});\n";

      // Process connected extract nodes
      const extractEdges = edges.filter(e => e.source === rootNode.id);
      for (const edge of extractEdges) {
        const extractNode = nodes.find(n => n.id === edge.target);
        if (extractNode && extractNode.type === 'extract') {
          const attribute = extractNode.data.attribute || 'text';
          if (attribute === 'text') {
            code += `        item["${attribute}"] = serde_json::json!(element.text().collect::<String>().trim());\n`;
          } else if (attribute === 'href') {
            code += `        if let Some(href) = element.value().attr("href") {\n`;
            code += `            item["url"] = serde_json::json!(href);\n`;
            code += `        }\n`;
          } else if (attribute === 'src') {
            code += `        if let Some(src) = element.value().attr("src") {\n`;
            code += `            item["image"] = serde_json::json!(src);\n`;
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

  async function handleLoadSite() {
    console.log('[ParserBuilder] handleLoadSite called, selectedSite:', selectedSite);
    try {
      error = null;
      
      if (!selectedSite) {
        console.log('[ParserBuilder] No site selected, clearing...');
        currentUrl = '';
        showPageViewer = false;
        nodes = [];
        edges = [];
        return;
      }
      
      // Validate site object
      if (!selectedSite.url) {
        console.error('[ParserBuilder] Selected site has no URL:', selectedSite);
        error = 'Выбранный сайт не имеет URL';
        return;
      }
      
      // Validate URL format
      try {
        new URL(selectedSite.url);
        console.log('[ParserBuilder] URL validated:', selectedSite.url);
      } catch (e) {
        console.error('[ParserBuilder] Invalid URL:', selectedSite.url, e);
        error = 'Неверный формат URL';
        return;
      }
      
      console.log('[ParserBuilder] Loading site URL into PageViewer:', selectedSite.url);
      // Load site URL into PageViewer
      currentUrl = selectedSite.url;
      showPageViewer = true;
      console.log('[ParserBuilder] PageViewer should be visible now');
      
      // Load parser config from site
      const config = selectedSite.parser_config || {};
      nodes = [];
      edges = [];
      
      // Create nodes from config
      if (config.list_selector) {
        nodes.push({
          id: 'list-selector',
          type: 'selector',
          position: { x: 100, y: 100 },
          data: {
            label: 'List Selector',
            selector: config.list_selector,
          },
        });
      }
      
      if (config.title_selector) {
        nodes.push({
          id: 'title-selector',
          type: 'extract',
          position: { x: 100, y: 200 },
          data: {
            label: 'Title Extract',
            attribute: 'text',
            selector: config.title_selector,
          },
        });
        
        if (nodes.length > 1) {
          edges.push({
            id: 'edge-1',
            source: 'list-selector',
            target: 'title-selector',
            type: 'smoothstep',
          });
        }
      }
    } catch (err) {
      console.error('Error loading site:', err);
      error = err instanceof Error ? err.message : 'Ошибка загрузки сайта';
    }
  }

  function handleSaveParser() {
    if (!selectedSite) {
      alert('Выберите сайт для сохранения');
      return;
    }

    // Generate config from nodes
    const rootNode = nodes.find(n => n.type === 'selector');
    const config: any = {};

    if (rootNode) {
      config.list_selector = rootNode.data.selector;
    }

    // Extract other selectors from extract nodes
    const extractNodes = nodes.filter(n => n.type === 'extract');
    for (const node of extractNodes) {
      if (node.data.selector) {
        const attribute = node.data.attribute || 'text';
        config[`${attribute}_selector`] = node.data.selector;
      }
    }

    // Save to site
    invoke('update_site', {
      id: selectedSite.id,
      name: selectedSite.name,
      url: selectedSite.url,
      parserConfig: config,
    }).then(() => {
      alert('Парсер сохранен!');
    }).catch((error) => {
      alert('Ошибка сохранения: ' + error);
    });
  }
</script>

<div class="parser-builder">
  <div class="builder-header">
    <h2>Конструктор парсера</h2>
    <div class="header-actions">
      <select 
        bind:value={selectedSite} 
        onchange={handleLoadSite} 
        class="site-select"
      >
        <option value={null}>Выберите сайт</option>
        {#each sites as site}
          <option value={site}>{site.name} ({site.url})</option>
        {/each}
      </select>
      {#if error}
        <div class="error-message" style="color: #ef4444; font-size: 0.875rem; padding: 0.5rem;">
          {error}
        </div>
      {/if}
      <button class="btn-secondary" onclick={() => showPageViewer = !showPageViewer}>
        {showPageViewer ? 'Скрыть' : 'Открыть'} страницу
      </button>
      {#if showPageViewer}
        <div class="selection-hint">
          💡 Нажмите "Выделить элемент" в панели страницы, затем кликните на элемент для создания ноды
        </div>
      {/if}
      <button class="btn-primary" onclick={generateParserCode}>
        <PlayIcon class="icon-small" />
        Генерировать код
      </button>
      <button class="btn-primary" onclick={handleSaveParser}>
        Сохранить парсер
      </button>
      {#if nodes.some(n => n.selected)}
        <button class="btn-danger" onclick={handleDeleteSelected}>
          <TrashIcon class="icon-small" />
          Удалить
        </button>
      {/if}
    </div>
  </div>

  <div class="builder-content">
    {#if showPageViewer}
      <div class="page-viewer-panel">
        <PageViewer 
          url={currentUrl} 
          onElementSelect={(selector, element, data) => handleElementSelect(selector, element, data)} 
        />
      </div>
    {/if}

    <div class="flow-panel" class:with-viewer={showPageViewer}>
      <SvelteFlow
        {nodes}
        {edges}
        {nodeTypes}
        onconnect={handleConnect}
        onnodeschange={handleNodesChange}
        onedgeschange={handleEdgesChange}
        onpaneclick={handlePaneClick}
        onpancontextmenu={handleContextMenu}
        class="flow-container"
      >
        <Background />
        <Controls />
        <MiniMap />
      </SvelteFlow>
    </div>

    {#if generatedCode}
      <div class="code-panel">
        <div class="code-header">
          <h3>Сгенерированный код</h3>
          <button class="btn-small" onclick={() => generatedCode = ''}>Закрыть</button>
        </div>
        <pre class="code-content"><code>{generatedCode}</code></pre>
      </div>
    {/if}
  </div>

  {#if contextMenu}
    <ContextMenu
      x={contextMenu.x}
      y={contextMenu.y}
      onClose={() => contextMenu = null}
      onAddNode={handleAddNode}
    />
  {/if}

  {#if selectedElementInfo}
    <ElementSelector
      selector={selectedElementInfo.selector}
      elementInfo={selectedElementInfo.elementInfo}
      onConfirm={confirmElementSelection}
      onCancel={cancelElementSelection}
      onAutoDetect={autoDetectElements}
    />
  {/if}
</div>

<style>
  .parser-builder {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: #0f172a;
  }

  .builder-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid #334155;
    background: #1e293b;
  }

  .builder-header h2 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 700;
    color: #e2e8f0;
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
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

  .btn-primary,
  .btn-secondary,
  .btn-danger {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 0.5rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.875rem;
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

  .btn-danger {
    background: #dc2626;
    color: white;
  }

  .btn-danger:hover {
    background: #b91c1c;
  }

  .btn-small {
    padding: 0.25rem 0.75rem;
    background: #334155;
    color: #e2e8f0;
    border: none;
    border-radius: 0.375rem;
    cursor: pointer;
    font-size: 0.75rem;
  }

  .builder-content {
    flex: 1;
    display: flex;
    overflow: hidden;
    position: relative;
  }

  .page-viewer-panel {
    width: 50%;
    border-right: 1px solid #334155;
  }

  .flow-panel {
    flex: 1;
    position: relative;
  }

  .flow-panel.with-viewer {
    width: 50%;
  }

  .flow-container {
    width: 100%;
    height: 100%;
  }

  .code-panel {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 300px;
    background: #1e293b;
    border-top: 1px solid #334155;
    display: flex;
    flex-direction: column;
    z-index: 10;
  }

  .code-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid #334155;
  }

  .code-header h3 {
    margin: 0;
    font-size: 1rem;
    color: #e2e8f0;
  }

  .code-content {
    flex: 1;
    overflow: auto;
    padding: 1rem;
    margin: 0;
    background: #0f172a;
    color: #10b981;
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
    line-height: 1.6;
  }

  .icon-small {
    width: 16px;
    height: 16px;
  }

  .selection-hint {
    padding: 0.5rem 0.75rem;
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    border-radius: 0.5rem;
    color: #0ea5e9;
    font-size: 0.8rem;
    white-space: nowrap;
  }
</style>
