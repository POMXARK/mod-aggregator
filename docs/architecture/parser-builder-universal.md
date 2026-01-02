# Универсальный конструктор парсеров

## Проблема

В текущей реализации каждый тип ноды имеет свой компонент с дублирующимся кодом:
- Одинаковая структура (Handle, node-header, node-content)
- Одинаковые стили
- Одинаковая логика синхронизации состояния
- Одинаковая структура полей конфигурации

## Решение: Configuration-Driven подход

Используем подход, как в CRM дашбордах - определяем поля через конфигурацию, а не через отдельные компоненты.

### Преимущества

1. **Меньше бойлерплейта** - один универсальный компонент вместо 5+
2. **Легче добавлять новые типы нод** - просто добавить конфигурацию
3. **Единообразный UI** - все ноды выглядят одинаково
4. **Условное отображение полей** - легко реализовать через конфигурацию
5. **Валидация** - централизованная через конфигурацию

## Структура

```
src/
├── lib/
│   ├── node-configs.ts      # Конфигурации всех типов нод
│   └── node-factory.ts      # Фабрика для создания нод
├── components/
│   ├── nodes/
│   │   └── UniversalNode.svelte  # Универсальный компонент ноды
│   └── fields/
│       └── FormField.svelte      # Универсальный компонент поля
```

## Использование

### 1. Определение конфигурации ноды

```typescript
// src/lib/node-configs.ts
export const nodeConfigs: Record<string, NodeTypeConfig> = {
  selector: {
    type: 'selector',
    label: 'Selector',
    color: '#0ea5e9',
    fields: [
      {
        key: 'selector',
        type: 'text',
        label: 'CSS Селектор',
        placeholder: '.mod-item',
        validation: { required: true },
      },
    ],
  },
  // ...
};
```

### 2. Создание ноды

```typescript
import { createNode } from '../lib/node-factory';

// Вместо большого switch/case
const newNode = createNode('selector', { x: 100, y: 100 });
nodes = [...nodes, newNode];
```

### 3. Регистрация в ParserBuilder

```typescript
// Вместо отдельных компонентов для каждого типа
const nodeTypes: NodeTypes = {
  universal: UniversalNode, // Один компонент для всех типов
};
```

## Примеры конфигураций

### Простая нода (Selector)

```typescript
selector: {
  type: 'selector',
  label: 'Selector',
  fields: [
    {
      key: 'selector',
      type: 'text',
      label: 'CSS Селектор',
      validation: { required: true },
    },
  ],
}
```

### Нода с условными полями (Extract)

```typescript
extract: {
  type: 'extract',
  label: 'Extract',
  fields: [
    {
      key: 'attribute',
      type: 'select',
      label: 'Атрибут',
      options: [
        { value: 'text', label: 'Текст' },
        { value: 'data-*', label: 'Data атрибут' },
      ],
    },
    {
      key: 'dataAttribute',
      type: 'text',
      label: 'Data атрибут',
      showIf: { field: 'attribute', value: 'data-*' }, // Условное отображение
    },
  ],
}
```

### Нода с валидацией (Filter)

```typescript
filter: {
  type: 'filter',
  label: 'Filter',
  fields: [
    {
      key: 'operator',
      type: 'select',
      label: 'Оператор',
      options: [
        { value: 'contains', label: 'Содержит' },
        { value: 'equals', label: 'Равно' },
      ],
    },
    {
      key: 'condition',
      type: 'text',
      label: 'Условие',
      validation: {
        required: true,
        min: 1,
      },
    },
  ],
}
```

## Миграция

### Старый подход

```svelte
<!-- SelectorNode.svelte -->
<script>
  let selector = $state(data.selector || '');
  // ... много бойлерплейта
</script>
<div class="selector-node">
  <!-- дублирующаяся структура -->
</div>
```

### Новый подход

```typescript
// node-configs.ts
selector: {
  fields: [{ key: 'selector', type: 'text', label: 'CSS Селектор' }],
}
```

## Расширение

Чтобы добавить новый тип ноды:

1. Добавить конфигурацию в `node-configs.ts`
2. Готово! Нода автоматически работает

Не нужно:
- Создавать новый компонент
- Дублировать стили
- Писать логику синхронизации состояния



































