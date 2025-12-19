# Mod Aggregator

Десктопное приложение для агрегации модов из различных сайтов с визуальным конструктором парсеров.

## Архитектура проекта

### Frontend (Svelte 5 + TypeScript)
- **Фреймворк**: Svelte 5 с TypeScript
- **Стилизация**: Tailwind CSS + кастомные стили
- **Граф нод**: @xyflow/svelte для визуального конструктора парсеров
- **Состояние**: Svelte 5 runes ($state, $derived, $effect)

### Backend (Rust + Tauri 2.0)
- **Фреймворк**: Tauri 2.0 для десктопных приложений
- **База данных**: SQLite через SQLx
- **Парсинг**: scraper crate для HTML парсинга
- **HTTP**: reqwest для запросов

## Структура проекта

```
mod-aggregator/
├── src/                    # Frontend код (Svelte 5)
│   ├── components/         # Svelte компоненты
│   ├── lib/                # Утилиты и composables
│   └── App.svelte          # Главный компонент
├── src-tauri/              # Backend код (Rust)
│   └── src/
│       ├── main.rs         # Tauri команды
│       ├── database.rs     # Работа с БД
│       ├── parser.rs       # Движок парсинга
│       └── models.rs       # Модели данных
└── .specify/               # SpecKit: структура для Spec-Driven Development
    ├── memory/             # Конституция проекта
    ├── templates/          # Шаблоны для спецификаций
    └── scripts/            # Вспомогательные скрипты
└── .cursor/rules/          # Правила для Cursor AI
```

## Основные компоненты

### Frontend
- **App.svelte** - главный компонент, роутинг между страницами
- **ParserBuilder.svelte** - визуальный конструктор парсеров с нодами
- **PageViewer.svelte** - просмотр страниц для выбора элементов
- **Sidebar.svelte** - боковая панель навигации
- **ModsList.svelte** - список модов
- **SitesManager.svelte** - управление сайтами

### Backend
- **database.rs** - работа с SQLite базой данных
- **parser.rs** - движок парсинга HTML
- **main.rs** - Tauri команды и обработчики

## Функционал

- Управление сайтами для парсинга
- Визуальный конструктор парсеров
- Сохранение и кеширование страниц
- Версионирование сохраненных страниц
- Автоматическое обновление модов
- Уведомления о новых версиях

## Разработка

```bash
# Установка зависимостей
npm install

# Запуск в режиме разработки
npm run tauri:dev

# Сборка
npm run tauri:build
```

## Документация

Документация проекта находится в папке `website/` и построена на Docusaurus.

### Локальный запуск

```bash
# Установка зависимостей
npm run docs:install

# Запуск в режиме разработки
npm run docs:start

# Сборка
npm run docs:build

# Запуск собранного сайта
npm run docs:serve
```

### Запуск в Docker

```bash
# Production (собранный сайт)
npm run docs:docker:build
npm run docs:docker:up

# Development (с hot reload)
cd website && docker-compose -f docker-compose.dev.yml up -d
```

Документация будет доступна по адресу http://localhost:3000

Подробнее см. [website/README.md](website/README.md)

## SpecKit - Spec-Driven Development

Проект использует [SpecKit](https://github.com/github/spec-kit) для структурированной разработки через спецификации.

### Установка

SpecKit установлен через `specify-cli` с поддержкой **Cursor Agent** (не Claude Code). Проверить установку:

```bash
specify check
```

### Конфигурация

- **AI Agent**: Cursor Agent (выбран при инициализации)
- **Script Type**: PowerShell (`.ps1`)
- **Структура**: `.specify/` директория с шаблонами и скриптами

### Быстрый старт

1. **Создайте конституцию проекта** (если еще не создана):
   ```
   /speckit.constitution
   ```
   Это создаст или обновит `.specify/memory/constitution.md` с принципами проекта.

2. **Создайте спецификацию** для новой фичи:
   ```
   /speckit.specify
   ```

3. **Создайте план реализации**:
   ```
   /speckit.plan
   ```

4. **Разбейте на задачи**:
   ```
   /speckit.tasks
   ```

5. **Выполните реализацию**:
   ```
   /speckit.implement
   ```

### Доступные команды

**Основные команды:**
- `/speckit.constitution` - установить принципы проекта
- `/speckit.specify` - создать спецификацию
- `/speckit.plan` - создать план реализации
- `/speckit.tasks` - разбить на задачи
- `/speckit.implement` - выполнить реализацию

**Дополнительные команды (опционально):**
- `/speckit.clarify` - задать структурированные вопросы для уточнения (перед `/speckit.plan`)
- `/speckit.analyze` - проверить согласованность артефактов (после `/speckit.tasks`, перед `/speckit.implement`)
- `/speckit.checklist` - создать чеклист качества (после `/speckit.plan`)

### Структура SpecKit

```
.specify/
├── memory/
│   └── constitution.md      # Конституция проекта (заполнить через /speckit.constitution)
├── templates/              # Шаблоны для спецификаций
│   ├── spec-template.md
│   ├── plan-template.md
│   ├── tasks-template.md
│   └── checklist-template.md
└── scripts/
    └── powershell/          # PowerShell скрипты для автоматизации
        ├── check-prerequisites.ps1
        ├── create-new-feature.ps1
        └── setup-plan.ps1
```

### Важные замечания

- Конституция проекта находится в `.specify/memory/constitution.md` и должна быть заполнена через команду `/speckit.constitution`
- Все спецификации создаются через команды SpecKit, не вручную
- Скрипты используют PowerShell (`.ps1`), так как проект на Windows

Подробнее см. [документацию SpecKit](https://github.github.io/spec-kit/)

## Комментарии и документация

- Все комментарии пишутся на русском языке
- Каждая функция должна иметь JSDoc комментарий
- README файлы в каждой директории описывают архитектуру компонентов
- Документация в формате Docusaurus находится в `website/docs/`
