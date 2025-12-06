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
    background: #1e293b;
    border-radius: 0.75rem;
    overflow: hidden;
    border: 1px solid #334155;
    transition: transform 0.2s, box-shadow 0.2s;
  }
  
  .mod-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
  }
  
  .mod-image {
    width: 100%;
    height: 200px;
    object-fit: cover;
    background: #0f172a;
  }
  
  .mod-content {
    padding: 1.25rem;
  }
  
  .mod-title {
    margin: 0 0 0.75rem 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #e2e8f0;
    line-height: 1.4;
  }
  
  .mod-author, .mod-version {
    margin: 0.5rem 0;
    font-size: 0.875rem;
    color: #94a3b8;
  }
  
  .mod-description {
    margin: 1rem 0;
    font-size: 0.9rem;
    color: #cbd5e1;
    line-height: 1.6;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  
  .mod-changes {
    margin: 1rem 0;
    padding: 0.75rem;
    background: #0f172a;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    color: #cbd5e1;
  }
  
  .mod-changes strong {
    color: #0ea5e9;
  }
  
  .mod-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid #334155;
  }
  
  .btn-link {
    background: transparent;
    border: none;
    color: #0ea5e9;
    cursor: pointer;
    font-weight: 600;
    padding: 0.5rem 0;
    transition: color 0.2s;
  }
  
  .btn-link:hover {
    color: #38bdf8;
  }
  
  .mod-date {
    font-size: 0.75rem;
    color: #64748b;
  }
</style>

