# Справочник компонентов

Полное описание всех компонентов проекта с их пропсами, методами и функциями.

## Содержание

1. [Основные компоненты](#основные-компоненты)
2. [Компоненты конструктора парсеров](#компоненты-конструктора-парсеров)
3. [Вспомогательные компоненты](#вспомогательные-компоненты)
4. [Компоненты нод](#компоненты-нод)
5. [Утилитарные компоненты](#утилитарные-компоненты)

---

## Основные компоненты

### App.svelte

Главный компонент приложения, управляет роутингом и состоянием.

**Props:** Нет (корневой компонент)

**State:**
- `currentPage: Page` - текущая страница ('mods' | 'sites' | 'parser' | 'notifications' | 'files' | 'collections')
- `sites: any[]` - список всех сайтов
- `selectedSiteId: number | null` - ID выбранного сайта для фильтрации
- `sidebarOpen: boolean` - состояние боковой панели (открыта/закрыта)
- `isTauriMode: boolean` - режим работы (Tauri/браузер)
- `sessionRestored: boolean` - флаг восстановления сессии
- `sessionWarnings: string[]` - предупреждения при восстановлении сессии

**Methods:**
- `restoreSession()` - восстанавливает состояние сессии из localStorage
- `loadSites()` - загружает список сайтов из базы данных
- `handlePageChange(page: Page)` - обрабатывает смену страницы
- `handleSiteSelect(siteId: number | null)` - обрабатывает выбор сайта
- `handleSiteAdded()` - обрабатывает добавление нового сайта
- `toggleSidebar()` - переключает состояние боковой панели

**Lifecycle:**
- `onMount` - инициализация: проверка режима работы, восстановление сессии, загрузка сайтов

---

### Sidebar.svelte

Боковая панель навигации приложения.

**Props:**
- `currentPage: string` - текущая активная страница
- `sites: any[]` - список сайтов для отображения
- `selectedSiteId: number | null` - ID выбранного сайта
- `isOpen: boolean` - состояние панели (открыта/закрыта)
- `onPageChange: (page: string) => void` - callback при смене страницы
- `onSiteSelect: (siteId: number | null) => void` - callback при выборе сайта
- `onToggle: () => void` - callback при переключении состояния панели

**Methods:**
- `handlePageClick(page: string)` - обрабатывает клик по странице
- `handleSiteClick(siteId: number | null)` - обрабатывает клик по сайту

**Features:**
- Поддержка collapsed режима (только иконки)
- Tooltips при скрытом состоянии
- Адаптивные размеры через CSS переменные

---

### ModsList.svelte

Компонент для отображения списка модов.

**Props:**
- `selectedSiteId: number | null` - ID сайта для фильтрации (null = все сайты)

**State:**
- `mods: any[]` - список модов
- `loading: boolean` - состояние загрузки
- `error: string | null` - ошибка загрузки

**Methods:**
- `loadMods()` - загружает список модов из базы данных
- `handleCheckUpdates()` - проверяет обновления модов

**Lifecycle:**
- `onMount` - загрузка модов при монтировании
- `$effect` - перезагрузка модов при изменении `selectedSiteId`

**Child Components:**
- `ModCard.svelte` - карточка отдельного мода

---

### ModCard.svelte

Карточка для отображения информации о моде.

**Props:**
- `mod: any` - объект мода с полями:
  - `title: string` - название
  - `author?: string` - автор
  - `version?: string` - версия
  - `description?: string` - описание
  - `changes?: string` - изменения
  - `image_url?: string` - URL изображения
  - `url: string` - URL на сайте
  - `updated_at: string` - дата обновления

**Methods:**
- `handleOpen()` - открывает мод на сайте в браузере

---

### SitesManager.svelte

Компонент для управления сайтами (CRUD операции).

**Props:**
- `onSiteAdded: () => void` - callback после добавления/изменения сайта

**State:**
- `sites: any[]` - список сайтов
- `editingSite: any | null` - редактируемый сайт (null = новый)
- `showForm: boolean` - видимость формы редактирования
- `loading: boolean` - состояние загрузки

**Methods:**
- `loadSites()` - загружает список сайтов
- `handleDelete(id: number)` - удаляет сайт с подтверждением
- `handleEdit(site: any)` - открывает форму редактирования
- `handleAdd()` - открывает форму для нового сайта
- `handleFormClose()` - закрывает форму
- `handleFormSubmit()` - обрабатывает сохранение сайта

**Child Components:**
- `SiteForm.svelte` - форма редактирования/создания сайта

---

### SiteForm.svelte

Форма для создания и редактирования сайта.

**Props:**
- `site: any | null` - объект сайта для редактирования (null = новый сайт)
- `onClose: () => void` - callback закрытия формы
- `onSubmit: () => void` - callback сохранения формы

**State:**
- `name: string` - название сайта
- `url: string` - URL сайта
- `listSelector: string` - CSS селектор списка элементов
- `titleSelector: string` - CSS селектор заголовка
- `urlSelector: string` - CSS селектор URL
- `versionSelector: string` - CSS селектор версии
- `authorSelector: string` - CSS селектор автора
- `imageSelector: string` - CSS селектор изображения
- `baseUrl: string` - базовый URL
- `listUrl: string` - URL списка

**Methods:**
- `handleSubmit()` - валидирует и сохраняет сайт через Tauri команду `create_site` или `update_site`
- `handleCancel()` - закрывает форму без сохранения

---

### NotificationsPanel.svelte

Панель уведомлений о новых версиях модов.

**Props:** Нет

**State:**
- `notifications: any[]` - список уведомлений
- `loading: boolean` - состояние загрузки
- `unreadCount: number` - количество непрочитанных (computed)

**Methods:**
- `loadNotifications()` - загружает уведомления из базы данных
- `handleMarkRead(id: number)` - отмечает уведомление как прочитанное
- `handleMarkAllRead()` - отмечает все уведомления как прочитанные

**Lifecycle:**
- `onMount` - загрузка уведомлений и настройка автообновления каждые 30 секунд

---

## Компоненты конструктора парсеров

### ParserBuilder.svelte

Главный компонент визуального конструктора парсеров.

**Props:** Нет (используется напрямую в App.svelte)

**State:**

Узлы и связи:
- `nodes: Node[]` - массив узлов графа
- `edges: Edge[]` - массив связей между узлами

Сайт и URL:
- `selectedSite: any | null` - выбранный сайт
- `sites: any[]` - список сайтов
- `currentUrl: string` - текущий URL страницы
- `savedSelectedSiteId: number | null` - сохраненный ID сайта для восстановления

Интерфейс:
- `showPageViewer: boolean` - видимость панели браузера
- `pageViewerWidth: number` - ширина панели браузера (пиксели)
- `chatPanelWidth: number` - ширина панели чата (пиксели)
- `bottomPanelHeight: number` - высота нижней панели
- `activeBottomTab: 'code' | 'results' | 'review' | 'runner' | null` - активная вкладка
- `showCodeEditor: boolean` - режим редактирования кода
- `contextMenu: { x: number; y: number } | null` - позиция контекстного меню

Код и результаты:
- `generatedCode: string` - сгенерированный код парсера
- `editedCode: string` - отредактированный код
- `parserResults: Array<{ data: Record<string, any>; expanded: boolean }>` - результаты парсинга
- `parserTestError: string | null` - ошибка тестирования
- `parserDiagnostics: any[]` - диагностика парсера
- `parserExtractionStats: any` - статистика извлечения
- `showParserResults: boolean` - видимость результатов

AI интеграция:
- `aiModelType: 'ollama' | 'openai' | 'anthropic' | 'google'` - тип AI модели
- `aiModelName: string` - название модели
- `aiApiKey: string` - API ключ
- `aiOllamaUrl: string` - URL Ollama сервера
- `aiDescription: string` - описание задачи для AI
- `showAIChat: boolean` - видимость чата AI
- `showAISettings: boolean` - видимость настроек AI
- `isAIGenerating: boolean` - состояние генерации AI
- `aiCodeReview: string | null` - отзыв AI по коду
- `isCheckingCodeWithAI: boolean` - состояние проверки кода AI
- `ollamaModels: string[]` - список доступных моделей Ollama
- `ollamaStatus: 'checking' | 'connected' | 'error' | null` - статус подключения Ollama
- `ollamaError: string | null` - ошибка подключения Ollama

Настройки парсера:
- `parserMaxElements: number` - максимальное количество элементов (по умолчанию 100)
- `parserTimeoutSeconds: number` - таймаут выполнения (по умолчанию 60)
- `parserSlowMode: boolean` - медленный режим (по умолчанию false)
- `parserDelayPerElement: number` - задержка между элементами в медленном режиме (мс, по умолчанию 500)

Выбор элементов:
- `selectedElementInfo: { selector: string; elementInfo: {...} } | null` - информация о выбранном элементе

**Methods:**

Управление узлами:
- `handleAddNode(type: string)` - добавляет новый узел указанного типа
- `handleConnect(connection: Connection)` - создает связь между узлами
- `handleNodesChange(changes: any[])` - обрабатывает изменения узлов (позиции)
- `handleEdgesChange(changes: any[])` - обрабатывает изменения связей
- `handleDeleteSelected()` - удаляет выбранные узлы и связанные связи
- `getNodeLabel(type: string): string` - возвращает метку для типа узла

Работа со страницей:
- `handleElementSelect(selector, element, elementData)` - обрабатывает выбор элемента на странице
- `confirmElementSelection()` - подтверждает выбор элемента и создает узел селектора
- `cancelElementSelection()` - отменяет выбор элемента
- `autoDetectElements()` - автоматически определяет похожие элементы
- `detectWithAI(selector, elementInfo)` - использует AI для определения элементов
- `simpleAutoDetect()` - простая автоматическая детекция
- `highlightElementsOnPage(selector)` - подсвечивает элементы на странице

Генерация кода:
- `generateParserCode()` - генерирует Rust код парсера из узлов с учетом настроек
- `generateParserConfig()` - генерирует JSON конфигурацию парсера из узлов

AI функционал:
- `checkOllama()` - проверяет подключение к Ollama
- `handleAIModelTypeChange()` - обрабатывает смену типа AI модели
- `checkCodeWithAI()` - проверяет код с помощью AI (отправляет JSON конфиг в чат, затем Rust код на проверку)
- `handleAIGenerate()` - генерирует парсер с помощью AI
- `createNodesFromAIConfig(config)` - создает узлы из AI конфигурации

Тестирование парсера:
- `handleTestParser()` - тестирует парсер на текущей странице

Сохранение/загрузка:
- `handleLoadSite()` - загружает сайт и его парсер конфигурацию
- `handleSaveParser()` - сохраняет парсер конфигурацию в базу данных

Управление состоянием UI:
- `saveUIState()` - сохраняет состояние интерфейса в localStorage
- `restoreUIState()` - восстанавливает состояние интерфейса из localStorage
- `exportUIState(): string` - экспортирует состояние UI в JSON
- `importUIState(jsonString: string): boolean` - импортирует состояние UI из JSON
- `applyUIState(state: any)` - применяет состояние UI
- `resetUIState()` - сбрасывает состояние UI на значения по умолчанию
- `exportUIStateToFile()` - экспортирует состояние UI в файл
- `importUIStateFromFile()` - импортирует состояние UI из файла
- `migrateUIState(state, fromVersion, toVersion)` - мигрирует состояние UI между версиями
- `debouncedSaveUIState()` - сохраняет состояние UI с задержкой

Утилиты:
- `handleNodeClick(event)` - обрабатывает клик по узлу
- `handlePaneClick(event)` - обрабатывает клик по области графа
- `handleContextMenu(event)` - обрабатывает контекстное меню (правый клик)
- `handleMouseMove(e)` - обрабатывает движение мыши при изменении размеров
- `handleMouseUp(e)` - обрабатывает отпускание мыши при изменении размеров
- `loadSites()` - загружает список сайтов

**Lifecycle:**
- `onMount` - инициализация: загрузка сайтов, восстановление состояния UI, проверка Ollama, настройка callbacks
- `$effect` - автогенерация кода при изменении узлов/связей, автооткрытие браузера при выборе сайта, сохранение состояния UI

**Child Components:**
- `BrowserPanel.svelte` - панель браузера
- `NodeEditor.svelte` - редактор узлов (SvelteFlow)
- `ChatPanel.svelte` - панель AI чата
- `GeneratedCodePanel.svelte` - панель сгенерированного кода
- `ContextMenu.svelte` - контекстное меню для добавления узлов
- `ElementSelector.svelte` - селектор элементов

---

### BrowserPanel.svelte

Панель браузера с просмотром страниц.

**Props:**
- `visible: boolean` (bindable) - видимость панели
- `width: number` (bindable) - ширина панели (пиксели)
- `url: string` (bindable) - URL страницы
- `siteId?: number | null` - ID сайта для кеширования
- `onElementSelect?: (selector, element, data) => void` - callback выбора элемента
- `onResize?: (newWidth: number) => void` - callback изменения размера
- `onResizeStart?: () => void` - callback начала изменения размера
- `onResizeEnd?: () => void` - callback окончания изменения размера

**Methods:**
- `handleResize(newWidth: number)` - обрабатывает изменение размера

**Child Components:**
- `PageViewer.svelte` - компонент просмотра страниц
- `ResizeHandle.svelte` - ручка изменения размера

---

### NodeEditor.svelte

Редактор узлов на основе SvelteFlow.

**Props:**
- `nodes: Node[]` (bindable) - массив узлов
- `edges: Edge[]` (bindable) - массив связей
- `withViewer?: boolean` - наличие панели браузера (для стилей)
- `withChat?: boolean` - наличие панели чата (для стилей)
- `withResults?: boolean` - наличие панели результатов (для стилей)
- `onConnect?: (connection: Connection) => void` - callback создания связи
- `onNodesChange?: (changes: any[]) => void` - callback изменения узлов
- `onEdgesChange?: (changes: any[]) => void` - callback изменения связей
- `onPaneClick?: (event: MouseEvent) => void` - callback клика по области
- `onPaneContextMenu?: (event: MouseEvent) => void` - callback контекстного меню
- `style?: string` - дополнительные стили

**Child Components:**
- `SelectorNode.svelte` - узел селектора
- `ExtractNode.svelte` - узел извлечения
- `FilterNode.svelte` - узел фильтрации
- `TransformNode.svelte` - узел трансформации
- `OutputNode.svelte` - узел вывода

---

### ChatPanel.svelte

Панель AI чата для генерации парсеров.

**Props:**
- `visible: boolean` (bindable) - видимость панели
- `width: number` (bindable) - ширина панели
- `aiModelType?: 'ollama' | 'openai' | 'anthropic' | 'google'` - тип AI модели
- `aiModelName?: string` - название модели
- `aiApiKey?: string` - API ключ
- `aiOllamaUrl?: string` - URL Ollama сервера
- `currentUrl?: string` - текущий URL страницы
- `nodes?: Node[]` - узлы графа
- `edges?: Edge[]` - связи графа
- `onCreateNodes?: (config: any) => void` - callback создания узлов из конфига
- `onGenerateCode?: () => void` - callback генерации кода
- `onTestParser?: () => Promise<void>` - callback тестирования парсера
- `onCheckCodeWithAI?: () => Promise<void>` - callback проверки кода AI
- `generatedCode?: string` - сгенерированный код
- `onApplyCode?: (code: string) => void` - callback применения кода
- `selectedElementInfo?: {...} | null` - информация о выбранном элементе
- `addAIMessage?: (content: string) => void` (bindable) - функция добавления сообщения AI
- `onResize?: (newWidth: number) => void` - callback изменения размера
- `onResizeStart?: () => void` - callback начала изменения размера
- `onResizeEnd?: () => void` - callback окончания изменения размера

**Child Components:**
- `AIChat.svelte` - компонент AI чата
- `ResizeHandle.svelte` - ручка изменения размера

---

### GeneratedCodePanel.svelte

Панель для отображения сгенерированного кода, результатов и запуска парсера.

**Props:**

Отображение:
- `visible: boolean` (bindable) - видимость панели
- `height: number` (bindable) - высота панели
- `activeBottomTab: 'code' | 'results' | 'review' | 'runner' | null` (bindable) - активная вкладка

Код:
- `generatedCode: string` - сгенерированный код
- `editedCode: string` (bindable) - отредактированный код
- `showCodeEditor: boolean` (bindable) - режим редактирования

Результаты:
- `showParserResults: boolean` - видимость результатов
- `parserResults: Array<{ data: Record<string, any>; expanded: boolean }>` - результаты парсинга
- `parserTestError: string | null` - ошибка тестирования
- `parserDiagnostics: any[]` - диагностика
- `parserExtractionStats: any` - статистика извлечения

AI:
- `aiCodeReview: string | null` - отзыв AI по коду
- `isCheckingCodeWithAI: boolean` - состояние проверки AI
- `aiApiKey?: string` - API ключ AI
- `aiModelType?: 'ollama' | 'openai' | 'anthropic' | 'google'` - тип AI модели

Парсер:
- `nodes?: Node[]` - узлы графа
- `edges?: Edge[]` - связи графа
- `currentUrl?: string` - текущий URL
- `siteId?: number | null` - ID сайта

Настройки парсера:
- `parserMaxElements?: number` - максимальное количество элементов
- `parserTimeoutSeconds?: number` - таймаут выполнения
- `parserSlowMode?: boolean` - медленный режим
- `parserDelayPerElement?: number` - задержка между элементами
- `onParserSettingsChange?: (settings) => void` - callback изменения настроек

Callbacks:
- `onHeightChange?: (newHeight: number) => void` - изменение высоты
- `onHeightChangeStart?: () => void` - начало изменения размера
- `onHeightChangeEnd?: () => void` - окончание изменения размера
- `onClose?: () => void` - закрытие панели
- `onTabChange?: (tab) => void` - смена вкладки
- `onCodeEdit?: (code: string) => void` - редактирование кода
- `onCodeApply?: (code: string) => void` - применение кода
- `onCodeCancel?: () => void` - отмена редактирования
- `onToggleEditor?: () => void` - переключение режима редактора
- `onCopyCode?: () => void` - копирование кода
- `onCheckCodeWithAI?: () => void` - проверка кода AI
- `onToggleResultExpansion?: (index: number) => void` - раскрытие результата
- `onClearReview?: () => void` - очистка отзыва AI
- `onRunnerResults?: (results, diagnostics, stats) => void` - результаты запуска парсера
- `onRunnerError?: (error: string) => void` - ошибка запуска парсера

**Methods:**
- `handleHeightChange(newHeight: number)` - обрабатывает изменение высоты
- `handleClose()` - обрабатывает закрытие панели
- `handleTabChange(tab)` - обрабатывает смену вкладки
- `handleCodeEdit()` - обрабатывает редактирование кода
- `handleCodeApply()` - обрабатывает применение кода
- `handleCodeCancel()` - обрабатывает отмену редактирования
- `handleToggleEditor()` - переключает режим редактора
- `handleCopyCode()` - копирует код в буфер обмена

**Child Components:**
- `ParserRunner.svelte` - компонент запуска парсера
- `ResizeHandle.svelte` - ручка изменения размера

---

### ParserRunner.svelte

Компонент для запуска парсера с настройками и отображением результатов.

**Props:**
- `nodes: Node[]` - узлы графа парсера
- `edges: Edge[]` - связи графа
- `currentUrl: string` - URL страницы для парсинга
- `siteId?: number | null` - ID сайта
- `onResults?: (results, diagnostics, stats) => void` - callback результатов
- `onError?: (error: string) => void` - callback ошибки
- `initialMaxElements?: number` - начальное максимальное количество элементов
- `initialTimeoutSeconds?: number` - начальный таймаут
- `initialSlowMode?: boolean` - начальный режим медленного выполнения
- `initialDelayPerElement?: number` - начальная задержка между элементами
- `onSettingsChange?: (settings) => void` - callback изменения настроек

**State:**
- `isRunning: boolean` - состояние выполнения парсера
- `progress: string` - текст прогресса
- `error: string | null` - ошибка выполнения
- `results: any[]` - результаты парсинга
- `diagnostics: any[]` - диагностика выполнения
- `extractionStats: any` - статистика извлечения
- `maxElements: number` - максимальное количество элементов
- `timeoutSeconds: number` - таймаут выполнения (секунды)
- `slowMode: boolean` - медленный режим
- `delayPerElement: number` - задержка между элементами (мс)
- `showSettings: boolean` - видимость настроек
- `isCancelled: boolean` - флаг отмены выполнения

**Methods:**
- `runParser()` - запускает парсер с настройками таймаута и ограничений
- `stopParser()` - останавливает выполнение парсера (AbortController)
- `clearResults()` - очищает результаты и ошибки
- `resetSettings()` - сбрасывает настройки на значения по умолчанию
- `notifySettingsChange()` - уведомляет об изменении настроек

**Features:**
- Поддержка ограничения количества элементов
- Поддержка таймаута выполнения
- Медленный режим с задержкой между элементами
- Остановка выполнения через AbortController
- Детальная диагностика выполнения
- Статистика извлечения данных

---

## Вспомогательные компоненты

### PageViewer.svelte

Компонент для просмотра веб-страниц в iframe с поддержкой выбора элементов.

**Props:**
- `url: string` (bindable) - URL страницы
- `siteId?: number | null` - ID сайта для кеширования
- `onElementSelect?: (selector, element, data) => void` - callback выбора элемента
- `onRefreshCache?: () => void` - callback обновления кеша

**Methods:**
- `loadPage(forceRefresh?: boolean)` - загружает страницу с учетом кеша
- `embedResources()` - встраивает внешние ресурсы (CSS, изображения) как base64
- `getSelectionScriptContent()` - возвращает JavaScript код для выделения элементов
- `getSelectionStyles()` - возвращает CSS стили для рамки выделения

**Child Components:**
- `PageIframe.svelte` - iframe для отображения страницы
- `ViewerHeader.svelte` - заголовок с кнопками управления

---

### ViewerHeader.svelte

Заголовок компонента просмотра страниц.

**Props:**
- `url: string` - текущий URL
- `onUrlChange?: (url: string) => void` - callback изменения URL
- `onRefreshCache?: () => void` - callback обновления кеша

**Child Components:**
- `RecentUrlsDropdown.svelte` - выпадающий список недавних URL

---

### AIChat.svelte

Компонент AI чата для взаимодействия с языковой моделью.

**Props:**
- `aiModelType?: 'ollama' | 'openai' | 'anthropic' | 'google'` - тип AI модели
- `aiModelName?: string` - название модели
- `aiApiKey?: string` - API ключ
- `aiOllamaUrl?: string` - URL Ollama сервера
- `currentUrl?: string` - текущий URL страницы
- `nodes?: Node[]` - узлы графа
- `edges?: Edge[]` - связи графа
- `onCreateNodes?: (config: any) => void` - callback создания узлов
- `selectedElementInfo?: {...} | null` - информация о выбранном элементе
- `onGenerateCode?: () => void` - callback генерации кода
- `onTestParser?: () => Promise<void>` - callback тестирования парсера
- `onCheckCodeWithAI?: () => Promise<void>` - callback проверки кода
- `generatedCode?: string` - сгенерированный код
- `onApplyCode?: (code: string) => void` - callback применения кода
- `addAIMessage?: (content: string) => void` (bindable) - функция добавления сообщения

**State:**
- `chats: Chat[]` - список чатов (из localStorage)
- `currentChatId: string | null` - ID текущего чата
- `messages: Message[]` - сообщения текущего чата
- `inputMessage: string` - текст ввода
- `isSending: boolean` - состояние отправки
- `attachedElement: {...} | null` - прикрепленный элемент
- `showChatList: boolean` - видимость списка чатов

**Methods:**
- `sendMessage()` - отправляет сообщение пользователя и получает ответ AI
- `createNewChat()` - создает новый чат
- `selectChat(chatId: string)` - выбирает чат
- `deleteChat(chatId: string)` - удаляет чат
- `updateMessagesFromChat()` - обновляет сообщения из текущего чата
- `extractJSONConfigFromMessage(content: string)` - извлекает JSON конфигурацию из сообщения
- `restoreConfigFromMessage(messageId: string)` - восстанавливает конфигурацию из сообщения
- `generateCodeFromMessage(messageId: string)` - генерирует код из сообщения
- `applyCodeFix(messageId: string)` - применяет исправление кода из сообщения
- `runParserFromChat()` - запускает парсер из чата

**Features:**
- Поддержка нескольких чатов
- Сохранение чатов в localStorage
- Извлечение JSON конфигураций из сообщений AI
- Восстановление конфигурации парсера из чата
- Применение исправлений кода из чата

---

## Компоненты нод

### SelectorNode.svelte

Узел для выбора элементов на странице (CSS селектор).

**Props:**
- `data: any` - данные узла
  - `label: string` - метка узла
  - `selector: string` - CSS селектор

**Methods:**
- `handleSelectorChange(value: string)` - обновляет селектор

---

### ExtractNode.svelte

Узел для извлечения данных из элементов.

**Props:**
- `data: any` - данные узла
  - `label: string` - метка узла
  - `attribute: string` - атрибут для извлечения ('text', 'href', 'src', и т.д.)
  - `selector?: string` - дополнительный селектор

**Methods:**
- `handleAttributeChange(value: string)` - обновляет атрибут

---

### FilterNode.svelte

Узел для фильтрации данных по условиям.

**Props:**
- `data: any` - данные узла
  - `label: string` - метка узла
  - `condition: string` - условие фильтрации
  - `operator: string` - оператор ('contains', 'equals', и т.д.)

**Methods:**
- `handleConditionChange(value: string)` - обновляет условие
- `handleOperatorChange(value: string)` - обновляет оператор

---

### TransformNode.svelte

Узел для трансформации данных.

**Props:**
- `data: any` - данные узла
  - `label: string` - метка узла
  - `function: string` - функция трансформации ('trim', 'uppercase', и т.д.)

**Methods:**
- `handleFunctionChange(value: string)` - обновляет функцию

---

### OutputNode.svelte

Узел вывода результата парсинга.

**Props:**
- `data: any` - данные узла
  - `label: string` - метка узла

---

## Утилитарные компоненты

### ResizeHandle.svelte

Универсальный компонент для изменения размера панелей.

**Props:**
- `direction: 'vertical' | 'horizontal'` - направление изменения размера
- `mode?: 'left' | 'right' | 'top' | 'bottom'` - режим изменения (по умолчанию 'left')
- `minValue?: number` - минимальное значение (по умолчанию 100)
- `maxValue?: number` - максимальное значение (по умолчанию Infinity)
- `getCurrentValue: () => number` - функция получения текущего значения
- `onResize: (newValue: number) => void` - callback изменения размера
- `onResizeStart?: () => void` - callback начала изменения
- `onResizeEnd?: () => void` - callback окончания изменения
- `className?: string` - дополнительные CSS классы
- `title?: string` - подсказка

**Methods:**
- `startResizing(e: PointerEvent)` - начинает изменение размера
- `stopResizing()` - останавливает изменение размера
- `handlePointerMove(e: PointerEvent)` - обрабатывает движение мыши
- `handlePointerUp(e: PointerEvent)` - обрабатывает отпускание мыши
- `handlePointerCancel(e: PointerEvent)` - обрабатывает отмену события

**Features:**
- Использует Pointer Events API для надежной работы
- `setPointerCapture` для захвата событий при выходе за пределы элемента
- Поддержка ограничений min/max
- Автоматическая очистка обработчиков событий

---

### ContextMenu.svelte

Контекстное меню для добавления узлов в конструктор.

**Props:**
- `x: number` - X координата меню
- `y: number` - Y координата меню
- `onAddNode?: (type: string) => void` - callback добавления узла
- `onClose?: () => void` - callback закрытия меню

**Methods:**
- `handleAddNode(type: string)` - обрабатывает добавление узла и закрывает меню

---

### ElementSelector.svelte

Компонент для выбора элементов на странице с предпросмотром.

**Props:**
- `selector: string` - CSS селектор
- `elementInfo?: {...}` - информация об элементе
- `onConfirm?: () => void` - callback подтверждения выбора
- `onCancel?: () => void` - callback отмены выбора

---

### Tooltip.svelte

Компонент подсказок для элементов интерфейса.

**Props:**
- `text: string` - текст подсказки
- `position?: 'top' | 'right' | 'bottom' | 'left'` - позиция подсказки
- `show?: boolean` - показывать ли подсказку

---

## Иконки

Все иконки находятся в `src/components/icons/`:

- `HomeIcon.svelte` - иконка дома (моды)
- `CogIcon.svelte` - иконка настроек (сайты)
- `CodeIcon.svelte` - иконка кода (конструктор)
- `BellIcon.svelte` - иконка уведомлений
- `FolderIcon.svelte` - иконка папки (файлы)
- `CollectionIcon.svelte` - иконка коллекций
- `PlusIcon.svelte` - иконка плюса (добавить)
- `TrashIcon.svelte` - иконка корзины (удалить)
- `PencilIcon.svelte` - иконка карандаша (редактировать)
- `PlayIcon.svelte` - иконка воспроизведения (запустить)
- `CheckIcon.svelte` - иконка галочки (подтвердить)
- `XMarkIcon.svelte` - иконка крестика (закрыть)
- `MenuIcon.svelte` - иконка меню
- `RefreshIcon.svelte` - иконка обновления
- `ArrowPathIcon.svelte` - иконка обновления (альтернативная)

Все иконки используют SVG и принимают проп `class` для стилизации.

---

## Composables (Переиспользуемая логика)

### useParserSettings.ts

Composable для управления настройками парсера.

**Параметры:**
- `initialMaxElements?: number` - начальное максимальное количество элементов (по умолчанию 100)
- `initialTimeoutSeconds?: number` - начальный таймаут выполнения (по умолчанию 60)
- `initialSlowMode?: boolean` - начальный режим медленной обработки (по умолчанию false)
- `initialDelayPerElement?: number` - начальная задержка между элементами (по умолчанию 500)
- `onSettingsChange?: (settings: ParserSettings) => void` - callback для уведомления об изменении настроек

**Возвращает:**
- `maxElements: number` - максимальное количество элементов
- `timeoutSeconds: number` - таймаут выполнения
- `slowMode: boolean` - медленный режим
- `delayPerElement: number` - задержка между элементами
- `resetSettings()` - сбрасывает настройки на значения по умолчанию
- `notifySettingsChange()` - уведомляет об изменении настроек

### useParserRunner.ts

Composable для запуска парсера и управления его выполнением.

**Параметры:**
- `nodes: Node[]` - узлы графа парсера
- `edges: Edge[]` - связи между узлами
- `currentUrl: string` - URL текущей страницы
- `siteId?: number | null` - ID сайта
- `getSettings: () => ParserSettings` - функция для получения текущих настроек
- `onResults?: (results, diagnostics, stats) => void` - callback для результатов
- `onError?: (error: string) => void` - callback для ошибок

**Возвращает:**
- `isRunning: boolean` - флаг выполнения парсера
- `progress: string` - текст прогресса
- `error: string | null` - ошибка выполнения
- `results: ParserResult[]` - результаты парсинга
- `diagnostics: any[]` - диагностика выполнения
- `extractionStats: any | null` - статистика извлечения
- `runParser()` - запускает парсер
- `stopParser()` - останавливает парсер
- `clearResults()` - очищает результаты

### useParserCodeGenerator.ts

Composable для генерации Rust кода парсера из узлов графа.

**Параметры:**
- `nodes: () => Node[]` - функция для получения узлов
- `edges: () => Edge[]` - функция для получения связей
- `settings: () => ParserCodeGeneratorSettings` - функция для получения настроек

**Возвращает:**
- `generateParserCode(): string` - генерирует Rust код парсера
- `generateParserConfig(): any` - генерирует JSON конфигурацию парсера

### useChatStorage.ts

Composable для управления хранением чатов в localStorage.

**Функции:**
- `loadChats(): Chat[]` - загружает чаты из localStorage
- `saveChats(chats: Chat[]): void` - сохраняет чаты в localStorage
- `createNewChat(chats: Chat[]): Chat` - создает новый чат

### useAIChat.ts

Composable для работы с AI чатом.

**Параметры:**
- `options: () => AIChatOptions` - функция для получения настроек AI

**Возвращает:**
- `sendMessageToAI(messageHistory, messageText, elementInfo?)` - отправляет сообщение в AI
- `extractJSONFromResponse(response)` - извлекает JSON из ответа AI
- `extractCodeFromMessage(content)` - извлекает код из сообщения
- `extractJSONConfigFromMessage(content)` - извлекает JSON конфигурацию из сообщения

---

## Компоненты ParserRunner

### ParserRunnerControls.svelte

Компонент с кнопками управления парсером.

**Props:**
- `isRunning: boolean` - флаг выполнения парсера
- `canRun: boolean` - можно ли запустить парсер
- `hasResults: boolean` - есть ли результаты
- `hasError: boolean` - есть ли ошибка
- `hasDiagnostics: boolean` - есть ли диагностика
- `showSettings: boolean` - показывать ли настройки
- `onRun: () => void` - callback для запуска
- `onStop: () => void` - callback для остановки
- `onClear: () => void` - callback для очистки
- `onToggleSettings: () => void` - callback для переключения настроек
- `runDisabledReason?: string` - причина, по которой парсер нельзя запустить

### ParserSettingsPanel.svelte

Панель настроек парсера.

**Props:**
- `settings: ParserSettings` - текущие настройки
- `isRunning: boolean` - флаг выполнения парсера
- `onSettingsChange: (settings: ParserSettings) => void` - callback для изменения настроек
- `onReset: () => void` - callback для сброса настроек

### ParserProgress.svelte

Индикатор прогресса выполнения парсера.

**Props:**
- `progress: string` - текст прогресса
- `isRunning: boolean` - флаг выполнения

### ParserError.svelte

Отображение ошибок выполнения парсера.

**Props:**
- `error: string | null` - текст ошибки
- `isRunning: boolean` - флаг выполнения

### ParserDiagnostics.svelte

Отображение диагностики выполнения парсера.

**Props:**
- `diagnostics: Diagnostic[]` - массив диагностических сообщений

### ParserResults.svelte

Отображение результатов парсинга.

**Props:**
- `results: ParserResult[]` - массив результатов
- `extractionStats: any | null` - статистика извлечения
- `onToggleExpand: (index: number) => void` - callback для разворачивания/сворачивания результата
- `onUpdateResult?: (index, updater) => void` - callback для обновления результата

---

## Дополнительные компоненты

### FileList.svelte

Компонент для управления файлами.

**Child Components:**
- `FileForm.svelte` - форма создания/редактирования файла
- `FileDragDrop.svelte` - компонент drag-and-drop для файлов
- `BatchOperations.svelte` - компонент пакетных операций

---

### CollectionViewer.svelte

Компонент для просмотра коллекций.

**Child Components:**
- `CollectionComposer.svelte` - композитор коллекций
- `CollectionLogicBuilder.svelte` - построитель логики коллекций

---

## Примечания

1. Все компоненты используют Svelte 5 runes (`$state`, `$derived`, `$effect`, `$props`)
2. `bindable()` используется для двухстороннего связывания пропсов
3. Компоненты используют TypeScript для типизации
4. Состояние UI сохраняется в localStorage через `saveUIState()` / `restoreUIState()`
5. Tauri команды вызываются через `invoke()` из `tauri-wrapper`

