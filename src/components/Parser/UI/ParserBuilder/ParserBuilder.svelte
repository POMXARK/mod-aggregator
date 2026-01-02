<script lang="ts">
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
  import ContextMenu from '@/components/ContextMenu.svelte';
  import ElementSelector from '@/components/ElementSelector.svelte';
  import ParserBuilderHeader from './components/ParserBuilderHeader.svelte';
  import AISettingsModal from './components/AISettingsModal.svelte';
  import ParserWorkspace from './components/ParserWorkspace.svelte';
  import type { Node, Edge } from '@xyflow/svelte';
  import type { Site } from '@/lib/api';
  import type { ParserResult, ParserSettings, ContextMenuState, SelectedElementInfo, BottomTabType } from './types/parser-builder.types';

  // Импортируем стили
  import './styles/layout.css';

  // Тип для изменений узлов (адаптирован из React Flow)
  type NodeChange = {
    id: string;
    type: string;
    dragging?: boolean;
    position?: { x: number; y: number };
  };
  import './styles/header.css';
  import './styles/modal.css';
  import './styles/workspace.css';

  // Импортируем composables
  import { useParserState } from '@/lib/composables/useAppState';
  import { useAISettings } from '@/lib/composables/useAppState';
  import { useUIState } from '@/lib/composables/useAppState';
  import { useParserOperations } from './composables/useParserOperations';

  // Используем новую универсальную систему управления состоянием
  const {
    uiState,
    showPageViewer,
    showAIChat,
    showParserResults,
    showUISettingsMenu,
    showCodeEditor,
    activeBottomTab,
    pageViewerWidth,
    chatPanelWidth,
    bottomPanelHeight,
    isResizingPageViewer,
    isResizingChatPanel,
    isResizingBottomPanel,
    currentChatId: uiCurrentChatId,
    setShowPageViewer,
    setShowAIChat,
    setShowParserResults,
    setShowUISettingsMenu,
    setShowCodeEditor,
    setActiveBottomTab,
    setPageViewerWidth,
    setChatPanelWidth,
    setBottomPanelHeight,
    setIsResizingPageViewer,
    setIsResizingChatPanel,
    setIsResizingBottomPanel,
    setCurrentChatId,
    saveState: saveUIState,
    loadState: loadUIState,
    resetState: resetUIState,
    exportUIStateToFile,
    importUIStateFromFile,
    applyUIState,
    diagnoseState,
  } = useUIState();

  const {
    parserState,
    nodes,
    edges,
    generatedCode,
    editedCode,
    currentUrl: parserCurrentUrl,
    selectedSiteId,
    setNodes,
    setEdges,
    setGeneratedCode,
    setEditedCode,
    setCurrentUrl,
    setSelectedSiteId,
    saveState: saveParserState,
    loadState: loadParserState,
    resetState: resetParserState,
  } = useParserState();

  const {
    aiSettings,
    modelType,
    modelName,
    apiKey,
    ollamaUrl,
    description,
    setModelType,
    setModelName,
    setApiKey,
    setOllamaUrl,
    setDescription,
    saveSettings,
    loadSettings,
    resetSettings,
  } = useAISettings();

  // Импортируем дополнительные функции для совместимости
  import { useParserState as useLegacyParserState } from './composables/useParserState';
  import { useAISettings as useLegacyAISettings } from './composables/useAISettings';

  // Переходные composables для совместимости со старым кодом
  const {
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
    parserSettingsStore,
    setSelectedSite,
    setError,
    setContextMenu,
    setSelectedElementInfo,
    setParserResults,
    setParserTestError,
    setParserDiagnostics,
    setParserExtractionStats,
    setAiCodeReview,
    addNode,
    connectNodes,
    updateNodes,
    deleteSelectedNodes,
    loadSites,
    selectElement,
    confirmElementSelection,
    cancelElementSelection,
  } = useLegacyParserState();

  const {
    isAIGeneratingStore,
    showAISettingsStore,
    setShowAISettings,
    checkOllama,
    generateParserWithAI,
  } = useLegacyAISettings();


  const {
    generateParserCode,
    autoDetectElements,
    saveParser,
    loadSite,
  } = useParserOperations();

  // Переменные nodes и edges уже определены в useParserState composable
  let selectedSite: Site | null = $derived($selectedSiteStore);
  let sites: Site[] = $derived($sitesStore);
  let error: string | null = $derived($errorStore);
  let showAISettings: boolean = $derived($showAISettingsStore);
  let isAIGenerating: boolean = $derived($isAIGeneratingStore);
  let currentUrl: string = $derived($currentUrlStore);
  let contextMenu: ContextMenuState | null = $derived($contextMenuStore);
  let selectedElementInfo: SelectedElementInfo | null = $derived($selectedElementInfoStore);
  let parserSettings: ParserSettings = $derived($parserSettingsStore);
  // currentChatId уже доступен как uiCurrentChatId из useUIState

  // Флаги состояния восстановления
  let stateRestored = $state(false);
  let restoredState: any = $state(null);

  // Флаг готовности компонентов для загрузки URL
  let componentsReady = $state(false);

  // Эффект для отслеживания готовности компонентов
  $effect(() => {
    // Ждем пока компоненты смонтируются
    if (typeof window !== 'undefined') {
      // Небольшая задержка для завершения инициализации
      setTimeout(() => {
        componentsReady = true;
        console.log('Components marked as ready for URL loading');
      }, 100);
    }
  });

  // Эффект для автоматической загрузки браузера при готовности компонентов
  $effect(() => {
    if (!componentsReady) return;

    if (stateRestored && restoredState && restoredState.stateUrl && !currentUrl) {
      // Если состояние восстановлено и в нем есть URL, загружаем эту страницу
      console.log('Auto-loading page from restored state:', restoredState.stateUrl);

      // Используем небольшую задержку для обеспечения полной инициализации
      setTimeout(() => {
        // Обновляем URL в новом unified state manager
        setCurrentUrl(restoredState.stateUrl);

        // Также обновляем legacy store для совместимости с UI
        currentUrlStore.set(restoredState.stateUrl);

        setShowPageViewer(true);
        console.log('Auto-loaded page from restored state');
      }, 200);
    }
  });

  // Сохранение состояния при изменении URL теперь происходит через useUIState

  // Переменные состояния теперь получаются из новой универсальной системы

  // Обработчики событий

  async function handleLoadSite() {
    await loadSite(
      selectedSite,
      (url, newNodes, newEdges) => {
        setCurrentUrl(url);
        // Обновляем nodes и edges через composable
        setNodes(newNodes);
        setEdges(newEdges);
        setShowPageViewer(true);
      },
      (errorMsg) => {
        setError(errorMsg);
      }
    );
  }

  async function handleAIGenerate() {
    if (!$parserCurrentUrl) {
      alert('Сначала загрузите страницу сайта');
      return;
    }

    await generateParserWithAI(
      $parserCurrentUrl,
      (config) => {
        console.log('AI generated config:', config);
        // Здесь нужно создать ноды из конфига
        alert('Парсер успешно сгенерирован с помощью AI!');
      },
      (error) => {
        alert(`Ошибка генерации парсера: ${error}`);
      }
    );
  }

  function handlePaneClick(_event: MouseEvent) {
    setContextMenu(null);
  }

  function handleContextMenu(event: MouseEvent) {
    event.preventDefault();
    setContextMenu({
      x: event.clientX,
      y: event.clientY,
    });
  }

  function handleAddNode(type: string) {
    addNode(type);
    setContextMenu(null);
    // Автоматически генерируем код при добавлении ноды
    generateParserCode(nodes, edges, parserSettings, (code) => {
      setGeneratedCode(code);
      setActiveBottomTab('code');
    });
  }

  function handleConnect(connection: any) {
    connectNodes(connection);
  }

  function handleNodesChange(changes: NodeChange[]) {
    updateNodes(changes);
  }

  function handleDeleteSelected() {
    deleteSelectedNodes();
  }

  function handleElementSelect(selector: string, element: HTMLElement | null, elementData?: unknown) {
    selectElement(selector, element, elementData);
  }



  async function handleSaveParser() {
    await saveParser(
      selectedSite,
      nodes,
      () => {
        alert('Парсер сохранен!');
      },
      (error) => {
        alert(`Ошибка сохранения: ${error}`);
      }
    );
  }

  // Обработчики для заголовка
  function onSiteChange(site: any) {
    setSelectedSite(site);
    if (site) {
      handleLoadSite();
    } else {
      setCurrentUrl('');
      setShowPageViewer(false);
    }
  }

  function onTogglePageViewer() {
    setShowPageViewer(!showPageViewer);
  }

  function onToggleAISettings() {
    setShowAISettings(!showAISettings);
  }

  function onToggleAIChat() {
    const currentValue = $showAIChat;
    setShowAIChat(!currentValue);
  }

  function onGenerateCode() {
    generateParserCode(nodes, edges, parserSettings, (code) => {
      setGeneratedCode(code);
      setActiveBottomTab('code');
    });
  }

  function onTestParser() {
    setActiveBottomTab('runner');
  }

  // Инициализация
  onMount(async () => {
    await loadSites();

    // Восстанавливаем состояния интерфейса ПЕРЕД установкой дефолтных значений
    // (переменные stateRestored и restoredState объявлены на верхнем уровне)
    try {
      // Сначала пытаемся найти последнее сохраненное состояние с URL
      const allKeys = Object.keys(localStorage);
      const stateKeys = allKeys.filter(key => key.includes('parser-builder-ui-state'));
      console.log('Found state keys:', stateKeys);

      // Логируем содержимое всех состояний для отладки
      stateKeys.forEach(key => {
        try {
          const state = JSON.parse(localStorage.getItem(key) || '{}');
          console.log(`State ${key}:`, {
            stateUrl: state.stateUrl,
            chatId: state.chatId,
            hasNodes: Array.isArray(state.nodes) && state.nodes.length > 0,
            nodesCount: Array.isArray(state.nodes) ? state.nodes.length : 0,
            hasGeneratedCode: !!state.generatedCode,
            generatedCodeLength: state.generatedCode?.length || 0
          });
        } catch (e) {
          console.error(`Error parsing state ${key}:`, e);
        }
      });

      // Ищем состояние с URL (не пустым)
      let stateWithChatId = null;
      let stateWithUrl = null;

      for (const key of stateKeys) {
        try {
          const state = JSON.parse(localStorage.getItem(key) || '{}');
          if (state.stateUrl && state.stateUrl.trim() !== '') {
            console.log('Considering state:', {
              key,
              stateUrl: state.stateUrl,
              chatId: state.chatId,
              hasNodes: Array.isArray(state.nodes) && state.nodes.length > 0,
              nodesCount: Array.isArray(state.nodes) ? state.nodes.length : 0
            });

            if (state.chatId) {
              // Состояние с chatId - наиболее приоритетное
              if (!stateWithChatId || (Array.isArray(state.nodes) && state.nodes.length > 0)) {
                stateWithChatId = state;
                console.log('Selected state with chatId:', state.stateUrl, state.chatId);
              }
            } else if (!stateWithChatId) {
              // Состояние без chatId, но с URL
              if (!stateWithUrl || (Array.isArray(state.nodes) && state.nodes.length > 0)) {
                stateWithUrl = state;
                console.log('Selected state with URL (no chatId):', state.stateUrl);
              }
            }
          }
        } catch (e) {
          console.error('Error parsing state:', e);
        }
      }

      // Предпочитаем состояние с chatId, затем с URL, затем общее состояние
      const finalState = stateWithChatId || stateWithUrl;

      if (finalState) {
        // Применяем состояние с URL
        console.log('Restoring state with URL:', finalState.stateUrl, finalState.chatId ? `chatId: ${finalState.chatId}` : '');
        applyUIState(finalState);

        // Если есть chatId, автоматически открываем чат
        if (finalState.chatId) {
          console.log('Auto-opening chat for restored state with chatId:', finalState.chatId);
          setShowAIChat(true);
          console.log('Set showAIChat to true via setShowAIChat');
        }

        // Восстанавливаем parser состояние (ноды, код)
        console.log('Restoring parser state from finalState:', {
          hasNodes: Array.isArray(finalState.nodes),
          nodesCount: Array.isArray(finalState.nodes) ? finalState.nodes.length : 0,
          hasEdges: Array.isArray(finalState.edges),
          edgesCount: Array.isArray(finalState.edges) ? finalState.edges.length : 0,
          hasGeneratedCode: !!finalState.generatedCode,
          generatedCodeLength: finalState.generatedCode?.length || 0,
          hasEditedCode: !!finalState.editedCode,
          editedCodeLength: finalState.editedCode?.length || 0
        });

        if (Array.isArray(finalState.nodes) && finalState.nodes.length > 0) {
          setNodes(finalState.nodes);
          console.log('Restored nodes from state:', finalState.nodes.length, finalState.nodes.map((n: Node) => ({ id: n.id, type: n.type })));
        }
        if (Array.isArray(finalState.edges) && finalState.edges.length > 0) {
          setEdges(finalState.edges);
          console.log('Restored edges from state:', finalState.edges.length);
        }
        if (finalState.generatedCode && finalState.generatedCode.trim()) {
          setGeneratedCode(finalState.generatedCode);
          // Синхронизируем с legacy store для совместимости
          generatedCodeStore.set(finalState.generatedCode);
          console.log('Restored generated code, length:', finalState.generatedCode.length);
        }
        if (finalState.editedCode && finalState.editedCode.trim()) {
          setEditedCode(finalState.editedCode);
          // Синхронизируем с legacy store для совместимости
          editedCodeStore.set(finalState.editedCode);
          console.log('Restored edited code, length:', finalState.editedCode.length);
        }

        restoredState = finalState;
        stateRestored = true;

        // Автоматически загружаем страницу с сохраненным URL
        setTimeout(() => {
          setCurrentUrl(finalState.stateUrl);
          setShowPageViewer(true);
        }, 10);
      } else {
        // Пытаемся восстановить общее состояние
        console.log('No URL state found, trying general state');
        restoredState = restoreUIState('', null);
        if (restoredState) {
          stateRestored = true;
        }
      }
    } catch (e) {
      console.error('Failed to restore UI state:', e);
    }

    // Проверяем Ollama при загрузке
    await checkOllama();
  });

  // Автосохранение происходит автоматически через новую систему управления состоянием
</script>

<div class="parser-builder">
  <ParserBuilderHeader
    {selectedSite}
    {sites}
    {error}
    {showPageViewer}
    showAIChat={$showAIChat}
    {showAISettings}
    {isAIGenerating}
    {currentUrl}
    {nodes}
    showUISettingsMenu={false}
    onSiteChange={onSiteChange}
    onTogglePageViewer={onTogglePageViewer}
    onAIGenerate={handleAIGenerate}
    onToggleAISettings={onToggleAISettings}
    onToggleAIChat={onToggleAIChat}
    onDiagnoseState={diagnoseState}
    onGenerateCode={onGenerateCode}
    onTestParser={onTestParser}
    onSaveParser={handleSaveParser}
    onDeleteSelected={handleDeleteSelected}
    onToggleUISettingsMenu={() => {}}
    onExportSettings={exportUIStateToFile}
    onImportSettings={importUIStateFromFile}
    onResetSettings={resetUIState}
  />

  <AISettingsModal
    visible={showAISettings}
    onClose={() => setShowAISettings(false)}
  />

  <ParserWorkspace
    {selectedSite}
    {currentUrl}
    {uiCurrentChatId}
    {showPageViewer}
    showAIChat={$showAIChat}
    {showParserResults}
    {activeBottomTab}
    {bottomPanelHeight}
    {pageViewerWidth}
    {chatPanelWidth}
    onElementSelect={handleElementSelect}
    onPaneClick={handlePaneClick}
    onContextMenu={handleContextMenu}
    onConnect={handleConnect}
    onNodesChange={handleNodesChange}
    onResizePageViewer={(width) => setPageViewerWidth(width)}
    onResizePageViewerStart={() => setIsResizingPageViewer(true)}
    onResizePageViewerEnd={() => setIsResizingPageViewer(false)}
    onResizeChat={(width) => setChatPanelWidth(width)}
    onResizeChatStart={() => setIsResizingChatPanel(true)}
    onResizeChatEnd={() => setIsResizingChatPanel(false)}
    onChatChange={(chatId) => setCurrentChatId(chatId)}
    onCodeGeneration={onGenerateCode}
    onApplyCode={(code) => setGeneratedCode(code)}
    onTabChange={(tab) => setActiveBottomTab(tab)}
    onCodeEdit={(code) => setEditedCode(code)}
    onCodeApply={(code) => setGeneratedCode(code)}
    onCodeCancel={() => setEditedCode(generatedCode)}
    onToggleEditor={() => {}}
    onCopyCode={() => navigator.clipboard.writeText(generatedCode)}
    onToggleResultExpansion={(index) => {
      const item = parserResults[index];
      item.expanded = !item.expanded;
      setParserResults([...parserResults]);
    }}
    onClearReview={() => setAiCodeReview(null)}
    onParserSettingsChange={(settings) => {
      // Обновляем настройки только если они действительно изменились
      if (
        parserSettings.maxElements !== settings.maxElements ||
        parserSettings.timeoutSeconds !== settings.timeoutSeconds ||
        parserSettings.slowMode !== settings.slowMode ||
        parserSettings.delayPerElement !== settings.delayPerElement
      ) {
        // setParserSettings(settings); // Нужно реализовать в composable
        // Автоматически регенерируем код при изменении настроек
        if (generatedCode && nodes.length > 0) {
          generateParserCode(nodes, edges, settings, (code) => {
            setGeneratedCode(code);
          });
        }
      }
    }}
    onRunnerResults={(results: ParserResult[], diagnostics, stats) => {
      setParserResults(results);
      setParserDiagnostics(diagnostics);
      setParserExtractionStats(stats);
      setShowParserResults(true);
      setActiveBottomTab('results');
    }}
    onRunnerError={(error) => {
      setParserTestError(error);
      setShowParserResults(true);
      setActiveBottomTab('results');
    }}
    onHeightChange={(height) => setBottomPanelHeight(height)}
    onHeightChangeStart={() => setIsResizingBottomPanel(true)}
    onHeightChangeEnd={() => setIsResizingBottomPanel(false)}
    onClose={() => {
      setActiveBottomTab(null);
      setGeneratedCode('');
      setShowParserResults(false);
      setAiCodeReview(null);
    }}
  />

  {#if contextMenu}
    <ContextMenu
      x={contextMenu.x}
      y={contextMenu.y}
      onClose={() => setContextMenu(null)}
      onAddNode={handleAddNode}
    />
  {/if}

  {#if selectedElementInfo}
    <ElementSelector
      selector={selectedElementInfo.selector}
      elementInfo={selectedElementInfo.elementInfo}
      onConfirm={confirmElementSelection}
      onCancel={cancelElementSelection}
      onAutoDetect={() => autoDetectElements(selectedElementInfo, () => {
        setSelectedElementInfo(null);
        generateParserCode(nodes, edges, parserSettings, (code) => {
          setGeneratedCode(code);
        });
      })}
    />
  {/if}
</div>

<style>
  /* Стили для ParserBuilder - общие кнопки не нужны, они в дочерних компонентах */
</style>
