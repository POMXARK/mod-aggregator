# Навык анализа кода и рефакторинга

Этот навык автоматизирует процесс анализа размера файлов в проекте и выявления кандидатов на рефакторинг.

## Быстрый старт

1. **Установите навык:**
   ```bash
   # Навык уже установлен в .claude/skills/code-analysis-refactoring/
   ```

2. **Запустите анализ:**
   ```bash
   # Через Claude Code с запросом:
   "Проведи анализ кода проекта и найди кандидатов на рефакторинг"
   ```

## Возможности

### 📊 Анализ размера файлов
- Подсчет строк кода во всех файлах проекта
- Категоризация по размеру (критические, высокий приоритет, средний приоритет)
- Исключение системных директорий (node_modules, .git, etc.)

### 📋 Генерация отчетов
- Детальный отчет `REFACTORING-ANALYSIS.md`
- Обновление статистики в `README.md`
- Краткий вывод в консоль

### 🔧 Рекомендации по рефакторингу
- Конкретные советы для каждой категории файлов
- Примеры паттернов рефакторинга
- Приоритезация задач

## Структура файлов

```
.claude/skills/code-analysis-refactoring/
├── SKILL.md              # Основное описание навыка
├── README.md             # Это файл
├── generate-report.js    # Скрипт генерации отчетов
└── config-template.js    # Шаблон конфигурации
```

## Использование в проекте

### Добавление команд в package.json

```json
{
  "scripts": {
    "analyze-refactoring": "node scripts/update-code-stats.js",
    "check-refactoring": "node scripts/quick-refactor-check.js"
  }
}
```

### Интеграция в CI/CD

```yaml
# .github/workflows/code-analysis.yml
name: Code Analysis
on: [push, pull_request]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      - run: npm install
      - run: npm run analyze-refactoring
      - uses: actions/upload-artifact@v3
        with:
          name: refactoring-report
          path: REFACTORING-ANALYSIS.md
```

## Настройка

### Кастомные пороги

Скопируйте `config-template.js` в корень проекта и настройте:

```javascript
export const refactoringConfig = {
  thresholds: {
    critical: 1500,    // Для вашего проекта
    high: 800,
    medium: 400,
    low: 150
  }
};
```

### Исключения

Настройте директории и файлы для исключения:

```javascript
excludeDirs: [
  'node_modules',
  'dist',
  'build',
  'coverage',
  // Добавьте свои
],
excludeFiles: [
  '*.min.js',
  '*.log',
  // Добавьте свои паттерны
]
```

## Категории анализа

### 🚨 Критические файлы (>2000 строк)
**Требуют немедленного внимания:**
- Разбить на модули/компоненты
- Вынести общую логику
- Применить паттерны проектирования

### ⚠️ Высокий приоритет (1000-2000 строк)
**Рекомендуется рефакторинг:**
- Извлечь функции/методы
- Разделить на подкомпоненты
- Проверить на дублирование

### 📋 Средний приоритет (500-1000 строк)
**Мониторить и планировать:**
- Следить за ростом
- Извлечь утилиты
- Проверить single responsibility

## Примеры использования

### Еженедельный анализ
```
"Проанализируй код проекта и покажи что нужно рефакторить"
```

### Проверка перед коммитом
```
"Проверь размер файлов перед коммитом"
```

### Мониторинг трендов
```
"Какие файлы сильно выросли за последний месяц?"
```

## Устранение неполадок

### Навык не активируется
1. Проверьте описание в SKILL.md
2. Попробуйте другие формулировки запроса
3. Убедитесь что навык в правильной директории

### Скрипты не работают
1. Проверьте наличие Node.js
2. Установите зависимости: `npm install`
3. Проверьте права доступа к файлам

### Неправильные результаты
1. Проверьте исключения директорий
2. Сравните с ручным подсчетом: `wc -l файл`
3. Настройте пороги в конфигурации

## Расширение навыка

### Добавление новых метрик
```javascript
// В generate-report.js добавить:
function calculateComplexity(filePath) {
  // Логика расчета цикломатической сложности
}

function analyzeDependencies(filePath) {
  // Анализ зависимостей между файлами
}
```

### Интеграция с другими инструментами
```javascript
// ESLint интеграция
function runEslintAnalysis() {
  // Запуск ESLint и анализ результатов
}

// Coverage анализ
function analyzeTestCoverage() {
  // Анализ покрытия тестами
}
```

## Полезные ссылки

- [Claude Code Skills Guide](https://docs.anthropic.com/claude/docs/skills)
- [Code Metrics Best Practices](https://martinfowler.com/bliki/CodeMetrics.html)
- [Refactoring Patterns](https://refactoring.guru/design-patterns)

## Контрибьютинг

1. Fork the skill
2. Add your improvements
3. Test thoroughly
4. Submit a pull request

## Лицензия

Этот навык распространяется под MIT лицензией.