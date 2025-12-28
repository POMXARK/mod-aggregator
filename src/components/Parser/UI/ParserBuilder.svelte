<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@/lib/tauri-wrapper';
  import type { Node, Edge, Connection } from '@xyflow/svelte';
  import ContextMenu from '../../ContextMenu.svelte';
  import ElementSelector from '../../ElementSelector.svelte';
  import BrowserPanel from './constructor/BrowserPanel.svelte';
  import NodeEditor from './constructor/NodeEditor.svelte';
  import ChatPanel from './constructor/ChatPanel.svelte';
  import GeneratedCodePanel from './constructor/GeneratedCodePanel.svelte';

  import { PlayIcon, TrashIcon } from '@/components/icons';
  import type { Site, ParserConfig } from '@/lib/api';

  let nodes = $state<Node[]>([]);
  let edges = $state<Edge[]>([]);
  let selectedSite = $state<Site | null>(null);
  let sites = $state<Site[]>([]);
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

  // Результаты тестирования парсера
  let parserResults = $state<Array<{ data: Record<string, unknown>; expanded: boolean }>>([]);
  let showParserResults = $state(false);
  let parserTestError = $state<string | null>(null);
  let parserDiagnostics = $state<unknown[]>([]);
  let parserExtractionStats = $state<unknown>(null);
  let showCodeEditor = $state(false);
  let editedCode = $state('');
  let isCheckingCodeWithAI = $state(false);
  let aiCodeReview = $state<string | null>(null);
  let bottomPanelHeight = $state(300);
  let activeBottomTab = $state<'code' | 'results' | 'review' | 'runner' | null>(null);
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  let isResizingBottomPanel = $state(false);
  let addAIMessageToChat = $state<((content: string) => void) | null>(null);

  // Настройки парсера (используются при генерации кода и тестировании)
  let parserMaxElements = $state<number>(100);
  let parserTimeoutSeconds = $state<number>(60);
  let parserSlowMode = $state<boolean>(false);
  let parserDelayPerElement = $state<number>(500);

  // Размеры панелей для вертикальных слайдеров
  let pageViewerWidth = $state(600); // пиксели от левого края
  let chatPanelWidth = $state(400); // пиксели от правого края
  let isResizingPageViewer = $state(false);
  let isResizingChatPanel = $state(false);

  // Функции-геттеры для чтения актуальных значений в обработчиках
  function getIsResizingPageViewer() {
    return isResizingPageViewer;
  }
  function getIsResizingChatPanel() {
    return isResizingChatPanel;
  }
  function setIsResizingPageViewer(value: boolean) {
    isResizingPageViewer = value;
  }
  function setIsResizingChatPanel(value: boolean) {
    isResizingChatPanel = value;
  }

  // Сохраненный ID выбранного сайта для восстановления
  let savedSelectedSiteId = $state<number | null>(null);

  // Меню настроек интерфейса
  let showUISettingsMenu = $state(false);

  // Callbacks для чата
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const onGenerateCode = $state<(() => void) | null>(null);
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const onTestParser = $state<(() => Promise<void>) | null>(null);
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const onCheckCodeWithAI = $state<(() => Promise<void>) | null>(null);

  // AI настройки
  let aiModelType = $state<'ollama' | 'openai' | 'anthropic' | 'google'>('ollama');
  let aiModelName = $state('llama3.2:3b');
  let aiApiKey = $state('');
  let aiOllamaUrl = $state('http://localhost:11434');
  let showAISettings = $state(false);
  let isAIGenerating = $state(false);
  let aiDescription = $state('Извлеки информацию о модах: название, ссылка, описание, изображение');
  let showAIChat = $state(false);

  // Ollama модели
  let ollamaModels = $state<string[]>([]);
  let ollamaStatus = $state<'checking' | 'connected' | 'error' | null>(null);
  let ollamaError = $state<string | null>(null);

  // Ключ для сохранения состояний интерфейса
  const UI_STATE_KEY = 'parser-builder-ui-state';
  const UI_STATE_VERSION = 1; // Версия формата сохраненных состояний

  // Значения по умолчанию для сброса
  const DEFAULT_UI_STATE = {
    pageViewerWidth: 600, // пиксели
    chatPanelWidth: 400, // пиксели
    bottomPanelHeight: 300,
    showPageViewer: false,
    showAIChat: false,
    showParserResults: false,
    showAISettings: false,
    showCodeEditor: false,
    activeBottomTab: null as 'code' | 'results' | 'review' | null,
    currentUrl: '',
    aiModelType: 'ollama' as 'ollama' | 'openai' | 'anthropic' | 'google',
    aiModelName: 'llama3.2:3b',
    aiApiKey: '',
    aiOllamaUrl: 'http://localhost:11434',
    aiDescription: 'Извлеки информацию о модах: название, ссылка, описание, изображение',
    selectedSiteId: null as number | null,
    generatedCode: '',
    editedCode: '',
  };

  // Функция миграции состояний между версиями
  function migrateUIState(state: unknown, fromVersion: number, toVersion: number): unknown {
    const migratedState = { ...state };

    // Миграция с версии 0 (без версии) на версию 1
    if (fromVersion < 1 && toVersion >= 1) {
      // Добавляем недостающие поля со значениями по умолчанию
      if (migratedState.pageViewerWidth === undefined) {
        migratedState.pageViewerWidth = DEFAULT_UI_STATE.pageViewerWidth;
      }
      // Миграция: если pageViewerWidth < 100, значит это старый формат (проценты)
      // Конвертируем в пиксели (предполагаем ширину контейнера ~1200px, 50% = 600px)
      if (migratedState.pageViewerWidth < 100) {
        const containerWidth = 1200; // примерная ширина
        migratedState.pageViewerWidth = Math.round(
          (migratedState.pageViewerWidth / 100) * containerWidth
        );
      }
      if (migratedState.chatPanelWidth === undefined) {
        migratedState.chatPanelWidth = DEFAULT_UI_STATE.chatPanelWidth;
      }
      // Можно добавить другие миграции при необходимости
    }

    return migratedState;
  }

  // Функция сохранения состояний интерфейса
  function saveUIState() {
    try {
      const state = {
        version: UI_STATE_VERSION,
        // Размеры панелей
        pageViewerWidth,
        chatPanelWidth,
        bottomPanelHeight,
        // Видимость панелей
        showPageViewer,
        showAIChat,
        showParserResults,
        showAISettings,
        showCodeEditor,
        // Активная вкладка
        activeBottomTab,
        // Текущий URL
        currentUrl,
        // Настройки AI
        aiModelType,
        aiModelName,
        aiApiKey,
        aiOllamaUrl,
        aiDescription,
        // Выбранный сайт (только ID, так как объект может измениться)
        selectedSiteId: selectedSite?.id || null,
        // Сохраненный код
        generatedCode: generatedCode || '',
        // Отредактированный код
        editedCode: editedCode || '',
      };
      localStorage.setItem(UI_STATE_KEY, JSON.stringify(state));
    } catch (e) {
      console.error('Failed to save UI state:', e);
    }
  }

  // Функция экспорта настроек в JSON
  function exportUIState(): string {
    try {
      const state = {
        version: UI_STATE_VERSION,
        pageViewerWidth,
        chatPanelWidth,
        bottomPanelHeight,
        showPageViewer,
        showAIChat,
        showParserResults,
        showAISettings,
        showCodeEditor,
        activeBottomTab,
        currentUrl,
        aiModelType,
        aiModelName,
        aiApiKey,
        aiOllamaUrl,
        aiDescription,
        selectedSiteId: selectedSite?.id || null,
        generatedCode: generatedCode || '',
        editedCode: editedCode || '',
      };
      return JSON.stringify(state, null, 2);
    } catch (e) {
      console.error('Failed to export UI state:', e);
      return '';
    }
  }

  // Функция импорта настроек из JSON
  function importUIState(jsonString: string): boolean {
    try {
      const state = JSON.parse(jsonString);

      // Проверяем версию и мигрируем при необходимости
      const stateVersion = state.version || 0;
      if (stateVersion !== UI_STATE_VERSION) {
        const migratedState = migrateUIState(state, stateVersion, UI_STATE_VERSION);
        applyUIState(migratedState);
      } else {
        applyUIState(state);
      }

      // Сохраняем импортированное состояние
      saveUIState();
      return true;
    } catch (e) {
      console.error('Failed to import UI state:', e);
      alert('Ошибка импорта настроек: ' + (e instanceof Error ? e.message : String(e)));
      return false;
    }
  }

  // Функция применения состояния (общая для restore и import)
  function applyUIState(state: unknown) {
    // Восстанавливаем размеры панелей
    if (typeof state.pageViewerWidth === 'number') {
      // Если значение < 100, это старый формат (проценты), конвертируем
      let width = state.pageViewerWidth;
      if (width < 100) {
        const containerWidth = 1200; // примерная ширина
        width = Math.round((width / 100) * containerWidth);
      }
      pageViewerWidth = Math.max(300, Math.min(1000, width));
    }
    if (typeof state.chatPanelWidth === 'number') {
      chatPanelWidth = Math.max(250, Math.min(800, state.chatPanelWidth));
    }
    if (typeof state.bottomPanelHeight === 'number') {
      bottomPanelHeight = Math.max(
        150,
        Math.min(window.innerHeight * 0.7, state.bottomPanelHeight)
      );
    }

    // Восстанавливаем видимость панелей
    if (typeof state.showPageViewer === 'boolean') {
      showPageViewer = state.showPageViewer;
    }
    if (typeof state.showAIChat === 'boolean') {
      showAIChat = state.showAIChat;
    }
    if (typeof state.showParserResults === 'boolean') {
      showParserResults = state.showParserResults;
    }
    if (typeof state.showAISettings === 'boolean') {
      showAISettings = state.showAISettings;
    }
    if (typeof state.showCodeEditor === 'boolean') {
      showCodeEditor = state.showCodeEditor;
    }

    // Восстанавливаем активную вкладку
    if (state.activeBottomTab && ['code', 'results', 'review'].includes(state.activeBottomTab)) {
      activeBottomTab = state.activeBottomTab;
    }

    // Восстанавливаем текущий URL
    if (typeof state.currentUrl === 'string') {
      currentUrl = state.currentUrl;
    }

    // Восстанавливаем настройки AI
    if (
      state.aiModelType &&
      ['ollama', 'openai', 'anthropic', 'google'].includes(state.aiModelType)
    ) {
      aiModelType = state.aiModelType;
    }
    if (typeof state.aiModelName === 'string') {
      aiModelName = state.aiModelName;
    }
    if (typeof state.aiApiKey === 'string') {
      aiApiKey = state.aiApiKey;
    }
    if (typeof state.aiOllamaUrl === 'string') {
      aiOllamaUrl = state.aiOllamaUrl;
    }
    if (typeof state.aiDescription === 'string') {
      aiDescription = state.aiDescription;
    }

    // Сохраняем ID выбранного сайта для последующего восстановления
    if (typeof state.selectedSiteId === 'number') {
      savedSelectedSiteId = state.selectedSiteId;
    }

    // Восстанавливаем код
    if (typeof state.generatedCode === 'string') {
      generatedCode = state.generatedCode;
    }
    if (typeof state.editedCode === 'string') {
      editedCode = state.editedCode;
    }
  }

  // Функция сброса настроек интерфейса к значениям по умолчанию
  function resetUIState() {
    if (
      confirm('Вы уверены, что хотите сбросить все настройки интерфейса к значениям по умолчанию?')
    ) {
      pageViewerWidth = DEFAULT_UI_STATE.pageViewerWidth;
      chatPanelWidth = DEFAULT_UI_STATE.chatPanelWidth;
      bottomPanelHeight = DEFAULT_UI_STATE.bottomPanelHeight;
      showPageViewer = DEFAULT_UI_STATE.showPageViewer;
      showAIChat = DEFAULT_UI_STATE.showAIChat;
      showParserResults = DEFAULT_UI_STATE.showParserResults;
      showAISettings = DEFAULT_UI_STATE.showAISettings;
      showCodeEditor = DEFAULT_UI_STATE.showCodeEditor;
      activeBottomTab = DEFAULT_UI_STATE.activeBottomTab;
      currentUrl = DEFAULT_UI_STATE.currentUrl;
      aiModelType = DEFAULT_UI_STATE.aiModelType;
      aiModelName = DEFAULT_UI_STATE.aiModelName;
      aiApiKey = DEFAULT_UI_STATE.aiApiKey;
      aiOllamaUrl = DEFAULT_UI_STATE.aiOllamaUrl;
      aiDescription = DEFAULT_UI_STATE.aiDescription;
      selectedSite = null;
      savedSelectedSiteId = null;
      generatedCode = DEFAULT_UI_STATE.generatedCode;
      editedCode = DEFAULT_UI_STATE.editedCode;

      // Сохраняем сброшенное состояние
      saveUIState();
      alert('Настройки интерфейса сброшены к значениям по умолчанию');
    }
  }

  // Функция экспорта настроек в файл
  function exportUIStateToFile() {
    try {
      const json = exportUIState();
      if (!json) {
        alert('Ошибка экспорта настроек');
        return;
      }

      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `parser-builder-ui-settings-${new Date().toISOString().split('T')[0]}.json`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);

      alert('Настройки экспортированы в файл');
    } catch (e) {
      console.error('Failed to export UI state to file:', e);
      alert('Ошибка экспорта настроек: ' + (e instanceof Error ? e.message : String(e)));
    }
  }

  // Функция импорта настроек из файла
  function importUIStateFromFile() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    input.onchange = e => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) {
        return;
      }

      const reader = new FileReader();
      reader.onload = event => {
        const content = event.target?.result as string;
        if (content) {
          if (importUIState(content)) {
            alert('Настройки успешно импортированы');
          }
        }
      };
      reader.onerror = () => {
        alert('Ошибка чтения файла');
      };
      reader.readAsText(file);
    };
    input.click();
  }

  // Функция восстановления состояний интерфейса
  function restoreUIState() {
    try {
      const stored = localStorage.getItem(UI_STATE_KEY);
      if (stored) {
        const state = JSON.parse(stored);

        // Проверяем версию и мигрируем при необходимости
        const stateVersion = state.version || 0;
        if (stateVersion !== UI_STATE_VERSION) {
          const migratedState = migrateUIState(state, stateVersion, UI_STATE_VERSION);
          applyUIState(migratedState);
        } else {
          applyUIState(state);
        }
      }
    } catch (e) {
      console.error('Failed to restore UI state:', e);
    }
  }

  // Восстанавливаем выбранный сайт после загрузки сайтов
  $effect(() => {
    if (sites.length > 0 && savedSelectedSiteId !== null && !selectedSite) {
      const site = sites.find(s => s.id === savedSelectedSiteId);
      if (site) {
        selectedSite = site;
        savedSelectedSiteId = null; // Очищаем после восстановления
      }
    }
  });

  onMount(async () => {
    await loadSites();

    // Восстанавливаем состояния интерфейса после загрузки сайтов
    restoreUIState();

    // Автоматически загружаем браузер, если выбран сайт
    if (selectedSite && selectedSite.url) {
      currentUrl = selectedSite.url;
      showPageViewer = true;
    }

    // Проверяем Ollama при загрузке, если выбран Ollama
    if (aiModelType === 'ollama') {
      await checkOllama();
    }
  });

  /**
   * Проверяет статус Ollama и загружает список моделей
   */
  async function checkOllama() {
    ollamaStatus = 'checking';
    ollamaError = null;

    try {
      const baseUrl = aiOllamaUrl.replace('/api/generate', '').replace('/api/tags', '');
      const models = await invoke<string[]>('ai_check_ollama', {
        ollamaUrl: baseUrl || null,
      });

      ollamaModels = models;
      ollamaStatus = 'connected';

      // Если выбранная модель не в списке, выбираем первую доступную
      if (models.length > 0 && !models.includes(aiModelName)) {
        aiModelName = models[0];
      }
    } catch (error: unknown) {
      console.error('Ollama check error:', error);
      ollamaStatus = 'error';
      ollamaError = error instanceof Error ? error.message : String(error);
      ollamaModels = [];
    }
  }

  /**
   * Обрабатывает изменение типа AI модели
   */
  async function handleAIModelTypeChange() {
    if (aiModelType === 'ollama') {
      await checkOllama();
    } else {
      ollamaStatus = null;
      ollamaError = null;
      ollamaModels = [];
    }
  }

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
   * Обрабатывает клик по области графа (не по узлу)
   *
   * Открывает контекстное меню при правом клике.
   *
   * @param event - событие мыши
   */
  function handlePaneClick(event: MouseEvent) {
    if (event.button === 2) {
      // Right click
      contextMenu = {
        x: event.clientX,
        y: event.clientY,
      };
    } else {
      contextMenu = null;
    }
  }

  /**
   * Обрабатывает контекстное меню (правый клик)
   *
   * Открывает контекстное меню для добавления узлов.
   *
   * @param event - событие контекстного меню
   */
  function handleContextMenu(event: MouseEvent) {
    event.preventDefault();
    contextMenu = {
      x: event.clientX,
      y: event.clientY,
    };
  }

  /**
   * Добавляет новый узел в граф парсера
   *
   * Создает узел указанного типа с уникальным ID и случайной позицией.
   *
   * @param type - тип узла ('selector', 'extract', 'filter', 'transform', 'output')
   */
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

  /**
   * Получает человекочитаемое название типа узла
   *
   * @param type - тип узла
   * @returns Название узла на русском языке
   */

  /**
   * Обрабатывает создание связи между узлами
   *
   * Создает новое ребро (edge) между двумя узлами в графе.
   *
   * @param connection - информация о связи между узлами
   */
  function handleConnect(connection: Connection) {
    const newEdge: Edge = {
      id: `edge-${Date.now()}`,
      source: connection.source!,
      target: connection.target!,
      type: 'smoothstep',
    };
    edges = [...edges, newEdge];
  }

  /**
   * Обрабатывает изменения узлов в графе
   *
   * Обновляет позиции узлов после их перемещения.
   *
   * @param changes - массив изменений узлов
   */
  function handleNodesChange(changes: unknown[]) {
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

  /**
   * Удаляет выбранные узлы из графа
   *
   * Удаляет узлы и все связанные с ними связи (edges).
   */
  function handleDeleteSelected() {
    const selectedNodeIds = nodes.filter(n => n.selected).map(n => n.id);
    nodes = nodes.filter(n => !n.selected);
    edges = edges.filter(
      e => !selectedNodeIds.includes(e.source) && !selectedNodeIds.includes(e.target)
    );
  }

  /**
   * Обрабатывает выбор элемента на странице
   *
   * Показывает панель с информацией о выбранном элементе вместо
   * немедленного создания узла.
   *
   * @param selector - CSS селектор выбранного элемента
   * @param element - DOM элемент (может быть null)
   * @param elementData - данные элемента (tagName, text, attributes, similarElements)
   */
  function handleElementSelect(
    selector: string,
    element: HTMLElement | null,
    elementData?: unknown
  ) {
    console.log('handleElementSelect called:', selector, element, elementData);

    // Show selection panel instead of creating node immediately
    selectedElementInfo = {
      selector,
      elementInfo: {
        tagName: elementData?.tagName || element?.tagName || 'UNKNOWN',
        text: elementData?.text || element?.textContent?.trim().substring(0, 100) || '',
        attributes: elementData?.attributes || {},
        similarElements: elementData?.similarElements,
      },
    };
  }

  /**
   * Подтверждает выбор элемента и создает узел селектора
   *
   * Создает новый узел типа 'selector' с селектором выбранного элемента
   * и автоматически генерирует код парсера.
   */
  function confirmElementSelection() {
    if (!selectedElementInfo) {
      return;
    }

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

    // $effect автоматически вызовет generateParserCode() при изменении nodes
  }

  /**
   * Отменяет выбор элемента
   *
   * Закрывает панель с информацией о выбранном элементе.
   */
  function cancelElementSelection() {
    selectedElementInfo = null;
  }

  /**
   * Автоматически определяет похожие элементы на странице
   *
   * Пытается найти похожие элементы и предлагает узлы для извлечения данных.
   */
  async function autoDetectElements() {
    if (!selectedElementInfo) {
      return;
    }

    // Try to find similar elements and suggest extraction nodes
    const iframe = document.querySelector('iframe');
    if (iframe?.contentWindow) {
      iframe.contentWindow.postMessage(
        {
          type: 'find-similar',
          selector: selectedElementInfo.selector,
        },
        '*'
      );
    }

    // Also try AI detection if available
    try {
      const suggestions = await detectWithAI(
        selectedElementInfo.selector,
        selectedElementInfo.elementInfo
      );
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

  /**
   * Определяет похожие элементы с помощью AI
   *
   * Использует AI для анализа HTML и предложения узлов парсера.
   *
   * @param selector - CSS селектор элемента
   * @param elementInfo - информация об элементе
   * @returns Массив предложений узлов для создания
   */
  async function detectWithAI(selector: string, elementInfo: unknown): Promise<unknown[]> {
    try {
      const { detectElementsWithAI } = await import('@/lib/ai-detector');

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

  /**
   * Простое эвристическое определение похожих элементов
   *
   * Использует простые правила для определения типа элементов
   * и предложения узлов для извлечения данных.
   */
  function simpleAutoDetect() {
    if (!selectedElementInfo) {
      return;
    }

    // Simple heuristic-based detection
    const elementInfo = selectedElementInfo.elementInfo;
    const suggestions: unknown[] = [];

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
    let selectorNode = nodes.find(
      n => n.type === 'selector' && n.data.selector === selectedElementInfo.selector
    );
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
      edges = [
        ...edges,
        {
          id: `edge-auto-${Date.now()}-${index}`,
          source: selectorNode.id,
          target: newNode.id,
          type: 'smoothstep',
        },
      ];
    });

    selectedElementInfo = null;

    // $effect автоматически вызовет generateParserCode() при изменении nodes
  }

  /**
   * Генерирует код парсера на Rust из узлов графа
   *
   * Преобразует визуальный граф парсера в код на языке Rust,
   * который можно использовать для парсинга веб-страниц.
   */
  function generateParserCode() {
    // Generate Rust parser code from nodes with settings
    let code = '// Generated parser code\n';
    code += `// Settings: max_elements=${parserMaxElements}, timeout=${parserTimeoutSeconds}s, slow_mode=${parserSlowMode}, delay=${parserDelayPerElement}ms\n\n`;
    code += 'use scraper::{Html, Selector};\n';
    if (parserTimeoutSeconds > 0 || parserSlowMode) {
      code += 'use std::time::{Instant, Duration};\n';
    }
    code += '\n';

    // Добавляем константы настроек
    if (parserMaxElements > 0) {
      code += `const MAX_ELEMENTS: usize = ${parserMaxElements};\n`;
    }
    if (parserTimeoutSeconds > 0) {
      code += `const TIMEOUT_SECONDS: u64 = ${parserTimeoutSeconds};\n`;
    }
    if (parserSlowMode && parserDelayPerElement > 0) {
      code += `const DELAY_PER_ELEMENT_MS: u64 = ${parserDelayPerElement};\n`;
    }
    if (
      parserMaxElements > 0 ||
      parserTimeoutSeconds > 0 ||
      (parserSlowMode && parserDelayPerElement > 0)
    ) {
      code += '\n';
    }

    code +=
      'pub fn parse_page(html: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {\n';
    code += '    let document = Html::parse_document(html);\n';
    code += '    let mut results = Vec::new();\n';
    if (parserTimeoutSeconds > 0 || parserSlowMode) {
      code += '    let start_time = Instant::now();\n';
    }
    code += '\n';

    // Find root selector node
    const rootNode = nodes.find(n => n.type === 'selector' && !edges.some(e => e.target === n.id));

    if (rootNode) {
      const selector = rootNode.data.selector || '';
      code += `    let selector = Selector::parse("${selector}")?;\n`;

      // Собираем все элементы с учетом ограничения
      if (parserMaxElements > 0) {
        code += '    let all_elements: Vec<_> = document.select(&selector).collect();\n';
        code += `    let elements_to_process: Vec<_> = if all_elements.len() > MAX_ELEMENTS {\n`;
        code += `        all_elements.into_iter().take(MAX_ELEMENTS).collect()\n`;
        code += '    } else {\n';
        code += '        all_elements\n';
        code += '    };\n\n';
        code += '    for element_ref in elements_to_process.iter() {\n';
        code += '        let element = element_ref;\n';
      } else {
        code += '    for element in document.select(&selector) {\n';
      }

      code += '        let mut item = serde_json::json!({});\n';

      // Проверка таймаута в цикле
      if (parserTimeoutSeconds > 0) {
        code += `        if start_time.elapsed() > Duration::from_secs(TIMEOUT_SECONDS) {\n`;
        code += `            break;\n`;
        code += `        }\n`;
      }

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

      code += '        results.push(item);\n';

      // Задержка в медленном режиме
      if (parserSlowMode && parserDelayPerElement > 0) {
        code += `        std::thread::sleep(Duration::from_millis(DELAY_PER_ELEMENT_MS));\n`;
      }

      code += '    }\n\n';
    }

    code += '    Ok(results)\n';
    code += '}\n';

    generatedCode = code;
    if (generatedCode) {
      // Автоматически открываем вкладку "Код" в нижней панели
      activeBottomTab = 'code';
      // Убеждаемся, что нижняя панель видна (она показывается автоматически при наличии generatedCode)
    }
  }

  /**
   * Генерирует JSON конфигурацию парсера из узлов графа
   */
  function generateParserConfig(): ParserConfig {
    const config: Partial<ParserConfig> = {};

    // Находим корневой selector node
    const rootNode = nodes.find(n => n.type === 'selector' && !edges.some(e => e.target === n.id));

    if (rootNode) {
      config.list_selector = rootNode.data.selector || '';

      // Извлекаем селекторы из связанных extract nodes
      const extractEdges = edges.filter(e => e.source === rootNode.id);
      for (const edge of extractEdges) {
        const extractNode = nodes.find(n => n.id === edge.target);
        if (extractNode && extractNode.type === 'extract') {
          const attribute = extractNode.data.attribute || 'text';
          if (!config[`${attribute}_selector`]) {
            config[`${attribute}_selector`] = extractNode.data.selector || '';
          }
        }
      }
    }

    return config;
  }

  /**
   * Проверяет код парсера с помощью AI
   */
  async function checkCodeWithAI() {
    // Получаем код для проверки - приоритет отредактированному коду, затем сгенерированному
    let codeToCheck = editedCode || generatedCode;

    // Логируем текущее состояние для отладки
    console.log('checkCodeWithAI вызван');
    console.log('editedCode:', editedCode ? editedCode.substring(0, 100) + '...' : 'пусто');
    console.log(
      'generatedCode:',
      generatedCode ? generatedCode.substring(0, 100) + '...' : 'пусто'
    );
    console.log(
      'codeToCheck до проверки:',
      codeToCheck ? codeToCheck.substring(0, 100) + '...' : 'пусто'
    );

    // Если кода нет или это JSON, генерируем код из узлов
    if (
      !codeToCheck ||
      codeToCheck.trim().length === 0 ||
      (codeToCheck.trim().startsWith('{') && codeToCheck.trim().endsWith('}'))
    ) {
      console.log('Код отсутствует или это JSON, генерирую код из узлов...');
      generateParserCode();
      codeToCheck = generatedCode;
      console.log(
        'Сгенерированный код:',
        codeToCheck ? codeToCheck.substring(0, 200) + '...' : 'пусто'
      );
    }

    // Финальная проверка
    if (!codeToCheck || codeToCheck.trim().length === 0) {
      alert(
        'Нет кода для проверки. Сначала сгенерируйте код парсера (нажмите "Генерировать код").'
      );
      return;
    }

    // Проверяем, что это действительно Rust код (должен содержать ключевые слова Rust)
    const rustKeywords = ['fn ', 'use ', 'pub ', 'let ', 'Result', 'Vec', 'Box'];
    const isRustCode = rustKeywords.some(keyword => codeToCheck.includes(keyword));

    if (!isRustCode) {
      console.warn('Код не похож на Rust, пытаюсь сгенерировать заново...');
      generateParserCode();
      codeToCheck = generatedCode;

      if (!codeToCheck || codeToCheck.trim().length === 0) {
        alert(
          'Не удалось сгенерировать Rust код. Убедитесь, что созданы узлы парсера и нажмите "Генерировать код".'
        );
        return;
      }
    }

    isCheckingCodeWithAI = true;
    aiCodeReview = null;

    try {
      // Сначала отправляем JSON конфигурацию в чат для просмотра
      const parserConfig = generateParserConfig();
      const configJson = JSON.stringify(parserConfig, null, 2);

      if (addAIMessageToChat) {
        addAIMessageToChat(
          `📋 **JSON конфигурация парсера:**\n\n\`\`\`json\n${configJson}\n\`\`\`\n\nПроверяю сгенерированный Rust код...`
        );
      }

      // Убеждаемся, что код не пустой и правильно отформатирован
      if (!codeToCheck.trim()) {
        throw new Error('Код для проверки пуст');
      }

      // Логируем код для отладки
      console.log('Проверяемый Rust код:', codeToCheck.substring(0, 200) + '...');
      console.log('Длина кода:', codeToCheck.length);

      // Формируем промпт с четким указанием, что нужно проверить именно Rust код
      const prompt = `Проверь следующий Rust код парсера на ошибки, оптимизацию и лучшие практики. Укажи конкретные проблемы и предложи улучшения.

ВАЖНО: Проверяй именно Rust код ниже, а не JSON конфигурацию!

Rust код парсера:
\`\`\`rust
${codeToCheck.trim()}
\`\`\`

Проанализируй этот код на:
1. Синтаксические ошибки
2. Логические ошибки
3. Оптимизацию производительности
4. Лучшие практики Rust
5. Обработку ошибок
6. Безопасность парсинга

Ответь на русском языке, структурированно и конкретно.`;

      let ollamaUrl = aiOllamaUrl;
      if (aiModelType === 'ollama' && ollamaUrl && !ollamaUrl.includes('/api/')) {
        ollamaUrl = `${ollamaUrl}/api/generate`;
      }

      const review = await invoke<string>('ai_chat', {
        messages: [
          [
            'system',
            'Ты эксперт по Rust и парсингу веб-страниц. Анализируй Rust код парсера и давай конкретные рекомендации. НЕ анализируй JSON конфигурацию, только Rust код.',
          ],
          ['user', prompt],
        ],
        modelType: aiModelType,
        modelName: aiModelName || null,
        apiKey: aiApiKey || null,
        ollamaUrl: ollamaUrl || null,
      });

      // Добавляем результат проверки в чат
      if (addAIMessageToChat) {
        addAIMessageToChat(`🤖 **Отзыв AI по коду:**\n\n${review}`);
      } else {
        // Fallback: если чат не открыт, показываем в отдельной панели
        aiCodeReview = review;
        if (aiCodeReview) {
          activeBottomTab = 'review';
        }
      }
    } catch (err: unknown) {
      console.error('AI code review error:', err);
      const errorMsg = `Ошибка проверки кода: ${err instanceof Error ? err.message : String(err)}`;
      if (addAIMessageToChat) {
        addAIMessageToChat(`❌ ${errorMsg}`);
      } else {
        aiCodeReview = errorMsg;
      }
    } finally {
      isCheckingCodeWithAI = false;
    }
  }

  /**
   * Тестирует парсер на текущей странице
   */
  async function handleTestParser() {
    if (nodes.length === 0) {
      alert('Создайте ноды парсера перед тестированием');
      return;
    }

    if (!currentUrl) {
      alert('Загрузите страницу для тестирования');
      return;
    }

    // Просто открываем вкладку runner, где пользователь может запустить парсер
    activeBottomTab = 'runner';
  }

  /**
   * Генерирует парсер с помощью AI
   *
   * Использует AI для анализа HTML и автоматической генерации конфигурации парсера
   */
  async function handleAIGenerate() {
    if (!currentUrl && !showPageViewer) {
      alert('Сначала загрузите страницу сайта или откройте PageViewer');
      return;
    }

    if (!aiDescription.trim()) {
      alert('Введите описание данных для извлечения');
      return;
    }

    isAIGenerating = true;
    error = null;

    try {
      // Получаем HTML страницы
      let html = '';

      if (showPageViewer && currentUrl) {
        // Пытаемся получить из кеша
        try {
          const cached = await invoke('get_cached_page', { url: currentUrl });
          if (cached) {
            html = cached;
          }
        } catch (e) {
          console.warn('Could not get cached page:', e);
        }

        // Если не получили из кеша, загружаем
        if (!html && currentUrl) {
          html = await invoke('fetch_page', {
            url: currentUrl,
            forceRefresh: false,
            siteId: selectedSite?.id || null,
          });
        }
      } else if (currentUrl) {
        html = await invoke('fetch_page', {
          url: currentUrl,
          forceRefresh: false,
          siteId: selectedSite?.id || null,
        });
      }

      if (!html) {
        throw new Error('Не удалось получить HTML страницы');
      }

      // Генерируем парсер через AI
      // Для Ollama добавляем /api/generate к базовому URL
      let ollamaUrl = aiOllamaUrl;
      if (aiModelType === 'ollama' && ollamaUrl && !ollamaUrl.includes('/api/')) {
        ollamaUrl = `${ollamaUrl}/api/generate`;
      }

      const config = await invoke('ai_generate_parser', {
        html,
        description: aiDescription,
        modelType: aiModelType,
        modelName: aiModelName || null,
        apiKey: aiApiKey || null,
        ollamaUrl: ollamaUrl || null,
      });

      // Создаем узлы из конфигурации
      createNodesFromAIConfig(config);

      // $effect автоматически вызовет generateParserCode() при изменении nodes

      showAISettings = false;
      alert('Парсер успешно сгенерирован с помощью AI!');
    } catch (err: unknown) {
      console.error('AI generation error:', err);
      const errorMessage = err instanceof Error ? err.message : String(err);
      error = `Ошибка генерации парсера: ${errorMessage}`;
      alert(`Ошибка генерации парсера: ${errorMessage}`);
    } finally {
      isAIGenerating = false;
    }
  }

  /**
   * Создает узлы графа из AI конфигурации
   */
  function createNodesFromAIConfig(config: ParserConfig) {
    // Очищаем существующие узлы
    nodes = [];
    edges = [];

    if (!config.list_selector) {
      throw new Error('AI не смог определить list_selector');
    }

    // Создаем selector node
    const selectorNode: Node = {
      id: 'selector-root-ai',
      type: 'selector',
      position: { x: 100, y: 200 },
      data: {
        selector: config.list_selector || '',
        label: 'Список элементов',
      },
    };
    nodes = [selectorNode];

    // Создаем extract nodes
    const extractFields = [
      { key: 'title_selector', attribute: 'text', label: 'Название' },
      { key: 'url_selector', attribute: 'href', label: 'Ссылка' },
      { key: 'description_selector', attribute: 'text', label: 'Описание' },
      { key: 'image_selector', attribute: 'src', label: 'Изображение' },
    ];

    let yOffset = 200;
    extractFields.forEach(field => {
      if (config[field.key]) {
        const extractNode: Node = {
          id: `extract-${field.key}-ai`,
          type: 'extract',
          position: { x: 400, y: yOffset },
          data: {
            selector: config[field.key],
            attribute: field.attribute,
            label: field.label,
          },
        };

        nodes = [...nodes, extractNode];
        edges = [
          ...edges,
          {
            id: `edge-${field.key}-ai`,
            source: selectorNode.id,
            target: extractNode.id,
            type: 'smoothstep',
          },
        ];

        yOffset += 100;
      }
    });

    // Если нет ни одного extract узла, создаем хотя бы один для текста
    if (edges.length === 0) {
      const defaultExtractNode: Node = {
        id: 'extract-default-ai',
        type: 'extract',
        position: { x: 400, y: 200 },
        data: {
          selector: config.title_selector || '',
          attribute: 'text',
          label: 'Текст',
        },
      };

      nodes = [...nodes, defaultExtractNode];
      edges = [
        ...edges,
        {
          id: 'edge-default-ai',
          source: selectorNode.id,
          target: defaultExtractNode.id,
          type: 'smoothstep',
        },
      ];
    }

    // Автоматически генерируем код после создания нод
    // $effect уже отслеживает изменения nodes и вызовет generateParserCode автоматически
    // Не нужно вызывать здесь, чтобы избежать двойного вызова
  }

  /**
   * Обрабатывает загрузку выбранного сайта
   *
   * Загружает URL сайта в PageViewer и создает узлы из конфигурации парсера.
   * Валидирует URL и обрабатывает ошибки загрузки.
   */
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

  /**
   * Сохраняет конфигурацию парсера в базу данных
   *
   * Генерирует конфигурацию парсера из узлов графа и обновляет
   * конфигурацию выбранного сайта в базе данных.
   */
  function handleSaveParser() {
    if (!selectedSite) {
      alert('Выберите сайт для сохранения');
      return;
    }

    // Generate config from nodes
    const rootNode = nodes.find(n => n.type === 'selector');
    const config: Partial<ParserConfig> = {};

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
    })
      .then(() => {
        alert('Парсер сохранен!');
      })
      .catch(error => {
        alert('Ошибка сохранения: ' + error);
      });
  }

  // Обработчики для изменения размеров панелей
  function handleMouseMove(e: MouseEvent | PointerEvent) {
    // Используем функции-геттеры для чтения актуальных значений
    const resizingPageViewer = getIsResizingPageViewer();
    const resizingChatPanel = getIsResizingChatPanel();

    if (!resizingPageViewer && !resizingChatPanel) {
      return; // Не обрабатываем, если ничего не изменяется
    }

    e.preventDefault();

    if (resizingPageViewer) {
      const builderContent = document.querySelector('.builder-content') as HTMLElement;
      if (builderContent) {
        const rect = builderContent.getBoundingClientRect();
        // Вычисляем ширину в пикселях от левого края контейнера
        const mouseX = e.clientX;
        const containerLeft = rect.left;

        // Вычисляем ширину в пикселях
        const newWidth = mouseX - containerLeft;

        // Ограничиваем значения (минимум 300px, максимум 1000px)
        if (newWidth >= 300 && newWidth <= 1000) {
          pageViewerWidth = Math.round(newWidth);
        } else if (newWidth < 300) {
          pageViewerWidth = 300;
        } else if (newWidth > 1000) {
          pageViewerWidth = 1000;
        }
      }
    } else if (resizingChatPanel) {
      const builderContent = document.querySelector('.builder-content') as HTMLElement;
      if (builderContent) {
        const rect = builderContent.getBoundingClientRect();
        // Вычисляем ширину в пикселях от правого края
        const mouseX = e.clientX;
        const containerRight = rect.right;

        const newWidth = containerRight - mouseX;

        // Ограничиваем значения
        if (newWidth >= 250 && newWidth <= 800) {
          chatPanelWidth = Math.round(newWidth);
        } else if (newWidth < 250) {
          chatPanelWidth = 250;
        } else if (newWidth > 800) {
          chatPanelWidth = 800;
        }
      }
    }
  }

  function handleMouseUp(e: MouseEvent | PointerEvent) {
    // Используем функции-геттеры для чтения актуальных значений
    const wasResizingPageViewer = getIsResizingPageViewer();
    const wasResizingChatPanel = getIsResizingChatPanel();
    const wasResizing = wasResizingPageViewer || wasResizingChatPanel;

    // Всегда сбрасываем флаги через функции-сеттеры
    setIsResizingPageViewer(false);
    setIsResizingChatPanel(false);

    // Убираем класс с body
    document.body.classList.remove('resizing-panel');

    if (wasResizing) {
      // Сохраняем состояние сразу после завершения изменения размера
      saveUIState();
      e.preventDefault();
      e.stopPropagation();
    }
  }

  // Добавляем глобальные обработчики при монтировании - они всегда активны
  onMount(() => {
    // Создаем обработчики для pointer events (более надежное отслеживание)
    const handleGlobalPointerMove = (e: PointerEvent) => {
      if (getIsResizingPageViewer() || getIsResizingChatPanel()) {
        handleMouseMove(e);
      }
    };

    const handleGlobalPointerUp = (e: PointerEvent) => {
      if (getIsResizingPageViewer() || getIsResizingChatPanel()) {
        handleMouseUp(e);
      }
    };

    // Используем pointer events для более надежного отслеживания
    window.addEventListener('pointermove', handleGlobalPointerMove, true);
    window.addEventListener('pointerup', handleGlobalPointerUp, true);
    document.addEventListener('pointermove', handleGlobalPointerMove, true);
    document.addEventListener('pointerup', handleGlobalPointerUp, true);

    // Также добавляем mouse events для совместимости
    const handleGlobalMouseMove = (e: MouseEvent) => {
      if (getIsResizingPageViewer() || getIsResizingChatPanel()) {
        handleMouseMove(e);
      }
    };

    const handleGlobalMouseUp = (e: MouseEvent) => {
      if (getIsResizingPageViewer() || getIsResizingChatPanel()) {
        handleMouseUp(e);
      }
    };

    window.addEventListener('mousemove', handleGlobalMouseMove, true);
    window.addEventListener('mouseup', handleGlobalMouseUp, true);
    document.addEventListener('mousemove', handleGlobalMouseMove, true);
    document.addEventListener('mouseup', handleGlobalMouseUp, true);

    return () => {
      window.removeEventListener('pointermove', handleGlobalPointerMove, true);
      window.removeEventListener('pointerup', handleGlobalPointerUp, true);
      document.removeEventListener('pointermove', handleGlobalPointerMove, true);
      document.removeEventListener('pointerup', handleGlobalPointerUp, true);
      window.removeEventListener('mousemove', handleGlobalMouseMove, true);
      window.removeEventListener('mouseup', handleGlobalMouseUp, true);
      document.removeEventListener('mousemove', handleGlobalMouseMove, true);
      document.removeEventListener('mouseup', handleGlobalMouseUp, true);
    };
  });

  // Автоматическое сохранение состояний интерфейса при их изменении
  // Используем debounce для избежания частых записей в localStorage
  let saveStateTimeout: ReturnType<typeof setTimeout> | null = null;

  function debouncedSaveUIState() {
    if (saveStateTimeout) {
      clearTimeout(saveStateTimeout);
    }
    saveStateTimeout = setTimeout(() => {
      saveUIState();
      saveStateTimeout = null;
    }, 300);
  }

  // Флаг для предотвращения бесконечных циклов при генерации кода
  let isGeneratingCode = $state(false);

  // Автоматически генерируем код при изменении узлов, связей или настроек парсера
  $effect(() => {
    // Отслеживаем изменения узлов, связей и настроек
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    nodes;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    edges;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    parserMaxElements;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    parserTimeoutSeconds;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    parserSlowMode;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    parserDelayPerElement;

    // Автоматически регенерируем код при изменении узлов, связей или настроек
    // НО только если код еще не генерируется и есть узлы
    if (nodes.length > 0 && !isGeneratingCode) {
      // Используем setTimeout для избежания множественных вызовов при массовых изменениях
      const timeoutId = setTimeout(() => {
        if (!isGeneratingCode) {
          isGeneratingCode = true;
          generateParserCode();
          // Сбрасываем флаг после небольшой задержки
          setTimeout(() => {
            isGeneratingCode = false;
          }, 200);
        }
      }, 100);
      return () => clearTimeout(timeoutId);
    }
  });

  // Отслеживаем изменения всех важных состояний
  $effect(() => {
    // Читаем все состояния для отслеживания изменений
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    pageViewerWidth;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    chatPanelWidth;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    bottomPanelHeight;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    showPageViewer;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    showAIChat;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    showParserResults;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    showAISettings;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    showCodeEditor;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    activeBottomTab;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    currentUrl;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    aiModelType;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    aiModelName;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    aiApiKey;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    aiOllamaUrl;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    aiDescription;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    selectedSite;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    generatedCode;
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    editedCode;

    // Сохраняем при изменении любого из этих состояний
    debouncedSaveUIState();
  });

  // Закрытие меню настроек при клике вне его
  $effect(() => {
    if (showUISettingsMenu) {
      const handleClickOutside = (e: MouseEvent) => {
        const target = e.target as HTMLElement;
        // Не закрываем меню, если клик был на кнопке или внутри меню
        if (target.closest && !target.closest('.ui-settings-menu')) {
          showUISettingsMenu = false;
        }
      };

      window.addEventListener('click', handleClickOutside);
      return () => {
        window.removeEventListener('click', handleClickOutside);
      };
    }
  });
</script>

<div class="parser-builder">
  <div class="builder-header">
    <h2>Конструктор парсера</h2>
    <div class="header-actions">
      <select bind:value={selectedSite} onchange={handleLoadSite} class="site-select">
        <option value={null}>Выберите сайт</option>
        {#each sites as site (site.id)}
          <option value={site}>{site.name} ({site.url})</option>
        {/each}
      </select>
      {#if error}
        <div class="error-message" style="color: #ef4444; font-size: 0.875rem; padding: 0.5rem;">
          {error}
        </div>
      {/if}
      <button class="btn-secondary" onclick={() => (showPageViewer = !showPageViewer)}>
        {showPageViewer ? 'Скрыть' : 'Открыть'} страницу
      </button>
      {#if showPageViewer}
        <div class="selection-hint">
          💡 Нажмите "Выделить элемент" в панели страницы, затем кликните на элемент для создания
          ноды
        </div>
      {/if}
      <button
        class="btn-primary"
        onclick={handleAIGenerate}
        disabled={isAIGenerating}
        title="Сгенерировать парсер с помощью AI"
      >
        {#if isAIGenerating}
          <span>🤖 Генерация...</span>
        {:else}
          <span>🤖 AI Генерация</span>
        {/if}
      </button>
      <button
        class="btn-secondary"
        onclick={() => (showAISettings = !showAISettings)}
        title="Настройки AI"
      >
        ⚙️ AI
      </button>
      <button
        class="btn-secondary"
        onclick={() => (showAIChat = !showAIChat)}
        title="Открыть чат с AI"
      >
        💬 Чат
      </button>
      <button class="btn-primary" onclick={generateParserCode}>
        <PlayIcon class="icon-small" />
        Генерировать код
      </button>
      <button
        class="btn-primary"
        onclick={() => {
          activeBottomTab = 'runner';
        }}
        disabled={nodes.length === 0 || !currentUrl}
        title={nodes.length === 0
          ? 'Создайте ноды парсера'
          : !currentUrl
            ? 'Загрузите страницу'
            : 'Запустить парсер на текущей странице'}
      >
        ▶️ Запустить парсер
      </button>
      <button class="btn-primary" onclick={handleSaveParser}> Сохранить парсер </button>
      {#if nodes.some(n => n.selected)}
        <button class="btn-danger" onclick={handleDeleteSelected}>
          <TrashIcon class="icon-small" />
          Удалить
        </button>
      {/if}
      <div class="ui-settings-menu">
        <button
          class="btn-secondary"
          onclick={(e) => {
            e.stopPropagation();
            showUISettingsMenu = !showUISettingsMenu;
          }}
          title="Настройки интерфейса"
        >
          ⚙️ Настройки
        </button>
        {#if showUISettingsMenu}
          <div class="ui-settings-dropdown">
            <button
              class="ui-settings-item"
              onclick={() => {
                exportUIStateToFile();
                showUISettingsMenu = false;
              }}
              title="Экспортировать настройки интерфейса в файл"
            >
              📤 Экспорт настроек
            </button>
            <button
              class="ui-settings-item"
              onclick={() => {
                importUIStateFromFile();
                showUISettingsMenu = false;
              }}
              title="Импортировать настройки интерфейса из файла"
            >
              📥 Импорт настроек
            </button>
            <div class="ui-settings-divider"></div>
            <button
              class="ui-settings-item ui-settings-danger"
              onclick={() => {
                resetUIState();
                showUISettingsMenu = false;
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

  <!-- Модальное окно настроек AI -->
  {#if showAISettings}
    <div class="ai-modal-overlay" onclick={() => (showAISettings = false)}>
      <div class="ai-modal" onclick={e => e.stopPropagation()}>
        <div class="ai-modal-header">
          <h3>Настройки AI</h3>
          <button class="ai-modal-close" onclick={() => (showAISettings = false)} title="Закрыть">
            ×
          </button>
        </div>
        <div class="ai-modal-content">
          <div class="ai-setting-group">
            <label class="ai-radio-label">
              <input
                type="radio"
                bind:group={aiModelType}
                value="ollama"
                onchange={handleAIModelTypeChange}
              />
              <span>Ollama (локально)</span>
            </label>
            <label class="ai-radio-label">
              <input type="radio" bind:group={aiModelType} value="openai" />
              <span>OpenAI API</span>
            </label>
          </div>

          {#if aiModelType === 'ollama'}
            <div class="ai-setting-group">
              <label>
                URL Ollama:
                <input
                  type="text"
                  bind:value={aiOllamaUrl}
                  placeholder="http://localhost:11434"
                  onblur={checkOllama}
                />
              </label>
              <button
                class="btn-secondary btn-small"
                onclick={checkOllama}
                disabled={ollamaStatus === 'checking'}
              >
                {#if ollamaStatus === 'checking'}
                  Проверка...
                {:else if ollamaStatus === 'connected'}
                  ✓ Обновить модели
                {:else}
                  Проверить Ollama
                {/if}
              </button>

              {#if ollamaStatus === 'error'}
                <div class="ollama-error">
                  ❌ {ollamaError || 'Не удалось подключиться к Ollama'}
                </div>
              {/if}

              {#if ollamaStatus === 'connected'}
                <label>
                  Модель:
                  {#if ollamaModels.length > 0}
                    <select bind:value={aiModelName} class="ai-model-select">
                      {#each ollamaModels as model (model)}
                        <option value={model}>{model}</option>
                      {/each}
                    </select>
                  {:else}
                    <input type="text" bind:value={aiModelName} placeholder="llama3.2:3b" />
                    <div class="ollama-hint">
                      Модели не найдены. Установите модель: <code>ollama pull llama3.2:3b</code>
                    </div>
                  {/if}
                </label>
              {:else if ollamaStatus === 'checking'}
                <div class="ollama-status">Проверка подключения к Ollama...</div>
              {:else}
                <label>
                  Модель:
                  <input type="text" bind:value={aiModelName} placeholder="llama3.2:3b" />
                  <div class="ollama-hint">
                    Нажмите "Проверить Ollama" для загрузки списка моделей
                  </div>
                </label>
              {/if}
            </div>
          {:else if aiModelType === 'openai'}
            <div class="ai-setting-group">
              <label>
                Модель:
                <input type="text" bind:value={aiModelName} placeholder="gpt-4o-mini" />
              </label>
              <label>
                API ключ:
                <input type="password" bind:value={aiApiKey} placeholder="sk-..." />
              </label>
            </div>
          {:else if aiModelType === 'anthropic'}
            <div class="ai-setting-group">
              <label>
                Модель:
                <input type="text" bind:value={aiModelName} placeholder="claude-3-haiku-20240307" />
              </label>
              <label>
                API ключ:
                <input type="password" bind:value={aiApiKey} placeholder="sk-ant-..." />
              </label>
            </div>
          {:else if aiModelType === 'google'}
            <div class="ai-setting-group">
              <label>
                Модель:
                <input type="text" bind:value={aiModelName} placeholder="gemini-pro" />
              </label>
              <label>
                API ключ:
                <input type="password" bind:value={aiApiKey} placeholder="AIza..." />
              </label>
            </div>
          {/if}

          <div class="ai-setting-group">
            <label>
              Описание данных для извлечения:
              <textarea
                bind:value={aiDescription}
                placeholder="Извлеки информацию о модах: название, ссылка, описание, изображение"
                rows="4"
              ></textarea>
            </label>
          </div>
        </div>
        <div class="ai-modal-footer">
          <button class="btn-secondary" onclick={() => (showAISettings = false)}> Отмена </button>
          <button class="btn-primary" onclick={() => (showAISettings = false)}> Сохранить </button>
        </div>
      </div>
    </div>
  {/if}

  <div
    class="builder-content"
    style="padding-bottom: {generatedCode || showParserResults || aiCodeReview
      ? bottomPanelHeight + 'px'
      : '0'}"
  >
    <BrowserPanel
      bind:visible={showPageViewer}
      bind:width={pageViewerWidth}
      bind:url={currentUrl}
      siteId={selectedSite?.id || null}
      onElementSelect={(selector, element, data) => handleElementSelect(selector, element, data)}
      onResize={newWidth => {
        pageViewerWidth = newWidth;
      }}
      onResizeStart={() => setIsResizingPageViewer(true)}
      onResizeEnd={() => {
        setIsResizingPageViewer(false);
        saveUIState();
      }}
    />

    <NodeEditor
      bind:nodes
      bind:edges
      withViewer={showPageViewer}
      withChat={showAIChat}
      withResults={showParserResults}
      onConnect={handleConnect}
      onNodesChange={handleNodesChange}
      onPaneClick={handlePaneClick}
      onPaneContextMenu={handleContextMenu}
      style={showPageViewer
        ? showAIChat && showParserResults
          ? `flex: 1 1 auto; width: calc(100% - ${pageViewerWidth}px - 4px - ${chatPanelWidth}px - 200px); min-width: 0`
          : showAIChat
            ? `flex: 1 1 auto; width: calc(100% - ${pageViewerWidth}px - 4px - ${chatPanelWidth}px); min-width: 0`
            : showParserResults
              ? `flex: 1 1 auto; width: calc(100% - ${pageViewerWidth}px - 4px - 200px); min-width: 0`
              : `flex: 1 1 auto; width: calc(100% - ${pageViewerWidth}px - 4px); min-width: 0`
        : showAIChat
          ? `width: calc(100% - ${chatPanelWidth}px)`
          : ''}
    />

    {#if showParserResults && activeBottomTab !== 'results'}
      <!-- Отключено: результаты теперь показываются только в нижней панели -->
      <div class="parser-results-panel">
        <div class="parser-results-header">
          <h3>Результаты парсинга</h3>
          <button class="btn-close" onclick={() => (showParserResults = false)} title="Закрыть">
            ×
          </button>
        </div>
        <div class="parser-results-content">
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
            <div class="parser-results-list">
              {#each parserResults as result, index (index)}
                <div class="parser-result-item">
                  <div class="result-header">
                    <span class="result-number">#{index + 1}</span>
                    <button
                      class="btn-expand"
                      onclick={() => {
                        const item = parserResults[index];
                        item.expanded = !item.expanded;
                        parserResults = [...parserResults];
                      }}
                    >
                      {result.expanded ? '▼' : '▶'}
                    </button>
                  </div>
                  {#if result.expanded}
                    <div class="result-content">
                      {#each Object.keys(result.data || {}) as key (key)}
                        {@const value = result.data[key]}
                        <div class="result-field">
                          <strong>{key}:</strong>
                          <span class="result-value">{String(value)}</span>
                        </div>
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
          {/if}
        </div>
      </div>
    {/if}

    <ChatPanel
      bind:visible={showAIChat}
      bind:width={chatPanelWidth}
      {aiModelType}
      {aiModelName}
      {aiApiKey}
      {aiOllamaUrl}
      {currentUrl}
      {nodes}
      {edges}
      onCreateNodes={config => {
        try {
          createNodesFromAIConfig(config);
          // $effect автоматически вызовет generateParserCode() при изменении nodes
          // Автоматически выделяем элементы на странице по list_selector
          // TODO: Реализовать функцию highlightElementsOnPage
        } catch (error) {
          console.error('Failed to create nodes from AI config:', error);
        }
      }}
      onGenerateCode={() => generateParserCode()}
      onTestParser={handleTestParser}
      onCheckCodeWithAI={checkCodeWithAI}
      {generatedCode}
      onApplyCode={(code: string) => {
        generatedCode = code;
        showCodeEditor = false;
        editedCode = '';
        activeBottomTab = 'code';
      }}
      {selectedElementInfo}
      bind:addAIMessage={addAIMessageToChat}
      onResize={newWidth => {
        chatPanelWidth = newWidth;
      }}
      onResizeStart={() => setIsResizingChatPanel(true)}
      onResizeEnd={() => {
        setIsResizingChatPanel(false);
        saveUIState();
      }}
    />

    <GeneratedCodePanel
      visible={generatedCode ||
        showParserResults ||
        (aiCodeReview && !showAIChat) ||
        activeBottomTab === 'runner'}
      bind:height={bottomPanelHeight}
      bind:generatedCode
      bind:editedCode
      bind:showCodeEditor
      {showParserResults}
      {parserResults}
      {parserTestError}
      {parserDiagnostics}
      {parserExtractionStats}
      {aiCodeReview}
      bind:activeBottomTab
      {isCheckingCodeWithAI}
      {aiApiKey}
      {aiModelType}
      {showAIChat}
      {nodes}
      {edges}
      {currentUrl}
      siteId={selectedSite?.id || null}
      {parserMaxElements}
      {parserTimeoutSeconds}
      {parserSlowMode}
      {parserDelayPerElement}
      onParserSettingsChange={settings => {
        // Обновляем настройки только если они действительно изменились
        if (
          parserMaxElements !== settings.maxElements ||
          parserTimeoutSeconds !== settings.timeoutSeconds ||
          parserSlowMode !== settings.slowMode ||
          parserDelayPerElement !== settings.delayPerElement
        ) {
          parserMaxElements = settings.maxElements;
          parserTimeoutSeconds = settings.timeoutSeconds;
          parserSlowMode = settings.slowMode;
          parserDelayPerElement = settings.delayPerElement;
          // Автоматически регенерируем код при изменении настроек, если код уже был сгенерирован
          if (generatedCode && nodes.length > 0) {
            generateParserCode();
          }
        }
      }}
      onRunnerResults={(res, diag, stats) => {
        parserResults = res.map((r: unknown) => ({
          data: r,
          expanded: false,
        }));
        parserDiagnostics = diag;
        parserExtractionStats = stats;
        showParserResults = true;
        activeBottomTab = 'results';
      }}
      onRunnerError={err => {
        parserTestError = err;
        showParserResults = true;
        activeBottomTab = 'results';
      }}
      onHeightChange={newHeight => {
        bottomPanelHeight = newHeight;
      }}
      onHeightChangeStart={() => {
        isResizingBottomPanel = true;
      }}
      onHeightChangeEnd={() => {
        isResizingBottomPanel = false;
        saveUIState();
      }}
      onClose={() => {
        activeBottomTab = null;
        generatedCode = '';
        showParserResults = false;
        aiCodeReview = null;
      }}
      onTabChange={tab => {
        activeBottomTab = tab;
      }}
      onCodeEdit={code => {
        editedCode = code;
      }}
      onCodeApply={code => {
        generatedCode = code;
        showCodeEditor = false;
        editedCode = '';
      }}
      onCodeCancel={() => {
        editedCode = generatedCode;
        showCodeEditor = false;
      }}
      onToggleEditor={() => {
        showCodeEditor = !showCodeEditor;
        if (showCodeEditor && !editedCode) {
          editedCode = generatedCode;
        }
      }}
      onCopyCode={() => {
        navigator.clipboard.writeText(generatedCode);
      }}
      onCheckCodeWithAI={checkCodeWithAI}
      onToggleResultExpansion={index => {
        const item = parserResults[index];
        item.expanded = !item.expanded;
        parserResults = [...parserResults];
      }}
      onClearReview={() => {
        aiCodeReview = null;
      }}
    />
  </div>

  {#if contextMenu}
    <ContextMenu
      x={contextMenu.x}
      y={contextMenu.y}
      onClose={() => (contextMenu = null)}
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
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: clamp(0.25rem, 0.5vw, 0.375rem);
    padding: clamp(0.25rem, 0.6vh, 0.375rem) clamp(0.4375rem, 1vw, 0.625rem);
    border-bottom: 1px solid #334155;
    background: #1e293b;
    flex-shrink: 0;
    min-height: auto;
    max-height: fit-content;
    /* Оптимизация производительности */
    contain: layout style;
  }

  .builder-header h2 {
    margin: 0;
    font-size: clamp(0.8125rem, 1.1vw, 1rem);
    font-weight: 700;
    color: #e2e8f0;
    white-space: nowrap;
    line-height: 1.1;
  }

  .header-actions {
    display: flex;
    flex-wrap: wrap;
    gap: clamp(0.25rem, 0.5vw, 0.375rem);
    align-items: center;
    flex: 1;
    min-width: 0;
    justify-content: flex-end;
  }

  /* Адаптация для маленьких экранов */
  @media (max-width: 1024px) {
    .builder-header {
      flex-direction: column;
      align-items: stretch;
      padding: clamp(0.375rem, 0.65vh, 0.5rem) clamp(0.5rem, 1vw, 0.75rem);
    }

    .builder-header h2 {
      font-size: clamp(0.875rem, 1.2vw, 1rem);
      margin-bottom: 0.25rem;
    }

    .header-actions {
      justify-content: flex-start;
      flex-wrap: wrap;
    }
  }

  @media (max-width: 768px) {
    .builder-header {
      padding: clamp(0.3125rem, 0.6vh, 0.4375rem) clamp(0.4375rem, 0.875vw, 0.625rem);
    }

    .builder-header h2 {
      font-size: clamp(0.8125rem, 1.1vw, 0.9375rem);
      margin-bottom: 0.1875rem;
    }

    .header-actions {
      gap: clamp(0.1875rem, 0.4vw, 0.3125rem);
    }
  }

  .site-select {
    padding: clamp(0.1875rem, 0.5vh, 0.3125rem) clamp(0.5rem, 1vw, 0.75rem);
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.25rem;
    color: #e2e8f0;
    font-size: clamp(0.625rem, 0.75vw, 0.6875rem);
    min-width: 0;
    flex: 0 1 auto;
    line-height: 1.2;
  }

  .btn-primary,
  .btn-secondary,
  .btn-danger {
    display: flex;
    align-items: center;
    gap: clamp(0.125rem, 0.3vw, 0.1875rem);
    padding: clamp(0.1875rem, 0.5vh, 0.3125rem) clamp(0.5rem, 1vw, 0.75rem);
    border: none;
    border-radius: 0.25rem;
    font-weight: 500;
    cursor: pointer;
    transition:
      background-color 0.2s ease,
      transform 0.1s ease;
    font-size: clamp(0.625rem, 0.75vw, 0.6875rem);
    white-space: nowrap;
    flex-shrink: 0;
    line-height: 1.2;
  }

  /* Компактная кнопка "Обновить кеш" - только иконка, маленький размер */
  .btn-compact {
    padding: clamp(0.25rem, 0.6vh, 0.375rem) !important;
    font-size: clamp(0.625rem, 0.75vw, 0.6875rem) !important;
    min-width: clamp(1.75rem, 2.5vw, 2rem) !important;
    max-width: clamp(1.75rem, 2.5vw, 2rem) !important;
    width: clamp(1.75rem, 2.5vw, 2rem) !important;
    height: clamp(1.75rem, 2.5vw, 2rem) !important;
    aspect-ratio: 1;
    justify-content: center;
    align-items: center;
    gap: 0 !important;
  }

  .btn-compact .icon-small {
    width: clamp(0.875rem, 1vw, 1rem) !important;
    height: clamp(0.875rem, 1vw, 1rem) !important;
    flex-shrink: 0;
    margin: 0 !important;
  }

  .btn-compact .btn-text {
    display: none !important; /* Всегда скрываем текст */
    width: 0 !important;
    height: 0 !important;
    overflow: hidden !important;
  }

  /* Адаптация для маленьких экранов */
  @media (max-width: 768px) {
    .btn-compact {
      min-width: clamp(1.5rem, 2vw, 1.75rem) !important;
      max-width: clamp(1.5rem, 2vw, 1.75rem) !important;
      width: clamp(1.5rem, 2vw, 1.75rem) !important;
      height: clamp(1.5rem, 2vw, 1.75rem) !important;
      padding: clamp(0.1875rem, 0.5vh, 0.3125rem) !important;
    }

    .btn-compact .icon-small {
      width: clamp(0.75rem, 0.9vw, 0.875rem) !important;
      height: clamp(0.75rem, 0.9vw, 0.875rem) !important;
    }
  }

  /* AI модальное окно */
  .ai-modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(4px);
  }

  .ai-modal {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.75rem;
    color: #e2e8f0;
    width: 90%;
    max-width: 600px;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    box-shadow:
      0 20px 25px -5px rgba(0, 0, 0, 0.5),
      0 10px 10px -5px rgba(0, 0, 0, 0.2);
    animation: modalFadeIn 0.2s ease-out;
  }

  @keyframes modalFadeIn {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  .ai-modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: clamp(1rem, 2vh, 1.25rem) clamp(1.25rem, 2.5vw, 1.5rem);
    border-bottom: 1px solid #334155;
  }

  .ai-modal-header h3 {
    margin: 0;
    font-size: clamp(1rem, 1.3vw, 1.25rem);
    color: #e2e8f0;
    font-weight: 600;
  }

  .ai-modal-close {
    background: transparent;
    border: none;
    color: #cbd5e1;
    font-size: 1.75rem;
    line-height: 1;
    cursor: pointer;
    padding: 0;
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.375rem;
    transition:
      background-color 0.2s ease,
      color 0.2s ease;
  }

  .ai-modal-close:hover {
    background: #334155;
    color: #e2e8f0;
  }

  .ai-modal-content {
    padding: clamp(1rem, 2vh, 1.25rem) clamp(1.25rem, 2.5vw, 1.5rem);
    overflow-y: auto;
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: clamp(1rem, 2vh, 1.25rem);
  }

  .ai-setting-group {
    display: flex;
    flex-direction: column;
    gap: clamp(0.75rem, 1.5vh, 1rem);
  }

  .ai-setting-group label {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    color: #cbd5e1;
  }

  .ai-radio-label {
    flex-direction: row !important;
    align-items: center;
    cursor: pointer;
    padding: clamp(0.5rem, 1vh, 0.75rem);
    border-radius: 0.375rem;
    transition: background-color 0.2s ease;
  }

  .ai-radio-label:hover {
    background: #334155;
  }

  .ai-radio-label input[type='radio'] {
    margin-right: 0.75rem;
    cursor: pointer;
    width: 1.125rem;
    height: 1.125rem;
  }

  .ai-radio-label span {
    flex: 1;
  }

  .ai-setting-group input[type='text'],
  .ai-setting-group input[type='password'],
  .ai-setting-group textarea {
    padding: clamp(0.5rem, 1vh, 0.75rem) clamp(0.75rem, 1.5vw, 1rem);
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    color: #e2e8f0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    font-family: inherit;
    transition: border-color 0.2s ease;
  }

  .ai-setting-group input[type='text']:focus,
  .ai-setting-group input[type='password']:focus,
  .ai-setting-group textarea:focus {
    outline: none;
    border-color: #0ea5e9;
    box-shadow: 0 0 0 3px rgba(14, 165, 233, 0.1);
  }

  .ai-setting-group textarea {
    resize: vertical;
    min-height: 100px;
    font-family: inherit;
  }

  .btn-small {
    padding: clamp(0.375rem, 0.75vh, 0.5rem) clamp(0.75rem, 1.5vw, 1rem);
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
    align-self: flex-start;
  }

  .ai-model-select {
    padding: clamp(0.5rem, 1vh, 0.75rem) clamp(0.75rem, 1.5vw, 1rem);
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    color: #e2e8f0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    font-family: inherit;
    width: 100%;
    cursor: pointer;
    transition: border-color 0.2s ease;
  }

  .ai-model-select:focus {
    outline: none;
    border-color: #0ea5e9;
    box-shadow: 0 0 0 3px rgba(14, 165, 233, 0.1);
  }

  .ollama-status {
    padding: clamp(0.5rem, 1vh, 0.75rem);
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    border-radius: 0.5rem;
    color: #0ea5e9;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
    text-align: center;
  }

  .ollama-error {
    padding: clamp(0.5rem, 1vh, 0.75rem);
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 0.5rem;
    color: #ef4444;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
  }

  .ollama-hint {
    margin-top: 0.25rem;
    font-size: clamp(0.625rem, 0.8vw, 0.75rem);
    color: #64748b;
    font-style: italic;
  }

  .ollama-hint code {
    background: #1e293b;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    font-family: 'Courier New', monospace;
    font-size: 0.875em;
    color: #10b981;
  }

  .ai-modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: clamp(1rem, 2vh, 1.25rem) clamp(1.25rem, 2.5vw, 1.5rem);
    border-top: 1px solid #334155;
  }

  /* Адаптация для маленьких экранов */
  @media (max-width: 768px) {
    .btn-primary,
    .btn-secondary,
    .btn-danger {
      padding: clamp(0.1875rem, 0.5vh, 0.3125rem) clamp(0.5rem, 1vw, 0.75rem);
      font-size: clamp(0.625rem, 0.75vw, 0.6875rem);
    }

    .site-select {
      width: 100%;
      margin-bottom: 0.1875rem;
      padding: clamp(0.1875rem, 0.5vh, 0.3125rem) clamp(0.5rem, 1vw, 0.75rem);
    }

    .ai-modal {
      width: 95%;
      max-height: 95vh;
    }

    .ai-modal-header,
    .ai-modal-content,
    .ai-modal-footer {
      padding: clamp(0.75rem, 1.5vh, 1rem);
    }

    .ai-setting-group {
      gap: clamp(0.5rem, 1vh, 0.75rem);
    }
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
    border-right: 1px solid #334155;
    position: relative;
    flex-shrink: 0;
    overflow: hidden;
  }

  .resize-handle-vertical {
    width: 4px;
    background: #334155;
    cursor: col-resize;
    flex-shrink: 0;
    position: relative;
    transition: background-color 0.2s ease;
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    touch-action: none;
  }

  .resize-handle-vertical:hover {
    background: #0ea5e9;
  }

  .resize-handle-vertical::before {
    content: '';
    position: absolute;
    left: -2px;
    right: -2px;
    top: 0;
    bottom: 0;
  }

  /* Предотвращаем выделение текста при перетаскивании */
  .builder-content:has(.resize-handle-vertical:hover) {
    user-select: none;
    -webkit-user-select: none;
  }

  .flow-panel {
    flex: 1;
    position: relative;
    min-width: 0; /* Позволяет панели сжиматься */
  }

  /* Динамическая ширина flow-panel устанавливается через inline style когда есть viewer */
  .flow-panel.with-viewer {
    flex: 0 0 auto; /* Отключаем flex-grow и flex-shrink, используем flex-basis из width */
    /* width устанавливается динамически через inline style */
  }

  .flow-panel.with-chat {
    width: calc(100% - 400px);
  }

  .flow-panel.with-results {
    width: calc(100% - 400px);
  }

  /* Динамические ширины для комбинаций с viewer устанавливаются через inline style */
  .flow-panel.with-viewer.with-chat {
    flex: 0 0 auto;
    /* width устанавливается динамически через inline style */
  }

  .flow-panel.with-viewer.with-results {
    flex: 0 0 auto;
    /* width устанавливается динамически через inline style */
  }

  .flow-panel.with-chat.with-results:not(.with-viewer) {
    width: calc(100% - 400px - 200px);
  }

  .flow-panel.with-viewer.with-chat.with-results {
    flex: 0 0 auto;
    /* width устанавливается динамически через inline style */
  }

  .parser-results-panel {
    width: 400px;
    min-width: 300px;
    background: #1e293b;
    border-left: 1px solid #334155;
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .parser-results-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: clamp(0.75rem, 1.5vh, 1rem) clamp(1rem, 2vw, 1.25rem);
    border-bottom: 1px solid #334155;
    background: #0f172a;
  }

  .parser-results-header h3 {
    margin: 0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    color: #e2e8f0;
    font-weight: 600;
  }

  .parser-results-header .btn-close {
    background: transparent;
    border: none;
    color: #cbd5e1;
    cursor: pointer;
    font-size: 1.5rem;
    line-height: 1;
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
    transition: background-color 0.2s ease;
  }

  .parser-results-header .btn-close:hover {
    background: #334155;
  }

  .parser-results-content {
    flex: 1;
    overflow-y: auto;
    padding: clamp(0.75rem, 1.5vh, 1rem);
  }

  .parser-error {
    padding: clamp(0.75rem, 1.5vh, 1rem);
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 0.5rem;
    color: #ef4444;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
  }

  .parser-empty {
    padding: clamp(1rem, 2vh, 1.5rem);
    text-align: center;
    color: #64748b;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
  }

  .parser-stats {
    padding: clamp(0.75rem, 1.5vh, 1rem);
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    border-radius: 0.5rem;
    margin-bottom: clamp(0.75rem, 1.5vh, 1rem);
    color: #0ea5e9;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
  }

  .parser-stats strong {
    color: #e2e8f0;
  }

  .parser-results-list {
    display: flex;
    flex-direction: column;
    gap: clamp(0.5rem, 1vh, 0.75rem);
  }

  .parser-result-item {
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    padding: clamp(0.75rem, 1.5vh, 1rem);
    transition: border-color 0.2s ease;
  }

  .parser-result-item:hover {
    border-color: #0ea5e9;
  }

  .result-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: clamp(0.5rem, 1vh, 0.75rem);
  }

  .result-number {
    font-weight: 600;
    color: #0ea5e9;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
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
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
    font-style: italic;
  }

  .result-content {
    display: flex;
    flex-direction: column;
    gap: clamp(0.375rem, 0.75vh, 0.5rem);
    margin-top: clamp(0.5rem, 1vh, 0.75rem);
    padding-top: clamp(0.5rem, 1vh, 0.75rem);
    border-top: 1px solid #334155;
  }

  .result-field {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .result-field strong {
    color: #0ea5e9;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
  }

  .result-value {
    color: #e2e8f0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    word-break: break-word;
    padding: clamp(0.375rem, 0.75vh, 0.5rem);
    background: #1e293b;
    border-radius: 0.25rem;
  }

  .result-diagnostics {
    margin-top: clamp(0.75rem, 1.5vh, 1rem);
    padding-top: clamp(0.75rem, 1.5vh, 1rem);
    border-top: 1px solid #334155;
  }

  .result-diagnostics strong {
    color: #0ea5e9;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
    display: block;
    margin-bottom: clamp(0.5rem, 1vh, 0.75rem);
  }

  .result-diagnostic {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    padding: clamp(0.375rem, 0.75vh, 0.5rem);
    margin-bottom: clamp(0.25rem, 0.5vh, 0.375rem);
    border-radius: 0.25rem;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
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

  .diagnostic-icon {
    flex-shrink: 0;
  }

  .parser-diagnostics {
    margin-top: clamp(0.75rem, 1.5vh, 1rem);
    padding: clamp(0.75rem, 1.5vh, 1rem);
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
  }

  .diagnostics-header {
    color: #0ea5e9;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    margin-bottom: clamp(0.5rem, 1vh, 0.75rem);
  }

  .diagnostics-list {
    display: flex;
    flex-direction: column;
    gap: clamp(0.375rem, 0.75vh, 0.5rem);
  }

  .diagnostic-item {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    padding: clamp(0.5rem, 1vh, 0.75rem);
    border-radius: 0.375rem;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
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

  .code-panel {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    background: #1e293b;
    border-top: 1px solid #334155;
    max-height: 50vh;
    display: flex;
    flex-direction: column;
    z-index: 100;
  }

  .code-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: clamp(0.75rem, 1.5vh, 1rem) clamp(1rem, 2vw, 1.25rem);
    border-bottom: 1px solid #334155;
    background: #0f172a;
  }

  .code-header h3 {
    margin: 0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    color: #e2e8f0;
    font-weight: 600;
  }

  .code-header-actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .code-editor {
    flex: 1;
    padding: clamp(0.75rem, 1.5vh, 1rem);
    background: #0f172a;
    border: none;
    color: #e2e8f0;
    font-family: 'Courier New', monospace;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    resize: none;
    min-height: 200px;
  }

  .code-editor:focus {
    outline: none;
  }

  .code-editor-actions {
    display: flex;
    gap: 0.5rem;
    padding: clamp(0.5rem, 1vh, 0.75rem) clamp(1rem, 2vw, 1.25rem);
    border-top: 1px solid #334155;
    background: #0f172a;
  }

  .ai-code-review {
    padding: clamp(0.75rem, 1.5vh, 1rem) clamp(1rem, 2vw, 1.25rem);
    background: rgba(14, 165, 233, 0.1);
    border-top: 1px solid rgba(14, 165, 233, 0.3);
  }

  .review-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: clamp(0.5rem, 1vh, 0.75rem);
    color: #0ea5e9;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
  }

  .btn-close-small {
    background: transparent;
    border: none;
    color: #cbd5e1;
    cursor: pointer;
    font-size: 1.25rem;
    line-height: 1;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    transition: background-color 0.2s ease;
  }

  .btn-close-small:hover {
    background: #334155;
  }

  .review-content {
    color: #e2e8f0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    line-height: 1.6;
    white-space: pre-wrap;
  }

  .code-content {
    flex: 1;
    overflow: auto;
    padding: clamp(0.75rem, 1.5vh, 1rem);
    margin: 0;
    background: #0f172a;
    color: #e2e8f0;
    font-family: 'Courier New', monospace;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    line-height: 1.5;
  }

  .code-content code {
    color: #e2e8f0;
  }

  .chat-panel {
    height: 100%;
    border-left: 1px solid #334155;
    flex-shrink: 0;
    overflow: hidden;
  }

  .builder-content {
    display: flex;
    flex: 1;
    overflow: hidden;
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
    background: #1e293b;
    border-top: 1px solid #334155;
    max-height: 50vh;
    display: flex;
    flex-direction: column;
    z-index: 100;
  }

  .code-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: clamp(0.75rem, 1.5vh, 1rem) clamp(1rem, 2vw, 1.25rem);
    border-bottom: 1px solid #334155;
    background: #0f172a;
  }

  .code-header h3 {
    margin: 0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    color: #e2e8f0;
    font-weight: 600;
  }

  .code-header-actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .code-editor {
    flex: 1;
    padding: clamp(0.75rem, 1.5vh, 1rem);
    background: #0f172a;
    border: none;
    color: #e2e8f0;
    font-family: 'Courier New', monospace;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    resize: none;
    min-height: 200px;
  }

  .code-editor:focus {
    outline: none;
  }

  .code-editor-actions {
    display: flex;
    gap: 0.5rem;
    padding: clamp(0.5rem, 1vh, 0.75rem) clamp(1rem, 2vw, 1.25rem);
    border-top: 1px solid #334155;
    background: #0f172a;
  }

  .ai-code-review {
    padding: clamp(0.75rem, 1.5vh, 1rem) clamp(1rem, 2vw, 1.25rem);
    background: rgba(14, 165, 233, 0.1);
    border-top: 1px solid rgba(14, 165, 233, 0.3);
  }

  .review-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: clamp(0.5rem, 1vh, 0.75rem);
    color: #0ea5e9;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
  }

  .btn-close-small {
    background: transparent;
    border: none;
    color: #cbd5e1;
    cursor: pointer;
    font-size: 1.25rem;
    line-height: 1;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    transition: background-color 0.2s ease;
  }

  .btn-close-small:hover {
    background: #334155;
  }

  .review-content {
    color: #e2e8f0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    line-height: 1.6;
    white-space: pre-wrap;
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
    padding: clamp(0.25rem, 0.5vh, 0.375rem) clamp(0.4375rem, 0.875vw, 0.625rem);
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    border-radius: 0.375rem;
    color: #0ea5e9;
    font-size: clamp(0.625rem, 0.75vw, 0.6875rem);
    white-space: normal;
    word-break: break-word;
    max-width: 100%;
    flex: 1 1 100%;
    order: -1;
    line-height: 1.3;
  }

  /* Адаптация подсказки для маленьких экранов */
  @media (max-width: 1024px) {
    .selection-hint {
      width: 100%;
      order: 0;
      margin-top: 0.1875rem;
      padding: clamp(0.25rem, 0.45vh, 0.3125rem) clamp(0.375rem, 0.75vw, 0.5625rem);
    }
  }

  @media (max-width: 768px) {
    .selection-hint {
      font-size: clamp(0.5625rem, 0.7vw, 0.625rem);
      padding: clamp(0.1875rem, 0.4vh, 0.25rem) clamp(0.3125rem, 0.625vw, 0.5rem);
      line-height: 1.25;
    }
  }

  /* Меню настроек интерфейса */
  .ui-settings-menu {
    position: relative;
  }

  .ui-settings-dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 0.25rem;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.3);
    min-width: 200px;
    z-index: 1000;
    overflow: hidden;
  }

  .ui-settings-item {
    display: block;
    width: 100%;
    padding: clamp(0.5rem, 1vh, 0.625rem) clamp(0.75rem, 1.5vw, 1rem);
    background: transparent;
    border: none;
    color: #e2e8f0;
    text-align: left;
    cursor: pointer;
    font-size: clamp(0.6875rem, 0.8vw, 0.75rem);
    transition: background-color 0.2s ease;
  }

  .ui-settings-item:hover {
    background: #334155;
  }

  .ui-settings-item.ui-settings-danger {
    color: #ef4444;
  }

  .ui-settings-item.ui-settings-danger:hover {
    background: rgba(239, 68, 68, 0.1);
  }

  .ui-settings-divider {
    height: 1px;
    background: #334155;
    margin: 0.25rem 0;
  }

  /* Глобальный класс для предотвращения выделения при перетаскивании */
  :global(body.resizing-panel) {
    user-select: none !important;
    -webkit-user-select: none !important;
    -moz-user-select: none !important;
    -ms-user-select: none !important;
    cursor: col-resize !important;
  }

  :global(body.resizing-panel *) {
    cursor: col-resize !important;
  }
</style>
