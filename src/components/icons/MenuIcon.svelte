<script lang="ts">
  interface Props {
    class?: string;
  }
  let { class: className = "" }: Props = $props();
</script>

<svg class={className} fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" style="width: 100%; height: 100%;">
  <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
</svg>





