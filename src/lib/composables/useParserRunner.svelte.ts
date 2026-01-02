import { invoke } from '@/lib/tauri-wrapper';
import type { Node, Edge } from '@xyflow/svelte';
import type { ParserSettings } from './useParserSettings.svelte.ts';

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
  const { nodes, edges, currentUrl, siteId, getSettings, onResults, onError } = options;

  let isRunning = $state(false);
  let progress = $state('');
  let error = $state<string | null>(null);
  let results = $state<ParserResult[]>([]);
  let diagnostics = $state<any[]>([]);
  let extractionStats = $state<any>(null);
  let isCancelled = $state(false);

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
    isCancelled = true;
    if (abortController) {
      abortController.abort();
      abortController = null;
    }
    isRunning = false;
    progress =
      'Запрос на остановку парсера отправлен. Rust код может продолжить работу до таймаута или завершения текущих элементов.';
  }

  /**
   * Запускает парсер с таймаутом и улучшенной обработкой ошибок
   */
  async function runParser() {
    if (nodes.length === 0) {
      const errMsg = 'Создайте ноды парсера перед тестированием';
      error = errMsg;
      if (onError) {
        onError(errMsg);
      }
      return;
    }

    if (!currentUrl) {
      const errMsg = 'Загрузите страницу для тестирования';
      error = errMsg;
      if (onError) {
        onError(errMsg);
      }
      return;
    }

    // Сброс состояния
    isRunning = true;
    isCancelled = false;
    error = null;
    results = [];
    diagnostics = [];
    extractionStats = null;
    progress = 'Получение HTML страницы...';

    // Создаем локальную копию для избежания циклических зависимостей
    const cancelled = false;

    // Создаем AbortController для возможности остановки
    abortController = new AbortController();
    const signal = abortController.signal;

    try {
      // Получаем HTML с таймаутом
      progress = 'Получение HTML страницы...';

      // Проверяем отмену перед получением HTML
      if (cancelled || signal.aborted) {
        throw new Error('Операция отменена');
      }

      const htmlPromise = invoke<string>('fetch_page', {
        url: currentUrl,
        forceRefresh: false,
        siteId: siteId || null,
      });

      const timeoutPromise = new Promise<string>((_, reject) =>
        setTimeout(() => {
          if (!cancelled && !signal.aborted) {
            reject(new Error('Таймаут получения HTML (30 секунд)'));
          }
        }, 30000)
      );

      const html = (await Promise.race([htmlPromise, timeoutPromise])) as string;

      // Проверяем отмену после получения HTML
      if (cancelled || isCancelled || signal.aborted) {
        throw new Error('Операция отменена');
      }

      if (!html) {
        throw new Error('Не удалось получить HTML страницы');
      }

      progress = `HTML получен (${(html.length / 1024).toFixed(1)} KB). Подготовка данных...`;

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

      progress = 'Запуск парсера...';

      // Проверяем отмену перед запуском
      if (cancelled || isCancelled || signal.aborted) {
        throw new Error('Операция отменена');
      }

      // Запускаем парсер с настройками и таймаутом
      const currentSettings = getSettings();
      const timeoutMs = currentSettings.timeoutSeconds * 1000;
      const parserTimeoutPromise = new Promise<any>((_, reject) => {
        const timeoutId = setTimeout(() => {
          if (!cancelled && !isCancelled && !signal.aborted) {
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
        if (cancelled || isCancelled || signal.aborted) {
          throw new Error('Операция отменена');
        }
        throw err;
      });

      const result = (await Promise.race([parserPromise, parserTimeoutPromise])) as any;

      // Проверяем отмену после выполнения
      if (cancelled || isCancelled || signal.aborted) {
        throw new Error('Операция отменена');
      }

      progress = 'Обработка результатов...';

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
        results = newResults;

        diagnostics = result.diagnostics || [];
        extractionStats = result.extraction_stats || null;

        const elementsFound = result.elements_found || 0;
        const executionTime = result.execution_time_ms || 0;
        const processedCount = result.elements_processed || elementsFound;
        progress = `Готово! Найдено элементов: ${elementsFound}, обработано: ${processedCount}, извлечено данных: ${results.length}, время: ${(executionTime / 1000).toFixed(2)}с`;

        if (onResults) {
          onResults(resultList, diagnostics, extractionStats);
        }

        // Если нет результатов, но есть элементы на странице
        if (resultList.length === 0 && elementsFound > 0) {
          const hasExtractNodes = edges.some(
            e => nodes.find(n => n.id === e.target)?.type === 'extract'
          );
          const warningMsg = hasExtractNodes
            ? `Найдено ${elementsFound} элементов по селектору "${result.selector || 'неизвестен'}", но данные не извлечены. Проверьте конфигурацию extract узлов.`
            : `Найдено ${elementsFound} элементов по селектору "${result.selector || 'неизвестен'}", но данные не извлечены. Добавьте extract узлы для извлечения данных, или проверьте, что элементы содержат текст.`;

          diagnostics.push({
            type: 'warning',
            message: warningMsg,
          });
        } else if (elementsFound === 0) {
          diagnostics.push({
            type: 'error',
            message: `Элементы не найдены по селектору "${result.selector || 'неизвестен'}". Проверьте правильность селектора и убедитесь, что страница загружена.`,
          });
        }
      } else {
        throw new Error(result.error || 'Парсер не вернул результаты');
      }
    } catch (err: any) {
      console.error('Parser runner error:', err);

      // Если операция была отменена, не показываем ошибку
      if (cancelled || isCancelled || signal.aborted || err?.message?.includes('отменен')) {
        progress =
          'Операция отменена пользователем. Если парсер уже выполнялся в Rust, он остановится при достижении таймаута.';
        error = null; // Не показываем ошибку при отмене
        results = [];
        diagnostics = [];
        extractionStats = null;
        return;
      }

      const errorMsg = err.message || err || 'Неизвестная ошибка';
      error = errorMsg;
      results = [];
      diagnostics = [];
      extractionStats = null;
      progress = 'Ошибка при выполнении парсера';

      if (onError) {
        onError(errorMsg);
      }
    } finally {
      isRunning = false;
      isCancelled = false;
      abortController = null;
      // Очищаем progress через небольшую задержку
      setTimeout(() => {
        if (!isRunning) {
          progress = '';
        }
      }, 2000);
    }
  }

  /**
   * Очищает результаты и ошибки
   */
  function clearResults() {
    error = null;
    results = [];
    diagnostics = [];
    extractionStats = null;
    progress = '';
  }

  /**
   * Переключает развернутость результата по индексу
   */
  function toggleResultExpanded(index: number) {
    results = results.map((r, i) =>
      i === index ? { ...r, expanded: !r.expanded } : r
    );
  }

  /**
   * Обновляет результат по индексу с помощью функции
   */
  function updateResult(index: number, updater: (result: ParserResult) => ParserResult) {
    results = results.map((r, i) => (i === index ? updater(r) : r));
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
    toggleResultExpanded,
    updateResult,
  };
}
