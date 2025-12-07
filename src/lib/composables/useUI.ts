/**
 * Composable для управления UI состоянием
 * 
 * Отделяет логику UI от компонентов. Предоставляет функции для управления
 * текущей страницей, состоянием боковой панели и выбранным сайтом.
 */

/**
 * Тип страницы приложения
 */
export type Page = 'mods' | 'sites' | 'parser' | 'notifications';

/**
 * Создает composable для управления UI состоянием
 * 
 * @returns Объект с состоянием UI и методами для управления
 */
export function useUI() {
  let currentPage = $state<Page>('mods');
  let sidebarOpen = $state(true);
  let selectedSiteId = $state<number | null>(null);

  /**
   * Устанавливает текущую страницу приложения
   * 
   * @param page - страница для установки
   */
  function setPage(page: Page) {
    currentPage = page;
  }

  /**
   * Переключает состояние боковой панели (открыта/закрыта)
   */
  function toggleSidebar() {
    sidebarOpen = !sidebarOpen;
  }

  /**
   * Устанавливает состояние боковой панели
   * 
   * @param open - true для открытия, false для закрытия
   */
  function setSidebarOpen(open: boolean) {
    sidebarOpen = open;
  }

  /**
   * Устанавливает выбранный сайт
   * 
   * @param siteId - ID сайта для выбора (null = снять выбор)
   */
  function setSelectedSite(siteId: number | null) {
    selectedSiteId = siteId;
  }

  return {
    currentPage,
    sidebarOpen,
    selectedSiteId,
    setPage,
    toggleSidebar,
    setSidebarOpen,
    setSelectedSite,
  };
}

