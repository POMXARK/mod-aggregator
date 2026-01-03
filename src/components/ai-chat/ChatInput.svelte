<script lang="ts">
  import type { Message } from '@/lib/composables';

  interface Props {
    inputMessage: string;
    isSending: boolean;
    attachedElement: Message['attachedElement'] | null;
    selectedElementInfo: {
      selector: string;
      elementInfo: {
        tagName: string;
        text: string;
        attributes: Record<string, string>;
        similarElements?: number;
      };
    } | null;
    nodesCount: number;
    hasGeneratedCode: boolean;
    hasCurrentUrl: boolean;
    onInputChange: (value: string) => void;
    onSend: () => void;
    onAttachElement: () => void;
    onRemoveAttachment: () => void;
    onGenerateCode: () => void;
    onCheckCode: () => void;
    onRunParser: () => void;
  }

  const {
    inputMessage,
    isSending,
    attachedElement,
    selectedElementInfo,
    nodesCount,
    hasGeneratedCode,
    hasCurrentUrl,
    onInputChange,
    onSend,
    onAttachElement,
    onRemoveAttachment,
    onGenerateCode,
    onCheckCode,
    onRunParser,
  }: Props = $props();

  function handleKeyPress(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      onSend();
    }
  }
</script>

<div class="chat-input">
  {#if attachedElement}
    <div class="attached-element-preview">
      <div class="attached-preview-header">
        <span class="attached-icon">📎</span>
        <span>Элемент: {attachedElement.tagName} ({attachedElement.selector})</span>
        <button class="btn-remove-attachment" onclick={onRemoveAttachment} title="Убрать">
          ×
        </button>
      </div>
    </div>
  {/if}

  {#if selectedElementInfo && !attachedElement}
    <div class="attach-element-hint">
      <button class="btn-attach" onclick={onAttachElement} title="Прикрепить выделенный элемент">
        📎 Прикрепить элемент
      </button>
    </div>
  {/if}

  <div class="chat-input-actions">
    <button
      class="btn-chat-action"
      onclick={onGenerateCode}
      disabled={nodesCount === 0}
      title="Генерировать код парсера"
    >
      📄 Код
    </button>
    <button
      class="btn-chat-action"
      onclick={onCheckCode}
      disabled={!hasGeneratedCode}
      title="Проверить код с помощью AI"
    >
      🤖 Проверить
    </button>
    <button
      class="btn-chat-action"
      onclick={onRunParser}
      disabled={nodesCount === 0 || !hasCurrentUrl}
      title="Запустить парсер"
    >
      ▶️ Тест
    </button>
  </div>
  <div class="chat-input-row">
    <textarea
      value={inputMessage}
      oninput={e => onInputChange((e.target as HTMLTextAreaElement).value)}
      onkeydown={handleKeyPress}
      placeholder="Опиши, какие данные нужно извлечь с сайта..."
      rows="2"
      disabled={isSending}
    ></textarea>
    <button
      class="btn-send"
      onclick={onSend}
      disabled={isSending || (!inputMessage.trim() && !attachedElement)}
    >
      {#if isSending}
        ⏳
      {:else}
        ➤
      {/if}
    </button>
  </div>
</div>

<style>
  .chat-input {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: clamp(0.75rem, 1.5vh, 1rem);
    border-top: 1px solid #334155;
    background: #0f172a;
  }

  .attached-element-preview {
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    border-radius: 0.5rem;
    padding: clamp(0.5rem, 1vh, 0.75rem);
  }

  .attached-preview-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
    color: #0ea5e9;
  }

  .btn-remove-attachment {
    background: transparent;
    border: none;
    color: #0ea5e9;
    cursor: pointer;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    font-size: 1rem;
    line-height: 1;
    margin-left: auto;
    transition: background-color 0.2s ease;
  }

  .btn-remove-attachment:hover {
    background: rgba(14, 165, 233, 0.2);
  }

  .attach-element-hint {
    display: flex;
    justify-content: flex-start;
  }

  .btn-attach {
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    color: #0ea5e9;
    cursor: pointer;
    padding: clamp(0.375rem, 0.75vh, 0.5rem) clamp(0.75rem, 1.5vw, 1rem);
    border-radius: 0.5rem;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
    transition: background-color 0.2s ease;
  }

  .btn-attach:hover {
    background: rgba(14, 165, 233, 0.2);
  }

  .chat-input-actions {
    display: flex;
    gap: 0.375rem;
    margin-bottom: 0.75rem;
    flex-wrap: wrap;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid rgba(51, 65, 85, 0.5);
  }

  .btn-chat-action {
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    color: #0ea5e9;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.6875rem;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .btn-chat-action:hover:not(:disabled) {
    background: rgba(14, 165, 233, 0.2);
    border-color: #0ea5e9;
  }

  .btn-chat-action:disabled {
    opacity: 0.7;
    cursor: not-allowed;
    filter: grayscale(50%);
  }

  .chat-input-row {
    display: flex;
    gap: 0.375rem;
    align-items: flex-start;
  }

  .chat-input textarea {
    flex: 1;
    padding: clamp(0.625rem, 1.25vh, 0.75rem) clamp(0.875rem, 1.75vw, 1rem);
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    color: #e2e8f0;
    font-size: clamp(0.8125rem, 1vw, 0.9375rem);
    font-family: inherit;
    resize: vertical;
    min-height: 3rem;
    max-height: 8rem;
    line-height: 1.5;
  }

  .chat-input textarea:focus {
    outline: none;
    border-color: #0ea5e9;
    box-shadow: 0 0 0 3px rgba(14, 165, 233, 0.1);
  }

  .chat-input textarea:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-send {
    padding: clamp(0.5rem, 1vh, 0.625rem) clamp(0.875rem, 1.75vw, 1rem);
    background: #0ea5e9;
    border: none;
    border-radius: 0.375rem;
    color: white;
    cursor: pointer;
    font-size: 1rem;
    transition: background-color 0.2s ease;
    min-width: 2.5rem;
    align-self: flex-start;
  }

  .btn-send:hover:not(:disabled) {
    background: #0284c7;
  }

  .btn-send:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
