# Session Components

Компоненты для сохранения и восстановления состояния сессии приложения.

## Компоненты

- **SessionRestore.svelte** - Компонент для управления настройками сессии и восстановления состояния

## Архитектура

Компонент использует Svelte 5 runes для реактивности. Коммуникация с backend через Tauri команды из `src-tauri/src/commands/session_state.rs`.

## Утилиты

- `src/lib/session/session-state.ts` - Утилиты для работы с состоянием сессии, включая debounced сохранение

## Сохраняемое состояние

- Порядок файлов (file_order)
- Настройки UI (view_mode, sidebar_collapsed, theme и т.д.)
- Открытые коллекции (open_collections)
- Выбранные файлы (selected_files)
- История действий (recent_actions, максимум 50)

## Автосохранение

Утилита `SessionStateManager` предоставляет debounced функции для автосохранения:
- `createDebouncedFileOrderSaver()` - автосохранение порядка файлов
- `createDebouncedPreferencesSaver()` - автосохранение настроек UI

## Backend команды

- `get_session_state()` - получить состояние сессии
- `update_file_order()` - обновить порядок файлов
- `update_ui_preferences()` - обновить настройки UI
- `restore_session()` - восстановить сессию при запуске
- `add_recent_action()` - добавить действие в историю
- И другие...

## Восстановление при запуске

Восстановление сессии происходит автоматически в `App.svelte` при монтировании компонента (если приложение запущено в Tauri режиме).
