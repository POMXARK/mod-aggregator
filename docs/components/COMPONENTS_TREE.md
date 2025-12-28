# Дерево компонентов Mod Aggregator

Полная иерархическая структура компонентов приложения Mod Aggregator - кроссплатформенного инструмента для агрегации, управления и парсинга файлов модов.

## Полная структура компонентов

```
📁 src/components/
├── 📁 ai-chat/                          # Компоненты чата с ИИ
│   ├── ChatHeader.svelte                # Заголовок чата
│   ├── ChatInput.svelte                 # Поле ввода сообщений
│   ├── ChatList.svelte                  # Список сообщений
│   ├── ChatMessages.svelte              # Отображение сообщений
│   └── MessageActions.svelte            # Действия с сообщениями
├── 📁 collections/                      # Управление коллекциями файлов
│   ├── CollectionComposer.svelte        # Создание коллекций
│   ├── CollectionLogicBuilder.svelte    # Построение логики коллекций
│   └── CollectionViewer.svelte          # Просмотр коллекций
├── 📁 common/                           # Общие компоненты
│   └── ResizeHandle.svelte              # Обработчик изменения размеров
├── 📁 constructor/                      # Конструктор парсера
│   ├── BrowserPanel.svelte              # Панель браузера для выбора элементов
│   ├── ChatPanel.svelte                 # Панель чата с ИИ
│   ├── GeneratedCodePanel.svelte        # Панель сгенерированного кода
│   ├── NodeEditor.svelte                # Редактор узлов парсера
│   ├── 📁 parser-runner/               # Компоненты запуска парсера
│   │   ├── ParserDiagnostics.svelte     # Диагностика результатов парсинга
│   │   ├── ParserError.svelte           # Отображение ошибок
│   │   ├── ParserProgress.svelte        # Индикатор прогресса
│   │   ├── ParserResults.svelte         # Результаты парсинга
│   │   ├── ParserRunnerControls.svelte  # Элементы управления
│   │   └── ParserSettingsPanel.svelte   # Настройки парсера
│   └── ParserRunner.svelte              # Основной компонент запуска
├── 📁 dependencies/                     # Управление зависимостями файлов
│   ├── DependencyEditor.svelte          # Редактор зависимостей
│   ├── DependencyGraph.svelte           # Граф зависимостей
│   ├── DependencyWarning.svelte         # Предупреждения о зависимостях
│   └── VersionSelector.svelte           # Выбор версии файла
├── 📁 files/                            # Управление файлами
│   ├── BatchOperations.svelte           # Пакетные операции
│   ├── FileDragDrop.svelte              # Перетаскивание файлов
│   ├── FileForm.svelte                  # Форма добавления файла
│   └── FileList.svelte                  # Список файлов
├── 📁 framework/                        # Фреймворк для создания узлов
│   ├── BaseField.svelte                 # Базовое поле формы
│   └── BaseNode.svelte                  # Базовый узел парсера
├── 📁 icons/                            # SVG иконки компонентов
│   ├── ArrowPathIcon.svelte             # Иконка повтора
│   ├── BellIcon.svelte                  # Иконка уведомлений
│   ├── CheckIcon.svelte                 # Иконка подтверждения
│   ├── CodeIcon.svelte                  # Иконка кода
│   ├── CogIcon.svelte                   # Иконка настроек
│   ├── CollectionIcon.svelte            # Иконка коллекции
│   ├── FolderIcon.svelte                # Иконка папки
│   ├── HomeIcon.svelte                  # Иконка дома
│   ├── MenuIcon.svelte                  # Иконка меню
│   ├── PencilIcon.svelte                # Иконка редактирования
│   ├── PlayIcon.svelte                  # Иконка воспроизведения
│   ├── PlusIcon.svelte                  # Иконка добавления
│   ├── RefreshIcon.svelte               # Иконка обновления
│   ├── TrashIcon.svelte                 # Иконка удаления
│   └── XMarkIcon.svelte                 # Иконка закрытия
├── 📁 import-export/                    # Импорт и экспорт данных
│   ├── ExportDialog.svelte              # Диалог экспорта
│   └── ImportDialog.svelte              # Диалог импорта
├── 📁 nodes/                            # Специфические типы узлов парсера
│   ├── ExtractNode.svelte               # Узел извлечения данных
│   ├── FilterNode.svelte                # Узел фильтрации
│   ├── OutputNode.svelte                # Узел вывода результатов
│   ├── SelectorNode.svelte              # Узел селектора CSS
│   ├── TransformNode.svelte             # Узел трансформации данных
│   └── UniversalNode.svelte             # Универсальный узел
├── 📁 session/                          # Управление сессиями
│   └── SessionRestore.svelte            # Восстановление сессии
├── AIChat.svelte                        # Основной компонент чата с ИИ
├── App.svelte                           # Корневой компонент приложения
├── ContextMenu.svelte                   # Контекстное меню
├── ElementSelector.svelte               # Выбор элементов на странице
├── ModCard.svelte                       # Карточка мода
├── ModsList.svelte                      # Список модов
├── NotificationsPanel.svelte            # Панель уведомлений
├── PageIframe.svelte                    # iframe для загрузки страниц
├── PageViewer.svelte                    # Просмотрщик страниц
├── ParserBuilder.svelte                 # Основной конструктор парсера
├── ParserNode.svelte                    # Отдельный узел парсера
├── RecentUrlsDropdown.svelte            # Выпадающий список недавних URL
├── Sidebar.svelte                       # Боковая панель навигации
├── SiteForm.svelte                      # Форма добавления сайта
├── SitesManager.svelte                  # Менеджер сайтов
├── Tooltip.svelte                       # Всплывающая подсказка
└── ViewerHeader.svelte                  # Заголовок просмотрщика
```

## Архитектурные группы компонентов

### 🏠 **Ядро приложения (Core)**
- **`App.svelte`** - корневой компонент, точка входа приложения
- **`Sidebar.svelte`** - боковая панель навигации с меню
- **`NotificationsPanel.svelte`** - панель уведомлений и сообщений

### 🎯 **Управление контентом (Content Management)**
- **`ModsList.svelte`** → **`ModCard.svelte`** - список и карточки модов
- **`SitesManager.svelte`** → **`SiteForm.svelte`** - управление сайтами парсинга
- **`PageViewer.svelte`** → **`PageIframe.svelte`** + **`ViewerHeader.svelte`** - просмотр веб-страниц

### 🏗️ **Конструктор парсеров (Parser Builder)**
- **`ParserBuilder.svelte`** - главный оркестратор конструктора
- **`BrowserPanel.svelte`** - панель браузера для выбора элементов
- **`NodeEditor.svelte`** - визуальный редактор узлов (на базе SvelteFlow)
- **`ChatPanel.svelte`** - интеграция с AI для помощи в разработке
- **`GeneratedCodePanel.svelte`** - отображение и редактирование сгенерированного кода

#### **Компоненты узлов парсера (Node Components)**
- **`SelectorNode.svelte`** - выбор элементов по CSS селекторам
- **`ExtractNode.svelte`** - извлечение данных из HTML элементов
- **`FilterNode.svelte`** - фильтрация и валидация данных
- **`TransformNode.svelte`** - трансформация данных (trim, format, etc.)
- **`OutputNode.svelte`** - финализация и вывод результатов
- **`UniversalNode.svelte`** - универсальный настраиваемый узел

### 🚀 **Система запуска парсера (Parser Runner)**
```
ParserRunner.svelte (main)
├── ParserRunnerControls.svelte    # Кнопки управления (Run/Stop/Clear)
├── ParserSettingsPanel.svelte     # Настройки парсера (timeout, limits, etc.)
├── ParserProgress.svelte          # Индикатор прогресса выполнения
├── ParserError.svelte             # Отображение ошибок выполнения
├── ParserDiagnostics.svelte       # Диагностическая информация
└── ParserResults.svelte           # Результаты парсинга с экспортом
```

### 📁 **Управление файлами (File Management)**
- **`FileList.svelte`** - список файлов с поиском и фильтрацией
- **`FileForm.svelte`** - форма добавления/редактирования файлов
- **`FileDragDrop.svelte`** - drag & drop загрузка файлов
- **`BatchOperations.svelte`** - пакетные операции над файлами

### 🎨 **Управление коллекциями (Collections)**
- **`CollectionViewer.svelte`** - просмотр коллекций файлов
- **`CollectionComposer.svelte`** - создание и редактирование коллекций
- **`CollectionLogicBuilder.svelte`** - построение логики коллекций

### 🔗 **Система зависимостей (Dependencies)**
- **`DependencyEditor.svelte`** - редактор зависимостей файлов
- **`DependencyGraph.svelte`** - визуализация графа зависимостей
- **`DependencyWarning.svelte`** - предупреждения о конфликтах
- **`VersionSelector.svelte`** - выбор версий файлов

### 🤖 **ИИ и чат (AI & Chat)**
```
AIChat.svelte (main)
└── ai-chat/
    ├── ChatHeader.svelte          # Заголовок с настройками
    ├── ChatMessages.svelte        # История сообщений
    ├── ChatInput.svelte           # Поле ввода с autocomplete
    ├── ChatList.svelte            # Список активных чатов
    └── MessageActions.svelte      # Действия с сообщениями
```

### 🔄 **Импорт/Экспорт (Import/Export)**
- **`ExportDialog.svelte`** - диалог экспорта данных и настроек
- **`ImportDialog.svelte`** - диалог импорта конфигураций

### 🎭 **Фреймворк компонентов (Framework)**
- **`BaseNode.svelte`** - базовый класс для всех типов узлов
- **`BaseField.svelte`** - базовые поля форм для узлов
- **`nodes/UniversalNode.svelte`** - универсальный настраиваемый узел

### 🛠️ **Утилитарные компоненты (Utilities)**
- **`ResizeHandle.svelte`** - изменение размера панелей (drag & drop)
- **`ContextMenu.svelte`** - контекстное меню для создания узлов
- **`ElementSelector.svelte`** - выбор элементов на странице
- **`Tooltip.svelte`** - всплывающие подсказки
- **`RecentUrlsDropdown.svelte`** - выпадающий список недавних URL

### 🎨 **Иконки (Icons)**
Полный набор SVG иконок в `src/components/icons/`:
- **Навигация:** `HomeIcon`, `MenuIcon`, `ArrowPathIcon`
- **Действия:** `PlusIcon`, `TrashIcon`, `PencilIcon`, `PlayIcon`
- **Состояние:** `CheckIcon`, `BellIcon`, `XMarkIcon`
- **Типы:** `CodeIcon`, `CogIcon`, `FolderIcon`, `CollectionIcon`

## Composable функции (переиспользуемая логика)

### **Parser Engine**
- **`useParserSettings.svelte.ts`** - управление настройками парсера (timeout, limits, slow mode)
- **`useParserRunner.ts`** - логика выполнения парсера с прогрессом
- **`useParserCodeGenerator.ts`** - генерация Rust кода из визуальных узлов

### **AI Integration**
- **`useAIChat.ts`** - управление чатом с ИИ (Ollama, OpenAI, Anthropic)
- **`useAISettings.ts`** - настройки AI моделей и API ключей
- **`useChatStorage.ts`** - сохранение истории чатов

### **UI State Management**
- **`useUI.ts`** - глобальное состояние интерфейса
- **`useRecentUrls.svelte.ts`** - управление недавними URL
- **`useSelection.svelte.ts`** - выделение элементов на странице

### **Data Management**
- **`useMods.ts`** - управление коллекцией модов
- **`useSites.ts`** - управление сайтами парсинга
- **`useNotifications.ts`** - система уведомлений
- **`usePageLoader.svelte.ts`** - загрузка и кеширование страниц

## Архитектура состояний и зависимостей

### 🔄 **Потоки данных между компонентами**

#### **Главное приложение (App → Components)**
```
App.svelte
├── ↓ sites, currentPage, notifications
│   Sidebar.svelte → ↑ selectedSite, activeTab
├── ↓ selectedSite, currentUrl
│   ParserBuilder.svelte
│   ├── ↓ url, siteId, onElementSelect
│   │   BrowserPanel.svelte → PageViewer.svelte → ↑ selectedElements
│   ├── ↕ nodes, edges (bindable)
│   │   NodeEditor.svelte → ↑ nodeChanges, connections
│   ├── ↓ aiSettings, nodes, generatedCode
│   │   ChatPanel.svelte → AIChat.svelte → ↑ parserConfigs
│   └── ↓ code, nodes, url, settings
│       GeneratedCodePanel.svelte → ParserRunner.svelte → ↑ results, errors
```

#### **Внутренние потоки ParserBuilder**
- **BrowserPanel ↔ ElementSelector:** Выбор элементов → создание узлов
- **NodeEditor ↔ ContextMenu:** Правый клик → добавление узлов
- **AIChat ↔ ParserRunner:** Генерация кода → тестирование
- **GeneratedCodePanel ↔ All Panels:** Синхронизация кода и настроек

### 💾 **Управление состоянием**

#### **LocalStorage (персистентное состояние)**
- **UI State:** размеры панелей, видимость, активные вкладки
- **AI Settings:** модель, API ключи, настройки Ollama
- **Recent URLs:** история посещенных страниц
- **Chat History:** сообщения AI чатов

#### **Runtime State (в памяти)**
- **Nodes & Edges:** граф парсера (SvelteFlow)
- **Parser Results:** результаты выполнения парсера
- **Page Cache:** кешированные HTML страницы
- **Notifications:** активные уведомления

#### **Reactive State (Svelte 5 Runes)**
```typescript
// Примеры реактивных состояний
let nodes = $state<Node[]>([]);              // Узлы парсера
let currentUrl = $state('');                 // Текущий URL
let isRunning = $state(false);               // Состояние выполнения
let generatedCode = $state('');              // Сгенерированный код
let parserResults = $state<Result[]>([]);    // Результаты парсинга
```

### 🔧 **Фреймворк и переиспользуемые компоненты**

#### **Base Components (базовые классы)**
- **`BaseNode.svelte`** - абстрактный базовый класс для всех узлов парсера
- **`BaseField.svelte`** - базовые поля форм с валидацией
- **`ResizeHandle.svelte`** - универсальный компонент изменения размера

#### **Composition API (Composable функции)**
```typescript
// Примеры использования composable
const settings = useParserSettings({ initialMaxElements: 100 });
const runner = useParserRunner({ nodes, onResults: handleResults });
const chat = useAIChat({ modelType: 'ollama', onMessage: handleMessage });
```

### 📊 **Архитектурные метрики**

#### **Размеры компонентов**
- **Маленькие (< 200 строк):** 60% компонентов (icons, utilities)
- **Средние (200-500 строк):** 30% компонентов (panels, editors)
- **Большие (> 500 строк):** 10% компонентов (ParserBuilder, AIChat)

#### **Уровни абстракции**
1. **UI Components** - чистые компоненты интерфейса
2. **Business Components** - компоненты с бизнес-логикой
3. **Container Components** - компоненты-оркестраторы
4. **Framework Components** - базовые классы и утилиты

### 🔗 **Интеграции и API**

#### **Tauri Commands (Rust backend)**
```typescript
// Примеры вызовов Tauri команд
await invoke('fetch_page', { url, forceRefresh: false });
await invoke('ai_generate_parser', { html, description, modelType });
await invoke('run_parser', { code, settings, url });
```

#### **Web APIs**
- **DOM API:** манипуляция элементами, события
- **Fetch API:** загрузка страниц и API вызовы
- **LocalStorage:** персистентное хранение данных
- **File API:** работа с файлами (drag & drop)

### 📚 **См. также**

- **[PROJECT_STRUCTURE.md](../../PROJECT_STRUCTURE.md)** - полная структура всего проекта
- **[Справочник компонентов](./COMPONENTS_REFERENCE.md)** - подробное API всех компонентов
- **[src/components/README.md](../../src/components/README.md)** - краткое описание компонентов
- **[architecture/overview.md](../../docs/architecture/overview.md)** - архитектурный обзор

