import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
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
    },
  },
  {
    rules: {
      '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
      '@typescript-eslint/explicit-function-return-type': 'off',
      '@typescript-eslint/no-explicit-any': 'warn',
      '@typescript-eslint/no-non-null-assertion': 'warn',
      'no-console': 'off', // Временно отключаем, чтобы не мешать разработке
      'no-debugger': 'error',
      'prefer-const': 'error',
      'no-var': 'error',
      'eqeqeq': ['error', 'always'],
      'curly': ['error', 'all'],
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
    ],
  }
);