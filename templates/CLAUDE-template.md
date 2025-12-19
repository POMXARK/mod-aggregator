# Контекст для Claude Code: Mod Aggregator

Этот файл содержит контекст проекта для работы с Claude Code и другими AI агентами.

## Технический стек

### Frontend
- Svelte 5 с TypeScript
- Tailwind CSS
- @xyflow/svelte для графов
- Tauri API для взаимодействия с backend

### Backend
- Rust + Tauri 2.0
- SQLite через SQLx
- scraper для парсинга HTML
- reqwest для HTTP запросов

## Структура проекта

```
mod-aggregator/
├── src/                    # Frontend (Svelte 5)
│   ├── components/         # Компоненты
│   │   ├── App.svelte
│   │   ├── ParserBuilder.svelte
│   │   ├── PageViewer.svelte
│   │   ├── ModsList.svelte
│   │   └── SitesManager.svelte
│   ├── lib/                # Утилиты
│   │   ├── tauri-wrapper.ts
│   │   └── api.ts
│   └── App.svelte
├── src-tauri/              # Backend (Rust)
│   └── src/
│       ├── main.rs
│       ├── database.rs
│       ├── parser.rs
│       └── models.rs
├── specs/                  # Спецификации новых фич
├── memory/                 # Конституция проекта
└── .cursor/rules/          # Правила для AI
```

## Ключевые компоненты

### Frontend
- **ParserBuilder.svelte**: Визуальный конструктор парсеров с нодами
- **PageViewer.svelte**: Просмотр и сохранение веб-страниц
- **ModsList.svelte**: Список модов с карточками
- **SitesManager.svelte**: Управление сайтами

### Backend
- **database.rs**: Работа с SQLite
- **parser.rs**: Движок парсинга HTML
- **models.rs**: Модели данных (Site, Mod, Notification)

## Модели данных

### Site
- `id`, `name`, `url`, `parser_config`, `created_at`, `updated_at`

### Mod
- `id`, `site_id`, `title`, `url`, `version`, `author`, `description`, `image_url`, `changes`

## Правила разработки

1. Все комментарии на русском языке
2. JSDoc для TypeScript функций
3. Документация для Rust функций
4. TDD где это возможно
5. Компонентный подход
6. Типизация обязательна

## Команды SpecKit

- `/speckit.plan` - создать план реализации из спецификации
- `/speckit.tasks` - разбить план на конкретные задачи
- `/speckit.implement` - выполнить задачи по порядку

## Текущие спецификации

Список активных спецификаций находится в директории `specs/`.

