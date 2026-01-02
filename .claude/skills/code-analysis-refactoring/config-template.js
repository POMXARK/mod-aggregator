/**
 * Шаблон конфигурации для анализа рефакторинга
 * Скопируйте этот файл в scripts/ и настройте под свои нужды
 */

export const refactoringConfig = {
  // Пороги для категоризации файлов (строки кода)
  thresholds: {
    critical: 2000,    // > 2000 строк - срочно рефакторить
    high: 1000,       // 1000-2000 строк - рекомендуется рефакторить
    medium: 500,      // 500-1000 строк - рассмотреть рефакторинг
    low: 200          // 200-500 строк - мониторить
  },

  // Расширения файлов для анализа
  extensions: [
    '.rs',      // Rust
    '.ts',      // TypeScript
    '.tsx',     // TypeScript React
    '.js',      // JavaScript
    '.jsx',     // JavaScript React
    '.svelte',  // Svelte
    '.vue',     // Vue.js
    '.py',      // Python
    '.java',    // Java
    '.cpp',     // C++
    '.c',       // C
    '.go',      // Go
    '.php',     // PHP
  ],

  // Директории для исключения из анализа
  excludeDirs: [
    'node_modules',
    'target',
    'dist',
    'build',
    '.git',
    '.cursor',
    '.claude',
    '.storybook',
    'coverage',
    '.nyc_output',
    'docs',
    'website/build',
    'website/.docusaurus',
  ],

  // Файлы для исключения (паттерны)
  excludeFiles: [
    '*.min.js',
    '*.min.css',
    'package-lock.json',
    'yarn.lock',
    'pnpm-lock.yaml',
    '*.log',
    '.DS_Store',
    'Thumbs.db',
  ],

  // Настройки отчета
  report: {
    maxFilesPerCategory: 20,    // Максимум файлов в каждой категории
    includeLanguageStats: true, // Включать статистику по языкам
    includeTrends: true,        // Включать анализ трендов (если есть история)
    generateMarkdown: true,     // Генерировать Markdown отчет
    generateJSON: false,        // Генерировать JSON отчет
  },

  // Кастомные правила для специфических типов файлов
  customRules: {
    // Для Svelte компонентов более строгие правила
    '.svelte': {
      critical: 1500,  // Svelte компоненты не должны быть > 1500 строк
      high: 800,
    },

    // Для тестовых файлов более мягкие правила
    'test': {
      critical: 3000,  // Тестовые файлы могут быть крупнее
      high: 1500,
    },

    // Для конфигурационных файлов
    'config': {
      critical: 1000,  // Конфиги не должны быть слишком большими
      high: 500,
    },
  },

  // Настройки для CI/CD
  ci: {
    failOnCriticalFiles: true,    // Падать если есть критические файлы
    failOnHighPriorityFiles: false, // Падать если есть файлы высокого приоритета
    maxCriticalFiles: 0,          // Максимум критических файлов
    maxHighPriorityFiles: 5,      // Максимум файлов высокого приоритета
  },

  // Уведомления и интеграции
  notifications: {
    slack: false,        // Отправлять в Slack
    email: false,        // Отправлять на email
    github: true,        // Создавать GitHub issue для критических файлов
  },
};

/**
 * Функция для получения настроек с учетом типа файла
 */
export function getFileConfig(filePath, baseConfig = refactoringConfig) {
  const fileName = filePath.toLowerCase();

  // Проверяем кастомные правила
  for (const [pattern, rules] of Object.entries(baseConfig.customRules)) {
    if (fileName.includes(pattern)) {
      return { ...baseConfig.thresholds, ...rules };
    }
  }

  return baseConfig.thresholds;
}

/**
 * Функция для проверки, нужно ли исключить файл
 */
export function shouldExcludeFile(filePath, config = refactoringConfig) {
  const fileName = path.basename(filePath);

  // Проверяем паттерны исключения
  for (const pattern of config.excludeFiles) {
    if (fileName.match(pattern.replace('*', '.*'))) {
      return true;
    }
  }

  return false;
}