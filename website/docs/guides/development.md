---
sidebar_position: 1
---

# Руководство по разработке

Руководство по настройке окружения разработки и работе с проектом.

## Требования

- **Node.js** >= 18.0
- **Rust** >= 1.70
- **npm** или **yarn**

## Установка

```bash
# Клонировать репозиторий
git clone https://github.com/your-org/mod-aggregator.git
cd mod-aggregator

# Установить зависимости
npm install

# Установить Tauri CLI (если еще не установлен)
npm install -g @tauri-apps/cli
```

## Запуск в режиме разработки

```bash
# Запустить frontend и backend вместе
npm run tauri:dev

# Только frontend (браузерный режим)
npm run dev

# С verbose логами
npm run tauri:dev:verbose
```

## Структура проекта

```
mod-aggregator/
├── src/                    # Frontend код (Svelte 5)
│   ├── components/         # Svelte компоненты
│   ├── lib/                # Утилиты и composables
│   └── App.svelte          # Главный компонент
├── src-tauri/              # Backend код (Rust)
│   └── src/
│       ├── main.rs         # Tauri команды
│       ├── database.rs     # Работа с БД
│       └── parser.rs       # Движок парсинга
├── website/                # Docusaurus документация
└── docs/                   # Исходная документация
```

## Разработка Frontend

### Компоненты Svelte 5

Используйте Svelte 5 runes для реактивности:

```svelte
<script lang="ts">
  let count = $state(0);
  let doubled = $derived(count * 2);
  
  $effect(() => {
    console.log('Count:', count);
  });
</script>
```

### Стилизация

- Используйте Tailwind CSS для утилитарных классов
- Изолируйте стили в `<style>` блоке для уникальных стилей
- Следуйте темной теме проекта

## Разработка Backend

### Tauri команды

Все команды должны быть асинхронными и возвращать `Result<T, String>`:

```rust
#[tauri::command]
pub async fn get_sites() -> Result<Vec<Site>, String> {
    // ...
}
```

### Работа с базой данных

Используйте SQLx для работы с SQLite:

```rust
let sites = sqlx::query_as::<_, Site>("SELECT * FROM sites")
    .fetch_all(&pool)
    .await?;
```

## Тестирование

```bash
# Unit тесты
npm test

# E2E тесты
npm run cypress:run

# Rust тесты
cd src-tauri && cargo test
```

## Сборка

```bash
# Сборка production версии
npm run tauri:build
```

## Полезные команды

```bash
# Проверка типов
npm run type-check

# Линтинг
npm run lint

# Форматирование
npm run format
```
























