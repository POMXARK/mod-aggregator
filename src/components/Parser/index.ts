/**
 * Parser Container - Контейнер конструктора парсеров
 * Отвечает за визуальное создание и тестирование парсеров
 */

// UI компоненты
export { default as ParserBuilder } from './UI/ParserBuilder.svelte';

// Конструктор компоненты
export { default as BrowserPanel } from './UI/constructor/BrowserPanel.svelte';
export { default as ChatPanel } from './UI/constructor/ChatPanel.svelte';
export { default as GeneratedCodePanel } from './UI/constructor/GeneratedCodePanel.svelte';
export { default as NodeEditor } from './UI/constructor/NodeEditor.svelte';

// Runner компоненты
export { default as ParserRunner } from './UI/constructor/ParserRunner.svelte';
export * from './UI/constructor/parser-runner.svelte';

// Узлы парсера
export * from './UI/nodes.svelte';

// Фреймворк
export * from './UI/framework.svelte';

// Data слой (пока пустой, будет содержать модели парсеров)
// export * from './Data';

// Конфигурация (пока пустая, будет содержать настройки парсера)
// export * from './Config';
