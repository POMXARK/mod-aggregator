---
name: state-persistence-svelte5-rust
description: Реализация системы сохранения и восстановления состояния в приложениях с Svelte 5 frontend и Rust backend. Использовать при работе с состоянием UI, настройками пользователя или данными приложения, которые нужно сохранять между сессиями.
version: 1.0.0
---

# State Persistence: Svelte 5 + Rust Implementation

## Критические правила Svelte 5

### Никогда не смешивай старый и новый API состояния
```typescript
// ❌ НЕПРАВИЛЬНО - смешивание API
const store = writable(''); // Старый API
let state = $state('');    // Новый API

// ✅ ПРАВИЛЬНО - только новый API
let uiState = $state<UIState>({ /* ... */ });
const appState = writable<AppState>({ /* ... */ }); // Только для межкомпонентного состояния
```

### Используй $derived вместо reactive statements
```typescript
// ❌ Неправильно в Svelte 5
$: combined = a + b;

// ✅ Правильно
let combined = $derived(a + b);
```

### $effect только на верхнем уровне компонента
```typescript
// ❌ Неправильно
async function loadData() {
  $effect(() => { // effect_orphan error
    console.log('Data loaded');
  });
}

// ✅ Правильно
$effect(() => {
  if (dataLoaded) {
    console.log('Data loaded');
  }
});
```

## Архитектура сохранения состояния

### Основные компоненты
1. **UI State Management** - управление состоянием интерфейса
2. **Parser State Management** - управление состоянием парсера
3. **Session State Manager** - координация сохранения
4. **Database Layer** - работа с SQLite через Rust

### Debounced сохранение для производительности
```typescript
let saveTimeout: ReturnType<typeof setTimeout> | null = null;

function debouncedSave(state: AppState) {
  if (saveTimeout) clearTimeout(saveTimeout);
  saveTimeout = setTimeout(async () => {
    await SessionStateManager.saveUIState(state);
  }, 500); // 500ms задержка
}
```

## Tauri Commands для сохранения

### Сохранение UI состояния
```rust
#[tauri::command]
pub async fn save_ui_state(ui_state: UIState) -> Result<(), String> {
    let db = Database::new().await?;
    let ui_prefs = UiPreferences {
        view_mode: ui_state.viewMode,
        window_width: ui_state.windowWidth,
        window_height: ui_state.windowHeight,
        sidebar_collapsed: ui_state.sidebarCollapsed,
        theme: ui_state.theme,
        current_page: ui_state.currentPage,
        selected_site_id: ui_state.selectedSiteId,
    };

    db.update_session_ui_preferences(&ui_prefs).await?;
    Ok(())
}
```

### Восстановление состояния при запуске
```typescript
onMount(async () => {
  try {
    const result = await SessionStateManager.restoreSession();
    if (result.uiState) {
      applyUIState(result.uiState);
    }
    if (result.parserState) {
      setNodes(result.parserState.nodes);
      setEdges(result.parserState.edges);
      setGeneratedCode(result.parserState.generatedCode);
      // Синхронизация с legacy stores
      generatedCodeStore.set(result.parserState.generatedCode);
    }
  } catch (error) {
    console.error('Failed to restore session:', error);
  }
});
```

## Структура базы данных

### Session State таблица
```sql
CREATE TABLE session_state (
  id INTEGER PRIMARY KEY CHECK(id = 1),
  file_order TEXT NOT NULL DEFAULT '[]',
  ui_preferences TEXT NOT NULL DEFAULT '{}',
  open_collections TEXT DEFAULT '[]',
  selected_files TEXT DEFAULT '[]',
  recent_actions TEXT DEFAULT '[]',
  current_page TEXT,
  selected_site_id INTEGER,
  last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### Методы работы с БД
```rust
impl Database {
    pub async fn get_session_state(&self) -> Result<SessionState, sqlx::Error> {
        let row = sqlx::query(/* SQL query */).fetch_optional(&self.pool).await?;
        // Парсинг и преобразование данных
    }

    pub async fn update_session_ui_preferences(&self, prefs: &UiPreferences) -> Result<(), sqlx::Error> {
        let json_str = serde_json::to_string(prefs)?;
        sqlx::query(/* update query */).bind(&json_str).execute(&self.pool).await?;
        Ok(())
    }
}
```

## Синхронизация состояний

### Проблема: Несинхронизированные состояния
```typescript
// Unified state manager
setCurrentUrl(url); // Обновляет новое состояние

// Legacy store (старый код)
currentUrlStore.set(url); // Нужно обновлять отдельно!
```

### Решение: Двусторонняя синхронизация
```typescript
// При обновлении unified state
setCurrentUrl(restoredState.stateUrl);
// Синхронизация с legacy store
currentUrlStore.set(restoredState.stateUrl);
```

## Обработка ошибок

### Graceful degradation при сбоях сохранения
```typescript
export async function safeSaveState(state: AppState): Promise<void> {
  try {
    await invoke('save_app_state', { state });
  } catch (error) {
    // Fallback to localStorage
    localStorage.setItem('app_state_backup', JSON.stringify(state));
    throw new Error(`Failed to save state: ${error}`);
  }
}
```

## Миграции базы данных

### Безопасные миграции с проверками
```rust
pub async fn safe_migrate(&self) -> Result<(), sqlx::Error> {
    // Проверяем текущую схему
    let schema = self.get_current_schema().await?;

    if !schema.has_column("current_page") {
        sqlx::query("ALTER TABLE session_state ADD COLUMN current_page TEXT")
            .execute(&self.pool)
            .await?;
    }
    // ... другие миграции

    Ok(())
}
```

## Лучшие практики

1. **Всегда используй Svelte 5 runes API** - не смешивай с legacy stores
2. **$effect только на верхнем уровне** компонента
3. **$bindable требует правильной типизации** и начальных значений
4. **Синхронизируй состояния** между unified и legacy stores
5. **Debounced сохранение** для производительности
6. **Безопасные миграции** с проверкой существования колонок
7. **Комплексная обработка ошибок** с fallback механизмами
8. **Валидация данных** перед сохранением
9. **Отладка и мониторинг** в development режиме

## Отладка

### State Inspector для development
```typescript
if (import.meta.env.DEV) {
  // @ts-ignore
  window.__APP_STATE__ = {
    uiState: $uiState,
    parserState: $parserState,
    sessionState: $sessionState
  };
}
```