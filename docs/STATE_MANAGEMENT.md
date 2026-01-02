# Система управления состоянием приложения

## Обзор

Приложение использует универсальную систему управления состоянием с автоматическим сохранением/загрузкой данных в localStorage. Система предотвращает конфликты между различными частями приложения и обеспечивает консистентность данных.

## Архитектура

### Основные компоненты

1. **StatePersistenceManager** - низкоуровневый менеджер персистентности
2. **AppStateManager** - высокоуровневый менеджер состояний приложения
3. **Composable функции** - удобные интерфейсы для компонентов

### Типы состояний

#### UI State (интерфейсное состояние)
```typescript
interface UIState {
  // Панели видимости
  showPageViewer: boolean;
  showAIChat: boolean;
  showParserResults: boolean;
  showUISettingsMenu: boolean;
  showCodeEditor: boolean;

  // Активные вкладки
  activeBottomTab: string | null;

  // Размеры панелей
  pageViewerWidth: number;
  chatPanelWidth: number;
  bottomPanelHeight: number;

  // Состояние изменения размеров
  isResizingPageViewer: boolean;
  isResizingChatPanel: boolean;
  isResizingBottomPanel: boolean;

  // Текущий чат
  currentChatId: string | null;
}
```

#### Parser State (состояние парсера)
```typescript
interface ParserState {
  // Узлы и связи
  nodes: any[];
  edges: any[];

  // Код
  generatedCode: string;
  editedCode: string;

  // Метаданные
  currentUrl: string;
  selectedSiteId: string | null;
}
```

#### AI Settings (настройки ИИ)
```typescript
interface AISettings {
  modelType: 'ollama' | 'openai' | 'anthropic' | 'google';
  modelName: string;
  apiKey: string;
  ollamaUrl: string;
  description: string;
}
```

## Использование

### В компонентах

#### UI состояние
```typescript
import { useUIState } from '@/lib/composables/useAppState';

// Для текущего URL и чата
const {
  showAIChat,
  setShowAIChat,
  pageViewerWidth,
  setPageViewerWidth,
  // ... другие геттеры и сеттеры
} = useUIState(currentUrl, currentChatId);
```

#### Parser состояние
```typescript
import { useParserState } from '@/lib/composables/useAppState';

const {
  nodes,
  setNodes,
  generatedCode,
  setGeneratedCode,
  // ... другие геттеры и сеттеры
} = useParserState(currentUrl, currentChatId);
```

#### AI настройки
```typescript
import { useAISettings } from '@/lib/composables/useAppState';

const {
  modelType,
  setModelType,
  apiKey,
  setApiKey,
  // ... другие геттеры и сеттеры
} = useAISettings();
```

### В шаблонах

```svelte
<!-- Прямое использование stores -->
<ChatPanel visible={$showAIChat} />

<!-- Через derived значения -->
{#if $showPageViewer}
  <PageViewer width={$pageViewerWidth} />
{/if}
```

## Ключевые особенности

### 1. Автоматическое сохранение

- **Debounce**: Изменения сохраняются не чаще раза в секунду
- **Контекстная зависимость**: Состояние сохраняется для конкретного URL и чата
- **Версионирование**: Поддержка миграции между версиями данных

### 2. Предотвращение конфликтов

- **Один источник правды**: Каждое состояние управляется одним менеджером
- **Изоляция**: Разные URL/чаты имеют независимые состояния
- **Транзакционность**: Изменения применяются атомарно

### 3. Производительность

- **Ленивая загрузка**: Состояния создаются только при первом обращении
- **Кеширование**: Загруженные состояния кешируются в памяти
- **Очистка**: Устаревшие данные автоматически удаляются

### 4. Надежность

- **Обработка ошибок**: Graceful fallback при проблемах с localStorage
- **Валидация**: Проверка корректности загружаемых данных
- **Восстановление**: Автоматическое восстановление поврежденных данных

## API Reference

### StatePersistenceManager

#### Создание персистентного состояния
```typescript
createPersistentState<T>(
  key: string,
  defaultValue: T,
  options?: {
    debounceMs?: number;
    serialize?: (value: T) => string;
    deserialize?: (value: string) => T;
    version?: number;
  }
): PersistentStore<T>
```

#### Создание группы состояний
```typescript
createPersistentStateGroup<T>(
  baseKey: string,
  defaultState: T,
  options?: {
    debounceMs?: number;
    version?: number;
  }
): PersistentStateGroup<T>
```

### AppStateManager

#### Получение состояний
```typescript
getUIState(url: string, chatId: string | null): PersistentStore<UIState>
getParserState(url: string, chatId: string | null): PersistentStore<ParserState>
getAISettings(): PersistentStore<AISettings>
```

#### Управление состоянием
```typescript
saveState(url: string, chatId: string | null, additionalData?: any): void
loadState(url: string, chatId: string | null): StateData | null
resetState(url: string): void
```

## Миграция с старой системы

### Старый подход (конфликтный)
```typescript
// Разные эффекты в разных компонентах
$effect(() => { saveCurrentState(url, chatId, { nodes, edges }); });
$effect(() => { saveCurrentState(url, chatId, { uiState }); });

// Ручное управление localStorage
localStorage.setItem('myState', JSON.stringify(data));
```

### Новый подход (универсальный)
```typescript
// Одна система для всего приложения
const { setNodes, setShowAIChat } = useParserState(url, chatId);
const { setShowPageViewer } = useUIState(url, chatId);

// Автоматическое сохранение без конфликтов
setNodes(newNodes); // Сохраняется автоматически
setShowAIChat(true); // Сохраняется автоматически
```

## Отладка

### Диагностика состояний
```typescript
import { appStateManager } from '@/lib/composables/useAppState';

// Получить статистику
const stats = appStateManager.getStateStats();
console.log('State stats:', stats);

// Экспортировать все состояния
const exportData = appStateManager.exportAllStates();
console.log('All states:', exportData);
```

### Очистка данных
```typescript
// Сбросить состояние для URL
appStateManager.resetState('https://example.com');

// Сбросить все состояния
appStateManager.importStates('{}');
```

## Лучшие практики

1. **Используйте composables**: Всегда используйте `useUIState`, `useParserState`, `useAISettings`
2. **Не дублируйте логику**: Не создавайте собственные эффекты сохранения
3. **Тестируйте изменения**: Проверяйте, что состояния сохраняются корректно
4. **Мониторьте производительность**: Следите за количеством сохранений в консоли

## Расширение системы

### Добавление нового типа состояния
```typescript
// 1. Определить интерфейс
interface NewState {
  property1: string;
  property2: number;
}

// 2. Добавить в AppStateManager
getNewState(context: string): PersistentStore<NewState> {
  // Логика создания/получения состояния
}

// 3. Создать composable
export function useNewState(context: string) {
  // Логика работы с состоянием
}
```

### Кастомная сериализация
```typescript
const store = usePersistentState('custom', defaultValue, {
  serialize: (value) => customSerialize(value),
  deserialize: (str) => customDeserialize(str),
  version: 2
});
```