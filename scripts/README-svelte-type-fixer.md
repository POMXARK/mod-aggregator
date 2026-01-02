# Python Svelte Type Fixer

## Обзор

Python скрипт `fix-svelte-types.py` автоматически исправляет проблемы типизации в Svelte/TypeScript файлах, связанные с неправильным использованием типов `Writable<T>` vs `T`.

## Основные проблемы, которые исправляет

### 1. Неправильная типизация store значений

**Проблема:**
```typescript
let myVar: MyType = writable({...});
// Ошибка: Cannot assign store value to variable of type MyType
```

**Исправление:**
```typescript
let myVar: Writable<MyType> = writable({...});
// Теперь правильно - переменная типа Writable<T> получает store
```

## Использование

### Автоматическое исправление одного файла

```bash
python scripts/fix-svelte-types.py path/to/file.svelte
```

### Исправление нескольких файлов

```bash
python scripts/fix-svelte-types.py file1.svelte file2.svelte file3.ts
```

### Без создания бэкапов

```bash
python scripts/fix-svelte-types.py file.svelte --no-backup
```

### Интеграция в статический анализатор

```powershell
# Анализ и автоматическое исправление
.\scripts\run-code-inspection.ps1 -Include "*.svelte,*.ts" -Fix

# Только анализ (без исправления)
.\scripts\run-code-inspection.ps1 -Include "*.svelte,*.ts"
```

## Логика работы

1. **Анализ импортов типов** - определяет доступные типы
2. **Парсинг объявлений переменных** - находит переменные с типами и присваиваниями
3. **Анализ значений** - определяет, является ли присваиваемое значение store
4. **Исправление типов** - изменяет `T` на `Writable<T>` при необходимости
5. **Создание бэкапов** - сохраняет оригинальные файлы с расширением `.bak`

## Формат отчета

После работы скрипт создает JSON отчет `svelte-type-fixes-report.json`:

```json
[
  {
    "file": "path/to/file.svelte",
    "fixes_applied": 1,
    "fixes": [
      {
        "type": "change-variable-type",
        "variable": "wrongType",
        "old_type": "AISettings",
        "new_type": "Writable<AISettings>",
        "line": 32,
        "reason": "Variable typed as T but assigned store value - changed to Writable<T>"
      }
    ]
  }
]
```

## Примеры исправлений

### Пример 1: Переменная получает store значение

**До:**
```typescript
let settings: AISettings = writable({...});
// Ошибка: Cannot assign store value to variable of type AISettings
```

**После:**
```typescript
let settings: Writable<AISettings> = writable({...});
// Теперь правильно - переменная типа Writable<T> получает store
```

### Пример 2: Корректное использование stores в Svelte 5

**Рекомендуемый подход:**
```typescript
// Прямой доступ к store через reactive binding
let settings = $aiSettingsStore;

// Или через $derived (тоже корректно)
let settings: AISettings = $derived(aiSettingsStore);
```

**Избегайте:**
```typescript
let settings: Writable<AISettings> = aiSettingsStore;
// Ошибка: пытаются присвоить store переменной типа Writable<T>
```

### Пример 3: Удаление неиспользуемых импортов

**До:**
```typescript
import { writable } from 'svelte/store';
import type { AISettings } from './types';

// writable не используется в коде
let settings: AISettings = { /* ... */ };
```

**После:**
```typescript
import type { AISettings } from './types';

// Неиспользуемый импорт удален
let settings: AISettings = { /* ... */ };
```

## Поиск неиспользуемых импортов

Анализатор теперь обнаруживает неиспользуемые импорты типов:

```bash
python scripts/analyze-svelte-types.py file.svelte
# Вывод: [WARNING] Line 1: AISettings is imported but never used
```

## Автоматическое исправление по одной ошибке

### Логика работы:
1. **Анализ файла** - поиск всех проблем
2. **Обработка первой ошибки** - применение исправления
3. **Повторный анализ** - проверка результатов
4. **Цикл продолжается** до устранения всех проблем

### Пример использования:
```bash
# Анализ и исправление всех проблем в файле
python scripts/analyze-svelte-types.py problematic-file.svelte
python scripts/fix-svelte-types.py problematic-file.svelte

# Или интегрированно в статический анализатор
.\scripts\run-code-inspection.ps1 -Include "*.svelte" -Fix
```

### Пример 2: Многострочное присваивание

**До:**
```typescript
let config: ParserSettings = writable({
  maxElements: 100,
  timeoutSeconds: 60,
  slowMode: false,
  delayPerElement: 500
});
```

**После:**
```typescript
let config: Writable<ParserSettings> = writable({
  maxElements: 100,
  timeoutSeconds: 60,
  slowMode: false,
  delayPerElement: 500
});
```

## Безопасность

- **Бэкапы**: По умолчанию создаются файлы с расширением `.bak`
- **Анализ типов**: Исправляет только явные проблемы типизации
- **Импорты**: Проверяет наличие импортов типов перед исправлением
- **Валидация**: Не исправляет встроенные типы TypeScript

## Интеграция в CI/CD

```yaml
# .github/workflows/ci.yml
- name: Run Type Analysis and Fix
  run: |
    .\scripts\run-code-inspection.ps1 -Include "*.svelte,*.ts" -Fix
    if ($LASTEXITCODE -ne 0) {
      echo "Type issues found and fixed"
      exit 1
    }
```

## Совместная работа с анализатором

1. **Анализатор** (`analyze-svelte-types.py`) находит проблемы
2. **Фиксер** (`fix-svelte-types.py`) исправляет их автоматически
3. **Повторный анализ** подтверждает исправление

```bash
# Полный цикл
python scripts/analyze-svelte-types.py *.svelte  # Найти проблемы
python scripts/fix-svelte-types.py *.svelte      # Исправить
python scripts/analyze-svelte-types.py *.svelte  # Проверить
```
