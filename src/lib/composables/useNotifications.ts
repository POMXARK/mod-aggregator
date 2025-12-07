/**
 * Composable для работы с уведомлениями
 * 
 * Отделяет бизнес-логику от компонентов. Предоставляет функции для загрузки
 * уведомлений и отметки их как прочитанных через Tauri API.
 */
import { invoke } from '../tauri-wrapper';

/**
 * Интерфейс для данных уведомления
 */
export interface Notification {
  id: number;
  title: string;
  message: string;
  read: boolean;
  created_at: string;
}

/**
 * Создает composable для работы с уведомлениями
 * 
 * @returns Объект с состоянием уведомлений и методами для работы с ними
 */
export function useNotifications() {
  let notifications = $state<Notification[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  /** Вычисляемое значение: количество непрочитанных уведомлений */
  let unreadCount = $derived(notifications.filter(n => !n.read).length);

  /**
   * Загружает список всех уведомлений из базы данных
   */
  async function loadNotifications() {
    loading = true;
    error = null;
    try {
      notifications = await invoke<Notification[]>('get_notifications');
    } catch (e: any) {
      error = e.toString();
      console.error('Failed to load notifications:', e);
    } finally {
      loading = false;
    }
  }

  /**
   * Отмечает уведомление как прочитанное
   * 
   * @param id - ID уведомления для отметки
   * @returns true при успехе, false при ошибке
   */
  async function markAsRead(id: number): Promise<boolean> {
    try {
      await invoke('mark_notification_read', { id });
      await loadNotifications();
      return true;
    } catch (e: any) {
      error = e.toString();
      console.error('Failed to mark notification as read:', e);
      return false;
    }
  }

  /**
   * Отмечает все уведомления как прочитанные
   * 
   * @returns true при успехе, false при ошибке
   */
  async function markAllAsRead(): Promise<boolean> {
    try {
      const unread = notifications.filter(n => !n.read);
      for (const notif of unread) {
        await invoke('mark_notification_read', { id: notif.id });
      }
      await loadNotifications();
      return true;
    } catch (e: any) {
      error = e.toString();
      console.error('Failed to mark all as read:', e);
      return false;
    }
  }

  return {
    notifications,
    loading,
    error,
    unreadCount,
    loadNotifications,
    markAsRead,
    markAllAsRead,
  };
}

