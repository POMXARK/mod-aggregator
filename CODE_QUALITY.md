# Система анализа качества кода

Этот проект использует комплексную систему статического анализа кода для обеспечения высокого качества и一致ности.

## Frontend (TypeScript/Svelte)

### ESLint
Проверяет JavaScript/TypeScript и Svelte код на ошибки и стиль.

```bash
# Проверка кода
npm run lint

# Автоматическое исправление
npm run lint:fix
```

**Конфигурация:** `eslint.config.js`

### Prettier
Форматирует код для一致ного стиля.

```bash
# Проверка форматирования
npm run format:check

# Форматирование кода
npm run format
```

**Конфигурация:** `.prettierrc`, `.prettierignore`

### TypeScript Compiler
Проверяет типы TypeScript.

```bash
npm run type-check
```

### Svelte Check
Проверяет Svelte компоненты.

```bash
npm run svelte-check
```

## Backend (Rust)

### Clippy
Линтер для Rust кода.

```bash
# Проверка с предупреждениями
npm run rust-check

# Строгая проверка (ошибки)
npm run rust-check-strict
```

**Конфигурация:** `clippy.toml`

### Rustfmt
Форматирует Rust код.

```bash
# Форматирование
npm run rust-format

# Проверка форматирования
npm run rust-format-check
```

**Конфигурация:** `rustfmt.toml`

### Cargo Test
Запускает тесты Rust.

```bash
npm run rust-test
```

## Документация (Markdown)

### Markdownlint
Проверяет Markdown файлы.

```bash
# Проверка
npm run markdown-check

# Автоматическое исправление
npm run markdown-fix
```

**Конфигурация:** `.markdownlint.json`

## Комплексный анализ

### Полная проверка качества
Запускает все инструменты анализа:

```bash
npm run analyze
```

### Полное автоматическое исправление
Исправляет все автоисправимые проблемы:

```bash
npm run analyze:fix
```

## CI/CD интеграция

Все инструменты можно интегрировать в CI/CD пайплайны:

```yaml
# Пример для GitHub Actions
- name: Check code quality
  run: npm run analyze

- name: Auto-fix code style
  run: npm run analyze:fix
```

## Рекомендации по использованию

1. **Перед коммитом:** Запускайте `npm run analyze:fix`
2. **В IDE:** Настройте автоматический запуск линтеров при сохранении
3. **В CI:** Используйте `npm run analyze` для блокировки мерджей с проблемами

## Инструменты для IDE

### VS Code
Установите расширения:
- ESLint
- Prettier - Code formatter
- Svelte for VS Code
- rust-analyzer
- markdownlint

### Cursor
Большинство инструментов работают out-of-the-box благодаря встроенной поддержке.

## Правила проекта

- Все новые файлы должны проходить проверку `npm run analyze`
- Используйте `npm run analyze:fix` перед коммитом
- Для Rust кода строго следуйте рекомендациям Clippy
- TypeScript типы обязательны для всех функций и компонентов













