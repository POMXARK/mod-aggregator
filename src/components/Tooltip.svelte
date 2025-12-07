<script lang="ts">
  interface Props {
    text: string;
    position?: 'left' | 'right' | 'top' | 'bottom';
    show?: boolean;
  }

  let { text, position = 'right', show = true }: Props = $props();
</script>

<div class="tooltip" data-position={position} data-show={show}>
  <slot />
  {#if show}
    <span class="tooltip-text">{text}</span>
  {/if}
</div>

<style>
  .tooltip {
    position: relative;
    display: inline-block;
  }

  .tooltip-text {
    visibility: hidden;
    opacity: 0;
    background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
    color: #e2e8f0;
    text-align: center;
    padding: clamp(0.5rem, 0.75vw, 0.625rem) clamp(0.75rem, 1vw, 0.875rem);
    border-radius: clamp(0.375rem, 0.5vw, 0.5rem);
    position: absolute;
    z-index: 1000;
    font-size: clamp(0.75rem, 0.9vw, 0.875rem);
    font-weight: 500;
    white-space: nowrap;
    pointer-events: none;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.6), 0 0 0 1px rgba(14, 165, 233, 0.2);
    border: 1px solid rgba(148, 163, 184, 0.2);
    transition: opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1), visibility 0.3s, transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    transform: scale(0.9);
    backdrop-filter: blur(10px);
  }

  .tooltip[data-position="right"] .tooltip-text {
    left: calc(100% + clamp(0.5rem, 0.75vw, 0.75rem));
    top: 50%;
    transform: translateY(-50%) scale(0.9);
  }

  .tooltip[data-position="left"] .tooltip-text {
    right: calc(100% + clamp(0.5rem, 0.75vw, 0.75rem));
    top: 50%;
    transform: translateY(-50%) scale(0.9);
  }

  .tooltip[data-position="top"] .tooltip-text {
    bottom: calc(100% + clamp(0.5rem, 0.75vw, 0.75rem));
    left: 50%;
    transform: translateX(-50%) scale(0.9);
  }

  .tooltip[data-position="bottom"] .tooltip-text {
    top: calc(100% + clamp(0.5rem, 0.75vw, 0.75rem));
    left: 50%;
    transform: translateX(-50%) scale(0.9);
  }

  .tooltip:hover .tooltip-text,
  .tooltip:focus-within .tooltip-text {
    visibility: visible;
    opacity: 1;
    transform: translateY(-50%) scale(1);
  }

  .tooltip[data-position="left"]:hover .tooltip-text,
  .tooltip[data-position="left"]:focus-within .tooltip-text {
    transform: translateY(-50%) scale(1);
  }

  .tooltip[data-position="top"]:hover .tooltip-text,
  .tooltip[data-position="top"]:focus-within .tooltip-text {
    transform: translateX(-50%) scale(1);
  }

  .tooltip[data-position="bottom"]:hover .tooltip-text,
  .tooltip[data-position="bottom"]:focus-within .tooltip-text {
    transform: translateX(-50%) scale(1);
  }

  /* Показывать tooltip при наведении, если show=true (для скрытого сайдбара) */
  .tooltip[data-show="true"]:hover .tooltip-text,
  .tooltip[data-show="true"]:focus-within .tooltip-text {
    visibility: visible;
    opacity: 1;
  }

  .tooltip[data-show="true"][data-position="right"]:hover .tooltip-text,
  .tooltip[data-show="true"][data-position="right"]:focus-within .tooltip-text {
    transform: translateY(-50%) scale(1);
  }

  .tooltip[data-show="true"][data-position="left"]:hover .tooltip-text,
  .tooltip[data-show="true"][data-position="left"]:focus-within .tooltip-text {
    transform: translateY(-50%) scale(1);
  }

  .tooltip[data-show="true"][data-position="top"]:hover .tooltip-text,
  .tooltip[data-show="true"][data-position="top"]:focus-within .tooltip-text {
    transform: translateX(-50%) scale(1);
  }

  .tooltip[data-show="true"][data-position="bottom"]:hover .tooltip-text,
  .tooltip[data-show="true"][data-position="bottom"]:focus-within .tooltip-text {
    transform: translateX(-50%) scale(1);
  }

  .tooltip-text::after {
    content: '';
    position: absolute;
    border: 6px solid transparent;
  }

  .tooltip[data-position="right"] .tooltip-text::after {
    right: 100%;
    top: 50%;
    transform: translateY(-50%);
    border-right-color: #1e293b;
  }

  .tooltip[data-position="left"] .tooltip-text::after {
    left: 100%;
    top: 50%;
    transform: translateY(-50%);
    border-left-color: #1e293b;
  }

  .tooltip[data-position="top"] .tooltip-text::after {
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    border-top-color: #1e293b;
  }

  .tooltip[data-position="bottom"] .tooltip-text::after {
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%);
    border-bottom-color: #1e293b;
  }
</style>

