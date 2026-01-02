<script lang="ts">
  import { AIChat, ResizeHandle } from '@/components';
  import type { Node, Edge } from '@xyflow/svelte';
  import type { ParserConfig } from '@/lib/api';

  interface Props {
    visible?: boolean;
    width: number;
    aiModelType?: 'ollama' | 'openai' | 'anthropic' | 'google';
    aiModelName?: string;
    aiApiKey?: string;
    aiOllamaUrl?: string;
    currentUrl?: string;
    nodes?: Node[];
    edges?: Edge[];
    onCreateNodes?: (config: ParserConfig) => void;
    onGenerateCode?: () => void;
    onTestParser?: () => Promise<void>;
    onCheckCodeWithAI?: () => Promise<void>;
    generatedCode?: string;
    onApplyCode?: (code: string) => void;
    selectedElementInfo?: {
      selector: string;
      elementInfo: {
        tagName: string;
        text: string;
        attributes: Record<string, string>;
        similarElements?: number;
      };
    } | null;
    addAIMessage?: (content: string) => void;
    onResize?: (newWidth: number) => void;
    onResizeStart?: () => void;
    onResizeEnd?: () => void;
    onChatChange?: (chatId: string | null) => void;
    className?: string;
  }

  let {
    visible = false,
    width = $bindable(400),
    aiModelType,
    aiModelName,
    aiApiKey,
    aiOllamaUrl,
    currentUrl,
    nodes,
    edges,
    onCreateNodes,
    onGenerateCode,
    onTestParser,
    onCheckCodeWithAI,
    generatedCode,
    onApplyCode,
    selectedElementInfo,
    addAIMessage = $bindable(),
    onResize,
    onResizeStart,
    onResizeEnd,
    onChatChange,
    className = '',
  }: Props = $props();


  // Отслеживание изменений видимости для правильной работы компонента

  function handleResize(newWidth: number) {
    width = newWidth;
    if (onResize) {
      onResize(newWidth);
    }
  }
</script>

{#if visible}
  <div class="chat-panel {className}" style="width: {width || 400}px">
    <AIChat
      {aiModelType}
      {aiModelName}
      {aiApiKey}
      {aiOllamaUrl}
      {currentUrl}
      {nodes}
      {edges}
      {onCreateNodes}
      {onGenerateCode}
      {onTestParser}
      {onCheckCodeWithAI}
      {generatedCode}
      {onApplyCode}
      {selectedElementInfo}
      bind:addAIMessage
      {onChatChange}
    />
  </div>
{/if}

<style>
  .chat-panel {
    flex-shrink: 0;
    height: 100%;
    display: flex;
    flex-direction: column;
    border-left: 1px solid #334155;
    background: #1e293b;
    position: relative;
    z-index: 1;
  }
</style>
