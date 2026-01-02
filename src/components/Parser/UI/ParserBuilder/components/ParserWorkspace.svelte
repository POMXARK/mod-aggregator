<script lang="ts">
  import type { Connection, Node, Edge } from '@xyflow/svelte';
  import type { Site } from '@/lib/api';
  import { get } from 'svelte/store';

  // Импорт стилей layout
  import '../styles/layout.css';

  // Тип для изменений узлов (адаптирован из React Flow)
  type NodeChange = {
    id: string;
    type: string;
    dragging?: boolean;
    position?: { x: number; y: number };
  };
  import BrowserPanel from '../../constructor/BrowserPanel.svelte';
  import NodeEditor from '../../constructor/NodeEditor.svelte';
  import ChatPanel from '../../constructor/ChatPanel.svelte';
  import GeneratedCodePanel from '../../constructor/GeneratedCodePanel.svelte';
  import { ResizeHandle } from '@/components';
  import type {
    BottomTabType,
    ParserSettings,
    ParserResult,
    AISettings,
    SelectedElementInfo
  } from '../types/parser-builder.types';
  import { useParserState } from '../composables/useParserState';
  import { useUIState } from '../composables/useUIState';
  import { useAISettings } from '../composables/useAISettings';
  import { useParserOperations } from '../composables/useParserOperations';

  // Глобальный счетчик для генерации ID узлов
  let nodeIdCounter = $state(1);

  // Тип для AI конфигурации (совместим с ParserConfig)
  type AIConfig = Record<string, unknown>;



  let {
    selectedSite,
    currentUrl,
    currentChatId,
    showPageViewer,
    showAIChat,
    showParserResults,
    activeBottomTab,
    bottomPanelHeight,
    pageViewerWidth,
    chatPanelWidth,
    onElementSelect,
    onPaneClick,
    onContextMenu,
    onConnect,
    onNodesChange,
    onResizePageViewer,
    onResizePageViewerStart,
    onResizePageViewerEnd,
    onResizeChat,
    onResizeChatStart,
    onResizeChatEnd,
    onChatChange,
    onCodeGeneration,
    onApplyCode,
    onTabChange,
    onCodeEdit,
    onCodeApply,
    onCodeCancel,
    onToggleEditor,
    onCopyCode,
    onToggleResultExpansion,
    onClearReview,
    onParserSettingsChange,
    onRunnerResults,
    onRunnerError,
    onHeightChange,
    onHeightChangeStart,
    onHeightChangeEnd,
    onClose,
  }: {
    selectedSite: Site | null;
    currentUrl: string;
    currentChatId: string | null;
    showPageViewer: boolean;
    showAIChat: boolean;
    showParserResults: boolean;
    activeBottomTab: BottomTabType;
    bottomPanelHeight: number;
    pageViewerWidth: number;
    chatPanelWidth: number;
    onElementSelect: (selector: string, element: HTMLElement | null, elementData?: unknown) => void;
    onPaneClick: (event: MouseEvent) => void;
    onContextMenu: (event: MouseEvent) => void;
    onConnect: (connection: Connection) => void;
    onNodesChange: (changes: NodeChange[]) => void;
    onResizePageViewer: (width: number) => void;
    onResizePageViewerStart: () => void;
    onResizePageViewerEnd: () => void;
    onResizeChat: (width: number) => void;
    onResizeChatStart: () => void;
    onResizeChatEnd: () => void;
    onChatChange: (chatId: string | null) => void;
    onCodeGeneration: () => void;
    onApplyCode: (code: string) => void;
    onTabChange: (tab: BottomTabType) => void;
    onCodeEdit: (code: string) => void;
    onCodeApply: (code: string) => void;
    onCodeCancel: () => void;
    onToggleEditor: () => void;
    onCopyCode: () => void;
    onToggleResultExpansion: (index: number) => void;
    onClearReview: () => void;
    onParserSettingsChange: (settings: ParserSettings) => void;
    onRunnerResults: (results: ParserResult[], diagnostics: unknown[], stats: unknown) => void;
    onRunnerError: (error: string) => void;
    onHeightChange: (height: number) => void;
    onHeightChangeStart: () => void;
    onHeightChangeEnd: () => void;
    onClose: () => void;
  } = $props();

  // Получаем числовые значения из stores с fallback - используем $state для bind
  let effectivePageViewerWidth = $state(get(pageViewerWidth) || 600);
  let effectiveChatPanelWidth = $state(get(chatPanelWidth) || 400);

  // Синхронизируем изменения из stores - используем stores напрямую (реактивно в $effect)
  $effect(() => {
    effectivePageViewerWidth = $pageViewerWidth || 600;
  });

  $effect(() => {
    effectiveChatPanelWidth = $chatPanelWidth || 400;
  });

  // Отслеживание изменений UI состояния для реактивности

  // Используем composables для управления состоянием
  const {
    nodesStore,
    edgesStore,
    generatedCodeStore,
    editedCodeStore,
    parserResultsStore,
    parserTestErrorStore,
    parserDiagnosticsStore,
    parserExtractionStatsStore,
    isCheckingCodeWithAIStore,
    aiCodeReviewStore,
    parserSettingsStore,
    selectedElementInfoStore,
    setNodes,
    setEdges,
    setGeneratedCode,
    connectNodes,
    updateNodes,
    selectElement,
    generateParserConfig,
  } = useParserState();

  const {
    saveCurrentState,
    setPageViewerWidth,
    setChatPanelWidth,
  } = useUIState(() => currentUrl, () => currentChatId);

  const {
    aiSettingsStore,
    getAddAIMessageToChat,
    checkCodeWithAI,
  } = useAISettings();

  const {
    generateParserCode,
    testParser,
  } = useParserOperations();

  // Производные значения из stores для удобства доступа
  let nodes: Node[] = $derived($nodesStore);
  let edges: Edge[] = $derived($edgesStore);
  let generatedCode: string = $derived($generatedCodeStore);
  let editedCode: string = $derived($editedCodeStore);
  let parserResults: ParserResult[] = $derived($parserResultsStore);
  let parserTestError: string | null = $derived($parserTestErrorStore);
  let parserDiagnostics: unknown[] = $derived($parserDiagnosticsStore);
  let parserExtractionStats: unknown = $derived($parserExtractionStatsStore);
  let isCheckingCodeWithAI: boolean = $derived($isCheckingCodeWithAIStore);
  let aiCodeReview: string | null = $derived($aiCodeReviewStore);
  let parserSettings: ParserSettings = $derived($parserSettingsStore);
  let aiSettings: AISettings = $derived($aiSettingsStore);
  let selectedElementInfo: SelectedElementInfo | null = $derived($selectedElementInfoStore);

  // Debug: отслеживаем состояние нижней панели
  $effect(() => {
    console.log('DEBUG BottomPanel:', {
      generatedCode: generatedCode?.substring(0, 50) + '...',
      activeBottomTab,
      showParserResults,
      aiCodeReview: aiCodeReview?.substring(0, 50) + '...',
      visible: !!(generatedCode || showParserResults || aiCodeReview || activeBottomTab === 'runner')
    });
  });

  // Callback функция для чата
  let addAIMessageToChat = $derived(getAddAIMessageToChat());

  // Для bind:addAIMessage нужна переменная, а не выражение



  // Обработчики событий
  function handleElementSelect(selector: string, element: HTMLElement | null, elementData?: unknown) {
    selectElement(selector, element, elementData);
    onElementSelect(selector, element, elementData);
  }

  function handleConnect(connection: Connection) {
    connectNodes(connection);
    onConnect(connection);
  }

  function handleResizePageViewer(width: number) {
    setPageViewerWidth(width);
    onResizePageViewer(width);
  }

  function handleResizeChatPanel(width: number) {
    console.log('💬 ParserWorkspace: handleResizeChatPanel called with:', width);
    setChatPanelWidth(width);
    onResizeChat(width);
  }

  function handleNodesChange(changes: unknown[]) {
    updateNodes(changes as NodeChange[]);
    onNodesChange(changes as NodeChange[]);
  }

  function handlePaneClick(event: MouseEvent) {
    onPaneClick(event);
  }

  function handleContextMenu(event: MouseEvent) {
    onContextMenu(event);
  }

  function handleLoadComplete() {
    console.log('onLoadComplete called, current state:', {
      currentUrl: currentUrl,
      currentChatId: currentChatId,
      nodesLength: nodes.length,
      edgesLength: edges.length,
      hasGeneratedCode: !!generatedCode,
      hasEditedCode: !!editedCode
    });

    // Восстановление состояния теперь происходит в onMount ParserBuilder,
    // здесь только логируем текущее состояние
    console.log('Page loaded successfully for URL:', currentUrl);
  }

  // Функция создания нод из AI конфига
  function createNodesFromConfig(config: AIConfig) {
    console.log('Creating nodes from AI config:', config);

    const newNodes: Node[] = [];
    const newEdges: Edge[] = [];


    try {
      // Создаем selector node
      if (config.list_selector) {
        const selectorNode: Node = {
          id: `selector-${nodeIdCounter++}`,
          type: 'selector',
          position: { x: 100, y: 100 },
          data: {
            label: 'List Selector',
            selector: config.list_selector,
          },
        };
        newNodes.push(selectorNode);

        // Создаем extract ноды для каждого селектора
        const extractSelectors = Object.keys(config).filter(key =>
          key.endsWith('_selector') && key !== 'list_selector' && config[key]
        );

        extractSelectors.forEach((selectorKey, index) => {
          // Извлекаем attribute из selectorKey (например, "name_selector" -> "name")
          const attribute = selectorKey.replace('_selector', '');

          const extractNode: Node = {
            id: `extract-${nodeIdCounter++}`,
            type: 'extract',
            position: { x: 100, y: 200 + index * 100 },
            data: {
              label: `${attribute.charAt(0).toUpperCase() + attribute.slice(1)} Extract`,
              attribute: attribute,
              selector: config[selectorKey],
            },
          };
          newNodes.push(extractNode);

          // Создаем связь между selector и extract node
          const edge: Edge = {
            id: `edge-${selectorNode.id}-${extractNode.id}`,
            source: selectorNode.id,
            target: extractNode.id,
            type: 'smoothstep',
          };
          newEdges.push(edge);
        });
      }

      // Добавляем ноды в store
      if (newNodes.length > 0) {
        setNodes(newNodes);
        console.log('Created nodes from config:', newNodes.length);
      }
      if (newEdges.length > 0) {
        setEdges(newEdges);
        console.log('Created edges from config:', newEdges.length);
      }

      // Генерируем код
      if (newNodes.length > 0) {
        generateParserCode(newNodes, newEdges, parserSettings, (code) => {
          setGeneratedCode(code);
          console.log('Generated code from config, length:', code.length);

          // Сохраняем состояние после создания нод и кода
          setTimeout(() => {
            // Получаем актуальные значения URL и chatId
            const currentUrlValue = String(currentUrl || '');
            const currentChatIdValue = currentChatId || null;
            console.log('Saving state after creating nodes from config:', {
              url: currentUrlValue,
              chatId: currentChatIdValue,
              nodesCount: newNodes.length,
              edgesCount: newEdges.length,
              codeLength: code.length
            });
            saveCurrentState(currentUrlValue, currentChatIdValue, {
              nodes: newNodes,
              edges: newEdges,
              generatedCode: code,
              editedCode: code
            });
            console.log('State saved successfully');

            // Также обновляем UI состояние, чтобы показать чат (если он был открыт)
            // Обновляем состояние с chatId, чтобы показать чат
            try {
              const stateKey = `parser-builder-ui-state${currentUrlValue ? `_${btoa(currentUrlValue)}` : ''}${currentChatIdValue ? `_chat_${currentChatIdValue}` : ''}`;
              const existingState = JSON.parse(localStorage.getItem(stateKey) || '{}');
              if (existingState) {
                existingState.showAIChat = true; // Показываем чат, поскольку конфигурация восстановлена из чата
                localStorage.setItem(stateKey, JSON.stringify(existingState));
                console.log('Updated UI state to show chat for state key:', stateKey);
              }
            } catch (e) {
              console.error('Error updating UI state:', e);
            }
          }, 100);
        });
      }

    } catch (error: unknown) {
      console.error('Error creating nodes from config:', error);

      const errorMessage = error instanceof Error ? error.message : String(error);
      if (addAIMessageToChat) {
        addAIMessageToChat(`❌ Ошибка создания нод из конфига: ${errorMessage}`);
      }
    }
  }

  async function handleTestParser() {
    await testParser(
      currentUrl,
      nodes,
      parserSettings,
      (results: ParserResult[], diagnostics, stats) => {
        onRunnerResults(results, diagnostics, stats);
      },
      (error) => {
        onRunnerError(error);
      }
    );
  }

  async function handleCheckCodeWithAI() {
    let codeToCheck = editedCode || generatedCode;

    if (!codeToCheck || codeToCheck.trim().length === 0) {
      generateParserCode(nodes, edges, parserSettings, (code) => {
        setGeneratedCode(code);
        codeToCheck = code;
      });
    }

    if (!codeToCheck || codeToCheck.trim().length === 0) {
      alert('Нет кода для проверки. Сначала сгенерируйте код парсера.');
      return;
    }

    const parserConfig = generateParserConfig();

    await checkCodeWithAI(
      codeToCheck,
      parserConfig,
      (message) => {
        if (addAIMessageToChat) {
          addAIMessageToChat(message);
        }
      },
      (error) => {
        if (addAIMessageToChat) {
          addAIMessageToChat(`❌ ${error}`);
        }
      }
    );
  }
</script>

<div
  class="builder-content"
  style="padding-bottom: {generatedCode || showParserResults || aiCodeReview
    ? bottomPanelHeight + 'px'
    : '0'}"
>
  <BrowserPanel
    visible={showPageViewer}
    bind:width={effectivePageViewerWidth}
    bind:url={currentUrl}
    siteId={selectedSite?.id || null}
    onElementSelect={handleElementSelect}
    onLoadComplete={handleLoadComplete}
    className="page-viewer-panel"
  />

  {#if showPageViewer}
    <ResizeHandle
      direction="vertical"
      mode="left"
      minValue={300}
      maxValue={typeof window !== 'undefined' ? window.innerWidth * 0.6 : 1000}
      currentValue={effectivePageViewerWidth}
      onResize={handleResizePageViewer}
      {onResizePageViewerStart}
      {onResizePageViewerEnd}
    />
  {/if}

  <NodeEditor
    nodes={nodes}
    edges={edges}
    onNodesChange={handleNodesChange}
    withViewer={showPageViewer}
    withChat={showAIChat}
    withResults={showParserResults}
    onConnect={handleConnect}
    onPaneClick={handlePaneClick}
    onPaneContextMenu={handleContextMenu}
    className="flow-panel"
    style={showPageViewer
      ? showAIChat
        ? `width: calc(100% - ${effectivePageViewerWidth}px - 8px - ${effectiveChatPanelWidth}px)`
        : `width: calc(100% - ${effectivePageViewerWidth}px - 4px)`
      : showAIChat
        ? `width: calc(100% - ${effectiveChatPanelWidth}px)`
        : 'width: 100%'}
  />

  {#if showAIChat}
    <ResizeHandle
      direction="vertical"
      mode="right"
      minValue={300}
      maxValue={typeof window !== 'undefined' ? window.innerWidth * 0.8 : 1200}
      currentValue={effectiveChatPanelWidth}
      onResize={handleResizeChatPanel}
      {onResizeChatStart}
      {onResizeChatEnd}
    />

    <ChatPanel
      visible={showAIChat}
      bind:width={effectiveChatPanelWidth}
      className="chat-panel"
      aiModelType={aiSettings.modelType}
      aiModelName={aiSettings.modelName}
      aiApiKey={aiSettings.apiKey}
      aiOllamaUrl={aiSettings.ollamaUrl}
      currentUrl={currentUrl}
      nodes={nodes}
      edges={edges}
      onCreateNodes={(config) => {
        // Обработка создания нод из AI
        createNodesFromConfig(config);
      }}
      onGenerateCode={onCodeGeneration}
      onTestParser={handleTestParser}
      onCheckCodeWithAI={handleCheckCodeWithAI}
      generatedCode={generatedCode}
      onApplyCode={onApplyCode}
      selectedElementInfo={selectedElementInfo}
      addAIMessage={addAIMessageToChat || undefined}
      onResize={onResizeChat}
      onResizeStart={onResizeChatStart}
      onResizeEnd={onResizeChatEnd}
      onChatChange={onChatChange}
    />
  {/if}

  <GeneratedCodePanel
    visible={!!(generatedCode || showParserResults || aiCodeReview || activeBottomTab === 'runner')}
    bind:height={bottomPanelHeight}
    generatedCode={generatedCode}
    bind:editedCode
    showCodeEditor={false}
    showParserResults={showParserResults}
    parserResults={parserResults}
    parserTestError={parserTestError}
    parserDiagnostics={parserDiagnostics}
    parserExtractionStats={parserExtractionStats}
    aiCodeReview={aiCodeReview}
    bind:activeBottomTab={activeBottomTab}
    onTabChange={(tab) => {
      activeBottomTab = tab;
    }}
    isCheckingCodeWithAI={isCheckingCodeWithAI}
    aiApiKey={aiSettings.apiKey}
    aiModelType={aiSettings.modelType}
    showAIChat={showAIChat}
    nodes={nodes}
    edges={edges}
    currentUrl={currentUrl}
    siteId={selectedSite?.id || null}
    parserMaxElements={parserSettings.maxElements}
    parserTimeoutSeconds={parserSettings.timeoutSeconds}
    parserSlowMode={parserSettings.slowMode}
    parserDelayPerElement={parserSettings.delayPerElement}
    onParserSettingsChange={onParserSettingsChange}
    onRunnerResults={onRunnerResults}
    onRunnerError={onRunnerError}
    onHeightChange={onHeightChange}
    onHeightChangeStart={onHeightChangeStart}
    onHeightChangeEnd={onHeightChangeEnd}
    onClose={onClose}
    onCodeEdit={onCodeEdit}
    onCodeApply={onCodeApply}
    onCodeCancel={onCodeCancel}
    onToggleEditor={onToggleEditor}
    onCopyCode={onCopyCode}
    onCheckCodeWithAI={handleCheckCodeWithAI}
    onToggleResultExpansion={onToggleResultExpansion}
    onClearReview={onClearReview}
  />
</div>
