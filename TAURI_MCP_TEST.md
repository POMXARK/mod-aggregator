# Тестирование Tauri MCP Plugin

## Статус настройки

✅ **Завершено:**
- MCP плагин добавлен в `Cargo.toml`
- Плагин настроен в `main.rs` для debug режима
- MCP сервер собран (`tauri-plugin-mcp/mcp-server-ts/build/index.js`)
- Конфигурация MCP создана (`.cursor/mcp.json`)

## Конфигурация

### Путь к сокету
- **В Rust (main.rs):** `\\.\pipe\tmp\tauri-mcp.sock`
- **В TypeScript (client.ts):** `\\\\.\\pipe\\tmp\\tauri-mcp.sock`
- **В MCP конфигурации:** `\\\\.\\pipe\\tmp\\tauri-mcp.sock`

### Конфигурация MCP (`.cursor/mcp.json`)
```json
{
  "mcpServers": {
    "tauri-mcp": {
      "command": "node",
      "args": [
        "C:\\Users\\User\\mod-aggregator\\tauri-plugin-mcp\\mcp-server-ts\\build\\index.js"
      ],
      "env": {
        "TAURI_MCP_IPC_PATH": "\\\\.\\pipe\\tmp\\tauri-mcp.sock"
      }
    }
  }
}
```

## Тестирование

### Вариант 1: Запуск Tauri приложения

1. **Запустите приложение в режиме разработки:**
   ```bash
   npm run tauri:dev
   ```

2. **Проверьте логи** - должно появиться сообщение:
   ```
   Development build detected, enabling MCP plugin
   [TAURI_MCP] Socket server started
   ```

3. **Проверьте, что сокет создан:**
   - На Windows named pipe создается автоматически при запуске сервера
   - Проверить можно через Process Explorer или через попытку подключения

### Вариант 2: Тестирование через MCP Inspector

1. **Убедитесь, что Tauri приложение запущено** (важно!)

2. **Запустите MCP Inspector:**
   ```bash
   cd tauri-plugin-mcp/mcp-server-ts
   npx @modelcontextprotocol/inspector node build/index.js
   ```

3. **Проверьте доступные инструменты:**
   - `take_screenshot` - создание скриншотов окна
   - `manage_window` - управление окном
   - `get_dom` - получение DOM
   - `execute_js` - выполнение JavaScript
   - `manage_local_storage` - управление localStorage
   - `simulate_mouse_movement` - симуляция мыши
   - `send_text_to_element` - отправка текста в элемент
   - `get_element_position` - получение позиции элемента

### Вариант 3: Тестирование через Cursor

1. **Перезапустите Cursor** после создания `.cursor/mcp.json`

2. **Проверьте доступность MCP сервера:**
   - Откройте панель MCP в Cursor
   - Должен появиться сервер `tauri-mcp`

3. **Запустите Tauri приложение:**
   ```bash
   npm run tauri:dev
   ```

4. **Используйте инструменты MCP:**
   - Попросите AI сделать скриншот приложения
   - Попросите AI получить DOM страницы
   - Попросите AI выполнить JavaScript код

## Возможные проблемы

### Проблема 1: "Connection refused"
**Решение:**
- Убедитесь, что Tauri приложение запущено
- Проверьте, что используется правильный путь к сокету
- Попробуйте использовать TCP режим вместо IPC

### Проблема 2: "Socket file not found"
**Решение:**
- На Windows named pipe создается автоматически
- Убедитесь, что приложение запущено в debug режиме
- Проверьте логи приложения на наличие ошибок

### Проблема 3: MCP сервер не виден в Cursor
**Решение:**
- Перезапустите Cursor
- Проверьте путь к `index.js` в конфигурации
- Убедитесь, что файл существует и доступен

### Проблема 4: Неправильный путь к сокету
**Решение:**
- В JSON конфигурации путь должен быть: `"\\\\.\\pipe\\tmp\\tauri-mcp.sock"`
- В Rust коде путь: `r"\\.\pipe\tmp\tauri-mcp.sock"`
- В TypeScript коде путь: `\\\\.\\pipe\\tmp\\tauri-mcp.sock`

## Альтернатива: TCP режим

Если IPC не работает, можно использовать TCP:

1. **Обновите `src-tauri/src/main.rs`:**
   ```rust
   .tcp("127.0.0.1".to_string(), 4000)
   ```

2. **Обновите `.cursor/mcp.json`:**
   ```json
   {
     "mcpServers": {
       "tauri-mcp": {
         "command": "node",
         "args": [
           "C:\\Users\\User\\mod-aggregator\\tauri-plugin-mcp\\mcp-server-ts\\build\\index.js"
         ],
         "env": {
           "TAURI_MCP_CONNECTION_TYPE": "tcp",
           "TAURI_MCP_TCP_HOST": "127.0.0.1",
           "TAURI_MCP_TCP_PORT": "4000"
         }
       }
     }
   }
   ```

## Следующие шаги

После успешного тестирования:
1. ✅ Убедитесь, что все инструменты работают
2. ✅ Протестируйте создание скриншотов
3. ✅ Протестируйте управление окном
4. ✅ Протестируйте доступ к DOM
5. ✅ Протестируйте выполнение JavaScript






















