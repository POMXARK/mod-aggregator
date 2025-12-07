/**
 * Composable для загрузки и обработки веб-страниц
 * 
 * Предоставляет функции для загрузки страниц через Tauri, обработки HTML
 * (удаление скриптов, встраивание скрипта выделения, встраивание ресурсов)
 * и управления состоянием загрузки
 */
import { invoke } from '../tauri-wrapper';
import { embedResources } from '../utils/page-resources';
import { removeScripts, embedSelectionScript } from '../utils/html-processor';
import { getSelectionScriptContent, getSelectionStyles } from '../utils/selection-script';
import { getNavigationScriptContent } from '../utils/navigation-script';

/**
 * Опции для настройки загрузчика страниц
 */
export interface PageLoaderOptions {
  /** ID сайта для привязки кеша (может быть функцией для динамического получения) */
  siteId?: number | null | (() => number | null);
  /** Callback для обработки ошибок загрузки */
  onError?: (error: string) => void;
  /** Callback, вызываемый после успешной загрузки */
  onLoadComplete?: () => void;
}

/**
 * Создает composable для загрузки и обработки веб-страниц
 * 
 * @param options - опции для настройки загрузчика
 * @returns Объект с состоянием загрузки и методом loadPage
 */
export function usePageLoader(options: PageLoaderOptions = {}) {
  let loading = $state(false);
  let error = $state<string | null>(null);
  let processedHtml = $state<string | null>(null);

  /**
   * Загружает страницу с учетом кеша и привязки к сайту
   * 
   * Загружает страницу через Tauri, обрабатывает HTML (удаляет скрипты,
   * встраивает скрипт выделения, встраивает ресурсы) и сохраняет результат.
   * 
   * @param url - URL страницы для загрузки
   * @param forceRefresh - если true, загружает с сервера и создает новую версию
   * @returns Promise с HTML содержимым страницы или null в случае ошибки
   */
  async function loadPage(url: string, forceRefresh: boolean = false): Promise<string | null> {
    if (!url || url.trim() === '') {
      error = 'URL не может быть пустым';
      if (options.onError) {
        options.onError(error);
      }
      return null;
    }

    // Нормализуем URL - убеждаемся, что это полный URL
    let normalizedUrl = url.trim();
    try {
      const urlObj = new URL(normalizedUrl);
      normalizedUrl = urlObj.href; // Получаем полный нормализованный URL
      
      // Убираем trailing slash для консистентности (если это не корневой путь)
      if (normalizedUrl.endsWith('/') && normalizedUrl.split('/').length > 4) {
        normalizedUrl = normalizedUrl.slice(0, -1);
      }
      
      console.log('[PageLoader] Normalized URL:', normalizedUrl, 'from:', url);
      
      // Проверяем, что URL действительно полный
      if (normalizedUrl !== url) {
        console.log('[PageLoader] URL was normalized:', url, '->', normalizedUrl);
      }
    } catch (e) {
      console.warn('[PageLoader] Invalid URL format:', url, e);
      // Если URL невалидный, используем как есть
    }

    loading = true;
    error = null;

    try {
      // Получаем siteId из опций
      const siteId = typeof options.siteId === 'function' ? options.siteId() : options.siteId;

      // Загружаем страницу через Tauri с нормализованным URL
      console.log('[PageLoader] Fetching page with normalized URL:', normalizedUrl, 'forceRefresh:', forceRefresh, 'siteId:', siteId);
      const html = await invoke<string>('fetch_page', { 
        url: normalizedUrl, 
        forceRefresh,
        siteId: siteId || null 
      });

      if (!html) {
        throw new Error('Получен пустой HTML');
      }

      console.log('[PageLoader] Received HTML, length:', html.length, 'chars');

      // Обрабатываем HTML
      let processed = html;

      // Удаляем скрипты (включая старые скрипты навигации)
      console.log('[PageLoader] Removing scripts from HTML...');
      processed = removeScripts(processed);
      console.log('[PageLoader] Scripts removed, processed HTML length:', processed.length, 'chars');

      // Встраиваем скрипт навигации, скрипт выделения и стили
      // Используем нормализованный URL для скрипта навигации
      const selectionScriptContent = getSelectionScriptContent();
      const navigationScriptContent = getNavigationScriptContent(normalizedUrl);
      const styles = getSelectionStyles();
      processed = embedSelectionScript(processed, selectionScriptContent, navigationScriptContent, styles, normalizedUrl);

      // Встраиваем ресурсы (CSS, изображения)
      // Сначала проверяем, есть ли папка кеша для этого URL
      try {
        let pageFolder: string;
        if (!forceRefresh) {
          // Пытаемся найти папку кеша для этого URL
          const cacheFolder = await invoke<string | null>('get_cache_folder_for_url', { url: normalizedUrl });
          if (cacheFolder) {
            pageFolder = cacheFolder;
            console.log('[PageLoader] Using cache folder for resources:', pageFolder);
          } else {
            // Создаем новую папку для ресурсов
            pageFolder = `saved_pages/page_${Date.now()}_${new URL(normalizedUrl).hostname.replace(/\./g, '_')}`;
          }
        } else {
          // При принудительном обновлении создаем новую папку
          pageFolder = `saved_pages/page_${Date.now()}_${new URL(normalizedUrl).hostname.replace(/\./g, '_')}`;
        }
        processed = await embedResources(processed, normalizedUrl, pageFolder);
      } catch (e) {
        console.warn('[PageLoader] Failed to embed resources:', e);
        // Продолжаем без встраивания ресурсов
      }

      processedHtml = processed;

      if (options.onLoadComplete) {
        options.onLoadComplete();
      }

      return processed;
    } catch (e: any) {
      const errorMessage = e?.message || e?.toString() || 'Неизвестная ошибка при загрузке страницы';
      error = errorMessage;
      console.error('[PageLoader] Error loading page:', e);
      
      if (options.onError) {
        options.onError(errorMessage);
      }
      
      return null;
    } finally {
      loading = false;
    }
  }

  return {
    get loading() { return loading; },
    get error() { return error; },
    get processedHtml() { return processedHtml; },
    loadPage,
  };
}

