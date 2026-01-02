# Руководство по тестированию Mod Aggregator

Это руководство описывает стратегию тестирования проекта Mod Aggregator и предоставляет инструкции по запуску различных типов тестов.

## Архитектура тестирования

Проект использует многоуровневый подход к тестированию для обеспечения качества и контролируемого рефакторинга:

### 1. Unit тесты (Vitest)
- **Назначение**: Тестирование отдельных функций и модулей в изоляции
- **Инструменты**: Vitest, @testing-library/svelte
- **Расположение**: `src/**/*.test.ts`, `src/**/*.test.svelte`

### 2. Property-based тесты (fast-check)
- **Назначение**: Тестирование бизнес логики с произвольными входными данными
- **Инструменты**: fast-check + Vitest
- **Расположение**: `src/**/*.property.test.ts`

### 3. Integration тесты (Vitest + Testing Library)
- **Назначение**: Тестирование взаимодействия компонентов
- **Инструменты**: @testing-library/svelte, user-event
- **Расположение**: `src/**/*.integration.test.ts`

### 4. Visual тесты (Storybook)
- **Назначение**: Визуальное тестирование компонентов в изоляции
- **Инструменты**: Storybook для Svelte
- **Расположение**: `src/**/*.stories.svelte`

### 5. E2E тесты (Playwright)
- **Назначение**: Тестирование полного пользовательского сценария
- **Инструменты**: Playwright
- **Расположение**: `tests/e2e/**/*.spec.ts`

## Запуск тестов

### Все тесты
```bash
npm test
```

### Unit тесты только
```bash
npm run test:run
```

### Тесты с UI
```bash
npm run test:ui
```

### E2E тесты
```bash
npx playwright test
```

### E2E тесты с UI
```bash
npx playwright test --ui
```

### Storybook
```bash
npm run storybook
```

## Структура тестов

### Unit тесты бизнес логики

#### CollectionLogic (`src/lib/collections/__tests__/collection-logic.test.ts`)
Тестирует основную бизнес логику коллекций:
- Валидация параметров условий
- Создание правил разных типов
- Генерация описаний правил
- Проверка зависимостей между правилами
- Валидация наборов правил
- Расчет статистики вычисления

```typescript
describe('CollectionLogic', () => {
  describe('validateConditionParams', () => {
    // Тесты валидации параметров для разных типов условий
  });

  describe('createBooleanRule', () => {
    // Тесты создания boolean правил
  });

  // ... остальные тесты
});
```

#### useParserBuilderState (`src/lib/composables/__tests__/useParserBuilderState.test.ts`)
Тестирует управление состоянием UI с миграциями:
- Миграции состояния между версиями
- Генерация ключей состояния
- Сохранение и восстановление состояния
- Экспорт/импорт состояния
- Обработка ошибок

### Property-based тесты

#### CollectionLogic Property Tests (`src/lib/collections/__tests__/collection-logic.property.test.ts`)
Тестирует бизнес логику с произвольными входными данными:
- Инварианты валидации параметров
- Свойства функций создания правил
- Корректность расчетов статистики
- Обработка краевых случаев

```typescript
describe('CollectionLogic - Property-based Tests', () => {
  it('should always return boolean for any condition type and params', () => {
    fc.assert(
      fc.property(conditionTypeArb, fc.anything(), (conditionType, params) => {
        const result = CollectionLogic.validateConditionParams(conditionType, params);
        expect(typeof result).toBe('boolean');
      })
    );
  });
});
```

### Integration тесты

#### ParserBuilder (`src/components/__tests__/ParserBuilder.integration.test.ts`)
Тестирует взаимодействие компонентов парсер билдера:
- Загрузка сайтов
- Ввод URL и загрузка страниц
- Создание и управление нодами
- Генерация и выполнение кода
- Управление панелями
- Сохранение состояния

### Visual тесты (Storybook)

#### Компоненты с stories
- **Sidebar** (`src/components/Sidebar.stories.svelte`): Навигация и управление сайтами
- **ModCard** (`src/components/ModCard.stories.svelte`): Отображение информации о моде
- **CollectionViewer** (`src/components/collections/CollectionViewer.stories.svelte`): Просмотр коллекций

Stories включают:
- Базовые состояния компонентов
- Разные размеры и конфигурации
- Интерактивные сценарии
- Состояния загрузки и ошибок
- Accessibility тесты

### E2E тесты

#### App Navigation (`tests/e2e/app-navigation.spec.ts`)
Тестирует основные пользовательские сценарии:
- Навигация между страницами
- Загрузка и отображение контента
- Взаимодействие с формами
- Обработка ошибок
- Адаптивный дизайн
- Доступность

## Стратегия тестирования

### Принципы

1. **TDD (Test-Driven Development)**: Тесты пишутся перед кодом
2. **Outside-In**: Начинаем с высокоуровневых тестов (E2E), затем спускаемся к unit тестам
3. **Property-based**: Для критичной бизнес логики используем property-based тесты
4. **Visual Testing**: Storybook для визуального регресса компонентов

### Когда писать тесты

#### Обязательно тестировать:
- ✅ Бизнес логика (CollectionLogic, миграции состояния)
- ✅ Компоненты с состоянием и логикой
- ✅ API взаимодействия (Tauri calls)
- ✅ Формы и валидация
- ✅ Критические пользовательские пути

#### Опционально тестировать:
- 🔸 Простые presentational компоненты (если нет логики)
- 🔸 Стилизация (через visual regression в Storybook)

### Покрытие тестами

Цель: > 80% покрытия по statements, branches и functions

```bash
# Проверка покрытия
npm run test:run -- --coverage
```

## Миграции и рефакторинг

### Тестирование миграций состояния

Для безопасного рефакторинга состояния UI созданы специальные тесты миграций:

```typescript
describe('migrateUIState', () => {
  describe('migration from version 0 to 1', () => {
    it('should add missing pageViewerWidth with default value', () => {
      // Тест миграции с добавлением полей
    });
  });
});
```

### Контролируемый рефакторинг

1. **Написать тесты** для существующей функциональности
2. **Рефакторить код** с постоянным запуском тестов
3. **Добавить новые тесты** для новой функциональности
4. **Проверить покрытие** и качество тестов

## Инструменты и зависимости

### Основные
- **Vitest**: Быстрый тестовый раннер для Vite
- **@testing-library/svelte**: Утилиты для тестирования Svelte компонентов
- **@testing-library/user-event**: Симуляция пользовательских взаимодействий
- **fast-check**: Property-based тестирование
- **Storybook**: Visual testing и документация компонентов
- **Playwright**: E2E тестирование

### Конфигурация

#### Vitest (`vitest.config.ts`)
```typescript
export default defineConfig({
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
  },
  // ... остальная конфигурация
});
```

#### Playwright (`playwright.config.ts`)
```typescript
export default defineConfig({
  testDir: './tests/e2e',
  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:5173',
  },
  // ... остальная конфигурация
});
```

## Лучшие практики

### Написание тестов

1. **Описательные названия**: `it('should validate boolean condition params')`
2. **AAA паттерн**: Arrange, Act, Assert
3. **Изоляция**: Каждый тест независим
4. **Mock внешние зависимости**: API calls, localStorage
5. **Тестировать поведение**: Не реализацию

### Примеры хороших тестов

```typescript
describe('CollectionLogic.validateConditionParams', () => {
  it('should validate boolean condition', () => {
    // Arrange
    const conditionType = 'boolean';
    const conditionParams = { value: true };

    // Act
    const result = CollectionLogic.validateConditionParams(conditionType, conditionParams);

    // Assert
    expect(result).toBe(true);
  });
});
```

### Mock стратегии

```typescript
// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Mock localStorage
const localStorageMock = {
  getItem: vi.fn(),
  setItem: vi.fn(),
};
Object.defineProperty(window, 'localStorage', { value: localStorageMock });
```

## CI/CD интеграция

### GitHub Actions
```yaml
- name: Run tests
  run: |
    npm run lint:all
    npm run type-check
    npm run test:run
    npx playwright install
    npx playwright test
```

### Pre-commit hooks
```bash
# Проверка перед коммитом
npm run lint:all && npm run test:run
```

## Troubleshooting

### Распространенные проблемы

1. **Тесты не запускаются**: Проверить `vitest.config.ts` и импорты
2. **Mock не работает**: Проверить порядок импортов (mock перед import)
3. **E2E падает**: Проверить `playwright.config.ts` и dev server
4. **Storybook не строится**: Проверить конфликты зависимостей

### Debug режим

```bash
# Vitest с debug
npm run test:run -- --reporter=verbose

# Playwright с video
npx playwright test --headed

# Storybook с debug
npm run storybook -- --loglevel=debug
```

## Ресурсы

- [Vitest документация](https://vitest.dev/)
- [Testing Library](https://testing-library.com/docs/svelte-testing-library/intro/)
- [Playwright документация](https://playwright.dev/)
- [Storybook для Svelte](https://storybook.js.org/docs/get-started/svelte)
- [fast-check документация](https://fast-check.dev/)

