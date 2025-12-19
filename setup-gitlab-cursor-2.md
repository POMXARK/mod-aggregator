# 🔗 Настройка GitLab в Cursor 2.0

## 🎯 СПОСОБ 1: SSH Ключи (Рекомендуемый)

### Шаг 1: Сгенерируйте SSH ключ для GitLab
```bash
# В WSL (поскольку Cursor подключен к WSL)
cd ~
ssh-keygen -t ed25519 -C "your-email@erkapharm.com" -f ~/.ssh/id_ed25519_gitlab
```

### Шаг 2: Добавьте публичный ключ в GitLab
1. Скопируйте ключ:
```bash
cat ~/.ssh/id_ed25519_gitlab.pub
```

2. В GitLab:
   - User Settings → SSH Keys
   - Вставьте публичный ключ
   - Title: "Cursor 2.0 WSL"
   - Add Key

### Шаг 3: Настройте SSH конфиг
```bash
# Создайте ~/.ssh/config
cat >> ~/.ssh/config << EOF
Host gitlab.kolos.studio
    HostName gitlab.kolos.studio
    User git
    IdentityFile ~/.ssh/id_ed25519_gitlab
    IdentitiesOnly yes
EOF
```

### Шаг 4: Протестируйте подключение
```bash
ssh -T git@gitlab.kolos.studio
# Должно быть: "Welcome to GitLab, @username!"
```

## 🎯 СПОСОБ 2: Personal Access Token

### Шаг 1: Создайте токен в GitLab
1. GitLab → User Settings → Access Tokens
2. Name: "Cursor 2.0"
3. Scopes: `api`, `read_repository`, `write_repository`
4. Create token
5. **СКОПИРУЙТЕ ТОКЕН СРАЗУ!**

### Шаг 2: Настройте Git
```bash
git config --global user.name "Your Name"
git config --global user.email "your-email@erkapharm.com"

# Для HTTPS используйте токен вместо пароля
git clone https://gitlab.kolos.studio/your-project.git
# Username: your-username
# Password: YOUR_TOKEN_HERE
```

## 🚀 КЛОНИРОВАНИЕ ПРОЕКТА В CURSOR 2.0

### Через SSH (рекомендуемый):
```
Command Palette (Ctrl+Shift+P) → Git: Clone
URL: git@gitlab.kolos.studio:your-group/your-project.git
```

### Через HTTPS:
```
Command Palette → Git: Clone
URL: https://gitlab.kolos.studio/your-group/your-project.git
```

## ⚙️ НАСТРОЙКИ GIT В CURSOR 2.0

### 1. Git интеграция
```
File → Preferences → Settings → Search "git"
- Git: Autofetch: true
- Git: Confirm Sync: false
- Git: Enable Smart Commit: true
```

### 2. GitLab Integration (если есть extension)
```
Extensions → GitLab Workflow
Установите для лучшей интеграции
```

## 🔧 РАБОТА С GIT В CURSOR 2.0

### Source Control Panel
- `Ctrl+Shift+G` - открыть Git panel
- Видите изменения, commits, branches

### Основные команды:
- **Commit:** `Ctrl+Enter` в message field
- **Push:** Синяя кнопка ↑ в Git panel
- **Pull:** Синяя кнопка ↓ в Git panel
- **Create Branch:** `Ctrl+Shift+P` → "Git: Create Branch"

### Merge Requests:
- Используйте GitLab extension для создания MR
- Или работайте через веб-интерфейс GitLab

## 🔍 ДИАГНОСТИКА ПРОБЛЕМ

### Если SSH не работает:
```bash
# Проверьте SSH ключ
ssh -v git@gitlab.kolos.studio

# Проверьте конфиг
cat ~/.ssh/config
```

### Если HTTPS не работает:
```bash
# Проверьте токен
git clone https://oauth2:YOUR_TOKEN@gitlab.kolos.studio/your-project.git
```

### Если Git не видит изменения:
```bash
# В терминале Cursor
git status
git add .
git commit -m "Your message"
```

## 📋 ЧЕК-ЛИСТ НАСТРОЙКИ

- [ ] SSH ключ сгенерирован
- [ ] Публичный ключ добавлен в GitLab
- [ ] SSH конфиг настроен
- [ ] Тестовое подключение работает
- [ ] Проект клонирован в Cursor
- [ ] Git integration настроен

## 🎯 РЕКОМЕНДАЦИИ

1. **Используйте SSH** - удобнее и безопаснее
2. **Настройте Git config** правильно
3. **Используйте GitLab extension** для лучшей интеграции
4. **Регулярно обновляйте токены** (если используете HTTPS)

## 🚀 ГОТОВО!

Теперь вы можете работать с GitLab прямо из Cursor 2.0! 🎉
