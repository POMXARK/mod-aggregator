---
name: static-analysis-frontend-warnings
description: Управление предупреждениями и hints в svelte-check для оптимизации CI/CD логов. Использовать при настройке статического анализа для проектов с большим количеством legacy кода или при необходимости фильтрации шумных сообщений.
version: 1.0.0
---

# Управление предупреждениями в svelte-check

## Уровни threshold в svelte-check

### `--threshold error` - Только критические ошибки
```bash
npx svelte-check --threshold error
```

**Показывает только:**
- ❌ Синтаксические ошибки
- ❌ Ошибки типизации
- ❌ Недоступные переменные/функции
- ✅ НЕ показывает warnings и hints

**Когда использовать:**
- В CI для быстрой проверки на блокирующие ошибки
- Когда проект имеет много hints о типах `any`
- Для фокуса на критических проблемах

### `--threshold warning` - Ошибки + важные предупреждения
```bash
npx svelte-check --threshold warning
```

**Показывает:**
- Все из `error` уровня
- ⚠️ Предупреждения о потенциальных проблемах
- ✅ НЕ показывает hints (информационные сообщения)

**Когда использовать:**
- В development среде
- Для постепенного улучшения качества кода
- Когда hints слишком шумные

### `--threshold warning` (по умолчанию) - Полный анализ
```bash
npx svelte-check --threshold warning
# или просто
npx svelte-check
```

**Показывает:**
- Все из `error` уровня
- ⚠️ Предупреждения (warnings)
- 💡 Hints о типах `any`, неиспользуемых переменных

## Стратегия выбора threshold

### Для разных стадий проекта

**Legacy проекты (JavaScript → TypeScript):**
```bash
# В CI: только ошибки
npx svelte-check --threshold error

# Локально: предупреждения для постепенного улучшения
npx svelte-check --threshold warning
```

**Новые проекты с полной типизацией:**
```bash
# В CI: предупреждения как ошибки
npx svelte-check --fail-on-warnings

# Локально: все сообщения
npx svelte-check --threshold warning
```

**Смешанные проекты:**
```bash
# Разделить анализ по директориям
npx svelte-check --threshold error src/legacy/
npx svelte-check --threshold warning src/new/
```

## Типичные предупреждения и обработка

### Hints о типах `any` (самые частые)

#### Проблема:
```
Hint: Property 'data' has any type (any)
Hint: Parameter 'callback' has any type (any)
```

#### Стратегии исправления:

**Вариант 1: Постепенная типизация**
```typescript
// Промежуточный этап
function processData(data: any) {
  // @ts-ignore - временно игнорируем
  return data.value;
}

// Финальный вариант
interface DataShape {
  value: string;
  metadata?: Record<string, unknown>;
}

function processData(data: DataShape) {
  return data.value;
}
```

**Вариант 2: Строгая типизация**
```typescript
function processData(data: unknown): string {
  if (typeof data === 'object' && data !== null && 'value' in data) {
    return String((data as { value: unknown }).value);
  }
  throw new Error('Invalid data shape');
}
```

**Вариант 3: Type guards**
```typescript
function isDataShape(obj: unknown): obj is { value: string } {
  return typeof obj === 'object' &&
         obj !== null &&
         'value' in obj &&
         typeof (obj as any).value === 'string';
}

function processData(data: unknown): string {
  if (isDataShape(data)) {
    return data.value;
  }
  throw new Error('Invalid data shape');
}
```

### Предупреждения о неиспользуемых переменных

#### Проблема:
```
Warning: 'unusedVar' is declared but never used
```

#### Исправления:
```typescript
// Вариант 1: Использовать переменную
let userName = 'John';
console.log(`Hello, ${userName}`);

// Вариант 2: Префикс подчеркивания
let _unusedVar = 'ignored';

// Вариант 3: Удалить если действительно не нужна
```

### Предупреждения о CSS

#### `css_unused_selector`
```
Warning: Unused CSS selector ".my-class"
```

#### Исправления:
```svelte
<style>
  /* Вариант 1: Использовать селектор */
  <div class="used-class">Content</div>

  /* Вариант 2: Глобальный селектор */
  :global(.external-class) { color: red; }

  /* Вариант 3: Удалить неиспользуемый */
</style>
```

### Предупреждения о store

#### `state_referenced_locally`
```
Warning: State referenced in its own scope
```

#### Исправления:
```typescript
// ❌ Неправильно
const store = writable(0);
const doubled = derived(store, $store => $store * 2);

// ✅ Правильно
const store = writable(0);
const doubled = derived(store, value => value * 2);
```

## Интеграция в CI/CD

### GitHub Actions для legacy проектов
```yaml
jobs:
  type-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - run: npm ci

      # Только ошибки для быстрой проверки
      - name: Type check (errors only)
        run: npx svelte-check --threshold error

      # Отдельный job для предупреждений
      - name: Type check (with warnings)
        run: npx svelte-check --threshold warning
        continue-on-error: true # Не блокировать сборку
```

### GitHub Actions для новых проектов
```yaml
jobs:
  type-check:
    runs-on: ubuntu-latest
    steps:
      - run: npm ci
      # Предупреждения как ошибки
      - name: Type check (strict)
        run: npx svelte-check --fail-on-warnings
```

### GitLab CI
```yaml
type_check:
  stage: test
  image: node:20
  before_script:
    - npm ci
  script:
    # Для legacy: только ошибки
    - npx svelte-check --threshold error
  allow_failure: false

type_check_warnings:
  stage: test
  image: node:20
  before_script:
    - npm ci
  script:
    # Предупреждения в отдельном job
    - npx svelte-check --threshold warning
  allow_failure: true # Не блокировать pipeline
```

## Расширенные стратегии

### Дифференцированный анализ
```bash
# Строго для новых компонентов
npx svelte-check --threshold warning src/components/new/

# Лояльно для legacy кода
npx svelte-check --threshold error src/components/legacy/
```

### Прогрессивное ужесточение

**Этап 1: Baseline**
```bash
# Создать baseline с текущими ошибками
npx svelte-check --threshold error > baseline-errors.log
```

**Этап 2: Добавление предупреждений**
```bash
# Через месяц: warnings как ошибки для новых файлов
npx svelte-check --fail-on-warnings --include "src/components/new/**"
```

**Этап 3: Полная строгость**
```bash
# Через 3 месяца: warnings как ошибки везде
npx svelte-check --fail-on-warnings
```

## Автоматизация

### Скрипты для локальной разработки
```json
{
  "scripts": {
    "type-check": "svelte-check --threshold warning",
    "type-check-errors": "svelte-check --threshold error",
    "type-check-warnings": "svelte-check --threshold warning",
    "type-check-ci": "svelte-check --threshold error"
  }
}
```

### Pre-commit hook
```bash
#!/bin/sh
. "$(dirname "$0")/_/husky.sh"

# В pre-commit только ошибки
npx svelte-check --threshold error
```

## Мониторинг качества

### Метрики для отслеживания
```bash
# Количество ошибок
npx svelte-check --threshold error 2>&1 | grep -c "Error:"

# Количество предупреждений
npx svelte-check --threshold warning 2>&1 | grep -c "Warning:"

# Количество hints
npx svelte-check --threshold warning 2>&1 | grep -c "Hint:"
```

### Автоматические отчеты
```bash
# Еженедельный отчет
0 9 * * 1 /path/to/project/scripts/weekly-report.sh
```

## Проблемы и решения

### False positives в warnings
```typescript
// Для обоснованных случаев
// @ts-ignore: svelte-check false positive
const dynamicImport = await import('./module');

// eslint-disable для специфических случаев
/* eslint-disable @typescript-eslint/no-unused-vars */
const _reserved = 'for future use';
/* eslint-enable @typescript-eslint/no-unused-vars */
```

### Производительность при большом проекте
```bash
# Анализ только измененных файлов
npx svelte-check --threshold error $(git diff --name-only HEAD~1 | grep '\.svelte\|\.ts')

# Параллельный анализ
npx svelte-check --threshold error src/components/ &
npx svelte-check --threshold error src/lib/ &
wait
```

### Конфликты с ESLint
```javascript
// eslint.config.js
export default [
  {
    files: ['**/*.svelte'],
    rules: {
      // Отключить правила, дублируемые svelte-check
      '@typescript-eslint/no-unused-vars': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
    }
  }
];
```

## Заключение

**Используй `--threshold error`** для чистых CI логов в legacy проектах.

**Применяй `--threshold warning`** для постепенного улучшения качества кода.

**Мониторь метрики** качества регулярно для отслеживания прогресса.

**Адаптируй стратегию** под специфику проекта для баланса между качеством и практичностью.