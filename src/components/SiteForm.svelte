<script lang="ts">
  import { invoke } from '../lib/tauri-wrapper';
  import XMarkIcon from './icons/XMarkIcon.svelte';
  
  interface Props {
    site: any | null;
    onClose: () => void;
    onSubmit: () => void;
  }
  
  let { site, onClose, onSubmit }: Props = $props();
  
  // Use $derived to reactively get initial values from site prop
  let initialName = $derived(site?.name || '');
  let initialUrl = $derived(site?.url || '');
  let initialListSelector = $derived(site?.parser_config?.list_selector || '');
  let initialTitleSelector = $derived(site?.parser_config?.title_selector || '');
  let initialUrlSelector = $derived(site?.parser_config?.url_selector || '');
  let initialVersionSelector = $derived(site?.parser_config?.version_selector || '');
  let initialAuthorSelector = $derived(site?.parser_config?.author_selector || '');
  let initialImageSelector = $derived(site?.parser_config?.image_selector || '');
  let initialBaseUrl = $derived(site?.parser_config?.base_url || '');
  let initialListUrl = $derived(site?.parser_config?.list_url || '');
  
  let name = $state(initialName);
  let url = $state(initialUrl);
  let listSelector = $state(initialListSelector);
  let titleSelector = $state(initialTitleSelector);
  let urlSelector = $state(initialUrlSelector);
  let versionSelector = $state(initialVersionSelector);
  let authorSelector = $state(initialAuthorSelector);
  let imageSelector = $state(initialImageSelector);
  let baseUrl = $state(initialBaseUrl);
  let listUrl = $state(initialListUrl);
  
  // Update state when site prop changes
  $effect(() => {
    if (site) {
      name = site.name || '';
      url = site.url || '';
      listSelector = site.parser_config?.list_selector || '';
      titleSelector = site.parser_config?.title_selector || '';
      urlSelector = site.parser_config?.url_selector || '';
      versionSelector = site.parser_config?.version_selector || '';
      authorSelector = site.parser_config?.author_selector || '';
      imageSelector = site.parser_config?.image_selector || '';
      baseUrl = site.parser_config?.base_url || '';
      listUrl = site.parser_config?.list_url || '';
    } else {
      name = '';
      url = '';
      listSelector = '';
      titleSelector = '';
      urlSelector = '';
      versionSelector = '';
      authorSelector = '';
      imageSelector = '';
      baseUrl = '';
      listUrl = '';
    }
  });
  
  let saving = $state(false);
  let error = $state<string | null>(null);
  
  async function handleSubmit() {
    if (!name || !url) {
      error = 'Заполните обязательные поля';
      return;
    }
    
    saving = true;
    error = null;
    
    const parserConfig = {
      list_url: listUrl || url,
      base_url: baseUrl || url,
      list_selector: listSelector,
      title_selector: titleSelector,
      url_selector: urlSelector,
      version_selector: versionSelector,
      author_selector: authorSelector,
      image_selector: imageSelector,
    };
    
    try {
      if (site) {
        await invoke('update_site', {
          id: site.id,
          name,
          url,
          parserConfig,
        });
      } else {
        await invoke('add_site', {
          name,
          url,
          parserConfig,
        });
      }
      onSubmit();
    } catch (e: any) {
      error = e.toString();
    } finally {
      saving = false;
    }
  }
  
  function handleClose() {
    onClose();
  }
</script>

<div class="modal-overlay" onclick={handleClose}>
  <div class="modal-content" onclick={(e) => e.stopPropagation()}>
    <div class="modal-header">
      <h2>{site ? 'Редактировать сайт' : 'Добавить сайт'}</h2>
      <button class="btn-close" onclick={handleClose}>
        <XMarkIcon class="icon" />
      </button>
    </div>
    
    <form class="form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
      {#if error}
        <div class="error">{error}</div>
      {/if}
      
      <div class="form-group">
        <label>Название *</label>
        <input type="text" bind:value={name} required />
      </div>
      
      <div class="form-group">
        <label>URL *</label>
        <input type="url" bind:value={url} required />
      </div>
      
      <div class="form-group">
        <label>URL списка модов</label>
        <input type="url" bind:value={listUrl} placeholder="Если отличается от основного URL" />
      </div>
      
      <div class="form-group">
        <label>Базовый URL</label>
        <input type="url" bind:value={baseUrl} placeholder="Для относительных ссылок" />
      </div>
      
      <h3 class="section-title">Селекторы парсера</h3>
      
      <div class="form-group">
        <label>Селектор списка модов *</label>
        <input type="text" bind:value={listSelector} placeholder=".mod-item" required />
      </div>
      
      <div class="form-group">
        <label>Селектор названия</label>
        <input type="text" bind:value={titleSelector} placeholder=".mod-title" />
      </div>
      
      <div class="form-group">
        <label>Селектор ссылки</label>
        <input type="text" bind:value={urlSelector} placeholder="a.mod-link" />
      </div>
      
      <div class="form-group">
        <label>Селектор версии</label>
        <input type="text" bind:value={versionSelector} placeholder=".mod-version" />
      </div>
      
      <div class="form-group">
        <label>Селектор автора</label>
        <input type="text" bind:value={authorSelector} placeholder=".mod-author" />
      </div>
      
      <div class="form-group">
        <label>Селектор изображения</label>
        <input type="text" bind:value={imageSelector} placeholder="img.mod-image" />
      </div>
      
      <div class="form-actions">
        <button type="button" class="btn-secondary" onclick={handleClose}>
          Отмена
        </button>
        <button type="submit" class="btn-primary" disabled={saving}>
          {saving ? 'Сохранение...' : 'Сохранить'}
        </button>
      </div>
    </form>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 2rem;
  }
  
  .modal-content {
    background: #1e293b;
    border-radius: 1rem;
    width: 100%;
    max-width: 600px;
    max-height: 90vh;
    overflow-y: auto;
    border: 1px solid #334155;
  }
  
  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #334155;
  }
  
  .modal-header h2 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 700;
    color: #e2e8f0;
  }
  
  .btn-close {
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    padding: 0.5rem;
    border-radius: 0.375rem;
    transition: all 0.2s;
  }
  
  .btn-close:hover {
    background: #334155;
    color: #e2e8f0;
  }
  
  .form {
    padding: 1.5rem;
  }
  
  .form-group {
    margin-bottom: 1.25rem;
  }
  
  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 600;
    color: #cbd5e1;
    font-size: 0.875rem;
  }
  
  .form-group input {
    width: 100%;
    padding: 0.75rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    color: #e2e8f0;
    font-size: 0.95rem;
    transition: border-color 0.2s;
  }
  
  .form-group input:focus {
    outline: none;
    border-color: #0ea5e9;
  }
  
  .section-title {
    margin: 2rem 0 1rem 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: #0ea5e9;
    padding-top: 1.5rem;
    border-top: 1px solid #334155;
  }
  
  .error {
    padding: 0.75rem;
    background: #7f1d1d;
    color: #fca5a5;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }
  
  .form-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 2rem;
    padding-top: 1.5rem;
    border-top: 1px solid #334155;
  }
  
  .btn-secondary {
    padding: 0.75rem 1.5rem;
    background: transparent;
    border: 1px solid #334155;
    color: #94a3b8;
    border-radius: 0.5rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }
  
  .btn-secondary:hover {
    background: #334155;
    color: #e2e8f0;
  }
  
  .btn-primary {
    padding: 0.75rem 1.5rem;
    background: #0ea5e9;
    color: white;
    border: none;
    border-radius: 0.5rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }
  
  .btn-primary:hover:not(:disabled) {
    background: #0284c7;
  }
  
  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .icon {
    width: 20px;
    height: 20px;
  }
</style>

