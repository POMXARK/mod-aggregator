#!/bin/bash

# Pre-commit hook для автоматической проверки кода с помощью IntelliJ IDEA inspection
# Помещает этот файл в .git/hooks/pre-commit и делает исполняемым

set -e

# Цвета для вывода
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Функции
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Конфигурация
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(git rev-parse --show-toplevel)"
INSPECTION_SCRIPT="$SCRIPT_DIR/run-code-inspection.sh"
RESULTS_DIR="$PROJECT_ROOT/inspection-results"
MAX_ERRORS="${MAX_ERRORS:-0}"        # Максимальное количество ошибок
MAX_WARNINGS="${MAX_WARNINGS:-50}"    # Максимальное количество предупреждений
ALLOW_WARNINGS="${ALLOW_WARNINGS:-true}"  # Разрешать предупреждения

log_info "=== Pre-commit Code Inspection ==="
log_info "Проект: $PROJECT_ROOT"

# Проверка существования скрипта inspection
if [ ! -f "$INSPECTION_SCRIPT" ]; then
    log_error "Скрипт inspection не найден: $INSPECTION_SCRIPT"
    log_error "Установите скрипт run-code-inspection.sh"
    exit 1
fi

# Создание директории для результатов
mkdir -p "$RESULTS_DIR"

# Запуск inspection только для измененных файлов
log_info "Анализ измененных файлов..."

# Получение списка измененных файлов
CHANGED_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.(java|kt|scala|py|js|ts|svelte|rs|go|php|cpp|c|h)$' || true)

if [ -z "$CHANGED_FILES" ]; then
    log_info "Измененных исходных файлов не найдено, пропуск inspection"
    exit 0
fi

log_info "Измененные файлы:"
echo "$CHANGED_FILES" | while read -r file; do
    log_info "  - $file"
done

# Создание временного профиля inspection только для измененных файлов
TEMP_PROFILE="/tmp/git-commit-inspection-profile.xml"
cat > "$TEMP_PROFILE" << EOF
<component name="InspectionProjectProfileManager">
  <profile version="1.0">
    <option name="myName" value="Git Commit Inspection" />
    <!-- Общие правила для всех языков -->
    <inspection_tool class="UnusedImport" enabled="true" level="WARNING" enabled_by_default="true" />
    <inspection_tool class="UnnecessaryModuleDependencyInspection" enabled="true" level="WARNING" enabled_by_default="true" />
    <inspection_tool class="ConstantConditions" enabled="true" level="WARNING" enabled_by_default="true" />

    <!-- Java/Kotlin -->
    <inspection_tool class="JavaDocMethod" enabled="true" level="WARNING" enabled_by_default="true" />
    <inspection_tool class="NullPointerException" enabled="true" level="ERROR" enabled_by_default="true" />

    <!-- JavaScript/TypeScript -->
    <inspection_tool class="JSUnusedGlobalSymbols" enabled="true" level="WARNING" enabled_by_default="true" />
    <inspection_tool class="JSHint" enabled="true" level="WARNING" enabled_by_default="true" />
    <inspection_tool class="ES6UnusedImports" enabled="true" level="WARNING" enabled_by_default="true" />

    <!-- Python -->
    <inspection_tool class="PyUnusedLocal" enabled="true" level="WARNING" enabled_by_default="true" />
    <inspection_tool class="PyPep8Naming" enabled="true" level="WARNING" enabled_by_default="true" />

    <!-- Rust -->
    <inspection_tool class="UnusedVariable" enabled="true" level="WARNING" enabled_by_default="true" />
    <inspection_tool class="UnnecessaryMut" enabled="true" level="WARNING" enabled_by_default="true" />
  </profile>
</component>
EOF

# Создание списка файлов для проверки (только измененные)
TEMP_FILE_LIST="/tmp/git-changed-files.txt"
echo "$CHANGED_FILES" | tr '\n' '\0' > "$TEMP_FILE_LIST"

# Запуск inspection
COMMIT_HASH=$(git rev-parse --short HEAD)
OUTPUT_FILE="$RESULTS_DIR/pre_commit_$COMMIT_HASH.xml"

log_info "Запуск code inspection..."

# Передаем переменные окружения в скрипт
export PROFILE_PATH="$TEMP_PROFILE"
export OUTPUT_DIR="$RESULTS_DIR"
export FORMAT="xml"
export FAIL_ON_ERROR="false"
export VERBOSE="true"

if bash "$INSPECTION_SCRIPT" --project "$PROJECT_ROOT" --output "$RESULTS_DIR" --format xml --profile "$TEMP_PROFILE" --include "$CHANGED_FILES"; then
    log_success "Code inspection завершен успешно"

    # Анализ результатов
    if [ -f "$OUTPUT_FILE" ]; then
        # Подсчет проблем
        if command -v xmllint &> /dev/null; then
            PROBLEM_COUNT=$(xmllint --xpath 'count(//problem)' "$OUTPUT_FILE" 2>/dev/null || echo "0")
            ERROR_COUNT=$(xmllint --xpath 'count(//problem[@severity="ERROR"])' "$OUTPUT_FILE" 2>/dev/null || echo "0")
            WARNING_COUNT=$(xmllint --xpath 'count(//problem[@severity="WARNING"])' "$OUTPUT_FILE" 2>/dev/null || echo "0")

            log_info "Результаты анализа:"
            log_info "  Всего проблем: $PROBLEM_COUNT"
            log_info "  Ошибок: $ERROR_COUNT"
            log_info "  Предупреждений: $WARNING_COUNT"

            # Проверка лимитов
            if [ "$ERROR_COUNT" -gt "$MAX_ERRORS" ]; then
                log_error "Превышен лимит ошибок: $ERROR_COUNT > $MAX_ERRORS"
                log_error "Исправьте ошибки перед коммитом или увеличьте MAX_ERRORS"
                log_error "Результаты: $OUTPUT_FILE"
                exit 1
            fi

            if [ "$ALLOW_WARNINGS" = "false" ] && [ "$WARNING_COUNT" -gt "$MAX_WARNINGS" ]; then
                log_error "Превышен лимит предупреждений: $WARNING_COUNT > $MAX_WARNINGS"
                log_error "Исправьте предупреждения или установите ALLOW_WARNINGS=true"
                log_error "Результаты: $OUTPUT_FILE"
                exit 1
            fi

            # Показать топ проблем
            if [ "$PROBLEM_COUNT" -gt 0 ]; then
                log_warn "Найденные проблемы:"
                # Извлекаем первые 5 проблем для показа
                xmllint --xpath '//problem[position() <= 5]' "$OUTPUT_FILE" 2>/dev/null | grep -E '(file|description|severity)' | head -15 | sed 's/^/  /' || true
                if [ "$PROBLEM_COUNT" -gt 5 ]; then
                    log_warn "  ... и еще $((PROBLEM_COUNT - 5)) проблем"
                fi
                log_warn "Полные результаты: $OUTPUT_FILE"
            fi

        else
            log_warn "xmllint не найден, пропуск детального анализа результатов"
        fi
    else
        log_warn "Файл с результатами не найден: $OUTPUT_FILE"
    fi

else
    log_error "Code inspection завершился с ошибкой"
    exit 1
fi

# Очистка временных файлов
rm -f "$TEMP_PROFILE" "$TEMP_FILE_LIST"

log_success "Pre-commit inspection пройден успешно"
exit 0

