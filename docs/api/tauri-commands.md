# Tauri Commands API

Документация всех Tauri команд для взаимодействия между frontend и backend.

## Общие правила

- Все команды асинхронные (`async fn`)
- Все команды возвращают `Result<T, String>`
- Ошибки возвращаются как `String` в `Err`
- Успешные результаты возвращаются в `Ok`

## Сайты (Sites)

### `get_sites()`

Получить список всех сайтов из базы данных.

**Параметры:** нет

**Возвращает:** `Result<Vec<Site>, String>`

**Пример:**
```typescript
const sites = await invoke<Site[]>('get_sites');
```

### `add_site(name: string, url: string, parser_config: ParserConfig)`

Добавить новый сайт в базу данных.

**Параметры:**
- `name: string` - название сайта
- `url: string` - URL сайта (должен быть валидным HTTP/HTTPS)
- `parser_config: ParserConfig` - конфигурация парсера

**Возвращает:** `Result<Site, String>`

**Ошибки:**
- `"Invalid URL"` - невалидный URL
- `"Site already exists"` - сайт с таким URL уже существует
- `"Database error: ..."` - ошибка базы данных

**Пример:**
```typescript
const site = await invoke<Site>('add_site', {
  name: 'Example Site',
  url: 'https://example.com',
  parser_config: {
    root: '.mod-list',
    children: { title: '.mod-title' }
  }
});
```

### `update_site(id: number, name: string, url: string, parser_config: ParserConfig)`

Обновить существующий сайт.

**Параметры:**
- `id: number` - ID сайта
- `name: string` - новое название
- `url: string` - новый URL
- `parser_config: ParserConfig` - новая конфигурация парсера

**Возвращает:** `Result<Site, String>`

### `delete_site(id: number)`

Удалить сайт из базы данных.

**Параметры:**
- `id: number` - ID сайта

**Возвращает:** `Result<(), String>`

## Страницы (Pages)

### `fetch_page(url: string, force_refresh: boolean, site_id?: number)`

Загрузить HTML страницу с сервера или из кэша.

**Параметры:**
- `url: string` - URL страницы
- `force_refresh: boolean` - игнорировать кэш и загрузить с сервера
- `site_id?: number` - ID сайта для привязки кэша (опционально)

**Возвращает:** `Result<string, String>` - HTML содержимое

**Ошибки:**
- `"Invalid URL"` - невалидный URL
- `"Network error: ..."` - ошибка сети
- `"Cache error: ..."` - ошибка работы с кэшем

**Пример:**
```typescript
// Загрузка с сервера
const html = await invoke<string>('fetch_page', {
  url: 'https://example.com/mods',
  force_refresh: true,
  site_id: 1
});

// Загрузка из кэша
const html = await invoke<string>('fetch_page', {
  url: 'https://example.com/mods',
  force_refresh: false,
  site_id: 1
});
```

### `get_cached_page(url: string, site_id?: number)`

Получить страницу из кэша без загрузки с сервера.

**Параметры:**
- `url: string` - URL страницы
- `site_id?: number` - ID сайта

**Возвращает:** `Result<string | null, String>` - HTML или null если нет в кэше

### `save_page_local(url: string, html: string, site_id: number)`

Сохранить страницу локально с привязкой к сайту и версионированием.

**Параметры:**
- `url: string` - URL страницы
- `html: string` - HTML содержимое
- `site_id: number` - ID сайта

**Возвращает:** `Result<SavedPage, String>`

### `get_saved_page_for_site(site_id: number)`

Получить последнюю сохраненную версию страницы для сайта.

**Параметры:**
- `site_id: number` - ID сайта

**Возвращает:** `Result<SavedPage | null, String>`

### `get_saved_page_versions(site_id: number)`

Получить все версии сохраненной страницы для сайта.

**Параметры:**
- `site_id: number` - ID сайта

**Возвращает:** `Result<Vec<SavedPage>, String>`

## Парсинг (Parsing)

### `parse_page(url: string, site_id: number, force_refresh: boolean)`

Парсить страницу используя конфигурацию парсера сайта.

**Параметры:**
- `url: string` - URL страницы
- `site_id: number` - ID сайта (используется для получения конфигурации парсера)
- `force_refresh: boolean` - принудительно обновить страницу

**Возвращает:** `Result<ParserResult, String>`

**Процесс:**
1. Загружает конфигурацию парсера для сайта
2. Загружает HTML страницу (из кэша или с сервера)
3. Валидирует селекторы
4. Парсит HTML
5. Возвращает результат

**Пример:**
```typescript
const result = await invoke<ParserResult>('parse_page', {
  url: 'https://example.com/mods',
  site_id: 1,
  force_refresh: false
});
```

## Моды (Mods)

### `get_mods(site_id?: number)`

Получить список модов.

**Параметры:**
- `site_id?: number` - фильтр по сайту (опционально)

**Возвращает:** `Result<Vec<Mod>, String>`

### `check_updates(site_id?: number)`

Проверить обновления модов для сайта.

**Параметры:**
- `site_id?: number` - ID сайта (опционально, если не указан - проверяет все)

**Возвращает:** `Result<Vec<ModUpdate>, String>`

**Процесс:**
1. Загружает список модов для сайта
2. Для каждого мода парсит страницу
3. Сравнивает версии
4. Создает уведомления для новых версий
5. Возвращает список обновлений

## Уведомления (Notifications)

### `get_notifications(unread_only?: boolean)`

Получить список уведомлений.

**Параметры:**
- `unread_only?: boolean` - только непрочитанные (по умолчанию false)

**Возвращает:** `Result<Vec<Notification>, String>`

### `mark_notification_read(id: number)`

Отметить уведомление как прочитанное.

**Параметры:**
- `id: number` - ID уведомления

**Возвращает:** `Result<(), String>`

## Кэш (Cache)

### `list_cached_pages()`

Получить список всех кэшированных страниц.

**Параметры:** нет

**Возвращает:** `Result<Vec<CachedPageInfo>, String>`

### `clear_page_cache()`

Очистить весь кэш страниц.

**Параметры:** нет

**Возвращает:** `Result<(), String>`

## Типы данных

### Site
```typescript
interface Site {
  id: number;
  name: string;
  url: string;
  parser_config: ParserConfig;
  created_at: string;
  updated_at: string;
}
```

### ParserConfig
```typescript
interface ParserConfig {
  root: string;  // CSS селектор корневого элемента
  children: Record<string, string | ParserNode>;
}
```

### ParserResult
```typescript
interface ParserResult {
  items: Array<Record<string, any>>;
  errors: string[];
}
```

### Mod
```typescript
interface Mod {
  id: number;
  site_id: number;
  name: string;
  version: string;
  url: string;
  created_at: string;
  updated_at: string;
}
```

### Notification
```typescript
interface Notification {
  id: number;
  mod_id: number;
  message: string;
  is_read: boolean;
  created_at: string;
}
```



































