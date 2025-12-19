# Фреймворк конструктора парсеров

## Обзор

Фреймворк для создания конструктора парсеров с максимальным переиспользованием логики. Код организован как коллекция переиспользуемых компонентов и утилит.

## Архитектура

```
src/lib/framework/
├── index.ts              # Главный экспорт
├── node-config.ts        # Конфигурация нод
├── node-registry.ts      # Реестр типов нод
├── node-factory.ts       # Фабрика создания нод
├── node-state.ts         # Управление состоянием
└── field-types.ts        # Типы полей и валидация

src/components/framework/
├── BaseNode.svelte       # Базовый компонент ноды
└── BaseField.svelte      # Базовый компонент поля
```

## Принципы

### 1. Configuration-Driven
Все определяется через конфигурацию, а не через код:

```typescript
registerNodeType({
  type: 'selector',
  label: 'Selector',
  fields: [
    { key: 'selector', type: 'text', label: 'CSS Селектор' }
  ],
});
```

### 2. Композиция компонентов
Компоненты собираются из базовых примитивов:

```
BaseField → BaseNode → UniversalNode
```

### 3. Единая точка входа
Все через `initParserFramework()`:

```typescript
import { initParserFramework, createNode } from '../lib/framework';
initParserFramework();
```

### 4. Переиспользование логики
- Один `BaseNode` для всех типов нод
- Один `BaseField` для всех типов полей
- Централизованная валидация
- Централизованное управление состоянием

## Использование

### Инициализация

```typescript
import { initParserFramework } from '../lib/framework';

// В компоненте
initParserFramework();
```

### Создание ноды

```typescript
import { createNode } from '../lib/framework';

// Вместо большого switch/case
const newNode = createNode('selector', {
  position: { x: 100, y: 100 }
});
```

### Регистрация компонента

```typescript
import UniversalNode from './nodes/UniversalNode.svelte';

const nodeTypes: NodeTypes = {
  universal: UniversalNode, // Один компонент для всех
};
```

### Добавление нового типа ноды

```typescript
import { registerNodeType } from '../lib/framework';

registerNodeType({
  type: 'custom',
  label: 'Custom Node',
  color: '#0ea5e9',
  fields: [
    {
      key: 'customField',
      type: 'text',
      label: 'Custom Field',
    },
  ],
});
```

## Преимущества

1. **Меньше кода** - один компонент вместо 5+
2. **Легче расширять** - только конфигурация
3. **Единообразный UI** - все ноды выглядят одинаково
4. **Централизованная валидация** - одна система для всех
5. **Переиспользование** - компоненты как коллекция

## Примеры

### Простая нода

```typescript
registerNodeType({
  type: 'simple',
  label: 'Simple',
  fields: [
    { key: 'value', type: 'text', label: 'Value' }
  ],
});
```

### Нода с условными полями

```typescript
registerNodeType({
  type: 'conditional',
  label: 'Conditional',
  fields: [
    {
      key: 'type',
      type: 'select',
      options: [
        { value: 'a', label: 'Type A' },
        { value: 'b', label: 'Type B' },
      ],
    },
    {
      key: 'fieldA',
      type: 'text',
      showIf: { field: 'type', operator: 'equals', value: 'a' },
    },
    {
      key: 'fieldB',
      type: 'text',
      showIf: { field: 'type', operator: 'equals', value: 'b' },
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
      key: 'email',
      type: 'text',
      validation: {
        required: true,
        pattern: '^[^@]+@[^@]+\\.[^@]+$',
        custom: (value) => {
          if (!value.includes('@')) return 'Неверный email';
          return null;
        },
      },
    },
  ],
});
```

