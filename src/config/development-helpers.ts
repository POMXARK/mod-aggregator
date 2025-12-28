/**
 * Development Helpers - быстрые функции для переключения режимов разработки
 *
 * Использование в браузерной консоли:
 * import('./config/development-helpers.js').then(m => {
 *   m.enableParserOnly();
 *   m.enableSitesOnly();
 *   m.enableAllComponents();
 * });
 */

import { applyDevelopmentProfile, resetToFullConfig } from './components';

// Функции для быстрого переключения профилей
export function enableParserOnly() {
  console.log('🔧 Switching to Parser-only development mode...');
  applyDevelopmentProfile('parserOnly');
  console.log('✅ Parser-only mode enabled. Reload the page to see changes.');
}

export function enableSitesOnly() {
  console.log('🌐 Switching to Sites-only development mode...');
  applyDevelopmentProfile('sitesOnly');
  console.log('✅ Sites-only mode enabled. Reload the page to see changes.');
}

export function enableFilesOnly() {
  console.log('📁 Switching to Files-only development mode...');
  applyDevelopmentProfile('filesOnly');
  console.log('✅ Files-only mode enabled. Reload the page to see changes.');
}

export function enableAllComponents() {
  console.log('✅ Switching to full application mode...');
  resetToFullConfig();
  console.log('✅ All components enabled. Reload the page to see changes.');
}

// Текущий статус
export function getCurrentConfig() {
  const saved = localStorage.getItem('components-config');
  const profile = localStorage.getItem('active-profile');
  console.log('Current configuration:', saved ? JSON.parse(saved) : 'default');
  console.log('Active profile:', profile || 'none');
  return { config: saved ? JSON.parse(saved) : null, profile };
}

// Сброс всего
export function resetEverything() {
  console.log('🔄 Resetting everything...');
  localStorage.removeItem('components-config');
  localStorage.removeItem('active-profile');
  console.log('✅ All settings reset. Reload the page.');
}

// Доступные команды
export const commands = {
  parser: enableParserOnly,
  sites: enableSitesOnly,
  files: enableFilesOnly,
  all: enableAllComponents,
  status: getCurrentConfig,
  reset: resetEverything,
};

console.log(`
🚀 Development Helpers Loaded!

Available commands in browser console:
• enableParserOnly()  - Focus on Parser development
• enableSitesOnly()   - Focus on Sites management
• enableFilesOnly()   - Focus on Files/Collections
• enableAllComponents() - Full application
• getCurrentConfig()  - Show current settings
• resetEverything()   - Reset all settings

Example:
enableParserOnly();
`);

