# 🔗 Подключение Cursor к WSL

## 🎯 СПОСОБ 1: WSL Extension (Рекомендуемый)

### Шаг 1: Установите WSL Extension
1. Откройте Cursor
2. Перейдите: `View → Extensions` (Ctrl+Shift+X)
3. Найдите: `WSL`
4. Установите: `WSL` от Microsoft

### Шаг 2: Подключитесь к WSL
1. В левом нижнем углу Cursor нажмите зеленую кнопку `><` (Remote Status)
2. Выберите: `Connect to WSL`
3. Выберите ваш дистрибутив: `Ubuntu`

### Шаг 3: Откройте папку
1. `File → Open Folder`
2. Перейдите: `/mnt/c/Users/User/mod-aggregator`
3. Или любой другой путь в WSL

## 🎯 СПОСОБ 2: Через UNC путь

### Откройте папку напрямую:
```
\\wsl$\Ubuntu\mnt\c\Users\User\mod-aggregator
```

### В Cursor:
1. `File → Open Folder`
2. Введите UNC путь выше

## 🎯 СПОСОБ 3: Remote SSH Extension

### Шаг 1: Установите Remote SSH
1. Extensions → `Remote SSH`
2. Установите extension

### Шаг 2: Добавьте SSH конфигурацию
Создайте `~/.ssh/config` в Windows:
```
Host WSL-Ubuntu
    HostName localhost
    User roman
    Port 2222
    StrictHostKeyChecking no
    UserKnownHostsFile /dev/null
```

### Шаг 3: Запустите SSH сервер в WSL
```bash
sudo apt install openssh-server
sudo service ssh start
```

### Шаг 4: Подключитесь
В Cursor: `Remote SSH: Connect to Host` → `WSL-Ubuntu`

## ✅ ПРОВЕРКА ПОДКЛЮЧЕНИЯ

После подключения:
- В левом нижнем углу должна быть: `WSL: Ubuntu`
- Терминал должен открываться в WSL
- Можно запускать скрипты: `./auto-connect.sh`

## 🔧 УСТРАНЕНИЕ ПРОБЛЕМ

### Если WSL extension не работает:
```bash
# В WSL проверьте
wsl -l -v
# Должен быть STATUS: Running
```

### Если UNC путь не открывается:
```bash
# Проверьте в PowerShell
Get-ChildItem \\wsl$\Ubuntu\mnt\c\Users\User\mod-aggregator
```

### Если терминал не работает в Cursor:
- Проверьте настройки: `Terminal → Integrated: Shell: Linux`
- Или установите: `Terminal → Select Default Profile → WSL`

## 📁 РАБОТА С ФАЙЛАМИ

### Синхронизация:
- Файлы автоматически синхронизируются между Windows и WSL
- Изменения в Cursor сохраняются в WSL

### Запуск скриптов:
```bash
# В терминале Cursor (должен быть WSL)
cd /mnt/c/Users/User/mod-aggregator
./auto-connect.sh
```

## 🎯 РЕКОМЕНДАЦИЯ

**Используйте СПОСОБ 1 (WSL Extension)!** 🚀

Это самый простой и надежный способ. После установки extension просто нажмите зеленую кнопку в левом нижнем углу и выберите WSL.
