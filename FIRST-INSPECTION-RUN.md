# 🚀 Первый запуск Code Inspection с WebStorm

## Быстрая настройка и запуск

### Шаг 1: Установка WebStorm

```powershell
# Установите WebStorm 2025.2.3 в папку:
# C:\Program Files\JetBrains\WebStorm 2025.2.3\
```

Или скачайте с официального сайта: https://www.jetbrains.com/webstorm/download/

### Шаг 2: Настройка скриптов

```powershell
# Запустите PowerShell в корне проекта и выполните:
.\scripts\setup-intellij-inspection.ps1
```

**Что делает скрипт:**
- Автоматически находит WebStorm 2025.2.3
- Настраивает переменные окружения
- Создает необходимые директории
- Проверяет профиль inspection

### Шаг 3: Первый тестовый запуск

```powershell
# Запустите в корне проекта:
.\scripts\run-code-inspection.ps1 -Verbose
```

**Ожидаемый результат:**
```
[15:30:00] INFO: === Запуск Code Inspection ===
[15:30:00] INFO: IDE: C:\Program Files\JetBrains\WebStorm 2025.2.3\bin\webstorm64.exe
[15:30:00] INFO: Проект: C:\Users\User\mod-aggregator
[15:30:00] INFO: Вывод: C:\Users\User\mod-aggregator\inspection-results\inspection_results_20250101_153000.xml
[15:30:01] INFO: Запуск анализа кода...
[15:30:15] SUCCESS: Анализ завершен успешно за 14.2с
[15:30:15] SUCCESS: Результаты сохранены в: C:\Users\User\mod-aggregator\inspection-results\inspection_results_20250101_153000.xml
[15:30:15] INFO: Ссылка на последние результаты: C:\Users\User\mod-aggregator\inspection-results\inspection_results_latest.xml
```

### Шаг 3: Просмотр результатов

Результаты сохраняются в директории `inspection-results/`:

```
inspection-results/
├── inspection_results_20250101_153000.xml    # Основной отчет
├── inspection_results_20250101_153000.json   # JSON для Cursor
├── inspection_results_latest.xml            # Ссылка на последний отчет
└── inspection_results_latest.json           # Ссылка на последний JSON
```

**Просмотр XML отчета:**
- Откройте файл в браузере или любом текстовом редакторе
- Или используйте IntelliJ IDEA для просмотра

## 🔧 Настройка профиля Inspection

Профиль уже создан в `.idea/inspectionProfiles/Project_Default.xml` с правилами для:

- **Java/Kotlin/Scala:** Javadoc, импорты, null-pointer, константы
- **JavaScript/TypeScript:** ESLint правила, неиспользуемые импорты
- **Python:** PEP8, неиспользуемые переменные
- **Rust:** borrow checker, неиспользуемые переменные

### Добавление своих правил

Откройте `Project_Default.xml` и добавьте нужные inspection:

```xml
<inspection_tool class="YourInspection" enabled="true" level="WARNING" enabled_by_default="true" />
```

## 📊 Форматы результатов

### XML (стандартный)
```xml
<problems>
  <problem>
    <file>src/Main.java</file>
    <line>15</line>
    <description>Unused import statement</description>
    <severity>WARNING</severity>
    <category>Declaration redundancy</category>
  </problem>
</problems>
```

### JSON (для автоматизации)
```json
{
  "problems": [
    {
      "file": "src/Main.java",
      "line": 15,
      "message": "Unused import statement",
      "severity": "WARNING",
      "category": "Declaration redundancy"
    }
  ],
  "summary": {
    "total": 1,
    "errors": 0,
    "warnings": 1
  }
}
```

## 🛠️ Расширенные опции

### Анализ только определенных файлов

```powershell
# Только Java файлы
.\scripts\run-code-inspection.ps1 -Include "*.java"

# Исключая тесты
.\scripts\run-code-inspection.ps1 -Exclude "**/test/**"

# Конкретные директории
.\scripts\run-code-inspection.ps1 -Include "src/main/java/**"
```

### HTML отчет для презентаций

```powershell
.\scripts\run-code-inspection.ps1 -Format html -Output ./reports
```

### Строгая проверка (CI/CD)

```powershell
# Завершится с ошибкой при нахождении проблем
.\scripts\run-code-inspection.ps1 -FailOnError
```

## 🔗 Интеграция с Git

### Pre-commit hook

```bash
# Установка hook
cp scripts/pre-commit-inspection.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

# Или на Windows
copy scripts\pre-commit-inspection.sh .git\hooks\pre-commit
```

### Настройка лимитов

Отредактируйте `.git/hooks/pre-commit`:

```bash
# Максимум ошибок перед коммитом
export MAX_ERRORS=0

# Максимум предупреждений
export MAX_WARNINGS=10

# Разрешать предупреждения
export ALLOW_WARNINGS=true
```

## 🚨 Устранение проблем

### "IntelliJ IDEA не найдена"

```powershell
# Проверить установку
.\scripts\setup-intellij-inspection.ps1

# Или указать путь вручную
.\scripts\run-code-inspection.ps1 -IdeaPath "C:\Path\To\idea64.exe"
```

### "Профиль не найден"

```powershell
# Проверить структуру проекта
dir .idea\inspectionProfiles\

# Создать профиль в IDEA:
# File -> Settings -> Editor -> Inspections -> Export Profile
```

### Пустой результат

```powershell
# Добавить verbose режим
.\scripts\run-code-inspection.ps1 -Verbose

# Проверить, что файлы индексированы в IDEA
# (откройте проект в IDEA и подождите индексации)
```

## 📈 Следующие шаги

1. **Регулярные запуски:** Добавьте в ежедневный workflow
2. **Интеграция с CI:** Настройте автоматические проверки
3. **Мониторинг трендов:** Следите за качеством кода со временем
4. **Кастомизация:** Настройте профили под ваш проект

## 🎯 Быстрые команды

```powershell
# Полный анализ проекта
.\scripts\run-code-inspection.ps1

# Анализ только текущих изменений
git diff --name-only | % { ".\scripts\run-code-inspection.ps1 -Include `"$_`"" }

# Генерация отчета для команды
.\scripts\run-code-inspection.ps1 -Format html -Output ./team-reports

# Строгая проверка для CI
.\scripts\run-code-inspection.ps1 -FailOnError -Verbose
```
