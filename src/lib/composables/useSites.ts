/**
 * Composable для работы с сайтами
 * 
 * Отделяет бизнес-логику от компонентов. Предоставляет функции для загрузки,
 * сохранения и удаления сайтов через Tauri API.
 */
import { invoke } from '../tauri-wrapper';

/**
 * Интерфейс для данных сайта
 */
export interface Site {
  id: number;
  name: string;
  url: string;
  parser_config?: any;
}

/**
 * Создает composable для работы с сайтами
 * 
 * @returns Объект с состоянием сайтов и методами для работы с ними
 */
export function useSites() {
  let sites = $state<Site[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  /**
   * Загружает список всех сайтов из базы данных
   */
  async function loadSites() {
    loading = true;
    error = null;
    try {
      sites = await invoke<Site[]>('get_sites');
    } catch (e: any) {
      error = e.toString();
      console.error('Failed to load sites:', e);
    } finally {
      loading = false;
    }
  }

  /**
   * Удаляет сайт из базы данных
   * 
   * @param id - ID сайта для удаления
   * @returns true при успехе, false при ошибке
   */
  async function deleteSite(id: number): Promise<boolean> {
    try {
      await invoke('delete_site', { id });
      await loadSites();
      return true;
    } catch (e: any) {
      error = e.toString();
      console.error('Failed to delete site:', e);
      return false;
    }
  }

  /**
   * Сохраняет сайт в базу данных (создает новый или обновляет существующий)
   * 
   * @param site - данные сайта для сохранения
   * @returns true при успехе, false при ошибке
   */
  async function saveSite(site: Partial<Site>): Promise<boolean> {
    try {
      await invoke('save_site', { site });
      await loadSites();
      return true;
    } catch (e: any) {
      error = e.toString();
      console.error('Failed to save site:', e);
      return false;
    }
  }

  return {
    sites,
    loading,
    error,
    loadSites,
    deleteSite,
    saveSite,
  };
}

