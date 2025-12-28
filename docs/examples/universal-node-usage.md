# Примеры использования универсальной системы нод

## Варианты решения

### Вариант 1: Полная замена (рекомендуется)

Заменить все отдельные компоненты нод на `UniversalNode`.

**Преимущества:**
- Минимум бойлерплейта
- Единообразный UI
- Легко добавлять новые типы

**Недостатки:**
- Требует рефакторинга существующих нод

### Вариант 2: Гибридный подход

Использовать `UniversalNode` для новых типов, старые компоненты оставить.

**Преимущества:**
- Постепенная миграция
- Не ломает существующий код

**Недостатки:**
- Два подхода одновременно

### Вариант 3: Обертка для совместимости

Создать обертку, которая использует `UniversalNode` внутри, но сохраняет старый API.

**Преимущества:**
- Полная обратная совместимость
- Можно мигрировать постепенно

## Сравнение кода

### До (старый подход)

```typescript
// ParserBuilder.svelte - 60+ строк бойлерплейта
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
    // ... еще 3 case
  }

  nodes = [...nodes, newNode];
}
```

### После (новый подход)

```typescript
// ParserBuilder.svelte - 5 строк
function handleAddNode(type: string) {
  const position = { 
    x: Math.random() * 400 + 100, 
    y: Math.random() * 400 + 100 
  };
  
  const newNode = createNode(type, position);
  if (newNode) {
    nodes = [...nodes, newNode];
  }
}
```

## Добавление нового типа ноды

### Старый подход (нужно создать компонент)

1. Создать `NewNodeType.svelte` (~100 строк)
2. Добавить в `nodeTypes`
3. Добавить case в `handleAddNode`
4. Дублировать стили

### Новый подход (только конфигурация)

```typescript
// node-configs.ts - добавить конфигурацию
newNodeType: {
  type: 'newNodeType',
  label: 'New Node',
  color: '#0ea5e9',
  fields: [
    {
      key: 'field1',
      type: 'text',
      label: 'Field 1',
    },
    {
      key: 'field2',
      type: 'select',
      label: 'Field 2',
      options: [
        { value: 'opt1', label: 'Option 1' },
      ],
    },
  ],
}
```

Готово! Нода автоматически работает.

## Миграция существующих нод

### Шаг 1: Добавить конфигурацию

```typescript
// node-configs.ts
selector: {
  type: 'selector',
  label: 'Selector',
  fields: [
    {
      key: 'selector',
      type: 'text',
      label: 'CSS Селектор',
      placeholder: '.mod-item',
    },
  ],
}
```

### Шаг 2: Обновить ParserBuilder

```typescript
// Заменить nodeTypes
const nodeTypes: NodeTypes = {
  universal: UniversalNode,
};

// Заменить handleAddNode
function handleAddNode(type: string) {
  const newNode = createNode(type, position);
  // ...
}
```

### Шаг 3: Удалить старые компоненты (опционально)

Можно оставить для обратной совместимости или удалить после тестирования.
























