<script lang="ts">
  import type { Chat } from '@/lib/composables';

  interface Props {
    chats: Chat[];
    currentChatId: string | null;
    onSelectChat: (chatId: string) => void;
    onDeleteChat: (chatId: string) => void;
    onCreateNew: () => void;
  }

  const { chats, currentChatId, onSelectChat, onDeleteChat, onCreateNew }: Props = $props();
</script>

<div class="chat-list-panel">
  <div class="chat-list-header">
    <h4>Чаты</h4>
    <button class="btn-new-chat" onclick={onCreateNew} title="Новый чат"> ➕ </button>
  </div>
  <div class="chat-list">
    {#each chats as chat (chat.id)}
      <div
        class="chat-item"
        class:active={chat.id === currentChatId}
        onclick={() => onSelectChat(chat.id)}
      >
        <div class="chat-item-title">{chat.title}</div>
        <div class="chat-item-meta">
          {chat.messages.length} сообщений • {new Date(chat.updatedAt).toLocaleDateString('ru-RU')}
        </div>
        <button
          class="btn-delete-chat"
          onclick={e => {
            e.stopPropagation();
            onDeleteChat(chat.id);
          }}
          title="Удалить чат"
        >
          ×
        </button>
      </div>
    {/each}
  </div>
</div>

<style>
  .chat-list-panel {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: #1e293b;
    border-bottom: 1px solid #334155;
    z-index: 100;
    max-height: 400px;
    display: flex;
    flex-direction: column;
  }

  .chat-list-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: clamp(0.75rem, 1.5vh, 1rem);
    border-bottom: 1px solid #334155;
  }

  .chat-list-header h4 {
    margin: 0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    color: #e2e8f0;
  }

  .btn-new-chat {
    background: #0ea5e9;
    border: none;
    color: white;
    cursor: pointer;
    padding: 0.375rem 0.75rem;
    border-radius: 0.375rem;
    font-size: 1rem;
    transition: background-color 0.2s ease;
  }

  .btn-new-chat:hover {
    background: #0284c7;
  }

  .chat-list {
    overflow-y: auto;
    flex: 1;
  }

  .chat-item {
    padding: clamp(0.75rem, 1.5vh, 1rem);
    border-bottom: 1px solid #334155;
    cursor: pointer;
    transition: background-color 0.2s ease;
    position: relative;
  }

  .chat-item:hover {
    background: #334155;
  }

  .chat-item.active {
    background: #0ea5e9;
  }

  .chat-item-title {
    font-weight: 600;
    color: #e2e8f0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    margin-bottom: 0.25rem;
  }

  .chat-item.active .chat-item-title {
    color: white;
  }

  .chat-item-meta {
    font-size: clamp(0.625rem, 0.8vw, 0.75rem);
    color: #64748b;
  }

  .chat-item.active .chat-item-meta {
    color: rgba(255, 255, 255, 0.8);
  }

  .btn-delete-chat {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    background: transparent;
    border: none;
    color: #cbd5e1;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
    font-size: 1.25rem;
    line-height: 1;
    transition:
      background-color 0.2s ease,
      color 0.2s ease;
    opacity: 0;
  }

  .chat-item:hover .btn-delete-chat {
    opacity: 1;
  }

  .btn-delete-chat:hover {
    background: rgba(239, 68, 68, 0.2);
    color: #ef4444;
  }
</style>
