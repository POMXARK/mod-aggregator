# Примеры использования Porto DI System

## 1. Использование Tauri команд

### Старый подход (зависит от вложенности):
```typescript
// src/Containers/Sites/UI/SiteForm.svelte
import { invoke } from '../../../../lib/tauri-wrapper';

await invoke('get_sites');
await invoke('create_site', { name: 'New Site', url: 'https://example.com' });
```

### Новый подход (независимо от вложенности):
```typescript
// Любой файл в любом контейнере
import { tauriCommands } from '@/lib/tauri-services';

await tauriCommands.get_sites();
await tauriCommands.create_site({ name: 'New Site', url: 'https://example.com' });
```

## 2. Использование сервисов (composables)

### Старый подход:
```typescript
// src/Containers/Sites/UI/SitesManager.svelte
import { useSites } from '../../../Shipments/Services/composables/useSites';
import { useNotifications } from '../../../Shipments/Services/composables/useNotifications';

const sitesService = useSites();
const notificationsService = useNotifications();
```

### Новый подход с Porto Context:
```typescript
// Любой файл в любом контейнере
import { portoContext } from '@/context/porto-context';

const { services } = portoContext;

// Синхронное использование (если сервис уже загружен)
const sitesService = await services.sites();
const notificationsService = await services.notifications();

// Или через хук
import { usePortoContext } from '@/context/porto-context';

function MyComponent() {
  const { services } = usePortoContext();

  // Использование в эффекте или обработчике
  async function loadData() {
    const sitesService = await services.sites();
    const sites = await sitesService.getSites();
    // ...
  }
}
```

## 3. Использование утилит

### Старый подход:
```typescript
// src/Containers/Parser/UI/constructor/BrowserPanel.svelte
import { htmlProcessor } from '../../../../../Utils/utils/html-processor';
import { pageResources } from '../../../../../Utils/utils/page-resources';

const processedHtml = htmlProcessor.process(html);
const resources = pageResources.extract(url);
```

### Новый подход:
```typescript
// Любой файл в любом контейнере
import { portoContext } from '@/context/porto-context';

const { utils } = portoContext;

const htmlUtil = await utils.html();
const pageUtil = await utils.pageResources();

const processedHtml = htmlUtil.process(html);
const resources = pageUtil.extract(url);
```

## 4. Использование UI компонентов

### Старый подход:
```typescript
// src/Containers/Sites/UI/SitesManager.svelte
import { PlusIcon, TrashIcon } from '../../../Shipments/UI';
import { Sidebar } from '../../../Shipments/UI';
```

### Новый подход:
```typescript
// Любой файл в любом контейнере
import { PlusIcon, TrashIcon } from '@/UI/icons';
import { Sidebar } from '@/UI';
```

## 5. Импорт типов данных

### Старый подход:
```typescript
// src/Containers/Files/UI/FileList.svelte
import type { File } from '../../../../Data/Models/types/file';
import type { Collection } from '../../../../Data/Models/types/collection';
```

### Новый подход:
```typescript
// Любой файл в любом контейнере
import type { File, Collection } from '@/Data/Models';
```

## 6. Полный пример компонента

```typescript
// src/Containers/Example/UI/ExampleComponent.svelte
<script lang="ts">
  // Porto DI System - все импорты независимо от вложенности
  import { tauriCommands } from '@/lib/tauri-services';
  import { portoContext } from '@/context/porto-context';
  import { PlusIcon, TrashIcon } from '@/UI/icons';
  import type { File } from '@/Data/Models';

  // Деструктуризация для удобства
  const { commands, services, utils } = portoContext;

  // Локальное состояние
  let files = $state<File[]>([]);
  let loading = $state(false);

  // Загрузка данных при монтировании
  $effect(() => {
    loadFiles();
  });

  async function loadFiles() {
    loading = true;
    try {
      // Использование Tauri команд
      files = await commands.get_files();

      // Показать уведомление через сервис
      const notifications = await services.notifications();
      notifications.showSuccess('Файлы загружены успешно');
    } catch (error) {
      const notifications = await services.notifications();
      notifications.showError('Ошибка загрузки файлов');
    } finally {
      loading = false;
    }
  }

  async function createFile(name: string) {
    try {
      // Использование команд
      const newFile = await commands.create_file({ name });

      // Использование утилит
      const htmlUtil = await utils.html();
      const processedName = htmlUtil.sanitize(name);

      files = [...files, { ...newFile, name: processedName }];

      // Уведомление об успехе
      const notifications = await services.notifications();
      notifications.showSuccess(`Файл "${processedName}" создан`);
    } catch (error) {
      const notifications = await services.notifications();
      notifications.showError('Ошибка создания файла');
    }
  }

  async function deleteFile(fileId: number) {
    try {
      await commands.delete_file(fileId);
      files = files.filter(f => f.id !== fileId);

      const notifications = await services.notifications();
      notifications.showSuccess('Файл удален');
    } catch (error) {
      const notifications = await services.notifications();
      notifications.showError('Ошибка удаления файла');
    }
  }
</script>

<div class="example-component">
  <h2>Управление файлами</h2>

  {#if loading}
    <p>Загрузка...</p>
  {:else}
    <div class="files-list">
      {#each files as file}
        <div class="file-item">
          <span>{file.name}</span>
          <button onclick={() => deleteFile(file.id)}>
            <TrashIcon />
          </button>
        </div>
      {/each}
    </div>

    <button onclick={() => createFile('Новый файл')}>
      <PlusIcon />
      Создать файл
    </button>
  {/if}
</div>

<style>
  .example-component {
    padding: 1rem;
  }

  .files-list {
    margin: 1rem 0;
  }

  .file-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    border: 1px solid #ddd;
    margin-bottom: 0.5rem;
    border-radius: 4px;
  }
</style>
```

## Преимущества DI системы

### 1. Независимость от структуры
- Импорты не ломаются при перемещении файлов
- Можно реорганизовывать код без изменения импортов

### 2. Централизованное управление
- Все сервисы в одном месте
- Легко добавлять новые зависимости

### 3. Lazy loading
- Сервисы загружаются только при необходимости
- Уменьшает размер бандла

### 4. Удобство разработки
- Короткие и понятные импорты
- Автодополнение в IDE

### 5. Тестируемость
- Легко заменять сервисы для тестирования
- Изоляция зависимостей

## Добавление новых сервисов

### 1. Добавить команду в TauriCommands:
```typescript
// src/lib/tauri-services.ts
export interface TauriCommands {
  new_command: (params: any) => Promise<any>;
}
```

### 2. Добавить сервис в PortoContext:
```typescript
// src/context/porto-context.ts
services: {
  newService: () => import('@/Services/composables/useNewService').then(m => m.useNewService);
}
```

### 3. Использовать в компонентах:
```typescript
const { services } = portoContext;
const newService = await services.newService();
```














