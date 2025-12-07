<script lang="ts">
  /**
   * Компонент для просмотра и взаимодействия с веб-страницами
   * 
   * Предоставляет функционал загрузки страниц, выделения элементов,
   * управления последними открытыми URL и работы с режимом выделения.
   */
  import { onMount } from 'svelte';
  import { useRecentUrls } from '../lib/composables/useRecentUrls.svelte.ts';
  import { usePageLoader } from '../lib/composables/usePageLoader.svelte.ts';
  import { useSelection } from '../lib/composables/useSelection.svelte.ts';
  import ViewerHeader from './ViewerHeader.svelte';
  import PageIframe from './PageIframe.svelte';

  interface Props {
    /** URL страницы для загрузки (bindable) */
    url: string;
    /** ID сайта для привязки кеша (опционально) */
    siteId?: number | null;
    /** Callback при выборе элемента на странице */
    onElementSelect?: (selector: string, element: HTMLElement | null, data?: any) => void;
    /** Callback при обновлении кеша */
    onRefreshCache?: () => void;
  }

  let { url = $bindable(), siteId, onElementSelect }: Props = $props();

  let iframeRef: HTMLIFrameElement | null = $state(null);
  let panelCollapsed = $state(false);
  let processedHtml = $state<string | null>(null);

  // Используем composables для разделения логики
  const recentUrls = useRecentUrls();
  const pageLoader = usePageLoader({
    get siteId() { return siteId; },
    onError: (error) => {
      console.error('[PageViewer] Page loader error:', error);
    },
    onLoadComplete: () => {
      recentUrls.saveRecentUrl(url);
    }
  });
  const selection = useSelection({
    onElementSelect: (selector, element, data) => {
      if (onElementSelect) {
        onElementSelect(selector, element, data);
      }
    },
    onInternalLink: (linkUrl) => {
      if (linkUrl) {
        console.log('[PageViewer] Internal link clicked:', linkUrl, 'current URL:', url);
        // Нормализуем URL (убираем trailing slash и т.д. для сравнения)
        const normalizedLinkUrl = linkUrl.replace(/\/$/, '');
        const normalizedCurrentUrl = url ? url.replace(/\/$/, '') : '';
        
        if (normalizedLinkUrl !== normalizedCurrentUrl) {
          console.log('[PageViewer] Updating URL from', normalizedCurrentUrl, 'to', normalizedLinkUrl);
          // Обновляем URL - $effect автоматически загрузит страницу
          url = linkUrl;
          // Сохраняем в список последних открытых
          recentUrls.saveRecentUrl(linkUrl);
        } else {
          console.log('[PageViewer] URL unchanged, skipping');
        }
      }
    }
  });

  onMount(() => {
    recentUrls.loadRecentUrls();
    
    // Настраиваем обработчик сообщений от iframe
    window.addEventListener('message', selection.handleMessage);
    return () => {
      window.removeEventListener('message', selection.handleMessage);
    };
  });

  /**
   * Обрабатывает загрузку страницы
   * @param forceRefresh - принудительно обновить с сервера
   */
  async function handleLoadPage(forceRefresh: boolean = false) {
    if (!url) {
      console.warn('[PageViewer] loadPage called with empty URL');
      return;
    }

    // Отключаем режим выделения при загрузке новой страницы
    if (selection.selectionMode) {
      selection.disableSelectionInIframe(iframeRef);
    }

    const html = await pageLoader.loadPage(url, forceRefresh);
    if (html) {
      processedHtml = html;
      
      // НЕ активируем режим выделения автоматически при загрузке
      // Пользователь должен включить его вручную через кнопку
    }
  }

  /**
   * Выбирает URL из списка последних открытых
   * @param selectedUrl - выбранный URL
   */
  async function selectRecentUrl(selectedUrl: string) {
    // Отключаем режим выделения при смене URL
    if (selection.selectionMode) {
      selection.disableSelectionInIframe(iframeRef);
    }
    url = selectedUrl;
    recentUrls.toggleRecentUrls(false);
    await handleLoadPage(false);
  }

  /**
   * Обновляет страницу из списка последних открытых
   * @param selectedUrl - URL для обновления
   */
  async function refreshRecentUrl(selectedUrl: string) {
    // Отключаем режим выделения при обновлении
    if (selection.selectionMode) {
      selection.disableSelectionInIframe(iframeRef);
    }
    url = selectedUrl;
    recentUrls.toggleRecentUrls(false);
    await handleLoadPage(true);
  }

  /**
   * Переключает режим выделения
   */
  function toggleSelectionMode() {
    selection.toggleSelectionMode(iframeRef);
  }

  /**
   * Обрабатывает изменение URL
   * @param newUrl - новый URL
   */
  function handleUrlChange(newUrl: string) {
    // Отключаем режим выделения при изменении URL
    if (selection.selectionMode && newUrl !== url) {
      selection.disableSelectionInIframe(iframeRef);
    }
    url = newUrl;
  }

  // Отслеживаем изменение URL и автоматически загружаем страницу
  let previousUrl = $state(url || '');
  let isLoading = $state(false);
  
  $effect(() => {
    // При изменении URL отключаем режим выделения и загружаем страницу
    const currentUrl = url || '';
    if (currentUrl !== previousUrl && currentUrl) {
      console.log('[PageViewer] URL changed from', previousUrl, 'to', currentUrl);
      
      // Обновляем previousUrl сразу, чтобы избежать повторных вызовов
      previousUrl = currentUrl;
      
      // Отключаем режим выделения
      if (selection.selectionMode) {
        selection.disableSelectionInIframe(iframeRef);
      }
      
      // Загружаем страницу только если не загружаем уже
      if (!isLoading) {
        isLoading = true;
        handleLoadPage(false).finally(() => {
          isLoading = false;
        });
      }
    }
  });

  /**
   * Обрабатывает клик вне области ввода URL
   */
  $effect(() => {
    function handleClickOutside(e: MouseEvent) {
      const target = e.target as HTMLElement;
      if (!target.closest('.url-input-wrapper')) {
        recentUrls.toggleRecentUrls(false);
      }
    }

    if (recentUrls.showRecentUrls) {
      document.addEventListener('click', handleClickOutside);
      return () => {
        document.removeEventListener('click', handleClickOutside);
      };
    }
  });

  // Отключаем режим выделения при загрузке новой страницы
  let previousProcessedHtml = $state<string | null>(null);
  $effect(() => {
    // При загрузке новой страницы отключаем режим выделения
    if (processedHtml !== previousProcessedHtml) {
      if (processedHtml && selection.selectionMode) {
        selection.disableSelectionInIframe(iframeRef);
      }
      previousProcessedHtml = processedHtml;
    }
  });

  // Отслеживаем готовность скрипта и активируем выделение если нужно
  // Но только если режим выделения был явно включен пользователем
  let lastEnabledHtml = $state<string | null>(null);
  $effect(() => {
    // Активируем выделение только если:
    // 1. Скрипт готов
    // 2. Режим выделения включен
    // 3. iframe готов
    // 4. HTML загружен
    // 5. HTML изменился (новая страница)
    if (selection.scriptReady && selection.selectionMode && iframeRef && processedHtml && processedHtml !== lastEnabledHtml) {
      lastEnabledHtml = processedHtml;
      // Небольшая задержка для полной загрузки iframe
      const timeoutId = setTimeout(() => {
        if (iframeRef && iframeRef.contentWindow && selection.selectionMode && processedHtml === lastEnabledHtml) {
          selection.enableSelectionInIframe(iframeRef);
        }
      }, 500);
      return () => clearTimeout(timeoutId);
    }
  });
</script>

<div class="page-viewer">
  <ViewerHeader
    {url}
    loading={pageLoader.loading}
    selectionMode={selection.selectionMode}
    recentUrls={recentUrls.recentUrls}
    showRecentUrls={recentUrls.showRecentUrls}
    panelCollapsed={panelCollapsed}
    onUrlChange={handleUrlChange}
    onLoad={handleLoadPage}
    onSelectRecent={selectRecentUrl}
    onRefreshRecent={refreshRecentUrl}
    onRemoveRecent={recentUrls.removeRecentUrl}
    onToggleSelection={toggleSelectionMode}
    onToggleCollapse={() => panelCollapsed = !panelCollapsed}
    onShowRecentUrls={(show) => recentUrls.toggleRecentUrls(show)}
  />

  {#if pageLoader.error}
    <div class="page-viewer__error">
      Ошибка: {pageLoader.error}
    </div>
  {/if}

  <PageIframe
    bind:iframeRef
    html={processedHtml}
    loading={pageLoader.loading}
    error={pageLoader.error}
  />
</div>

<style>
  @import '../styles/page-viewer.css';
</style>
