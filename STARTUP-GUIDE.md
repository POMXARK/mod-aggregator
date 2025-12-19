# 🚀 ПОРЯДОК ЗАПУСКА ERKAPHARM VPN С НУЛЯ

## 📋 ПОЛНЫЙ СПИСОК ШАГОВ

### ШАГ 1: ПОДГОТОВКА (ОДИН РАЗ ПРИ ПЕРВОЙ НАСТРОЙКЕ)
```bash
# Перейти в директорию проекта
cd C:\Users\User\mod-aggregator

# Сделать скрипты исполняемыми (в WSL)
wsl -d Ubuntu
cd /mnt/c/Users/User/mod-aggregator
chmod +x *.sh
```

### ШАГ 2: ЗАПУСК ПОСЛЕ ВКЛЮЧЕНИЯ ПК

#### 2.1 Запуск WSL
```bash
# Запустить Ubuntu в WSL
wsl -d Ubuntu
cd /mnt/c/Users/User/mod-aggregator
```

#### 2.2 Подключение VPN
```bash
# Автоматическое подключение
./auto-connect.sh
```
*Должен появиться TUN интерфейс и сообщение о подключении*

#### 2.3 Настройка маршрутизации (Windows)
```powershell
# Открыть PowerShell ОТ ИМЕНИ АДМИНИСТРАТОРА
# Выполнить:
cd C:\Users\User\mod-aggregator
.\setup-selective-admin.bat
```

#### 2.4 Настройка SSH ключа (один раз)
```bash
# Настроить SSH ключ для автоматического входа
./setup-ssh-key.sh
```

#### 2.5 Запуск SSH туннеля
```bash
# Автоматический запуск с ключом
./start-ssh-tunnel.sh

# Или вручную с ключом:
ssh -i ~/.ssh/id_rsa_erkapharm -f -N \
  -L 8194:basket-01.dev.erkapharm.ru:8194 \
  -L 9996:fndr-01.prod.shop.local:3030 \
  -L 9998:ws.erkapharm.com:8990 \
  -L 11006:mbsdevcrm15sp1.manzanagroup.ru:11006 \
  -L 9999:localhost:27017 \
  -p 22 xcom@xcom-01.dev.erkapharm.ru
```

#### 2.5 Проверка работы
```cmd
# В Windows командной строке
cd C:\Users\User\mod-aggregator
.\test-full-system.bat
```

---

## 🛑 ПОРЯДОК ОСТАНОВКИ

### ШАГ 1: Остановка SSH туннеля
```bash
# В WSL найти и остановить SSH процесс
ps aux | grep ssh
kill <SSH_PID>
```

### ШАГ 2: Отключение VPN
```bash
# В WSL
sudo pkill openconnect
```

### ШАГ 3: Очистка маршрутизации (опционально)
```powershell
# От имени администратора
route delete 10.0.0.0 mask 255.0.0.0
route delete 172.16.0.0 mask 255.240.0.0
route delete 172.21.0.0 mask 255.255.0.0
route delete 192.168.0.0 mask 255.255.0.0
```

### ШАГ 4: Выход из WSL
```bash
exit
```

---

## ⚡ БЫСТРЫЙ ЗАПУСК (КОГДА ВСЁ НАСТРОЕНО)

### Вариант 1: Полностью автоматический
```bash
# WSL
wsl -d Ubuntu
cd /mnt/c/Users/User/mod-aggregator
./auto-connect.sh

# Windows (админ)
cd C:\Users\User\mod-aggregator
.\setup-selective-admin.bat

# SSH туннель
ssh -f -N -L 8194:basket-01.dev.erkapharm.ru:8194 -L 9996:fndr-01.prod.shop.local:3030 -L 9998:ws.erkapharm.com:8990 -L 11006:mbsdevcrm15sp1.manzanagroup.ru:11006 -L 9999:localhost:27017 -p 22 xcom@xcom-01.dev.erkapharm.ru
```

### Вариант 2: Проверка перед запуском
```cmd
cd C:\Users\User\mod-aggregator
.\test-full-system.bat
```

---

## 🔍 ДИАГНОСТИКА ПРОБЛЕМ

### Если VPN не подключается:
```bash
# WSL
./test-vpn-connectivity.sh
./connect-insecure.sh
```

### Если маршрутизация не работает:
```powershell
# Админ PowerShell
.\setup-selective-admin.bat
```

### Если SSH туннель не работает:
```bash
# Проверить VPN и маршруты, затем
ssh -v -f -N -L 8194:basket-01.dev.erkapharm.ru:8194 -p 22 xcom@xcom-01.dev.erkapharm.ru
```

---

## 📱 ПЕРЕМЕННЫЕ ОКРУЖЕНИЯ ДЛЯ РАЗРАБОТКИ

После запуска добавить в проект:

```bash
MANZANA_CLIENT_DEV_URL=https://mnz-cosn.erkapharm.com/customerofficeservice/
MANZANA_CLIENT_DEV_URL__=http://localhost:11006/CustomerOfficeService/
MANZANA_CLIENT_DEV_URL___=https://mnz-cosn.erkapharm.com/customerofficeservice/
MONGO_URI=mongodb://localhost:9999
```

---

## ✅ КОНТРОЛЬНЫЙ СПИСОК ЗАПУСКА

- [ ] WSL запущен (`wsl -d Ubuntu`)
- [ ] VPN подключен (`./auto-connect.sh`)
- [ ] Маршруты настроены (`.\setup-selective-admin.bat`)
- [ ] SSH туннель запущен (ssh команда)
- [ ] Сервисы доступны (localhost:8194, :9999 и т.д.)
- [ ] Переменные окружения установлены

---

## 🎯 РЕЗЮМЕ

**Запуск:** WSL → VPN → Маршруты → SSH туннель → Работа  
**Остановка:** SSH kill → VPN kill → Очистка маршрутов (опционально)
