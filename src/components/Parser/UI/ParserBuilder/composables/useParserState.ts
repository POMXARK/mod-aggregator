import { invoke } from '@/lib/tauri-wrapper';
import { writable, get } from 'svelte/store';
import type { Node, Edge, Connection } from '@xyflow/svelte';
import type { Site, ParserConfig } from '@/lib/api';

// Тип для изменений узлов (адаптирован из React Flow)
type NodeChange = {
  id: string;
  type: string;
  dragging?: boolean;
  position?: { x: number; y: number };
};
import type {
  ParserResult,
  ParserSettings,
  SelectedElementInfo,
  ContextMenuState
} from '../types/parser-builder.types';

/**
 * Composable для управления состоянием парсера
 * Управляет nodes, edges, кодом и базовыми настройками парсера
 */
export function useParserState() {
  // Состояние парсера
  const nodesStore = writable<Node[]>([]);
  const edgesStore = writable<Edge[]>([]);
  const selectedSiteStore = writable<Site | null>(null);
  const sitesStore = writable<Site[]>([]);
  const currentUrlStore = writable('');
  const generatedCodeStore = writable('');
  const editedCodeStore = writable('');
  const errorStore = writable<string | null>(null);

  // Выбранные элементы
  const contextMenuStore = writable<ContextMenuState | null>(null);
  const selectedElementInfoStore = writable<SelectedElementInfo | null>(null);

  // Результаты тестирования
  const parserResultsStore = writable<ParserResult[]>([]);
  const parserTestErrorStore = writable<string | null>(null);
  const parserDiagnosticsStore = writable<unknown[]>([]);
  const parserExtractionStatsStore = writable<unknown>(null);
  const isCheckingCodeWithAIStore = writable(false);
  const aiCodeReviewStore = writable<string | null>(null);

  // Настройки парсера
  const parserSettingsStore = writable<ParserSettings>({
    maxElements: 100,
    timeoutSeconds: 60,
    slowMode: false,
    delayPerElement: 500
  });

  // Внутренние ID
  const savedSelectedSiteIdStore = writable<number | null>(null);


  // Функции-сеттеры для изменения состояния
  const setNodes = (value: Node[]) => nodesStore.set(value);
  const setEdges = (value: Edge[]) => edgesStore.set(value);
  const setSelectedSite = (value: Site | null) => selectedSiteStore.set(value);
  const setSites = (value: Site[]) => sitesStore.set(value);
  const setCurrentUrl = (value: string) => currentUrlStore.set(value);
  const setGeneratedCode = (value: string) => generatedCodeStore.set(value);
  const setEditedCode = (value: string) => editedCodeStore.set(value);
  const setError = (value: string | null) => errorStore.set(value);
  const setContextMenu = (value: ContextMenuState | null) => contextMenuStore.set(value);
  const setSelectedElementInfo = (value: SelectedElementInfo | null) => selectedElementInfoStore.set(value);
  const setParserResults = (value: ParserResult[]) => parserResultsStore.set(value);
  const setParserTestError = (value: string | null) => parserTestErrorStore.set(value);
  const setParserDiagnostics = (value: unknown[]) => parserDiagnosticsStore.set(value);
  const setParserExtractionStats = (value: unknown) => parserExtractionStatsStore.set(value);
  const setIsCheckingCodeWithAI = (value: boolean) => isCheckingCodeWithAIStore.set(value);
  const setAiCodeReview = (value: string | null) => aiCodeReviewStore.set(value);
  const setParserSettings = (value: ParserSettings) => parserSettingsStore.set(value);
  const setSavedSelectedSiteId = (value: number | null) => savedSelectedSiteIdStore.set(value);

  // Специфические функции для работы с состоянием

  /**
   * Добавляет новый узел в граф
   */
  function addNode(type: string) {
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

    nodesStore.update(currentNodes => [...currentNodes, newNode]);
  }

  /**
   * Создает связь между узлами
   */
  function connectNodes(connection: Connection) {
    const newEdge: Edge = {
      id: `edge-${Date.now()}`,
      source: connection.source!,
      target: connection.target!,
      type: 'smoothstep',
    };
    edgesStore.update(currentEdges => [...currentEdges, newEdge]);
  }

  /**
   * Обновляет позиции узлов
   */
  function updateNodes(changes: NodeChange[]) {
    nodesStore.update(currentNodes => {
      const updatedNodes = [...currentNodes];
      for (const change of changes) {
        if (change.type === 'position' && change.dragging === false && change.position) {
          const nodeIndex = updatedNodes.findIndex(n => n.id === change.id);
          if (nodeIndex !== -1) {
            updatedNodes[nodeIndex] = { ...updatedNodes[nodeIndex], position: change.position };
          }
        }
      }
      return updatedNodes;
    });
  }

  /**
   * Удаляет выбранные узлы
   */
  function deleteSelectedNodes() {
    nodesStore.update(currentNodes => {
      const selectedNodeIds = currentNodes.filter(n => n.selected).map(n => n.id);
      const filteredNodes = currentNodes.filter(n => !n.selected);
      
      edgesStore.update(currentEdges => 
        currentEdges.filter(
          e => !selectedNodeIds.includes(e.source) && !selectedNodeIds.includes(e.target)
        )
      );
      
      return filteredNodes;
    });
  }

  /**
   * Загружает список сайтов
   */
  async function loadSites() {
    try {
      const loadedSites = await invoke('get_sites');
      sitesStore.set(loadedSites);
    } catch (err) {
      console.error('Failed to load sites:', err);
      errorStore.set('Ошибка загрузки сайтов');
    }
  }

  /**
   * Генерирует конфигурацию парсера из узлов
   */
  function generateParserConfig(): ParserConfig {
    // Получаем текущие значения из stores
    const currentNodes = get(nodesStore);
    const currentEdges = get(edgesStore);

    // Находим корневой selector node
    const rootNode = currentNodes.find((n: Node) => n.type === 'selector' && !currentEdges.some((e: Edge) => e.target === n.id));

    const config: ParserConfig = {
      list_selector: String(rootNode?.data?.selector || ''),
    };

    if (rootNode) {
      // Извлекаем селекторы из связанных extract nodes
      const extractEdges = currentEdges.filter((e: Edge) => e.source === rootNode.id);
      for (const edge of extractEdges) {
        const extractNode = currentNodes.find((n: Node) => n.id === edge.target);
        if (extractNode && extractNode.type === 'extract') {
          const attribute = extractNode.data.attribute || 'text';
          if (!config[`${attribute}_selector`]) {
            config[`${attribute}_selector`] = String(extractNode.data.selector || '');
          }
        }
      }
    }

    return config;
  }

  /**
   * Обрабатывает выбор элемента
   */
  function selectElement(selector: string, element: HTMLElement | null, elementData?: unknown) {
    const data = elementData as {
      tagName?: string;
      text?: string;
      attributes?: Record<string, string>;
      similarElements?: number;
    } | undefined;

    selectedElementInfoStore.set({
      selector,
      elementInfo: {
        tagName: data?.tagName || element?.tagName || 'UNKNOWN',
        text: data?.text || element?.textContent?.trim().substring(0, 100) || '',
        attributes: data?.attributes || {},
        similarElements: data?.similarElements,
      },
    });
  }

  /**
   * Подтверждает выбор элемента и создает узел
   */
  function confirmElementSelection() {
    const currentSelectedElementInfo = get(selectedElementInfoStore);
    if (!currentSelectedElementInfo) {
      return;
    }

    const newNode: Node = {
      id: `node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
      type: 'selector',
      position: { x: Math.random() * 300 + 100, y: Math.random() * 300 + 100 },
      data: {
        label: 'Selector',
        selector: currentSelectedElementInfo.selector,
      },
    };

    nodesStore.update(currentNodes => [...currentNodes, newNode]);
    selectedElementInfoStore.set(null);
  }

  /**
   * Отменяет выбор элемента
   */
  function cancelElementSelection() {
    selectedElementInfoStore.set(null);
  }

  /**
   * Очищает состояние парсера
   */
  // function clearParserState() {
  //   nodes = [];
  //   edges = [];
  //   generatedCode = '';
  //   editedCode = '';
  //   error = null;
  //   selectedElementInfo = null;
  //   parserResults = [];
  //   parserTestError = null;
  //   parserDiagnostics = [];
  //   parserExtractionStats = null;
  //   aiCodeReview = null;
  // }

  return {
    // Stores
    nodesStore,
    edgesStore,
    selectedSiteStore,
    sitesStore,
    currentUrlStore,
    generatedCodeStore,
    editedCodeStore,
    errorStore,
    contextMenuStore,
    selectedElementInfoStore,
    parserResultsStore,
    parserTestErrorStore,
    parserDiagnosticsStore,
    parserExtractionStatsStore,
    isCheckingCodeWithAIStore,
    aiCodeReviewStore,
    parserSettingsStore,

    // Сеттеры
    setNodes,
    setEdges,
    setSelectedSite,
    setSites,
    setCurrentUrl,
    setGeneratedCode,
    setEditedCode,
    setError,
    setContextMenu,
    setSelectedElementInfo,
    setParserResults,
    setParserTestError,
    setParserDiagnostics,
    setParserExtractionStats,
    setIsCheckingCodeWithAI,
    setAiCodeReview,
    setParserSettings,
    setSavedSelectedSiteId,

    // Действия
    addNode,
    connectNodes,
    updateNodes,
    deleteSelectedNodes,
    loadSites,
    generateParserConfig,
    selectElement,
    confirmElementSelection,
    cancelElementSelection,
  };
}

