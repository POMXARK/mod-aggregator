# Porto Architecture в Mod Aggregator

## Обзор Porto Architecture

**Porto Architecture** - это масштабируемый архитектурный паттерн для организации кода в больших приложениях. Он разделяет проект на три основных слоя, обеспечивая четкое разделение ответственности и высокую поддерживаемость.

## Три столпа Porto

### 🔲 **Containers** - Функциональные контейнеры
Независимые модули, каждый из которых представляет собой законченную функциональную область приложения.

**Структура контейнера:**
```
ContainerName/
├── UI/           # Компоненты пользовательского интерфейса
├── Data/         # Бизнес-логика и модели данных
└── Config/       # Конфигурация и настройки
```

### 🚢 **Shipments** - Переиспользуемые компоненты
Общие компоненты, сервисы и утилиты, доступные всем контейнерам.

**Структура shipments:**
```
Shipments/
├── UI/           # Общие UI компоненты (иконки, модалы, утилиты)
├── Utils/        # Вспомогательные функции
├── Services/     # Общие сервисы (composables, API клиенты)
└── Config/       # Глобальные настройки
```

### 💾 **Data** - Слой данных
Централизованный доступ к данным, моделям и репозиториям.

**Структура data:**
```
Data/
├── Models/       # TypeScript интерфейсы и типы
├── Repositories/ # Классы для работы с данными
└── DTOs/         # Объекты передачи данных
```

## Контейнеры Mod Aggregator

### **Mods Container**
Управление коллекцией модов
- **UI:** ModsList.svelte, ModCard.svelte
- **Data:** модели модов, фильтры, сортировка
- **Config:** настройки отображения модов

### **Sites Container**
Управление сайтами парсинга
- **UI:** SitesManager.svelte, SiteForm.svelte
- **Data:** модели сайтов, конфигурации парсеров
- **Config:** настройки сайтов

### **Parser Container** ⭐
Визуальный конструктор парсеров
- **UI:** ParserBuilder, NodeEditor, ParserRunner, Framework
- **Data:** модели парсеров, генерация кода
- **Config:** настройки парсера (timeout, limits)

### **Files Container**
Управление файлами модов
- **UI:** FileList, FileForm, BatchOperations
- **Data:** модели файлов, загрузка, валидация
- **Config:** настройки загрузки файлов

### **Collections Container**
Управление коллекциями файлов
- **UI:** CollectionViewer, CollectionComposer, CollectionLogicBuilder
- **Data:** модели коллекций, связи файлов
- **Config:** настройки коллекций

## Shipments Mod Aggregator

### **UI Components**
Общие компоненты интерфейса:
- **Icons:** полный набор SVG иконок
- **AIChat:** компонент чата с ИИ
- **Sidebar:** боковая панель навигации
- **ContextMenu:** контекстное меню
- **Tooltip:** подсказки
- **ResizeHandle:** изменение размеров

### **Services (Composables)**
Переиспользуемая бизнес-логика:
- **useParserSettings:** управление настройками парсера
- **useParserRunner:** выполнение парсера
- **useAIChat:** работа с ИИ
- **useSites:** управление сайтами
- **useNotifications:** уведомления

### **Utils**
Вспомогательные функции:
- **html-processor:** обработка HTML
- **selection-script:** выделение элементов
- **page-resources:** управление ресурсами

## Data Layer

### **Models**
TypeScript определения:
```typescript
// Модели данных
export interface Mod {
  id: number;
  name: string;
  description: string;
  version: string;
  // ...
}

export interface ParserConfig {
  list_selector: string;
  title_selector?: string;
  url_selector?: string;
  // ...
}
```

### **Repositories**
Классы для работы с данными:
```typescript
class ModsRepository {
  async getAll(): Promise<Mod[]> { /* ... */ }
  async create(mod: CreateModDTO): Promise<Mod> { /* ... */ }
  async update(id: number, mod: UpdateModDTO): Promise<Mod> { /* ... */ }
}
```

### **DTOs**
Объекты передачи данных:
```typescript
export interface CreateModDTO {
  name: string;
  description: string;
  fileIds: number[];
}

export interface ParserResultDTO {
  data: Record<string, any>[];
  diagnostics: Diagnostic[];
  stats: ParserStats;
}
```

## Принципы работы

### **1. Единая точка входа**
```typescript
// src/index.ts
export * from './Containers';
export * from './Shipments';
export * from './Data';
```

### **2. Импорт по слоям**
```typescript
// В App.svelte
import { ModsList, SitesManager } from '@/Containers';
import { Sidebar, AIChat } from '@/Shipments';
import type { Mod, Site } from '@/Data/Models';
```

### **3. Независимость контейнеров**
Каждый контейнер может работать автономно и тестироваться отдельно.

### **4. Переиспользование shipments**
Общие компоненты используются всеми контейнерами через импорты.

## Добавление нового контейнера

### **Шаг 1: Создание структуры**
```bash
mkdir -p src/Containers/NewFeature/{UI,Data,Config}
```

### **Шаг 2: Создание компонентов**
```typescript
// src/Containers/NewFeature/UI/NewFeatureList.svelte
<script lang="ts">
// Компонент списка
</script>

<template>
  <!-- UI компонента -->
</template>
```

### **Шаг 3: Создание индекса**
```typescript
// src/Containers/NewFeature/index.ts
export { default as NewFeatureList } from './UI/NewFeatureList.svelte';
// export * from './Data';
// export * from './Config';
```

### **Шаг 4: Регистрация в главном индексе**
```typescript
// src/Containers/index.ts
export * from './Mods';
export * from './Sites';
// ... другие контейнеры
export * from './NewFeature'; // Новый контейнер
```

## Преимущества Porto

### **🎯 Модульность**
- Четкое разделение ответственности
- Независимые контейнеры
- Легкое тестирование

### **🔄 Переиспользование**
- Общие компоненты в Shipments
- Централизованный Data слой
- DRY принципы

### **👥 Командная разработка**
- Параллельная разработка контейнеров
- Централизованная поддержка Shipments
- Четкие интерфейсы

### **📈 Масштабируемость**
- Легкое добавление новых фич
- Поддержка роста до сотен компонентов
- Архитектура не устаревает

## Миграция на Porto

Проект Mod Aggregator был успешно мигрирован на Porto архитектуру:

1. ✅ **Анализ структуры** - определены контейнеры и shipments
2. ✅ **Создание директорий** - базовая структура Porto
3. ✅ **Перемещение компонентов** - контейнеры организованы
4. ✅ **Исправление импортов** - все пути обновлены для новой структуры
5. ✅ **Тестирование сборки** - приложение компилируется без ошибок
6. ✅ **Запуск приложения** - Tauri приложение работает корректно
7. ✅ **Документация** - созданы руководства по Porto

## Dependency Injection System

Porto архитектура включает мощную систему Dependency Injection:

### 1. Path Mapping
```json
{
  "paths": {
    "@/*": ["./src/*"],
    "@/Containers/*": ["./src/Containers/*"],
    "@/Shipments/*": ["./src/Shipments/*"],
    "@/Services/*": ["./src/Shipments/Services/*"]
  }
}
```

### 2. Tauri Service Locator
Централизованное управление Tauri командами:
```typescript
import { tauriCommands } from '@/lib/tauri-services';
await tauriCommands.get_sites();
```

### 3. Porto Context
Глобальный контекст со всеми сервисами:
```typescript
import { portoContext } from '@/context/porto-context';
const { commands, services, utils } = portoContext;
```

## Следующие шаги

1. **Реализовать Data слой** - добавить репозитории и DTOs
2. **Добавить Config слой** - настройки для каждого контейнера
3. **Создать тесты** - unit и integration тесты для контейнеров
4. **Расширить DI систему** - добавить больше сервисов в контекст
5. **Документировать API** - спецификации для каждого контейнера

## Заключение

**Porto Architecture успешно внедрена и протестирована!** 🎉

### ✅ **Достигнутые результаты:**

- **Масштабируемость**: Легкое добавление новых контейнеров и функциональности
- **Поддерживаемость**: Четкая организация кода и разделение ответственности
- **Тестируемость**: Каждый контейнер можно тестировать изолированно
- **Командная разработка**: Параллельная работа над разными модулями
- **Переиспользование**: Общие компоненты доступны всем контейнерам
- **Работоспособность**: Приложение компилируется и запускается без ошибок

### 🚀 **Архитектура готова к развитию:**

- Добавляйте новые контейнеры по мере роста проекта
- Расширяйте Shipments для общих компонентов
- Реализуйте Data слой (репозитории, DTOs)
- Настройте автоматическое тестирование контейнеров

Porto Architecture обеспечивает долгосрочную поддерживаемость и масштабируемость проекта Mod Aggregator.
