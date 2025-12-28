<script lang="ts">
  import PageViewer from '../PageViewer.svelte';
  import ResizeHandle from '../common/ResizeHandle.svelte';

  interface Props {
    visible: boolean;
    width: number;
    url: string;
    siteId?: number | null;
    onElementSelect?: (selector: string, element: HTMLElement | null, data?: unknown) => void;
    onResize?: (newWidth: number) => void;
    onResizeStart?: () => void;
    onResizeEnd?: () => void;
  }

  let {
    visible = $bindable(),
    width = $bindable(),
    url = $bindable(),
    siteId,
    onElementSelect,
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
  <div class="browser-panel" style="width: {width}px">
    <PageViewer
      bind:url
      {siteId}
      {onElementSelect}
      onRefreshCache={() => console.log('Cache refreshed')}
    />
  </div>
  <ResizeHandle
    direction="vertical"
    mode="left"
    minValue={300}
    maxValue={window.innerWidth * 0.8}
    getCurrentValue={() => width}
    onResize={handleResize}
    {onResizeStart}
    {onResizeEnd}
  />
{/if}

<style>
  .browser-panel {
    flex-shrink: 0;
    height: 100%;
    overflow: hidden;
    border-right: 1px solid #334155;
    background: #0f172a;
  }
</style>
