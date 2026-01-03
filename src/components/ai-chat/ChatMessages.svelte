<script lang="ts">
  import type { Message } from '@/lib/composables';
  import MessageActions from '@/components/ai-chat/MessageActions.svelte';

  interface Props {
    messages: Message[];
    hasJsonConfig: (content: string) => boolean;
    canRunParser: boolean;
    onApplyCode: (messageId: string) => void;
    onRestoreConfig: (messageId: string) => void;
    onGenerateCode: (messageId: string) => void;
    onRunParser: () => void;
    onCopy: (messageId: string) => void;
    onCopyText: (messageId: string) => void;
    onDelete: (messageId: string) => void;
  }

  const {
    messages,
    hasJsonConfig,
    canRunParser,
    onApplyCode,
    onRestoreConfig,
    onGenerateCode,
    onRunParser,
    onCopy,
    onCopyText,
    onDelete,
  }: Props = $props();

  let chatContainer: HTMLDivElement;
</script>

<div class="chat-messages" bind:this={chatContainer}>
  {#each messages as message (message.id)}
    <div class="message message-{message.role}">
      <div class="message-header">
        <MessageActions
          {message}
          hasJsonConfig={hasJsonConfig(message.content)}
          hasCode={message.content.includes('```') || message.content.includes('<code>')}
          hasParserMention={message.content.includes('Парсер') ||
            message.content.includes('парсер') ||
            message.content.includes('конфигурацию') ||
            message.content.includes('селектор')}
          {canRunParser}
          onApplyCode={() => onApplyCode(message.id)}
          onRestoreConfig={() => onRestoreConfig(message.id)}
          onGenerateCode={() => onGenerateCode(message.id)}
          {onRunParser}
          onCopy={() => onCopy(message.id)}
          onCopyText={() => onCopyText(message.id)}
          onDelete={() => onDelete(message.id)}
        />
      </div>
      <div class="message-content">
        {#if message.attachedElement}
          <div class="attached-element">
            <div class="attached-element-header">
              <span class="attached-icon">📎</span>
              <span class="attached-label">Прикреплен элемент</span>
            </div>
            <div class="attached-element-info">
              <div class="attached-info-row">
                <strong>Селектор:</strong> <code>{message.attachedElement.selector}</code>
              </div>
              <div class="attached-info-row">
                <strong>Тег:</strong>
                <span class="tag-badge">{message.attachedElement.tagName}</span>
              </div>
              {#if message.attachedElement.text}
                <div class="attached-info-row">
                  <strong>Текст:</strong>
                  <span class="text-preview">{message.attachedElement.text.slice(0, 100)}</span>
                </div>
              {/if}
            </div>
          </div>
        {/if}

        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html message.content
          .replace(
            /```(?:rust|rs|json)?\s*([\s\S]*?)\s*```/g,
            '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
          )
          .replace(
            /\{[\s\S]*?"list_selector"[\s\S]*?\}/g,
            '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
          )
          .replace(
            /<pre[^>]*>[\s\S]*?<\/pre>/gi,
            '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
          )
          .replace(
            /<code[^>]*>[\s\S]*?<\/code>/gi,
            '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
          )
          .replace(
            /<div[^>]*class="code-header"[^>]*>[\s\S]*?<\/div>/gi,
            '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
          )
          .replace(
            /<div[^>]*class="code-content"[^>]*>[\s\S]*?<\/div>/gi,
            '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
          )
          .replace(
            /<h3[^>]*>Код<\/h3>/gi,
            '<div class="code-reference">[Код сгенерирован и доступен в нижней панели]</div>'
          )
          .replace(/<button[^>]*>Проверить AI<\/button>/gi, '')
          .replace(/<button[^>]*>Редактировать<\/button>/gi, '')
          .replace(/<button[^>]*>Копировать<\/button>/gi, '')
          .replace(/\n/g, '<br>')}
      </div>
      <div class="message-time">
        {message.timestamp.toLocaleTimeString('ru-RU', { hour: '2-digit', minute: '2-digit' })}
      </div>
    </div>
  {/each}
</div>

<style>
  .chat-messages {
    flex: 1;
    overflow-y: auto;
    padding: clamp(0.75rem, 1.5vh, 1rem);
    display: flex;
    flex-direction: column;
    gap: clamp(0.75rem, 1.5vh, 1rem);
  }

  .message {
    display: flex;
    flex-direction: column;
    max-width: 85%;
    animation: messageFadeIn 0.3s ease-out;
    position: relative;
  }

  @keyframes messageFadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .message-header {
    display: flex;
    justify-content: flex-end;
    margin-bottom: 0.25rem;
  }

  .message-user {
    align-self: flex-end;
  }

  .message-assistant {
    align-self: flex-start;
  }

  .message-content {
    padding: clamp(0.625rem, 1.25vh, 0.75rem) clamp(0.875rem, 1.75vw, 1rem);
    border-radius: 0.75rem;
    font-size: clamp(0.875rem, 1.1vw, 1rem);
    line-height: 1.5;
    word-wrap: break-word;
  }

  .message-user .message-content {
    background: #0ea5e9;
    color: white;
    border-bottom-right-radius: 0.25rem;
  }

  .message-assistant .message-content {
    background: #334155;
    color: #e2e8f0;
    border-bottom-left-radius: 0.25rem;
  }

  .attached-element {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 0.5rem;
    padding: clamp(0.5rem, 1vh, 0.75rem);
    margin-bottom: 0.75rem;
  }

  .attached-element-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
    font-weight: 600;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
  }

  .attached-icon {
    font-size: 1rem;
  }

  .attached-label {
    color: rgba(255, 255, 255, 0.9);
  }

  .attached-element-info {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
  }

  .attached-info-row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: center;
  }

  .attached-info-row strong {
    color: rgba(255, 255, 255, 0.9);
  }

  .attached-info-row code {
    background: rgba(0, 0, 0, 0.3);
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    font-family: 'Courier New', monospace;
    font-size: 0.875em;
    color: #10b981;
  }

  .tag-badge {
    background: rgba(14, 165, 233, 0.2);
    color: #0ea5e9;
    padding: 0.125rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.875em;
    font-weight: 600;
  }

  .text-preview {
    color: rgba(255, 255, 255, 0.8);
    font-style: italic;
  }

  .code-reference {
    padding: 0.5rem;
    background: rgba(14, 165, 233, 0.1);
    border: 1px solid rgba(14, 165, 233, 0.3);
    border-radius: 0.375rem;
    color: #0ea5e9;
    font-size: 0.875rem;
    margin: 0.5rem 0;
    font-style: italic;
  }

  .message-time {
    font-size: clamp(0.625rem, 0.8vw, 0.75rem);
    color: #64748b;
    margin-top: 0.25rem;
    padding: 0 0.5rem;
  }

  .message-user .message-time {
    text-align: right;
  }

  .message:hover .message-actions {
    opacity: 1;
  }
</style>
