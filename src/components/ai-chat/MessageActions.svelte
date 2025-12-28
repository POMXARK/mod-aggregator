<script lang="ts">
  import type { Message } from '@/lib/composables';

  interface Props {
    message: Message;
    hasJsonConfig: boolean;
    hasCode: boolean;
    hasParserMention: boolean;
    canRunParser: boolean;
    onApplyCode: () => void;
    onRestoreConfig: () => void;
    onGenerateCode: () => void;
    onRunParser: () => void;
    onCopy: () => void;
    onDelete: () => void;
  }

  const {
    message,
    hasJsonConfig,
    hasCode,
    hasParserMention,
    canRunParser,
    onApplyCode,
    onRestoreConfig,
    onGenerateCode,
    onRunParser,
    onCopy,
    onDelete,
  }: Props = $props();
</script>

{#if message.role === 'assistant'}
  <div class="message-actions">
    {#if hasCode}
      <button class="btn-message-action" onclick={onApplyCode} title="Применить исправление кода">
        ✅ Применить код
      </button>
    {/if}
    {#if hasJsonConfig || hasParserMention}
      {#if hasJsonConfig}
        <button
          class="btn-message-action"
          onclick={onRestoreConfig}
          title="Восстановить конфигурацию парсера из этого сообщения (создаст узлы и код)"
        >
          🔄 Восстановить конфигурацию
        </button>
        <button
          class="btn-message-action"
          onclick={onGenerateCode}
          title="Сгенерировать код из этого сообщения (создаст узлы и код, как при 'повтори')"
        >
          📄 Генерировать код
        </button>
      {/if}
      <button
        class="btn-message-action"
        onclick={onRunParser}
        disabled={!canRunParser}
        title="Запустить парсер"
      >
        ▶️ Запустить парсер
      </button>
    {/if}
    <button class="btn-message-action" onclick={onCopy} title="Копировать сообщение">
      📋 Копировать
    </button>
    <button class="btn-delete-message" onclick={onDelete} title="Удалить сообщение"> × </button>
  </div>
{/if}

<style>
  .message-actions {
    display: flex;
    gap: 0.25rem;
    align-items: center;
    opacity: 0;
    transition: opacity 0.2s ease;
  }

  .message:hover .message-actions {
    opacity: 1;
  }

  .btn-message-action {
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    color: #0ea5e9;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    line-height: 1;
    transition: all 0.2s ease;
  }

  .btn-message-action:hover:not(:disabled) {
    background: rgba(14, 165, 233, 0.2);
    border-color: #0ea5e9;
  }

  .btn-message-action:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-delete-message {
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    font-size: 0.875rem;
    line-height: 1;
    transition:
      background-color 0.2s ease,
      color 0.2s ease;
  }

  .btn-delete-message:hover {
    background: rgba(239, 68, 68, 0.2);
    color: #ef4444;
  }
</style>
