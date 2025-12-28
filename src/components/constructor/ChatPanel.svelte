<script lang="ts">
  import { AIChat, ResizeHandle } from '@/components';
  import type { Node, Edge } from '@xyflow/svelte';
  import type { ParserConfig } from '@/lib/api';

  interface Props {
    visible: boolean;
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
  }

  let {
    visible = $bindable(),
    width = $bindable(),
    aiModelType = 'ollama',
    aiModelName = 'llama3.2:3b',
    aiApiKey = '',
    aiOllamaUrl = 'http://localhost:11434',
    currentUrl = '',
    nodes = [],
    edges = [],
    onCreateNodes,
    onGenerateCode,
    onTestParser,
    onCheckCodeWithAI,
    generatedCode = '',
    onApplyCode,
    selectedElementInfo = null,
    addAIMessage = $bindable(),
    onResize,
    onResizeStart,
    onResizeEnd,
  }: Props = $props();

  function handleResize(newWidth: number) {
    width = newWidth;
    if (onResize) {
      onResize(newWidth);
    }
  }
</script>

{#if visible}
  <ResizeHandle
    direction="vertical"
    mode="right"
    minValue={300}
    maxValue={window.innerWidth * 0.8}
    getCurrentValue={() => width}
    onResize={handleResize}
    {onResizeStart}
    {onResizeEnd}
  />
  <div class="chat-panel" style="width: {width}px">
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
  }
</style>
