/**
 * Composable для управления выделением элементов в iframe
 * 
 * Предоставляет функции для включения/выключения режима выделения
 * и обработки сообщений от скрипта выделения в iframe
 */

/**
 * Интерфейс для callback функций выделения
 */
export interface SelectionCallbacks {
  /** Callback при выборе элемента в iframe */
  onElementSelect?: (selector: string, element: HTMLElement | null, data?: any) => void;
  /** Callback при клике на внутреннюю ссылку в iframe */
  onInternalLink?: (linkUrl: string) => void;
}

/**
 * Создает composable для управления выделением элементов в iframe
 * 
 * @param callbacks - callback функции для обработки событий выделения
 * @returns Объект с состоянием выделения и методами управления
 */
export function useSelection(callbacks: SelectionCallbacks = {}) {
  let selectionMode = $state(false);
  let scriptReady = $state(false);

  /**
   * Включает режим выделения элементов в iframe
   * 
   * Отправляет сообщение в iframe для активации скрипта выделения.
   * 
   * @param iframeRef - ссылка на iframe элемент
   */
  function enableSelectionInIframe(iframeRef: HTMLIFrameElement | null) {
    if (!iframeRef || !iframeRef.contentWindow) {
      console.warn('[Selection] Cannot enable selection: iframe not ready');
      return;
    }

    try {
      iframeRef.contentWindow.postMessage(
        { type: 'enable-selection' },
        '*'
      );
      selectionMode = true;
      console.log('[Selection] Selection mode enabled in iframe');
    } catch (e) {
      console.error('[Selection] Failed to enable selection:', e);
    }
  }

  /**
   * Выключает режим выделения элементов в iframe
   * 
   * Отправляет сообщение в iframe для деактивации скрипта выделения.
   * 
   * @param iframeRef - ссылка на iframe элемент
   */
  function disableSelectionInIframe(iframeRef: HTMLIFrameElement | null) {
    if (!iframeRef || !iframeRef.contentWindow) {
      return;
    }

    try {
      iframeRef.contentWindow.postMessage(
        { type: 'disable-selection' },
        '*'
      );
      selectionMode = false;
      console.log('[Selection] Selection mode disabled in iframe');
    } catch (e) {
      console.error('[Selection] Failed to disable selection:', e);
    }
  }

  /**
   * Обрабатывает сообщения от скрипта выделения в iframe
   * 
   * Обрабатывает сообщения типа 'parser-script-ready', 'element-selected',
   * 'navigate-internal-link' и вызывает соответствующие callbacks.
   * 
   * @param event - событие сообщения от iframe
   */
  function handleMessage(event: MessageEvent) {
    try {
      if (!event.data || typeof event.data !== 'object') {
        return;
      }

      const { type } = event.data;

      switch (type) {
        case 'parser-script-ready':
          scriptReady = true;
          console.log('[Selection] Script ready in iframe');
          break;

        case 'element-selected':
          if (callbacks.onElementSelect) {
            const { selector, tagName, text, attributes, similarElements } = event.data;
            // Создаем объект с данными элемента
            const elementData = {
              tagName,
              text,
              attributes,
              similarElements,
            };
            callbacks.onElementSelect(selector, null as any, elementData);
          }
          break;

        case 'navigate-internal-link':
          console.log('[Selection] Received navigate-internal-link message:', event.data);
          if (callbacks.onInternalLink && event.data.url) {
            console.log('[Selection] Calling onInternalLink with URL:', event.data.url);
            callbacks.onInternalLink(event.data.url);
          } else {
            console.warn('[Selection] Missing onInternalLink callback or URL:', {
              hasCallback: !!callbacks.onInternalLink,
              url: event.data.url
            });
          }
          break;

        default:
          // Игнорируем другие типы сообщений
          break;
      }
    } catch (e) {
      console.error('[Selection] Error handling message:', e);
    }
  }

  /**
   * Переключает режим выделения элементов в iframe
   * 
   * Включает режим выделения, если он выключен, и наоборот.
   * 
   * @param iframeRef - ссылка на iframe элемент
   */
  function toggleSelectionMode(iframeRef: HTMLIFrameElement | null) {
    if (selectionMode) {
      disableSelectionInIframe(iframeRef);
    } else {
      enableSelectionInIframe(iframeRef);
    }
  }

  return {
    get selectionMode() { return selectionMode; },
    get scriptReady() { return scriptReady; },
    enableSelectionInIframe,
    disableSelectionInIframe,
    handleMessage,
    toggleSelectionMode,
  };
}

