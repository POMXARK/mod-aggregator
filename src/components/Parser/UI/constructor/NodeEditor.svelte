<script lang="ts">
  import { SvelteFlow, Background, Controls, MiniMap } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import type { Node, Edge, Connection, NodeTypes } from '@xyflow/svelte';

  import SelectorNode from '@/components/nodes/SelectorNode.svelte';
  import ExtractNode from '@/components/nodes/ExtractNode.svelte';
  import FilterNode from '@/components/nodes/FilterNode.svelte';
  import TransformNode from '@/components/nodes/TransformNode.svelte';
  import OutputNode from '@/components/nodes/OutputNode.svelte';

  const nodeTypes: NodeTypes = {
    selector: SelectorNode,
    extract: ExtractNode,
    filter: FilterNode,
    transform: TransformNode,
    output: OutputNode,
  };

  interface Props {
    nodes: Node[];
    edges: Edge[];
    withViewer?: boolean;
    withChat?: boolean;
    withResults?: boolean;
    onConnect?: (connection: Connection) => void;
    onNodesChange?: (changes: unknown[]) => void;
    onEdgesChange?: (changes: unknown[]) => void;
    onPaneClick?: (event: MouseEvent) => void;
    onPaneContextMenu?: (event: MouseEvent) => void;
    style?: string;
  }

  let {
    nodes = $bindable(),
    edges = $bindable(),
    withViewer = false,
    withChat = false,
    withResults = false,
    onConnect,
    onNodesChange,
    onEdgesChange,
    onPaneClick,
    onPaneContextMenu,
    style = '',
  }: Props = $props();
</script>

<div
  class="node-editor"
  class:has-viewer={withViewer}
  class:has-chat={withChat}
  class:has-results={withResults}
  {style}
>
  <SvelteFlow
    {nodes}
    {edges}
    {nodeTypes}
    {onConnect}
    {onNodesChange}
    {onEdgesChange}
    {onPaneClick}
    onpancontextmenu={onPaneContextMenu}
    class="flow-container"
  >
    <Background />
    <Controls />
    <MiniMap />
  </SvelteFlow>
</div>

<style>
  .node-editor {
    flex: 1 1 auto;
    min-width: 0;
    height: 100%;
    position: relative;
    background: #0f172a;
  }

  .flow-container {
    width: 100%;
    height: 100%;
  }
</style>
