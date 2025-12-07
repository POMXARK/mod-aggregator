/**
 * Composable для управления списком последних открытых URL
 * 
 * Предоставляет функции для загрузки, сохранения и управления списком
 * последних открытых URL в localStorage
 */
const STORAGE_KEY = 'parser-recent-urls';
const MAX_URLS = 10;

/**
 * Создает composable для работы с последними открытыми URL
 * 
 * @returns Объект с состоянием и методами для работы с последними URL
 */
export function useRecentUrls() {
  let recentUrls = $state<string[]>([]);
  let showRecentUrls = $state(false);

  /**
   * Загружает список последних открытых URL из localStorage
   */
  function loadRecentUrls() {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        recentUrls = JSON.parse(stored);
      }
    } catch (e) {
      console.error('Failed to load recent URLs:', e);
      recentUrls = [];
    }
  }

  /**
   * Сохраняет URL в список последних открытых
   * 
   * Добавляет URL в начало списка и ограничивает список до MAX_URLS элементов.
   * 
   * @param url - URL для сохранения
   */
  function saveRecentUrl(url: string) {
    if (!url || url.trim() === '') return;
    
    try {
      const urls = recentUrls.filter(u => u !== url);
      urls.unshift(url);
      recentUrls = urls.slice(0, MAX_URLS);
      localStorage.setItem(STORAGE_KEY, JSON.stringify(recentUrls));
    } catch (e) {
      console.error('Failed to save recent URL:', e);
    }
  }

  /**
   * Удаляет URL из списка последних открытых
   * 
   * @param urlToRemove - URL для удаления
   */
  function removeRecentUrl(urlToRemove: string) {
    try {
      recentUrls = recentUrls.filter(u => u !== urlToRemove);
      localStorage.setItem(STORAGE_KEY, JSON.stringify(recentUrls));
    } catch (e) {
      console.error('Failed to remove recent URL:', e);
    }
  }

  /**
   * Переключает видимость списка последних открытых URL
   * 
   * @param show - если указано, устанавливает видимость, иначе переключает
   */
  function toggleRecentUrls(show?: boolean) {
    if (show !== undefined) {
      showRecentUrls = show;
    } else {
      showRecentUrls = !showRecentUrls;
    }
  }

  return {
    get recentUrls() { return recentUrls; },
    get showRecentUrls() { return showRecentUrls; },
    loadRecentUrls,
    saveRecentUrl,
    removeRecentUrl,
    toggleRecentUrls,
  };
}

