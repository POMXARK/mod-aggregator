// For more info, see https://github.com/storybookjs/eslint-plugin-storybook#configuration-flat-config-format
import storybook from "eslint-plugin-storybook";

import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import unusedImports from 'eslint-plugin-unused-imports';
import globals from 'globals';
import ts from 'typescript-eslint';
import svelteConfig from './svelte.config.js';
import customRules from './eslint-plugin-custom-rules.js';

/** @type {import('eslint').Linter.Config[]} */
export default ts.config(
  js.configs.recommended,
  ...ts.configs.recommended,
  ...svelte.configs.recommended,
  {
    plugins: {
      'custom-rules': customRules,
      'unused-imports': unusedImports,
    },
  },
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
        // Svelte 5 runes
        $state: 'readonly',
        $derived: 'readonly',
        $effect: 'readonly',
        $props: 'readonly',
        $bindable: 'readonly',
        // Browser APIs
        alert: 'readonly',
        confirm: 'readonly',
        prompt: 'readonly',
      }
    }
  },
  {
    files: ['**/*.{ts,js,svelte}', '**/*.svelte.ts', '**/*.svelte.js'],
    languageOptions: {
      parserOptions: {
        projectService: true,
        extraFileExtensions: ['.svelte'],
        parser: ts.parser,
        svelteConfig
      }
    }
  },
  {
    files: ['**/*.svelte'],
    rules: {
      'custom-rules/require-svelte-extension': 'error', // Проверяем расширения .svelte в импортах
      'custom-rules/no-bind-functions': 'error', // Запрещаем bind: с функциями
      'prefer-const': 'off', // Отключаем для Svelte файлов - bindable переменные должны быть let
      'no-redeclare': 'error', // Обнаружение повторного объявления переменных
      // Усиленные правила для обнаружения неиспользуемых переменных в Svelte
      '@typescript-eslint/no-unused-vars': ['error', {
        argsIgnorePattern: '^_',
        varsIgnorePattern: '^_',
        caughtErrorsIgnorePattern: '^_',
        destructuredArrayIgnorePattern: '^_',
        ignoreRestSiblings: false,
        // Специальные настройки для Svelte 5 reactive переменных
        vars: 'all',
        args: 'all',
        caughtErrors: 'all',
        ignoreRestSiblings: false
      }],
      'no-unused-vars': 'off', // Отключаем базовое правило
      'unused-imports/no-unused-imports': 'error',
      'unused-imports/no-unused-vars': ['error', {
        vars: 'all',
        varsIgnorePattern: '^(_|unused)',
        args: 'after-used',
        argsIgnorePattern: '^_',
        ignoreRestSiblings: false,
        // Дополнительные проверки для Svelte
        varsIgnorePattern: '^_',
        caughtErrorsIgnorePattern: '^_'
      }],
      // Дополнительные правила для Svelte
      'svelte/valid-compile': 'error',
    },
  },
  {
    files: ['**/*.ts', '**/*.tsx', '**/*.js'],
    rules: {
      'prefer-const': 'error', // Включаем только для не-Svelte файлов
      'no-redeclare': 'error', // Обнаружение повторного объявления переменных
      // Строгие правила для обнаружения неиспользуемых переменных и импортов
      '@typescript-eslint/no-unused-vars': ['error', {
        argsIgnorePattern: '^_',
        varsIgnorePattern: '^_',
        caughtErrorsIgnorePattern: '^_',
        destructuredArrayIgnorePattern: '^_'
      }],
      'unused-imports/no-unused-imports': 'error',
      'unused-imports/no-unused-vars': ['error', {
        vars: 'all',
        varsIgnorePattern: '^_',
        args: 'after-used',
        argsIgnorePattern: '^_'
      }],
    },
  },
  // Специальные правила для Svelte компонентов
  {
    files: ['**/*.svelte'],
    rules: {
      // Дополнительные правила для обнаружения неиспользуемых переменных в Svelte
      'svelte/no-unused-svelte-ignore': 'warn',
      'svelte/valid-compile': 'error',
    },
  },
  {
    rules: {
      '@typescript-eslint/explicit-function-return-type': 'off',
      '@typescript-eslint/no-explicit-any': 'warn',
      '@typescript-eslint/no-non-null-assertion': 'warn',
      'no-console': 'off', // Временно отключаем, чтобы не мешать разработке
      'no-debugger': 'error',
      // 'prefer-const': 'error', // Отключено глобально для Svelte
      'no-var': 'error',
      'eqeqeq': ['error', 'always'],
      'curly': ['error', 'all'],
      'no-redeclare': 'error', // Обнаружение повторного объявления переменных (как в WebStorm)
      'no-unused-vars': 'off', // Отключаем базовое правило, используем TypeScript версию
      'custom-rules/require-svelte-extension': 'error', // Проверяем расширения .svelte в импортах
    },
  },
  {
    ignores: [
      'node_modules/',
      'dist/',
      'build/',
      'src-tauri/',
      'coverage/',
      '.svelte-kit/',
      '*.config.js',
      '*.config.ts',
      'vite.config.ts',
      'vitest.config.ts',
      'tailwind.config.js',
      'postcss.config.js',
      'public/',
      'website/',
      '**/*.test.*',
      '**/*.spec.*',
      'cypress/',
      '**/*.md',
      'scripts/',
      '*.bat',
      '*.sh',
      '*.ps1',
      'docs/',
      'examples/',
      'memory/',
      'porto_ru/',
      'specs/',
      'templates/',
      'tests/',
      // Исключаем git worktrees из линтера
      '**/.cursor/**',
      '**/worktrees/**',
      'C:/Users/User/.cursor/**',
    ],
  },
  storybook.configs["flat/recommended"]
);