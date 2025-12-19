# Примеры использования фреймворка конструктора

## Быстрый старт

### 1. Инициализация

```typescript
// В главном компоненте
import { initParserFramework } from '../lib/framework';

onMount(() => {
  initParserFramework();
});
```

### 2. Использование в ParserBuilder

```typescript
import { createNode, getRegisteredNodeTypes } from '../lib/framework';
import UniversalNode from './nodes/UniversalNode.svelte';

// Один компонент для всех типов
const nodeTypes: NodeTypes = {
  universal: UniversalNode,
};

// Создание ноды - одна строка
function handleAddNode(type: string) {
  const newNode = createNode(type, { 
    position: { x: 100, y: 100 } 
  });
  if (newNode) nodes = [...nodes, newNode];
}
```

## Добавление нового типа ноды

### Вариант 1: В node-registry.ts (рекомендуется)

```typescript
// src/lib/framework/node-registry.ts
registerNodeType({
  type: 'myCustomNode',
  label: 'My Custom Node',
  color: '#0ea5e9',
  fields: [
    {
      key: 'customField',
      type: 'text',
      label: 'Custom Field',
      validation: { required: true },
    },
  ],
});
```

### Вариант 2: Динамически

```typescript
import { registerNodeType } from '../lib/framework';

registerNodeType({
  type: 'dynamic',
  label: 'Dynamic Node',
  fields: [
    { key: 'value', type: 'text', label: 'Value' }
  ],
});
```

## Расширенные примеры

### Нода с условными полями

```typescript
registerNodeType({
  type: 'conditional',
  label: 'Conditional',
  fields: [
    {
      key: 'mode',
      type: 'select',
      options: [
        { value: 'simple', label: 'Simple' },
        { value: 'advanced', label: 'Advanced' },
      ],
    },
    {
      key: 'simpleField',
      type: 'text',
      showIf: { field: 'mode', operator: 'equals', value: 'simple' },
    },
    {
      key: 'advancedField1',
      type: 'text',
      showIf: { field: 'mode', operator: 'equals', value: 'advanced' },
    },
    {
      key: 'advancedField2',
      type: 'number',
      showIf: { field: 'mode', operator: 'equals', value: 'advanced' },
    },
  ],
});
```

### Нода с валидацией

```typescript
registerNodeType({
  type: 'validated',
  label: 'Validated',
  fields: [
    {
      key: 'url',
      type: 'text',
      label: 'URL',
      placeholder: 'https://example.com',
      validation: {
        required: true,
        pattern: '^https?://.+',
        custom: (value) => {
          try {
            new URL(value);
            return null;
          } catch {
            return 'Неверный формат URL';
          }
        },
      },
    },
    {
      key: 'timeout',
      type: 'number',
      label: 'Timeout (ms)',
      validation: {
        required: true,
        min: 100,
        max: 60000,
      },
    },
  ],
});
```

### Нода с множественным выбором

```typescript
registerNodeType({
  type: 'multiselect',
  label: 'Multi Select',
  fields: [
    {
      key: 'fields',
      type: 'multiselect',
      label: 'Выберите поля',
      options: [
        { value: 'title', label: 'Title' },
        { value: 'url', label: 'URL' },
        { value: 'version', label: 'Version' },
      ],
    },
  ],
});
```

## Миграция существующих нод

### До

```svelte
<!-- SelectorNode.svelte - ~100 строк -->
<script>
  let selector = $state(data.selector || '');
  // ... много бойлерплейта
</script>
<div class="selector-node">
  <!-- дублирующаяся структура -->
</div>
```

### После

```typescript
// node-registry.ts - 10 строк конфигурации
registerNodeType({
  type: 'selector',
  label: 'Selector',
  fields: [
    { key: 'selector', type: 'text', label: 'CSS Селектор' }
  ],
});
```

## Сравнение кода

### Создание ноды

**До:**
```typescript
function handleAddNode(type: string) {
  const nodeId = `node-${Date.now()}-...`;
  let newNode: Node;
  
  switch (type) {
    case 'selector':
      newNode = {
        id: nodeId,
        type: 'selector',
        position: { x: 100, y: 100 },
        data: { label: 'Selector', selector: '' },
      };
      break;
    // ... еще 4 case по 10+ строк каждый
  }
  
  nodes = [...nodes, newNode];
}
```

**После:**
```typescript
function handleAddNode(type: string) {
  const newNode = createNode(type, { 
    position: { x: 100, y: 100 } 
  });
  if (newNode) nodes = [...nodes, newNode];
}
```

### Добавление нового типа

**До:**
- Создать компонент (~100 строк)
- Добавить в nodeTypes
- Добавить case в switch
- Дублировать стили

**После:**
- Добавить конфигурацию (~10 строк)
- Готово!

