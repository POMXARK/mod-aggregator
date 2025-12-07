<script lang="ts">
  /**
   * Компонент для отображения веб-страницы в iframe
   * 
   * Загружает HTML содержимое в iframe через Blob URL для безопасного
   * отображения обработанных страниц.
   */

  interface Props {
    /** HTML содержимое страницы для отображения */
    html: string | null;
    /** Флаг состояния загрузки */
    loading: boolean;
    /** Сообщение об ошибке, если загрузка не удалась */
    error: string | null;
    /** Ссылка на iframe элемент (bindable) */
    iframeRef?: HTMLIFrameElement | null;
  }

  let { html, loading, error, iframeRef = $bindable<HTMLIFrameElement | null>(null) }: Props = $props();

  let blobUrl: string | null = $state(null);

  /**
   * Загружает HTML в iframe через Blob URL
   * 
   * Создает Blob URL из HTML содержимого и загружает его в iframe.
   * Автоматически очищает предыдущий Blob URL при обновлении.
   */
  let previousHtml = $state<string | null>(null);
  $effect(() => {
    // Обновляем только если HTML действительно изменился
    if (!html || html === previousHtml) return;
    if (!iframeRef) return;

    previousHtml = html;

    // Очищаем предыдущий Blob URL
    if (blobUrl) {
      try {
        URL.revokeObjectURL(blobUrl);
        blobUrl = null;
      } catch (e) {
        console.warn('[PageIframe] Failed to revoke blob URL:', e);
      }
    }

    try {
      const blob = new Blob([html], { type: 'text/html;charset=utf-8' });
      const newBlobUrl = URL.createObjectURL(blob);
      blobUrl = newBlobUrl;
      
      // Устанавливаем src только если iframe еще существует
      if (iframeRef && iframeRef.contentWindow) {
        iframeRef.src = newBlobUrl;
      }
    } catch (e) {
      console.error('[PageIframe] Failed to create Blob URL:', e);
    }

    return () => {
      if (blobUrl) {
        try {
          URL.revokeObjectURL(blobUrl);
          blobUrl = null;
        } catch (e) {
          // Ignore cleanup errors
        }
      }
    };
  });
</script>

<div class="page-iframe__container">
  {#if loading}
    <div class="page-iframe__loading">Загрузка страницы...</div>
  {/if}
  {#if error}
    <div class="page-iframe__error">Ошибка: {error}</div>
  {/if}
  <iframe
    bind:this={iframeRef}
    class="page-iframe"
    title="Просмотр веб-страницы"
    sandbox="allow-same-origin allow-scripts allow-forms allow-popups allow-top-navigation"
  ></iframe>
</div>

<style>
  .page-iframe__container {
    flex: 1;
    position: relative;
    overflow: hidden;
  }

  .page-iframe__loading {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: #94a3b8;
    font-size: 1.1rem;
    z-index: 10;
  }

  .page-iframe__error {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: #fca5a5;
    font-size: 1rem;
    z-index: 10;
    background: #7f1d1d;
    padding: 1rem;
    border-radius: 0.5rem;
  }

  .page-iframe {
    width: 100%;
    height: 100%;
    border: none;
    background: white;
  }
</style>

