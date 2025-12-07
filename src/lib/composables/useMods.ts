/**
 * Composable для работы с модами
 * 
 * Отделяет бизнес-логику от компонентов. Предоставляет функции для загрузки
 * модов и проверки обновлений через Tauri API.
 */
import { invoke } from '../tauri-wrapper';

/**
 * Интерфейс для данных мода
 */
export interface Mod {
  id: number;
  site_id: number;
  title: string;
  url: string;
  version?: string;
  author?: string;
  description?: string;
  image_url?: string;
  changes?: string;
  updated_at: string;
}

/**
 * Создает composable для работы с модами
 * 
 * @returns Объект с состоянием модов и методами для работы с ними
 */
export function useMods() {
  let mods = $state<Mod[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  /**
   * Загружает список модов из базы данных
   * 
   * @param siteId - ID сайта для фильтрации (null = все сайты)
   */
  async function loadMods(siteId: number | null = null) {
    loading = true;
    error = null;
    try {
      mods = await invoke<Mod[]>('get_mods', { siteId });
    } catch (e: any) {
      error = e.toString();
      console.error('Failed to load mods:', e);
    } finally {
      loading = false;
    }
  }

  /**
   * Проверяет обновления модов для указанного сайта или всех сайтов
   * 
   * Загружает страницы сайтов, парсит моды и сравнивает с существующими в базе данных.
   * Создает записи о новых модах и обновлениях существующих.
   * 
   * @param siteId - ID сайта для проверки (null = все сайты)
   * @returns true при успехе, false при ошибке
   */
  async function checkUpdates(siteId: number | null = null): Promise<boolean> {
    loading = true;
    error = null;
    try {
      await invoke('check_updates', { siteId });
      await loadMods(siteId);
      return true;
    } catch (e: any) {
      error = e.toString();
      console.error('Failed to check updates:', e);
      return false;
    } finally {
      loading = false;
    }
  }

  return {
    mods,
    loading,
    error,
    loadMods,
    checkUpdates,
  };
}

