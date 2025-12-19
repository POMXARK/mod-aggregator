# 🔗 Подключение Cursor 2.0 к WSL

## 🎯 Cursor 2.0: Новая архитектура подключения

Cursor 2.0 имеет улучшенную поддержку remote development и WSL интеграции.

## ✅ СПОСОБ 1: Встроенная WSL поддержка (Рекомендуемый для 2.0)

### Шаг 1: Проверьте версию
```
Cursor → Help → About Cursor
```
Убедитесь, что версия 2.x.x

### Шаг 2: Подключение к WSL
1. **Откройте Command Palette:**
   - `Cmd/Ctrl + Shift + P`
   - Найдите: `WSL: Connect to WSL`

2. **Или используйте статус бар:**
   - Внизу слева нажмите на индикатор соединения
   - Выберите `Connect to WSL`

3. **Выберите дистрибутив:**
   - Ubuntu (или ваш дистрибутив)

### Шаг 3: Откройте workspace
```
File → Open Folder → /mnt/c/Users/User/mod-aggregator
```

## ✅ СПОСОБ 2: Через Remote Development Extension Pack

### Шаг 1: Установите extension pack
1. Откройте Extensions (`Cmd/Ctrl + Shift + X`)
2. Найдите: `Remote Development`
3. Установите extension pack от Microsoft

### Шаг 2: Подключитесь
1. Command Palette (`Cmd/Ctrl + Shift + P`)
2. `Remote-WSL: Connect to WSL`
3. Выберите дистрибутив

## ✅ СПОСОБ 3: Через Dev Containers (для 2.0)

Cursor 2.0 имеет улучшенную поддержку dev containers:

### Шаг 1: Создайте .devcontainer
```json
{
    "name": "ErkaPharm Development",
    "context": "..",
    "dockerFile": "Dockerfile",
    "remoteUser": "vscode",
    "mounts": [
        "source=/mnt/c/Users/User/mod-aggregator,target=/workspaces,type=bind,consistency=cached"
    ],
    "extensions": [
        "ms-vscode.vscode-typescript-next",
        "ms-vscode.vscode-json",
        "ms-vscode-remote.remote-wsl"
    ]
}
```

### Шаг 2: Откройте в container
```
Command Palette → Dev Containers: Reopen in Container
```

## 🔧 ОСОБЕННОСТИ CURSOR 2.0

### Улучшенная производительность:
- Быстрее индексация кода в WSL
- Лучшая поддержка больших проектов
- Оптимизированная память

### AI интеграция:
- Claude работает с файлами в WSL
- Автодополнение для bash скриптов
- Анализ кода в remote окружении

### Remote UI:
- Индикатор соединения в статус баре
- Быстрое переключение между local/remote
- Уведомления о статусе соединения

## 📋 ПРОВЕРКА ПОДКЛЮЧЕНИЯ

После подключения проверьте:
- Статус бар: `WSL: Ubuntu`
- Терминал: автоматически WSL bash
- File explorer: пути WSL (`/mnt/c/...`)

## 🚀 ЗАПУСК ПРОЕКТА В CURSOR 2.0

```bash
# В терминале Cursor (автоматически WSL)
cd /mnt/c/Users/User/mod-aggregator
./auto-connect.sh

# Затем в PowerShell:
.\setup-selective-admin.bat
```

## 🔍 УСТРАНЕНИЕ ПРОБЛЕМ

### Если WSL не подключается:
```bash
# В Windows PowerShell
wsl -l -v
# Должен быть Running
```

### Если extensions не работают:
```
Cursor → Settings → Extensions → Remote
Убедитесь что remote extensions включены
```

### Performance issues:
```
Cursor → Settings → Remote → WSL
Настройте memory и CPU limits
```

## 🎯 РЕКОМЕНДАЦИИ ДЛЯ CURSOR 2.0

1. **Используйте встроенную WSL поддержку** - самая быстрая
2. **Обновите extensions** до последних версий
3. **Используйте dev containers** для изоляции проектов
4. **Мониторьте память** - WSL может быть ресурсоемким

## 📚 ДОПОЛНИТЕЛЬНЫЕ РЕСУРСЫ

- [Cursor 2.0 Documentation](https://cursor.sh/docs)
- [WSL в VS Code](https://code.visualstudio.com/docs/remote/wsl)
- [Dev Containers](https://containers.dev/)

## ✅ ГОТОВО!

Cursor 2.0 с WSL - это мощная комбинация для разработки! 🚀
