# Implementation Plan: File Dependency Management and Advanced Collections

**Branch**: `003-file-dependency-system` | **Date**: 2025-12-19 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-file-dependency-system/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Реализация системы управления зависимостями файлов/модов с поддержкой:
- Определение зависимостей между файлами (name@version идентификация, как в npm)
- Ручное добавление и редактирование файлов через UI формы
- Импорт/экспорт файлов и коллекций в JSON формате
- Продвинутая логика включения/выключения для коллекций через UI-конструктор
- Множественный выбор и пакетные операции
- Сохранение состояния сессии (порядок файлов, настройки UI)

**Технический подход**: Расширение существующей Tauri 2.0 архитектуры (Svelte 5 frontend + Rust backend) с добавлением новых таблиц в SQLite, Tauri команд для работы с зависимостями, и UI компонентов для управления зависимостями и коллекциями. Все настройки должны быть редактируемыми через UI, система должна поддерживать добавление новых функциональных блоков.

## Technical Context

**Language/Version**: 
- Frontend: TypeScript 5.0+ с Svelte 5.0+
- Backend: Rust 2021 edition (latest stable)

**Primary Dependencies**: 
- Frontend: Svelte 5, @xyflow/svelte (для визуальных редакторов), Tailwind CSS, @tauri-apps/api 2.0
- Backend: Tauri 2.0, SQLx 0.7 (SQLite), serde/serde_json, tokio, reqwest, scraper

**Storage**: SQLite база данных через SQLx (расширение существующей схемы)

**Testing**: 
- Frontend: Vitest + Testing Library для компонентов
- Backend: Стандартные Rust тесты (cargo test)
- E2E: Cypress для критических пользовательских сценариев

**Target Platform**: Desktop (Windows, macOS, Linux) через Tauri 2.0

**Project Type**: Desktop application (Tauri) - frontend + backend структура

**Performance Goals**: 
- Обнаружение отсутствующих зависимостей: <1 секунда (SC-002)
- Обнаружение циклических зависимостей: <2 секунды (SC-008)
- Просмотр файлов из 10+ коллекций без деградации производительности (SC-009)
- Пакетные операции на 50+ файлах одновременно (SC-006)

**Constraints**: 
- Все настройки должны быть редактируемыми через UI (требование пользователя)
- Система должна поддерживать добавление новых функциональных блоков (требование пользователя)
- Офлайн-работа: система должна работать без интернета для локальных операций
- Сохранение состояния должно быть надежным (100% успешность восстановления, SC-007)

**Scale/Scope**: 
- Поддержка тысяч файлов с зависимостями
- До 50 последних действий в истории (SC-010)
- Комбинирование 5+ коллекций без потери данных (SC-005)
- Множественный выбор до 50+ файлов для пакетных операций

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Component-First Architecture ✅
- ✅ Зависимости файлов реализуются как отдельные компоненты (DependencyGraph, DependencyEditor)
- ✅ UI-конструктор логики коллекций - отдельный компонент (CollectionLogicBuilder)
- ✅ Формы добавления/редактирования файлов - переиспользуемые компоненты
- ✅ Все компоненты самодостаточны и независимо тестируемы

### II. Type Safety (NON-NEGOTIABLE) ✅
- ✅ Frontend: TypeScript strict mode, все типы определены для моделей зависимостей
- ✅ Backend: Rust типы для File, Dependency, Collection, CollectionLogicRule
- ✅ Нет `any` типов, все интерфейсы Tauri команд типизированы

### III. Test-Driven Development (TDD) ✅
- ✅ Критическая логика (обнаружение циклических зависимостей, валидация) требует тестов перед реализацией
- ✅ Frontend: Vitest + Testing Library для компонентов управления зависимостями
- ✅ Backend: Rust тесты для алгоритмов графа зависимостей

### IV. Separation of Concerns ✅
- ✅ Frontend: UI компоненты для управления зависимостями, визуализация графа
- ✅ Backend: Бизнес-логика зависимостей, валидация, работа с БД
- ✅ Коммуникация только через Tauri команды

### V. Universal Extensibility ✅
- ✅ Система зависимостей универсальна (не привязана к конкретным играм)
- ✅ UI-конструктор логики коллекций позволяет добавлять новые типы условий через UI
- ✅ Поддержка добавления новых функциональных блоков (требование пользователя)

### VI. Documentation & Communication (NON-NEGOTIABLE) ✅
- ✅ Вся документация на русском языке
- ✅ JSDoc для TypeScript функций
- ✅ Rust документация для всех публичных функций
- ✅ README в каждом новом модуле

### VII. Proactive Problem-Solving ✅
- ✅ При реализации предлагать несколько вариантов решений
- ✅ Проактивно предлагать улучшения после завершения задач

**Статус**: ✅ Все принципы соблюдены. Можно переходить к Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/                                    # Frontend (Svelte 5)
├── components/
│   ├── dependencies/                   # Новые компоненты для зависимостей
│   │   ├── DependencyGraph.svelte      # Визуализация графа зависимостей
│   │   ├── DependencyEditor.svelte    # Редактор зависимостей файла
│   │   ├── DependencyWarning.svelte   # Предупреждения о зависимостях
│   │   └── VersionSelector.svelte     # Выбор версии зависимости
│   ├── collections/                    # Компоненты для коллекций
│   │   ├── CollectionLogicBuilder.svelte  # UI-конструктор логики
│   │   ├── CollectionComposer.svelte  # Объединение коллекций
│   │   └── CollectionViewer.svelte    # Просмотр файлов из коллекций
│   ├── files/                          # Компоненты для файлов
│   │   ├── FileForm.svelte             # Форма добавления/редактирования
│   │   ├── FileList.svelte             # Список файлов с множественным выбором
│   │   ├── BatchOperations.svelte      # Пакетные операции
│   │   └── FileDragDrop.svelte        # Drag & drop для сортировки
│   ├── import-export/                  # Импорт/экспорт
│   │   ├── ImportDialog.svelte         # Диалог импорта
│   │   └── ExportDialog.svelte         # Диалог экспорта
│   └── session/                        # Сохранение состояния
│       └── SessionRestore.svelte       # Восстановление сессии
├── lib/
│   ├── dependencies/                   # Утилиты для зависимостей
│   │   ├── dependency-graph.ts         # Логика графа зависимостей
│   │   ├── dependency-validator.ts     # Валидация зависимостей
│   │   └── version-resolver.ts          # Разрешение версий
│   ├── collections/                    # Утилиты для коллекций
│   │   ├── collection-logic.ts        # Логика условий коллекций
│   │   └── collection-composer.ts      # Объединение коллекций
│   ├── import-export/                  # Импорт/экспорт
│   │   └── json-serializer.ts           # Сериализация в JSON
│   └── session/                        # Сохранение состояния
│       └── session-state.ts            # Управление состоянием сессии
└── types/
    ├── dependency.ts                   # Типы для зависимостей
    ├── collection.ts                   # Типы для коллекций
    └── session.ts                       # Типы для состояния сессии

src-tauri/                              # Backend (Rust)
└── src/
    ├── commands/
    │   ├── dependencies.rs              # Tauri команды для зависимостей
    │   ├── collections.rs               # Tauri команды для коллекций
    │   ├── files.rs                     # Tauri команды для файлов (расширение)
    │   └── import_export.rs             # Tauri команды для импорта/экспорта
    ├── models/
    │   ├── file.rs                      # Модель File (name@version)
    │   ├── dependency.rs                # Модель FileDependency
    │   ├── collection.rs               # Модель Collection
    │   ├── collection_logic.rs          # Модель CollectionLogicRule
    │   └── session_state.rs             # Модель SessionState
    ├── services/
    │   ├── dependency_service.rs        # Сервис работы с зависимостями
    │   │   ├── graph.rs                 # Алгоритмы графа зависимостей
    │   │   ├── validator.rs             # Валидация зависимостей
    │   │   └── resolver.rs              # Разрешение конфликтов версий
    │   ├── collection_service.rs        # Сервис работы с коллекциями
    │   │   └── logic_evaluator.rs        # Вычисление логики коллекций
    │   └── import_export_service.rs     # Сервис импорта/экспорта
    ├── database/
    │   ├── migrations/                  # Миграции БД
    │   │   ├── 003_add_dependencies.sql  # Таблицы зависимостей
    │   │   ├── 004_add_collections.sql   # Таблицы коллекций
    │   │   └── 005_add_session_state.sql # Таблица состояния сессии
    │   └── schema.rs                    # Расширение схемы БД
    └── utils/
        └── json_serializer.rs            # Сериализация/десериализация JSON

tests/                                   # Тесты
├── frontend/
│   ├── components/
│   │   └── dependencies/               # Тесты компонентов зависимостей
│   └── lib/
│       └── dependencies/               # Тесты утилит
└── backend/
    ├── services/
    │   └── dependency_service/          # Тесты сервисов
    └── integration/
        └── dependencies/                # Интеграционные тесты
```

**Structure Decision**: Используется существующая структура Tauri приложения (frontend в `src/`, backend в `src-tauri/`). Новые компоненты добавляются в соответствующие директории с группировкой по функциональности (dependencies, collections, files, import-export, session). Backend модули организованы по слоям: commands (Tauri API), models (структуры данных), services (бизнес-логика), database (работа с БД).

