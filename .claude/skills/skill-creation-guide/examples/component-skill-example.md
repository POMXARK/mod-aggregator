---
name: component-creation
description: Создание переиспользуемых UI компонентов в Svelte 5. Использовать при разработке компонентов, добавлении новых элементов интерфейса или улучшении существующих компонентов.
version: 1.0.0
---

# Создание UI компонентов в Svelte 5

## Обзор
Этот навык помогает создавать качественные, переиспользуемые компоненты в Svelte 5 с правильным использованием runes и TypeScript.

## Когда использовать
- Создание новых UI компонентов
- Рефакторинг существующих компонентов
- Добавление новых элементов интерфейса
- Улучшение компонентной архитектуры

## Инструкции по созданию компонента

### Шаг 1: Определение интерфейса

```typescript
// types.ts
export interface ComponentProps {
  title: string;
  variant?: 'primary' | 'secondary' | 'danger';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  onClick?: () => void;
}

export type ComponentVariant = NonNullable<ComponentProps['variant']>;
export type ComponentSize = NonNullable<ComponentProps['size']>;
```

### Шаг 2: Создание компонента

```svelte
<script lang="ts">
  import type { ComponentProps } from './types';

  // Props с дефолтными значениями
  let {
    title,
    variant = 'primary',
    size = 'md',
    disabled = false,
    onClick
  }: ComponentProps = $props();

  // Вычисляемые классы
  let buttonClasses = $derived(() => {
    const classes = ['button', `button-${variant}`, `button-${size}`];
    if (disabled) classes.push('button-disabled');
    return classes.join(' ');
  });
</script>

<button
  class={buttonClasses}
  {disabled}
  onclick={onClick}
  type="button"
>
  {title}
</button>

<style>
  .button {
    /* Базовые стили */
  }

  .button-primary { /* ... */ }
  .button-secondary { /* ... */ }
  .button-danger { /* ... */ }

  .button-sm { /* ... */ }
  .button-md { /* ... */ }
  .button-lg { /* ... */ }

  .button-disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
```

### Шаг 3: Экспорт компонента

```typescript
// index.ts
export { default as Button } from './Button.svelte';
export type { ComponentProps as ButtonProps } from './types';
```

### Шаг 4: Создание сторис для тестирования

```typescript
// Button.stories.ts
import type { Meta, StoryObj } from '@storybook/svelte';
import Button from './Button.svelte';

const meta = {
  title: 'Components/Button',
  component: Button,
  argTypes: {
    variant: {
      control: { type: 'select' },
      options: ['primary', 'secondary', 'danger']
    },
    size: {
      control: { type: 'select' },
      options: ['sm', 'md', 'lg']
    }
  }
} satisfies Meta<Button>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    title: 'Click me',
    variant: 'primary'
  }
};

export const Secondary: Story = {
  args: {
    title: 'Secondary',
    variant: 'secondary'
  }
};
```

## Лучшие практики

### 1. Использование runes
```typescript
// ✅ Правильно
let count = $state(0);
let doubled = $derived(count * 2);

$effect(() => {
  console.log('Count changed:', count);
});

// ❌ Неправильно (legacy)
let count = writable(0);
$: doubled = $count * 2;
```

### 2. Правильная типизация
```typescript
// ✅ Строгая типизация
interface Props {
  required: string;
  optional?: number;
}

let { required, optional = 0 }: Props = $props();
```

### 3. Доступность (a11y)
```svelte
<button
  role="button"
  aria-label={ariaLabel}
  aria-describedby={describedBy}
  tabindex={disabled ? -1 : 0}
>
  {title}
</button>
```

### 4. Производительность
```typescript
// ✅ Мемоизация вычислений
let expensiveValue = $derived(() => {
  // Только при изменении зависимостей
  return heavyComputation(data);
});

// ✅ Стабильные ссылки
let handleClick = $derived(() => {
  return () => doSomething(data);
});
```

## Тестирование компонента

### Unit тесты
```typescript
// Button.test.ts
import { render } from '@testing-library/svelte';
import { describe, it, expect } from 'vitest';
import Button from './Button.svelte';

describe('Button', () => {
  it('renders with correct title', () => {
    const { getByText } = render(Button, {
      props: { title: 'Test Button' }
    });

    expect(getByText('Test Button')).toBeInTheDocument();
  });

  it('applies correct variant class', () => {
    const { container } = render(Button, {
      props: { title: 'Button', variant: 'primary' }
    });

    expect(container.firstChild).toHaveClass('button-primary');
  });
});
```

### Visual regression тесты
```typescript
// Button.visual.test.ts
import { test, expect } from '@playwright/experimental-ct-svelte';
import Button from './Button.svelte';

test('Button visual test', async ({ mount }) => {
  const component = await mount(Button, {
    props: { title: 'Visual Test' }
  });

  await expect(component).toHaveScreenshot();
});
```

## Структура файлов компонента

```
src/components/Button/
├── Button.svelte          # Основной компонент
├── types.ts              # TypeScript типы
├── index.ts              # Экспорты
├── Button.stories.ts     # Storybook stories
├── Button.test.ts        # Unit тесты
├── Button.visual.test.ts # Visual тесты
└── README.md             # Документация
```

## Примеры использования

### Базовое использование
```svelte
<script>
  import { Button } from '$components';
</script>

<Button
  title="Save"
  variant="primary"
  onclick={handleSave}
/>
```

### С формами
```svelte
<script>
  import { Button } from '$components';
</script>

<form onsubmit={handleSubmit}>
  <!-- form fields -->
  <Button
    title="Submit"
    variant="primary"
    type="submit"
  />
</form>
```

### В циклах
```svelte
<script>
  import { Button } from '$components';

  let actions = [
    { title: 'Edit', action: editItem },
    { title: 'Delete', action: deleteItem }
  ];
</script>

{#each actions as { title, action }}
  <Button
    {title}
    variant="secondary"
    onclick={action}
  />
{/each}
```