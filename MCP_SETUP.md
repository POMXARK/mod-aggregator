# Настройка Tauri MCP Plugin для Cursor

## Установка завершена

Плагин `tauri-plugin-mcp` установлен и настроен для использования только в режиме разработки (debug).

## Конфигурация MCP для Cursor

Для подключения MCP сервера к Cursor нужно добавить конфигурацию в настройки Cursor.

### Путь к конфигурации

Конфигурация MCP для Cursor обычно находится в:
- Windows: `%APPDATA%\Cursor\User\globalStorage\saoudrizwan.claude-dev\settings\cline_mcp_settings.json`
- Или в настройках Cursor: Settings → Features → Model Context Protocol

### Конфигурация для Windows (IPC Mode)

Добавьте следующую конфигурацию в файл `cline_mcp_settings.json`:

```json
{
  "mcpServers": {
    "tauri-mcp": {
      "command": "node",
      "args": ["C:\\Users\\User\\mod-aggregator\\tauri-plugin-mcp\\mcp-server-ts\\build\\index.js"],
      "env": {
        "TAURI_MCP_IPC_PATH": "\\\\.\\pipe\\tauri-mcp"
      }
    }
  }
}
```

**Важно:** Замените `C:\\Users\\User\\mod-aggregator` на ваш реальный путь к проекту.

### Альтернатива: TCP Mode

Если IPC не работает, можно использовать TCP:

1. Обновите `src-tauri/src/main.rs`:
```rust
.tcp("127.0.0.1".to_string(), 4000)
```

2. Обновите конфигурацию Cursor:
```json
{
  "mcpServers": {
    "tauri-mcp": {
      "command": "node",
      "args": ["C:\\Users\\User\\mod-aggregator\\tauri-plugin-mcp\\mcp-server-ts\\build\\index.js"],
      "env": {
        "TAURI_MCP_CONNECTION_TYPE": "tcp",
        "TAURI_MCP_TCP_HOST": "127.0.0.1",
        "TAURI_MCP_TCP_PORT": "4000"
      }
    }
  }
}
```

## Использование

После настройки:

1. Запустите приложение в режиме разработки: `npm run tauri:dev`
2. MCP сервер автоматически запустится и будет доступен через Cursor
3. В Cursor можно использовать инструменты MCP для:
   - Создания скриншотов приложения
   - Управления окном
   - Доступа к DOM
   - Симуляции пользовательского ввода

## Проверка работы

После запуска приложения проверьте логи - должно появиться сообщение:
```
Development build detected, enabling MCP plugin
```

## Примечания

- Плагин активен только в debug режиме (`#[cfg(debug_assertions)]`)
- В production сборке плагин не включается
- Имя приложения в конфигурации должно совпадать с `productName` из `tauri.conf.json`: "Mod Aggregator"

## Конфигурация добавлена

Конфигурация MCP добавлена в проект-специфичный файл:
`.cursor/mcp.json`

Это правильный способ настройки MCP для проекта - конфигурация будет работать только для этого проекта.

Глобальный файл конфигурации был восстановлен к пустому состоянию.

После перезапуска Cursor плагин будет доступен для использования в этом проекте.

