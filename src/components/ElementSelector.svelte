<script lang="ts">
  interface Props {
    selector: string;
    elementInfo: {
      tagName: string;
      text: string;
      attributes: Record<string, string>;
      similarElements?: number;
    };
    onConfirm: () => void;
    onCancel: () => void;
    onAutoDetect?: () => void;
  }

  let { selector, elementInfo, onConfirm, onCancel, onAutoDetect }: Props = $props();
</script>

<div class="element-selector">
  <div class="selector-header">
    <h3>Выбранный элемент</h3>
    <button class="btn-close" onclick={onCancel} aria-label="Закрыть">×</button>
  </div>
  
  <div class="selector-content">
    <div class="info-group">
      <div class="info-label">CSS Селектор:</div>
      <code class="selector-code">{selector}</code>
    </div>
    
    <div class="info-group">
      <div class="info-label">Тег:</div>
      <span class="tag-badge">{elementInfo.tagName}</span>
    </div>
    
    {#if elementInfo.text}
      <div class="info-group">
        <div class="info-label">Текст:</div>
        <div class="text-preview">{elementInfo.text}</div>
      </div>
    {/if}
    
    {#if elementInfo.similarElements !== undefined}
      <div class="info-group">
        <div class="info-label">Похожих элементов:</div>
        <span class="count-badge">{elementInfo.similarElements}</span>
      </div>
    {/if}
    
    {#if Object.keys(elementInfo.attributes).length > 0}
      <div class="info-group">
        <div class="info-label">Атрибуты:</div>
        <div class="attributes-list">
          {#each Object.entries(elementInfo.attributes) as [key, value]}
            <div class="attribute-item">
              <strong>{key}:</strong> <span>{value}</span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
  
  <div class="selector-actions">
    {#if onAutoDetect}
      <button class="btn-auto" onclick={onAutoDetect}>
        🔍 Автоопределение
      </button>
    {/if}
    <button class="btn-confirm" onclick={onConfirm}>
      ✓ Создать ноду и сгенерировать код
    </button>
    <button class="btn-cancel" onclick={onCancel}>
      Отмена
    </button>
  </div>
</div>

<style>
  .element-selector {
    position: fixed;
    bottom: 20px;
    right: 20px;
    width: 400px;
    max-height: 600px;
    background: #1e293b;
    border: 2px solid #0ea5e9;
    border-radius: 0.75rem;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
    z-index: 10000;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .selector-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.25rem;
    background: #0f172a;
    border-bottom: 1px solid #334155;
  }

  .selector-header h3 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: #0ea5e9;
  }

  .btn-close {
    background: transparent;
    border: none;
    color: #94a3b8;
    font-size: 1.5rem;
    cursor: pointer;
    padding: 0;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.25rem;
    transition: all 0.2s;
  }

  .btn-close:hover {
    background: #334155;
    color: #e2e8f0;
  }

  .selector-content {
    padding: 1.25rem;
    overflow-y: auto;
    flex: 1;
  }

  .info-group {
    margin-bottom: 1rem;
  }

  .info-group:last-child {
    margin-bottom: 0;
  }

  .info-label {
    display: block;
    font-size: 0.75rem;
    font-weight: 600;
    color: #94a3b8;
    margin-bottom: 0.5rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .selector-code {
    display: block;
    padding: 0.75rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    color: #10b981;
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
    word-break: break-all;
    line-height: 1.5;
  }

  .tag-badge {
    display: inline-block;
    padding: 0.375rem 0.75rem;
    background: #1e3a8a;
    color: #0ea5e9;
    border-radius: 0.375rem;
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .text-preview {
    padding: 0.75rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    color: #e2e8f0;
    font-size: 0.875rem;
    max-height: 100px;
    overflow-y: auto;
    line-height: 1.5;
  }

  .count-badge {
    display: inline-block;
    padding: 0.375rem 0.75rem;
    background: #1e3a8a;
    color: #0ea5e9;
    border-radius: 0.375rem;
    font-weight: 600;
  }

  .attributes-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .attribute-item {
    padding: 0.5rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    font-size: 0.875rem;
  }

  .attribute-item strong {
    color: #0ea5e9;
    margin-right: 0.5rem;
  }

  .attribute-item span {
    color: #cbd5e1;
  }

  .selector-actions {
    display: flex;
    gap: 0.5rem;
    padding: 1rem 1.25rem;
    background: #0f172a;
    border-top: 1px solid #334155;
  }

  .btn-auto,
  .btn-confirm,
  .btn-cancel {
    flex: 1;
    padding: 0.75rem 1rem;
    border: none;
    border-radius: 0.5rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.875rem;
  }

  .btn-auto {
    background: #7c3aed;
    color: white;
  }

  .btn-auto:hover {
    background: #6d28d9;
  }

  .btn-confirm {
    background: #10b981;
    color: white;
  }

  .btn-confirm:hover {
    background: #059669;
  }

  .btn-cancel {
    background: #334155;
    color: #e2e8f0;
  }

  .btn-cancel:hover {
    background: #475569;
  }
</style>

