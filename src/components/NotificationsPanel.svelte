<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '../lib/tauri-wrapper';
  import BellIcon from './icons/BellIcon.svelte';
  import CheckIcon from './icons/CheckIcon.svelte';
  import { format } from 'date-fns';
  import { ru } from 'date-fns/locale';
  
  let notifications = $state<any[]>([]);
  let loading = $state(false);
  
  onMount(() => {
    loadNotifications();
    // Refresh every 30 seconds
    const interval = setInterval(loadNotifications, 30000);
    return () => clearInterval(interval);
  });
  
  async function loadNotifications() {
    loading = true;
    try {
      notifications = await invoke('get_notifications');
    } catch (error) {
      console.error('Failed to load notifications:', error);
    } finally {
      loading = false;
    }
  }
  
  async function handleMarkRead(id: number) {
    try {
      await invoke('mark_notification_read', { id });
      await loadNotifications();
    } catch (error) {
      console.error('Failed to mark notification as read:', error);
    }
  }
  
  async function handleMarkAllRead() {
    try {
      for (const notif of notifications.filter(n => !n.read)) {
        await invoke('mark_notification_read', { id: notif.id });
      }
      await loadNotifications();
    } catch (error) {
      console.error('Failed to mark all as read:', error);
    }
  }
  
  const unreadCount = $derived(notifications.filter(n => !n.read).length);
</script>

<div class="notifications-panel">
  <div class="header">
    <div>
      <h2>Уведомления</h2>
      {#if unreadCount > 0}
        <span class="badge">{unreadCount} новых</span>
      {/if}
    </div>
    {#if unreadCount > 0}
      <button class="btn-secondary" onclick={handleMarkAllRead}>
        Отметить все прочитанными
      </button>
    {/if}
  </div>
  
  {#if loading && notifications.length === 0}
    <div class="loading">Загрузка...</div>
  {:else if notifications.length === 0}
    <div class="empty">
      <BellIcon class="icon-large" />
      <p>Нет уведомлений</p>
    </div>
  {:else}
    <div class="notifications-list">
      {#each notifications as notification}
        <div class="notification-item" class:unread={!notification.read}>
          <div class="notification-content">
            <h3>{notification.title}</h3>
            <p>{notification.message}</p>
            <span class="notification-date">
              {format(new Date(notification.created_at), 'dd MMM yyyy, HH:mm', { locale: ru })}
            </span>
          </div>
          {#if !notification.read}
            <button 
              class="btn-mark-read" 
              onclick={() => handleMarkRead(notification.id)}
              title="Отметить прочитанным"
            >
              <CheckIcon class="icon-small" />
            </button>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .notifications-panel {
    max-width: min(800px, 95vw);
    margin: 0 auto;
    width: 100%;
  }
  
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: clamp(1.5rem, 2.5vh, 2rem);
    flex-wrap: wrap;
    gap: clamp(1rem, 1.5vw, 1.5rem);
  }
  
  .header > div {
    display: flex;
    align-items: center;
    gap: clamp(0.75rem, 1vw, 1rem);
  }
  
  .header h2 {
    margin: 0;
    font-size: clamp(1.75rem, 3vw, 2rem);
    font-weight: 700;
    color: #e2e8f0;
    line-height: 1.2;
  }
  
  .badge {
    padding: clamp(0.25rem, 0.375vw, 0.375rem) clamp(0.625rem, 1vw, 0.75rem);
    background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
    color: white;
    border-radius: clamp(0.875rem, 1.25vw, 1rem);
    font-size: clamp(0.75rem, 1vw, 0.875rem);
    font-weight: 600;
    box-shadow: 0 2px 8px rgba(14, 165, 233, 0.3);
  }
  
  .btn-secondary {
    padding: clamp(0.625rem, 1vw, 0.75rem) clamp(1.25rem, 2vw, 1.5rem);
    background: transparent;
    border: 1px solid rgba(51, 65, 85, 0.5);
    color: #94a3b8;
    border-radius: clamp(0.375rem, 0.5vw, 0.5rem);
    font-weight: 600;
    font-size: clamp(0.875rem, 1vw, 0.95rem);
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    white-space: nowrap;
  }
  
  .btn-secondary:hover {
    background: rgba(51, 65, 85, 0.8);
    color: #e2e8f0;
    border-color: rgba(148, 163, 184, 0.5);
    transform: translateY(-2px);
  }

  .btn-secondary:active {
    transform: translateY(0);
  }
  
  .loading, .empty {
    text-align: center;
    padding: clamp(2rem, 4vh, 3rem);
    color: #64748b;
    font-size: clamp(1rem, 1.5vw, 1.125rem);
  }
  
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: clamp(0.75rem, 1vw, 1rem);
  }
  
  .empty svg {
    color: #475569;
  }
  
  .notifications-list {
    display: flex;
    flex-direction: column;
    gap: clamp(0.75rem, 1vw, 1rem);
  }
  
  .notification-item {
    background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
    border: 1px solid rgba(51, 65, 85, 0.5);
    border-radius: clamp(0.5rem, 0.75vw, 0.75rem);
    padding: clamp(1.25rem, 2vw, 1.5rem);
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    gap: clamp(1rem, 1.5vw, 1.5rem);
  }
  
  .notification-item.unread {
    border-color: rgba(14, 165, 233, 0.6);
    background: linear-gradient(135deg, rgba(30, 58, 138, 0.3) 0%, #1e293b 100%);
    box-shadow: 0 4px 16px rgba(14, 165, 233, 0.2);
  }
  
  .notification-item:hover {
    transform: translateY(-4px);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.4);
    border-color: rgba(14, 165, 233, 0.5);
  }
  
  .notification-content {
    flex: 1;
    min-width: 0;
  }
  
  .notification-content h3 {
    margin: 0 0 clamp(0.375rem, 0.5vw, 0.5rem) 0;
    font-size: clamp(1rem, 1.5vw, 1.125rem);
    font-weight: 600;
    color: #e2e8f0;
    line-height: 1.3;
    word-break: break-word;
  }
  
  .notification-content p {
    margin: 0 0 clamp(0.5rem, 0.75vw, 0.75rem) 0;
    color: #cbd5e1;
    line-height: 1.6;
    font-size: clamp(0.875rem, 1.1vw, 0.95rem);
    word-break: break-word;
  }
  
  .notification-date {
    font-size: clamp(0.6875rem, 0.9vw, 0.75rem);
    color: #64748b;
  }
  
  .btn-mark-read {
    padding: clamp(0.375rem, 0.5vw, 0.5rem);
    background: transparent;
    border: 1px solid rgba(51, 65, 85, 0.5);
    border-radius: clamp(0.25rem, 0.375vw, 0.375rem);
    color: #94a3b8;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: clamp(2rem, 2.5vw, 2.5rem);
    min-height: clamp(2rem, 2.5vw, 2.5rem);
    flex-shrink: 0;
  }
  
  .btn-mark-read:hover {
    background: rgba(51, 65, 85, 0.8);
    color: #0ea5e9;
    border-color: #0ea5e9;
    transform: scale(1.1);
  }

  .btn-mark-read:active {
    transform: scale(0.95);
  }
  
  .icon-large {
    width: clamp(3rem, 4vw, 3.5rem);
    height: clamp(3rem, 4vw, 3.5rem);
  }
  
  .icon-small {
    width: clamp(1rem, 1.25vw, 1.125rem);
    height: clamp(1rem, 1.25vw, 1.125rem);
  }
</style>

