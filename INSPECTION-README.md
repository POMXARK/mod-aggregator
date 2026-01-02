# Автоматизация Code Inspection с WebStorm для фронтенда

Этот проект предоставляет полную автоматизацию запуска code inspection в WebStorm для фронтенд проектов с сохранением результатов в различные форматы для дальнейшего анализа.

## 🚀 Быстрый старт

### 1. Установка

```bash
# Сделать скрипты исполняемыми
chmod +x scripts/run-code-inspection.sh
chmod +x scripts/xml-to-json-converter.py
chmod +x scripts/pre-commit-inspection.sh

# Создать директорию для результатов
mkdir -p inspection-results
```

### 2. Базовое использование

```bash
# Запуск inspection для всего проекта
./scripts/run-code-inspection.sh

# С указанием пути к IDEA
./scripts/run-code-inspection.sh --idea-path "/Applications/IntelliJ IDEA CE.app/Contents/MacOS/idea"

# Только для Java файлов
./scripts/run-code-inspection.sh --include "*.java"

# Экспорт в HTML для отчетов
./scripts/run-code-inspection.sh --format html --output ./reports
```

### 3. Настройка Git hooks

```bash
# Установка pre-commit hook
cp scripts/pre-commit-inspection.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

# Настройка переменных окружения (опционально)
export MAX_ERRORS=0      # Максимум ошибок перед коммитом
export MAX_WARNINGS=50   # Максимум предупреждений
export ALLOW_WARNINGS=true
```

## 📋 Конфигурация

### inspection-config.json

Основной файл конфигурации содержит все настройки:

```json
{
  "jetbrains": {
    "ideaPath": {
      "linux": "/opt/idea/bin/idea.sh",
      "macos": "/Applications/IntelliJ IDEA CE.app/Contents/MacOS/idea",
      "windows": "C:\\Program Files\\JetBrains\\IntelliJ IDEA\\bin\\idea64.exe"
    }
  },
  "inspection": {
    "defaultProfile": "Project_Default",
    "outputFormat": "xml",
    "includePatterns": ["**/*.java", "**/*.kt"],
    "excludePatterns": ["**/node_modules/**"]
  },
  "hooks": {
    "preCommit": {
      "enabled": true,
      "maxErrors": 0,
      "maxWarnings": 50
    }
  }
}
```

### Профили Inspection

Создайте профили в `.idea/inspectionProfiles/`:

```xml
<!-- .idea/inspectionProfiles/Project_Default.xml -->
<component name="InspectionProjectProfileManager">
  <profile version="1.0">
    <option name="myName" value="Project Default" />
    <inspection_tool class="JavaDocMethod" enabled="true" level="WARNING" />
    <inspection_tool class="UnusedImport" enabled="true" level="WARNING" />
  </profile>
</component>
```

## 🛠️ Скрипты

### run-code-inspection.sh

Основной скрипт для запуска inspection:

```bash
# Все опции
./scripts/run-code-inspection.sh \
  --project /path/to/project \
  --output ./inspection-results \
  --format xml \
  --profile Project_Default \
  --include "*.java" \
  --exclude "test/**" \
  --idea-path "/path/to/idea" \
  --fail-on-error \
  --verbose
```

### xml-to-json-converter.py

Конвертация результатов из XML в JSON для Cursor:

```bash
# Базовое использование
python3 scripts/xml-to-json-converter.py inspection-results/results.xml inspection-results/results.json

# С фильтрацией только ошибок
python3 scripts/xml-to-json-converter.py results.xml results.json --filter-severity ERROR

# Форматированный вывод
python3 scripts/xml-to-json-converter.py results.xml results.json --pretty
```

### pre-commit-inspection.sh

Hook для автоматической проверки перед коммитом.

## 📊 Форматы вывода

### XML (стандартный)

```xml
<problems>
  <problem>
    <file>src/Main.java</file>
    <line>15</line>
    <description>Unused import statement</description>
    <severity>WARNING</severity>
    <category>Declaration redundancy</category>
    <inspection>UnusedImport</inspection>
  </problem>
</problems>
```

### JSON (для интеграции)

```json
{
  "timestamp": "2025-01-01T12:00:00Z",
  "project": "my-project",
  "problems": [
    {
      "file": "src/Main.java",
      "line": 15,
      "message": "Unused import statement",
      "severity": "WARNING",
      "category": "Declaration redundancy",
      "inspection": "UnusedImport"
    }
  ],
  "summary": {
    "total": 1,
    "errors": 0,
    "warnings": 1,
    "info": 0
  }
}
```

### HTML (для отчетов)

Генерирует красивый HTML отчет с группировкой по файлам и severity.

## 🔧 Интеграция с CI/CD

### GitHub Actions

```yaml
# .github/workflows/code-inspection.yml
name: Code Inspection

on: [push, pull_request]

jobs:
  inspect:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3

    - name: Setup IntelliJ IDEA
      run: |
        wget -q https://download.jetbrains.com/idea/ideaIC-2023.3.4.tar.gz
        tar -xzf ideaIC-2023.3.4.tar.gz
        echo "IDEA_PATH=$PWD/idea-IC-233.14475.28/bin/idea.sh" >> $GITHUB_ENV

    - name: Create inspection profile
      run: |
        mkdir -p .idea/inspectionProfiles
        cp inspection-config.xml .idea/inspectionProfiles/Project_Default.xml

    - name: Run code inspection
      run: ./scripts/run-code-inspection.sh --fail-on-error

    - name: Upload results
      uses: actions/upload-artifact@v3
      with:
        name: inspection-results
        path: inspection-results/
```

### Jenkins Pipeline

```groovy
pipeline {
    agent any

    stages {
        stage('Code Inspection') {
            steps {
                script {
                    // Download IDEA if needed
                    sh './scripts/setup-idea.sh'

                    // Run inspection
                    sh './scripts/run-code-inspection.sh --fail-on-error'

                    // Archive results
                    archiveArtifacts artifacts: 'inspection-results/*.xml', fingerprint: true
                }
            }

            post {
                always {
                    publishHTML([
                        allowMissing: true,
                        alwaysLinkToLastBuild: true,
                        keepAll: true,
                        reportDir: 'inspection-results',
                        reportFiles: 'results.html',
                        reportName: 'Code Inspection Report'
                    ])
                }
            }
        }
    }
}
```

## 🎯 Использование в различных IDE

### IntelliJ IDEA

```bash
# Полный путь к executable
IDEA_PATH="/Applications/IntelliJ IDEA CE.app/Contents/MacOS/idea"
./scripts/run-code-inspection.sh --idea-path "$IDEA_PATH"
```

### WebStorm

```bash
WEBSTORM_PATH="/Applications/WebStorm.app/Contents/MacOS/webstorm"
./scripts/run-code-inspection.sh --idea-path "$WEBSTORM_PATH"
```

### CLion (C/C++)

```bash
CLION_PATH="/Applications/CLion.app/Contents/MacOS/clion"
./scripts/run-code-inspection.sh --idea-path "$CLION_PATH"
```

### PyCharm

```bash
PYCHARM_PATH="/Applications/PyCharm CE.app/Contents/MacOS/pycharm"
./scripts/run-code-inspection.sh --idea-path "$PYCHARM_PATH"
```

## 📈 Мониторинг и статистика

### Анализ трендов

```bash
# Скрипт для анализа трендов
./scripts/analyze-trends.sh --input inspection-results/ --output trends.json

# Генерация графиков
./scripts/generate-charts.py trends.json charts/
```

### Отчеты

```bash
# Генерация summary отчета
./scripts/generate-report.sh --input inspection-results/results.xml --output report.md

# Отправка уведомлений
./scripts/send-notifications.sh --config notification-config.json --results results.json
```

## 🐛 Устранение неполадок

### "Command not found" для idea.sh

```bash
# Проверить путь
find /Applications -name "idea" -type f

# Или установить переменную
export IDEA_PATH="/correct/path/to/idea"
```

### Inspection не находит профиль

```bash
# Проверить структуру проекта
ls -la .idea/inspectionProfiles/

# Создать профиль вручную
mkdir -p .idea/inspectionProfiles
# Создать Project_Default.xml
```

### Пустой результат

```bash
# Добавить verbose режим
./scripts/run-code-inspection.sh --verbose

# Проверить, что файлы индексированы в IDEA
# Открыть проект в IDEA и подождать индексации
```

### Ошибки прав доступа

```bash
# На Linux/Mac
chmod +x "$IDEA_PATH"

# Создать директорию для результатов
mkdir -p inspection-results
chmod 755 inspection-results
```

## 🔒 Безопасность

- Скрипты проверяют существование файлов перед выполнением
- Переменные окружения имеют безопасные значения по умолчанию
- Нет выполнения произвольного кода из конфигурационных файлов
- Результаты сохраняются только в указанную директорию

## 📚 API для интеграции

### Программный запуск

```typescript
import { spawn } from 'child_process';

function runInspection(options: InspectionOptions): Promise<InspectionResult> {
  return new Promise((resolve, reject) => {
    const args = [
      '--project', options.projectPath,
      '--output', options.outputDir,
      '--format', options.format
    ];

    const child = spawn('./scripts/run-code-inspection.sh', args, {
      stdio: 'inherit'
    });

    child.on('close', (code) => {
      if (code === 0) {
        resolve(parseResults(options.outputDir));
      } else {
        reject(new Error(`Inspection failed with code ${code}`));
      }
    });
  });
}
```

### Парсинг результатов

```typescript
import { parseXmlResults, parseJsonResults } from './parsers';

async function analyzeResults(resultPath: string): Promise<Analysis> {
  const results = await parseXmlResults(resultPath);

  return {
    totalProblems: results.summary.total,
    errorsByFile: groupByFile(results.problems, 'ERROR'),
    warningsByCategory: groupByCategory(results.problems, 'WARNING')
  };
}
```

## 🤝 Contributing

1. Fork репозиторий
2. Создайте feature branch (`git checkout -b feature/amazing-feature`)
3. Commit изменения (`git commit -m 'Add amazing feature'`)
4. Push branch (`git push origin feature/amazing-feature`)
5. Создайте Pull Request

### Требования к контрибуции

- Скрипты должны работать на Linux, macOS и Windows
- Код должен быть документирован
- Добавляйте тесты для новых функций
- Следуйте стилю существующего кода

## 📄 Лицензия

Этот проект лицензирован под MIT License - см. файл [LICENSE](LICENSE) для деталей.

## 🆘 Поддержка

- 📧 **Email**: support@example.com
- 💬 **Issues**: [GitHub Issues](https://github.com/your-repo/issues)
- 📖 **Wiki**: [Project Wiki](https://github.com/your-repo/wiki)

---

## Быстрые команды

```bash
# Полная проверка проекта
./scripts/run-code-inspection.sh --verbose --fail-on-error

# Проверка только измененных файлов
git diff --name-only | xargs ./scripts/run-code-inspection.sh --include

# Генерация отчета
./scripts/run-code-inspection.sh --format html && open inspection-results/results.html

# Очистка старых результатов
find inspection-results -name "*.xml" -mtime +7 -delete
```
