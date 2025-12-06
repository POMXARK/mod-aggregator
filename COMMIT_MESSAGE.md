# Commit Message

```
fix(svelte5): исправить реактивность пропсов и монтирование компонентов для Svelte 5 runes mode

fix(svelte5): fix props reactivity and component mounting for Svelte 5 runes mode

Исправления:
- Sidebar.svelte: убрал $derived, использую пропсы напрямую (в Svelte 5 пропсы реактивны по умолчанию)
- main.ts: заменен new App() на mount() из Svelte 5 для корректного монтирования компонентов
- Упрощена логика обработки пропсов в компонентах согласно новому API Svelte 5

Fixes:
- Sidebar.svelte: removed $derived, using props directly (props are reactive by default in Svelte 5)
- main.ts: replaced new App() with mount() from Svelte 5 for proper component mounting
- Simplified props handling logic in components according to new Svelte 5 API

Функционал:
- Навигационные кнопки (Моды, Сайты, Конструктор, Уведомления) корректно обновляют активное состояние
- Переключение между страницами работает через реактивные пропсы
- Обработчики событий передаются как обычные пропсы (Svelte 5 синтаксис)

Features:
- Navigation buttons (Mods, Sites, Builder, Notifications) correctly update active state
- Page switching works through reactive props
- Event handlers are passed as regular props (Svelte 5 syntax)

Измененные файлы:
- src/components/Sidebar.svelte
- src/main.ts

Changed files:
- src/components/Sidebar.svelte
- src/main.ts
```

