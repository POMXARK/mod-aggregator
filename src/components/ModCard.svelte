<script lang="ts">
  import { open } from '@tauri-apps/plugin-shell';
  
  interface Props {
    mod: any;
  }
  
  let { mod }: Props = $props();
  
  async function handleOpen() {
    await open(mod.url);
  }
</script>

<div class="mod-card">
  {#if mod.image_url}
    <img src={mod.image_url} alt={mod.title} class="mod-image" />
  {/if}
  
  <div class="mod-content">
    <h3 class="mod-title">{mod.title}</h3>
    
    {#if mod.author}
      <p class="mod-author">Автор: {mod.author}</p>
    {/if}
    
    {#if mod.version}
      <p class="mod-version">Версия: {mod.version}</p>
    {/if}
    
    {#if mod.description}
      <p class="mod-description">{mod.description}</p>
    {/if}
    
    {#if mod.changes}
      <div class="mod-changes">
        <strong>Изменения:</strong>
        <p>{mod.changes}</p>
      </div>
    {/if}
    
    <div class="mod-footer">
      <button class="btn-link" onclick={handleOpen}>
        Открыть на сайте
      </button>
      <span class="mod-date">
        Обновлено: {new Date(mod.updated_at).toLocaleDateString('ru-RU')}
      </span>
    </div>
  </div>
</div>

<style>
  .mod-card {
    background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
    border-radius: clamp(0.5rem, 0.75vw, 0.75rem);
    overflow: hidden;
    border: 1px solid rgba(51, 65, 85, 0.5);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    height: 100%;
    display: flex;
    flex-direction: column;
  }
  
  .mod-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.4);
    border-color: rgba(14, 165, 233, 0.5);
  }
  
  .mod-image {
    width: 100%;
    height: clamp(12rem, 20vh, 200px);
    object-fit: cover;
    background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
    transition: transform 0.3s;
  }

  .mod-card:hover .mod-image {
    transform: scale(1.05);
  }
  
  .mod-content {
    padding: clamp(1rem, 1.5vw, 1.25rem);
    flex: 1;
    display: flex;
    flex-direction: column;
  }
  
  .mod-title {
    margin: 0 0 clamp(0.5rem, 0.75vw, 0.75rem) 0;
    font-size: clamp(1.125rem, 1.75vw, 1.25rem);
    font-weight: 600;
    color: #e2e8f0;
    line-height: 1.4;
    word-break: break-word;
  }
  
  .mod-author, .mod-version {
    margin: clamp(0.375rem, 0.5vw, 0.5rem) 0;
    font-size: clamp(0.75rem, 1vw, 0.875rem);
    color: #94a3b8;
    line-height: 1.4;
  }
  
  .mod-description {
    margin: clamp(0.75rem, 1vw, 1rem) 0;
    font-size: clamp(0.8125rem, 1.1vw, 0.9rem);
    color: #cbd5e1;
    line-height: 1.6;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
    flex: 1;
  }
  
  .mod-changes {
    margin: clamp(0.75rem, 1vw, 1rem) 0;
    padding: clamp(0.625rem, 0.875vw, 0.75rem);
    background: rgba(15, 23, 42, 0.6);
    border-radius: clamp(0.375rem, 0.5vw, 0.5rem);
    font-size: clamp(0.75rem, 1vw, 0.875rem);
    color: #cbd5e1;
    border-left: 3px solid #0ea5e9;
    line-height: 1.5;
  }
  
  .mod-changes strong {
    color: #0ea5e9;
    font-weight: 600;
  }
  
  .mod-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: clamp(0.75rem, 1vw, 1rem);
    padding-top: clamp(0.75rem, 1vw, 1rem);
    border-top: 1px solid rgba(51, 65, 85, 0.5);
    flex-wrap: wrap;
    gap: clamp(0.5rem, 0.75vw, 0.75rem);
  }
  
  .btn-link {
    background: transparent;
    border: none;
    color: #0ea5e9;
    cursor: pointer;
    font-weight: 600;
    font-size: clamp(0.8125rem, 1.1vw, 0.875rem);
    padding: clamp(0.375rem, 0.5vw, 0.5rem) 0;
    transition: all 0.2s;
    text-decoration: none;
  }
  
  .btn-link:hover {
    color: #38bdf8;
    text-decoration: underline;
  }
  
  .mod-date {
    font-size: clamp(0.6875rem, 0.9vw, 0.75rem);
    color: #64748b;
    white-space: nowrap;
  }
</style>

