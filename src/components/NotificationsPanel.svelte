<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
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
    max-width: 800px;
    margin: 0 auto;
  }
  
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }
  
  .header > div {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  
  .header h2 {
    margin: 0;
    font-size: 2rem;
    font-weight: 700;
    color: #e2e8f0;
  }
  
  .badge {
    padding: 0.25rem 0.75rem;
    background: #0ea5e9;
    color: white;
    border-radius: 1rem;
    font-size: 0.875rem;
    font-weight: 600;
  }
  
  .btn-secondary {
    padding: 0.75rem 1.5rem;
    background: transparent;
    border: 1px solid #334155;
    color: #94a3b8;
    border-radius: 0.5rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }
  
  .btn-secondary:hover {
    background: #334155;
    color: #e2e8f0;
  }
  
  .loading, .empty {
    text-align: center;
    padding: 3rem;
    color: #64748b;
  }
  
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }
  
  .empty svg {
    color: #475569;
  }
  
  .notifications-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .notification-item {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.75rem;
    padding: 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    transition: all 0.2s;
  }
  
  .notification-item.unread {
    border-color: #0ea5e9;
    background: #1e3a8a;
  }
  
  .notification-item:hover {
    transform: translateY(-2px);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
  }
  
  .notification-content {
    flex: 1;
  }
  
  .notification-content h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: #e2e8f0;
  }
  
  .notification-content p {
    margin: 0 0 0.75rem 0;
    color: #cbd5e1;
    line-height: 1.6;
  }
  
  .notification-date {
    font-size: 0.75rem;
    color: #64748b;
  }
  
  .btn-mark-read {
    padding: 0.5rem;
    background: transparent;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    color: #94a3b8;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .btn-mark-read:hover {
    background: #334155;
    color: #0ea5e9;
    border-color: #0ea5e9;
  }
  
  .icon-large {
    width: 48px;
    height: 48px;
  }
  
  .icon-small {
    width: 18px;
    height: 18px;
  }
</style>

