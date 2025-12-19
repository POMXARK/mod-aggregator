<!-- 
  Универсальный компонент ноды
  Использует BaseNode из фреймворка
-->
<script lang="ts">
  import BaseNode from '../framework/BaseNode.svelte';
  import type { NodeProps } from '@xyflow/svelte';
  
  type Props = NodeProps<any>;
  let props: Props = $props();
</script>

<BaseNode {...props} />
