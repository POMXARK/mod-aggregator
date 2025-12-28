# Porto Dependency Injection (DI) System

## Обзор

Porto DI System - это централизованная система Dependency Injection для Porto архитектуры, которая позволяет импортировать зависимости независимо от вложенности файлов.

## Возможности

### 1. Path Mapping для коротких импортов

```typescript
// Было:
import { invoke } from '../../../../lib/tauri-wrapper';

// Стало:
import { invoke } from '@/lib/tauri-wrapper';
```

### 2. Tauri Service Locator

Централизованное управление всеми Tauri командами:

```typescript
import { tauriCommands } from '@/lib/tauri-services';

// Использование:
await tauriCommands.get_sites();
await tauriCommands.create_mod({ name: 'New Mod' });
```

### 3. Porto Context

Глобальный контекст со всеми сервисами и компонентами:

```typescript
import { portoContext } from '@/context/porto-context';

// Доступ ко всем сервисам:
const { commands, services, utils, types } = portoContext;

// Использование:
await commands.get_sites();
const sitesService = services.sites();
const htmlUtil = utils.html;
```

## Настройка tsconfig.json

```json
{
  "compilerOptions": {
    "paths": {
      "@/*": ["./src/*"],
      "@/lib/*": ["./src/lib/*"],
      "@/Containers/*": ["./src/Containers/*"],
      "@/Shipments/*": ["./src/Shipments/*"],
      "@/Data/*": ["./src/Data/*"],
      "@/Services/*": ["./src/Shipments/Services/*"],
      "@/UI/*": ["./src/Shipments/UI/*"],
      "@/Utils/*": ["./src/Shipments/Utils/*"],
      "@/Config/*": ["./src/Shipments/Config/*"]
    }
  }
}
```

## Использование

### Импорт Tauri команд

```typescript
// Старый способ (зависит от вложенности)
import { invoke } from '../../../../lib/tauri-wrapper';

// Новый способ (независимо от вложенности)
import { tauriCommands } from '@/lib/tauri-services';

// Использование
const sites = await tauriCommands.get_sites();
```

### Импорт сервисов (composables)

```typescript
// Старый способ
import { useSites } from '../../../Shipments/Services/composables/useSites';

// Новый способ
import { portoContext } from '@/context/porto-context';
const { services } = portoContext;
const sitesService = services.sites();
```

### Импорт UI компонентов

```typescript
// Старый способ
import { Sidebar } from '../../../Shipments/UI';

// Новый способ
import { Sidebar } from '@/UI';
```

### Импорт типов данных

```typescript
// Старый способ
import type { File } from '../../../Data/Models/types/file';

// Новый способ
import { portoContext } from '@/context/porto-context';
type File = portoContext.types.File;
```

## Структура Porto Context

```typescript
interface PortoContext {
  // Tauri команды
  commands: TauriCommands;

  // Сервисы (composables)
  services: {
    sites: typeof useSites;
    mods: typeof useMods;
    notifications: typeof useNotifications;
    // ... остальные сервисы
  };

  // Утилиты
  utils: {
    html: typeof htmlProcessor;
    navigation: typeof navigationScript;
    // ... остальные утилиты
  };

  // Типы данных
  types: {
    File: typeof File;
    Collection: typeof Collection;
    // ... остальные типы
  };
}
```

## Преимущества

### 1. Независимость от вложенности
- Импорты не ломаются при перемещении файлов
- Единообразие импортов во всем проекте

### 2. Централизованное управление
- Все зависимости в одном месте
- Легко добавлять новые сервисы и команды

### 3. Удобство разработки
- Короткие и понятные импорты
- Автодополнение в IDE

### 4. Масштабируемость
- Легко расширять систему новыми сервисами
- Поддержка паттернов DI

## Примеры миграции

### До (зависит от вложенности):

```typescript
// src/Containers/Sites/UI/SiteForm.svelte
import { invoke } from '../../../../lib/tauri-wrapper';
import { useSites } from '../../../Shipments/Services/composables/useSites';
import type { File } from '../../../Data/Models/types/file';
```

### После (независимо от вложенности):

```typescript
// src/Containers/Sites/UI/SiteForm.svelte
import { tauriCommands } from '@/lib/tauri-services';
import { portoContext } from '@/context/porto-context';

// Использование
await tauriCommands.get_sites();
const sitesService = portoContext.services.sites();
type File = portoContext.types.File;
```

## Добавление новых сервисов

### 1. Добавить в TauriCommands интерфейс

```typescript
// src/lib/tauri-services.ts
export interface TauriCommands {
  // Добавить новую команду
  new_command: (params: any) => Promise<any>;
}
```

### 2. Добавить в PortoContext

```typescript
// src/context/porto-context.ts
export interface PortoContext {
  services: {
    // Добавить новый сервис
    newService: typeof useNewService;
  };
}
```

### 3. Экспортировать в index.ts

```typescript
// src/Containers/NewContainer/index.ts
export { default as NewComponent } from './UI/NewComponent.svelte';
```

## Заключение

Porto DI System обеспечивает:
- **Независимость от структуры файлов**
- **Централизованное управление зависимостями**
- **Удобство разработки и поддержки**
- **Масштабируемость архитектуры**

Система позволяет легко расширять функциональность и поддерживать чистоту кода в больших проектах с Porto архитектурой.



