---
name: static-analysis-frontend
description: Проведение статического анализа Svelte 5 проектов с использованием svelte-check. Использовать при проверке качества кода, выявлении ошибок типизации и обеспечении соответствия best practices Svelte 5.
version: 1.0.0
---

# Статический анализ Svelte 5 проектов

## Основные команды запуска

### Анализ всего проекта
```bash
npx svelte-check
```

### Анализ конкретной директории
```bash
npx svelte-check src/components/
```

### Анализ конкретного файла
```bash
npx svelte-check src/components/Component.svelte
```

### Строгий анализ (предупреждения как ошибки)
```bash
npx svelte-check --fail-on-warnings
```

### Только ошибки (без warnings/hints)
```bash
npx svelte-check --threshold error
```

## Итеративный процесс исправления

**КРИТИЧЕСКИ ВАЖНО**: Всегда исправляй ошибки итеративно до полного их отсутствия.

### Алгоритм исправления:
1. Запусти анализ на файле/директории
2. Найди первую ошибку в выводе
3. Исправь ошибку в коде
4. Запусти анализ снова на том же файле
5. Повторяй пока не останется ошибок
6. Только после исправления всех ошибок переходи к следующему файлу

### Проверка успешности:
- **Exit code 0**: Ошибок нет ✅
- **Exit code 1**: Есть ошибки, требуют исправления ❌

## Типичные ошибки Svelte 5 и исправления

### `legacy_reactive_statement_invalid`
```svelte
<!-- ❌ Неправильно в Svelte 5 -->
$: doubled = count * 2;
$: console.log('Count changed:', count);

<!-- ✅ Правильно -->
let doubled = $derived(count * 2);

$effect(() => {
  console.log('Count changed:', count);
});
```

### `state_referenced_locally`
```typescript
// ❌ Неправильно
const composable = useData(() => $myStore, () => $otherStore);

// ✅ Правильно
import { get } from 'svelte/store';
const composable = useData(() => get(myStore), () => get(otherStore));
```

### `fn is not a function`
```typescript
// ❌ Неправильно
const getValue = () => somePrimitive;
let value = $derived(getValue()); // Ошибка

// ✅ Правильно
const valueStore = writable(somePrimitive);
let value = $derived($valueStore);
```

### `store_invalid_shape`
```typescript
// ❌ Неправильно
export function useData() {
  const store = writable('');
  return {
    getData: () => store, // Возвращает функцию
  };
}

// ✅ Правильно
export function useData() {
  const store = writable('');
  return {
    dataStore: store, // Возвращает store напрямую
  };
}
```

### `props_rest_readonly`
```svelte
<script>
  // ❌ Неправильно
  let { ...props }: Props = $props();
  props.value = 'new value'; // Ошибка - readonly
</script>
```

### `attribute_duplicate`
```svelte
<!-- ❌ Неправильно -->
<NodeEditor
  nodes={nodes}
  onNodesChange={handleNodesChange}
  onNodesChange={handleOtherChange} <!-- Дублирование -->
/>

<!-- ✅ Правильно -->
<NodeEditor
  nodes={nodes}
  onNodesChange={handleNodesChange}
/>
```

### `css_unused_selector`
```svelte
<style>
  /* ❌ Неправильно */
  .unused-class { color: red; }

  /* ✅ Правильно */
  :global(.external-class) { color: red; }
  /* Или удали неиспользуемый селектор */
</style>
```

## Архитектурные принципы Svelte 5

### Runes Mode Best Practices

#### State Management
```typescript
export function useComponentState() {
  // State
  let count = $state(0);
  let items = $state<string[]>([]);

  // Derived values
  let doubled = $derived(count * 2);
  let hasItems = $derived(items.length > 0);

  // Effects
  $effect(() => {
    console.log('Count changed:', count);
  });

  // Actions
  function increment() {
    count++;
  }

  return {
    count: readonly(count), // Экспортируем как readonly
    doubled,
    hasItems,
    increment,
  };
}
```

#### Props и Bindings
```svelte
<script lang="ts">
  interface Props {
    title: string;
    count: number;
    onUpdate?: (value: number) => void;
  }

  let { title, count, onUpdate }: Props = $props();
</script>

<!-- Правильное использование bind -->
<input bind:value={title} />
<button onclick={() => onUpdate?.(count + 1)}>
  Increment ({count})
</button>
```

### Composable Architecture

#### Store-based Composables
```typescript
export function useAsyncData<T>(url: string) {
  const dataStore = writable<T | null>(null);
  const loadingStore = writable(false);
  const errorStore = writable<string | null>(null);

  async function fetchData() {
    loadingStore.set(true);
    errorStore.set(null);

    try {
      const response = await fetch(url);
      const data = await response.json();
      dataStore.set(data);
    } catch (err) {
      errorStore.set(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      loadingStore.set(false);
    }
  }

  $effect(() => {
    if (url) {
      fetchData();
    }
  });

  return {
    dataStore,
    loadingStore,
    errorStore,
    fetchData,
    refetch: fetchData,
  };
}
```

## Интеграция в процесс разработки

### Pre-commit hooks
```bash
# .husky/pre-commit
npx svelte-check --threshold error
```

### CI/CD Pipeline
```yaml
# .github/workflows/ci.yml
- name: Type check
  run: npx svelte-check --fail-on-warnings
```

### VS Code настройки
```json
{
  "svelte.enable-ts-plugin": true,
  "typescript.preferences.noSemicolons": "off",
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  }
}
```

## Автоматизация и скрипты

### package.json скрипты
```json
{
  "scripts": {
    "type-check": "svelte-check --threshold warning",
    "type-check-errors": "svelte-check --threshold error",
    "type-check-ci": "svelte-check --threshold error"
  }
}
```

### ESLint интеграция
```javascript
// eslint.config.js
export default [
  {
    files: ['**/*.svelte'],
    languageOptions: {
      parser: '@typescript-eslint/parser',
      parserOptions: {
        project: './tsconfig.json',
        extraFileExtensions: ['.svelte']
      }
    },
    rules: {
      // Svelte-specific rules
      'svelte/no-unused-svelte-ignore': 'error',
      'svelte/valid-compile': 'error',
    }
  }
];
```

## Стратегия исправления ошибок

### Приоритизация
1. **Критические ошибки** (красные) - исправлять первыми
2. **Предупреждения** (желтые) - исправлять вторыми
3. **Информационные сообщения** (синие) - исправлять последними

### Систематический подход
- Исправлять по **одной ошибке** за раз
- После каждого исправления **запускай анализ заново**
- Проверяй, что исправление не создало новых ошибок

## Производительность и оптимизации

### Для больших проектов
```bash
# Анализ только измененных файлов
npx svelte-check --threshold error $(git diff --name-only HEAD~1 | grep '\.svelte\|\.ts')

# Параллельный анализ директорий
npx svelte-check --threshold error src/components/ &
npx svelte-check --threshold error src/lib/ &
wait
```

### Memory leaks prevention
```typescript
// Правильная очистка эффектов
$effect(() => {
  const timer = setInterval(() => {
    console.log('tick');
  }, 1000);

  return () => clearInterval(timer); // Правильная очистка
});
```

## Финальная проверка

**ОБЯЗАТЕЛЬНО** выполняй финальную проверку файла 3 раза подряд после исправления всех ошибок, чтобы убедиться в отсутствии регрессий.