import { invoke } from '@/lib/tauri-wrapper';
import { writable, get } from 'svelte/store';
import type { Node, Edge } from '@xyflow/svelte';
import type { ParserSettings } from './useParserSettings.svelte';

export interface ParserResult {
  data: Record<string, any>;
  expanded: boolean;
}

export interface ParserRunnerState {
  isRunning: boolean;
  progress: string;
  error: string | null;
  results: ParserResult[];
  diagnostics: any[];
  extractionStats: any | null;
}

export interface UseParserRunnerOptions {
  nodes: Node[];
  edges: Edge[];
  currentUrl: string;
  siteId?: number | null;
  getSettings: () => ParserSettings;
  onResults?: (results: any[], diagnostics: any[], stats: any) => void;
  onError?: (error: string) => void;
}

/**
 * Composable для запуска парсера
 *
 * Управляет выполнением парсера: получение HTML, запуск парсера,
 * обработка результатов и ошибок, возможность остановки выполнения.
 *
 * @param options - опции для настройки парсера
 * @returns Объект с состоянием и методами для управления парсером
 */
export function useParserRunner(options: UseParserRunnerOptions) {
  let { nodes, edges, currentUrl, siteId, getSettings, onResults, onError } = options;

  // Создаем реактивные stores
  const isRunning = writable(false);
  const progress = writable('');
  const error = writable<string | null>(null);
  const results = writable<ParserResult[]>([]);
  const diagnostics = writable<any[]>([]);
  const extractionStats = writable<any>(null);
  const isCancelled = writable(false);

  // Для остановки парсера
  let abortController: AbortController | null = null;

  /**
   * Останавливает выполнение парсера
   *
   * ВАЖНО: Rust код не может быть остановлен немедленно после начала выполнения.
   * AbortController отменяет только JavaScript промисы, но Rust код выполняется
   * синхронно в цикле обработки элементов. Парсер остановится автоматически
   * при достижении таймаута или завершении обработки текущих элементов.
   */
  function stopParser() {
    isCancelled.set(true);
    if (abortController) {
      abortController.abort();
      abortController = null;
    }
    isRunning.set(false);
    progress.set(
      'Запрос на остановку парсера отправлен. Rust код может продолжить работу до таймаута или завершения текущих элементов.'
    );
  }

  /**
   * Запускает парсер с таймаутом и улучшенной обработкой ошибок
   */
  async function runParser() {
    if (nodes.length === 0) {
      const errMsg = 'Создайте ноды парсера перед тестированием';
      error.set(errMsg);
      if (onError) {
        onError(errMsg);
      }
      return;
    }

    if (!currentUrl) {
      const errMsg = 'Загрузите страницу для тестирования';
      error.set(errMsg);
      if (onError) {
        onError(errMsg);
      }
      return;
    }

    // Сброс состояния
    isRunning.set(true);
    isCancelled.set(false);
    error.set(null);
    results.set([]);
    diagnostics.set([]);
    extractionStats.set(null);
    progress.set('Получение HTML страницы...');

    // Создаем AbortController для возможности остановки
    abortController = new AbortController();
    const signal = abortController.signal;

    try {
      // Получаем HTML с таймаутом
      progress.set('Получение HTML страницы...');

      // Проверяем отмену перед получением HTML
      if (isCancelled || signal.aborted) {
        throw new Error('Операция отменена');
      }

      const htmlPromise = invoke<string>('fetch_page', {
        url: currentUrl,
        forceRefresh: false,
        siteId: siteId || null,
      });

      const timeoutPromise = new Promise<string>((_, reject) =>
        setTimeout(() => {
          if (!isCancelled && !signal.aborted) {
            reject(new Error('Таймаут получения HTML (30 секунд)'));
          }
        }, 30000)
      );

      const html = (await Promise.race([htmlPromise, timeoutPromise])) as string;

      // Проверяем отмену после получения HTML
      if (isCancelled || signal.aborted) {
        throw new Error('Операция отменена');
      }

      if (!html) {
        throw new Error('Не удалось получить HTML страницы');
      }

      progress.set(`HTML получен (${(html.length / 1024).toFixed(1)} KB). Подготовка данных...`);

      // Преобразуем nodes и edges в формат для Tauri
      // const nodesData = nodes.map(node => ({
      //   id: node.id,
      //   type: node.type,
      //   data: node.data,
      // }));

      // const edgesData = edges.map(edge => ({
      //   source: edge.source,
      //   target: edge.target,
      // }));

      progress.set('Запуск парсера...');

      // Проверяем отмену перед запуском
      // TODO: нужно получить текущее значение isCancelled из store

      // Преобразуем nodes и edges в формат для Tauri
      const nodesData = nodes.map(node => ({
        id: node.id,
        type: node.type,
        data: node.data,
      }));

      const edgesData = edges.map(edge => ({
        source: edge.source,
        target: edge.target,
      }));

      // Запускаем парсер с настройками и таймаутом
      const currentSettings = getSettings();
      const timeoutMs = currentSettings.timeoutSeconds * 1000;
      const parserTimeoutPromise = new Promise<any>((_, reject) => {
        const timeoutId = setTimeout(() => {
          if (!isCancelled && !signal.aborted) {
            reject(new Error(`Таймаут выполнения парсера (${currentSettings.timeoutSeconds} секунд)`));
          }
        }, timeoutMs);

        // Отслеживаем отмену и очищаем таймаут
        signal.addEventListener('abort', () => {
          clearTimeout(timeoutId);
          reject(new Error('Операция отменена'));
        });
      });

      const parserPromise = invoke<any>('test_parser_from_nodes', {
        html,
        nodes: nodesData,
        edges: edgesData,
        maxElements: currentSettings.maxElements > 0 ? currentSettings.maxElements : null,
        timeoutMillis: timeoutMs > 0 ? timeoutMs : null,
        slowMode: currentSettings.slowMode,
        delayPerElementMs: currentSettings.slowMode ? currentSettings.delayPerElement : 0,
      }).catch(err => {
        // Если операция была отменена, пробрасываем специальную ошибку
        if (isCancelled || signal.aborted) {
          throw new Error('Операция отменена');
        }
        throw err;
      });

      const result = (await Promise.race([parserPromise, parserTimeoutPromise])) as any;

      // Проверяем отмену после выполнения
      if (isCancelled || signal.aborted) {
        throw new Error('Операция отменена');
      }

      progress.set('Обработка результатов...');

      // Логируем результат для отладки
      console.log('Parser result:', {
        success: result.success,
        resultsCount: result.results?.length || 0,
        elementsFound: result.elements_found || 0,
        diagnostics: result.diagnostics?.length || 0,
        selector: result.selector,
        fullResult: result,
      });

      if (result.success !== false) {
        // Создаем новый массив с реактивными объектами
        const resultList = result.results || [];
        const newResults = resultList.map((r: any) => ({
          data: r,
          expanded: false,
        }));
        results.set(newResults);

        diagnostics.set(result.diagnostics || []);
        extractionStats.set(result.extraction_stats || null);

        const elementsFound = result.elements_found || 0;
        const executionTime = result.execution_time_ms || 0;
        const processedCount = result.elements_processed || elementsFound;
        progress.set(`Готово! Найдено элементов: ${elementsFound}, обработано: ${processedCount}, извлечено данных: ${resultList.length}, время: ${(executionTime / 1000).toFixed(2)}с`);

        if (onResults) {
          onResults(resultList, get(diagnostics), get(extractionStats));
        }

        // Если нет результатов, но есть элементы на странице
        if (resultList.length === 0 && elementsFound > 0) {
          const hasExtractNodes = edges.some(
            e => nodes.find(n => n.id === e.target)?.type === 'extract'
          );
          const warningMsg = hasExtractNodes
            ? `Найдено ${elementsFound} элементов по селектору "${result.selector || 'неизвестен'}", но данные не извлечены. Проверьте конфигурацию extract узлов.`
            : `Найдено ${elementsFound} элементов по селектору "${result.selector || 'неизвестен'}", но данные не извлечены. Добавьте extract узлы для извлечения данных, или проверьте, что элементы содержат текст.`;

          diagnostics.update(current => [...current, {
            type: 'warning',
            message: warningMsg,
          }]);
        } else if (elementsFound === 0) {
          diagnostics.update(current => [...current, {
            type: 'error',
            message: `Элементы не найдены по селектору "${result.selector || 'неизвестен'}". Проверьте правильность селектора и убедитесь, что страница загружена.`,
          }]);
        }
      } else {
        throw new Error(result.error || 'Парсер не вернул результаты');
      }
    } catch (err: any) {
      console.error('Parser runner error:', err);

      // Если операция была отменена, не показываем ошибку
      if (isCancelled || signal.aborted || err?.message?.includes('отменен')) {
        progress.set(
          'Операция отменена пользователем. Если парсер уже выполнялся в Rust, он остановится при достижении таймаута.'
        );
        error.set(null); // Не показываем ошибку при отмене
        results.set([]);
        diagnostics.set([]);
        extractionStats.set(null);
        return;
      }

      const errorMsg = err.message || err || 'Неизвестная ошибка';
      error.set(errorMsg);
      results.set([]);
      diagnostics.set([]);
      extractionStats.set(null);
      progress.set('Ошибка при выполнении парсера');

      if (onError) {
        onError(errorMsg);
      }
    } finally {
      isRunning.set(false);
      isCancelled.set(false);
      abortController = null;
      // Очищаем progress через небольшую задержку
      setTimeout(() => {
        if (!isRunning) {
          progress.set('');
        }
      }, 2000);
    }
  }

  /**
   * Очищает результаты и ошибки
   */
  function clearResults() {
    error.set(null);
    results.set([]);
    diagnostics.set([]);
    extractionStats.set(null);
    progress.set('');
  }

  return {
    // State
    isRunning,
    progress,
    error,
    results,
    diagnostics,
    extractionStats,
    // Methods
    runParser,
    stopParser,
    clearResults,
  };
}
