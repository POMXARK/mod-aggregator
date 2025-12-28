# Dependencies Components

Компоненты для управления зависимостями файлов/модов.

## Компоненты

- **DependencyGraph.svelte** - Визуализация графа зависимостей
- **DependencyEditor.svelte** - Редактор зависимостей файла
- **DependencyWarning.svelte** - Предупреждения о зависимостях
- **VersionSelector.svelte** - Выбор версии зависимости

## Архитектура

Все компоненты используют Svelte 5 runes ($state, $derived, $effect) для реактивности. Коммуникация с backend через Tauri команды из `src-tauri/src/commands/dependencies.rs`.

## Зависимости

- `@tauri-apps/api` - для вызова Tauri команд
- `src/lib/dependencies/` - утилиты для работы с зависимостями
- `src/types/dependency.ts` - TypeScript типы
























