<script lang="ts">
  interface Props {
    title: string;
    onToggleChatList: () => void;
    onClear: () => void;
  }

  const { title, onToggleChatList, onClear }: Props = $props();
</script>

<div class="chat-header">
  <div class="chat-header-left">
    <button class="btn-chat-list" onclick={onToggleChatList} title="Список чатов"> 💬 </button>
    <h3>{title}</h3>
  </div>
  <div class="chat-header-right">
    <button class="btn-clear" onclick={onClear} title="Очистить чат"> 🗑️ </button>
  </div>
</div>

<style>
  .chat-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: clamp(0.75rem, 1.5vh, 1rem) clamp(1rem, 2vw, 1.25rem);
    border-bottom: 1px solid #334155;
    background: #0f172a;
  }

  .chat-header-left {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex: 1;
    min-width: 0;
  }

  .btn-chat-list {
    background: transparent;
    border: none;
    color: #cbd5e1;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
    font-size: 1rem;
    transition: background-color 0.2s ease;
    flex-shrink: 0;
  }

  .btn-chat-list:hover {
    background: #334155;
  }

  .chat-header h3 {
    margin: 0;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    color: #e2e8f0;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .chat-header-right {
    display: flex;
    gap: 0.5rem;
  }

  .btn-clear {
    background: transparent;
    border: none;
    color: #cbd5e1;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
    font-size: 1rem;
    transition: background-color 0.2s ease;
  }

  .btn-clear:hover {
    background: #334155;
  }
</style>
