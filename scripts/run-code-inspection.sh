#!/bin/bash

# Скрипт для автоматизации запуска code inspection в IntelliJ IDEA
# Использует командную строку IDEA для анализа кода и сохранения результатов

set -e

# Конфигурация по умолчанию
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
IDEA_PATH=""
PROJECT_PATH="${PROJECT_PATH:-$PROJECT_ROOT}"
OUTPUT_DIR="${OUTPUT_DIR:-$PROJECT_ROOT/inspection-results}"
FORMAT="${FORMAT:-xml}"
PROFILE="${PROFILE:-Project_Default}"
INCLUDE_PATTERN="${INCLUDE:-*}"
EXCLUDE_PATTERN="${EXCLUDE:-}"
FAIL_ON_ERROR="${FAIL_ON_ERROR:-false}"
VERBOSE="${VERBOSE:-false}"
TIMESTAMP_FORMAT="${TIMESTAMP_FORMAT:-%Y%m%d_%H%M%S}"

# Цвета для вывода
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

show_help() {
    cat << EOF
Скрипт для автоматизации запуска code inspection в IntelliJ IDEA

Использование: $0 [OPTIONS]

Опции:
    -h, --help              Показать эту справку
    -p, --project PATH      Путь к проекту (по умолчанию: корень проекта)
    -o, --output DIR        Директория для результатов (по умолчанию: ./inspection-results)
    -f, --format FORMAT     Формат вывода: xml, html, json (по умолчанию: xml)
    --profile NAME          Имя профиля inspection (по умолчанию: Project_Default)
    --include PATTERN       Шаблон включения файлов (по умолчанию: *)
    --exclude PATTERN       Шаблон исключения файлов
    --idea-path PATH        Путь к исполняемому файлу IDEA
    --fail-on-error         Завершить с ошибкой при нахождении проблем
    --verbose               Подробный вывод
    --timestamp-format FMT  Формат timestamp (по умолчанию: %Y%m%d_%H%M%S)

Переменные окружения:
    IDEA_PATH               Путь к IntelliJ IDEA
    PROJECT_PATH            Путь к проекту
    OUTPUT_DIR              Директория для результатов
    FORMAT                  Формат вывода
    PROFILE                 Имя профиля
    INCLUDE                 Шаблон включения
    EXCLUDE                 Шаблон исключения
    FAIL_ON_ERROR           Завершить с ошибкой при проблемах
    VERBOSE                 Подробный вывод

Примеры:
    # Базовое использование
    $0

    # С указанием пути к IDEA
    $0 --idea-path "/Applications/IntelliJ IDEA CE.app/Contents/MacOS/idea"

    # Только Java файлы с подробным выводом
    $0 --include "*.java" --verbose

    # Экспорт в HTML для отчетов
    $0 --format html --output ./reports

    # Строгая проверка (завершить с ошибкой при проблемах)
    $0 --fail-on-error
EOF
}

# Парсинг аргументов
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -p|--project)
            PROJECT_PATH="$2"
            shift 2
            ;;
        -o|--output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        -f|--format)
            FORMAT="$2"
            shift 2
            ;;
        --profile)
            PROFILE="$2"
            shift 2
            ;;
        --include)
            INCLUDE_PATTERN="$2"
            shift 2
            ;;
        --exclude)
            EXCLUDE_PATTERN="$2"
            shift 2
            ;;
        --idea-path)
            IDEA_PATH="$2"
            shift 2
            ;;
        --fail-on-error)
            FAIL_ON_ERROR=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --timestamp-format)
            TIMESTAMP_FORMAT="$2"
            shift 2
            ;;
        *)
            log_error "Неизвестная опция: $1"
            show_help
            exit 1
            ;;
    esac
done

# Автоопределение пути к IDEA
if [ -z "$IDEA_PATH" ]; then
    if [ -n "$IDEA_PATH" ]; then
        # Используем переменную окружения
        :
    elif command -v idea.sh &> /dev/null; then
        IDEA_PATH="idea.sh"
    elif command -v idea &> /dev/null; then
        IDEA_PATH="idea"
    elif [ -f "/Applications/IntelliJ IDEA CE.app/Contents/MacOS/idea" ]; then
        IDEA_PATH="/Applications/IntelliJ IDEA CE.app/Contents/MacOS/idea"
    elif [ -f "/Applications/IntelliJ IDEA.app/Contents/MacOS/idea" ]; then
        IDEA_PATH="/Applications/IntelliJ IDEA.app/Contents/MacOS/idea"
    elif [ -f "/Applications/WebStorm.app/Contents/MacOS/webstorm" ]; then
        IDEA_PATH="/Applications/WebStorm.app/Contents/MacOS/webstorm"
    elif [ -f "/opt/idea/bin/idea.sh" ]; then
        IDEA_PATH="/opt/idea/bin/idea.sh"
    elif [ -f "$HOME/.local/share/JetBrains/Toolbox/apps/IDEA-U/ch-0/*/bin/idea.sh" ]; then
        IDEA_PATH=$(find "$HOME/.local/share/JetBrains/Toolbox/apps/IDEA-U" -name "idea.sh" | head -1)
    elif [ -f "C:\Program Files\JetBrains\IntelliJ IDEA Community Edition\bin\idea64.exe" ]; then
        IDEA_PATH="C:\Program Files\JetBrains\IntelliJ IDEA Community Edition\bin\idea64.exe"
    elif [ -f "C:\Program Files\JetBrains\IntelliJ IDEA\bin\idea64.exe" ]; then
        IDEA_PATH="C:\Program Files\JetBrains\IntelliJ IDEA\bin\idea64.exe"
    elif [ -f "C:\Program Files\JetBrains\WebStorm\bin\webstorm64.exe" ]; then
        IDEA_PATH="C:\Program Files\JetBrains\WebStorm\bin\webstorm64.exe"
    else
        log_error "Не удалось найти IntelliJ IDEA. Укажите путь через --idea-path или переменную IDEA_PATH"
        exit 1
    fi
fi

# Проверка существования IDEA
if [ ! -f "$IDEA_PATH" ] && ! command -v "$IDEA_PATH" &> /dev/null; then
    log_error "IntelliJ IDEA не найдена по пути: $IDEA_PATH"
    log_error "Установите IDEA или укажите правильный путь через --idea-path"
    exit 1
fi

# Проверка существования проекта
if [ ! -d "$PROJECT_PATH" ]; then
    log_error "Директория проекта не существует: $PROJECT_PATH"
    exit 1
fi

# Создание выходной директории
mkdir -p "$OUTPUT_DIR"

# Определение файла профиля
PROFILE_PATH=""
if [ -n "$PROFILE" ]; then
    PROFILE_DIR="$PROJECT_PATH/.idea/inspectionProfiles"
    if [ -d "$PROFILE_DIR" ]; then
        PROFILE_FILE="$PROFILE_DIR/${PROFILE}.xml"
        if [ -f "$PROFILE_FILE" ]; then
            PROFILE_PATH="$PROFILE_FILE"
        fi
    fi
fi

# Генерация имени файла с timestamp
TIMESTAMP=$(date +"$TIMESTAMP_FORMAT")
OUTPUT_FILE="$OUTPUT_DIR/inspection_results_$TIMESTAMP.$FORMAT"

# Построение аргументов командной строки
ARGS=("$PROJECT_PATH")

if [ -n "$PROFILE_PATH" ]; then
    ARGS+=("$PROFILE_PATH")
else
    ARGS+=("")  # Пустой профиль для использования встроенного
fi

ARGS+=("$OUTPUT_FILE")
ARGS+=("-v2")  # Verbose level 2

# Добавление фильтров
if [ "$INCLUDE_PATTERN" != "*" ]; then
    ARGS+=("--include=$INCLUDE_PATTERN")
fi

if [ -n "$EXCLUDE_PATTERN" ]; then
    ARGS+=("--exclude=$EXCLUDE_PATTERN")
fi

# Вывод информации
log_info "=== Запуск Code Inspection ==="
log_info "IDE: $IDEA_PATH"
log_info "Проект: $PROJECT_PATH"
log_info "Профиль: ${PROFILE:-'встроенный'}"
log_info "Вывод: $OUTPUT_FILE"
log_info "Формат: $FORMAT"

if [ "$VERBOSE" = true ]; then
    log_info "Включения: $INCLUDE_PATTERN"
    if [ -n "$EXCLUDE_PATTERN" ]; then
        log_info "Исключения: $EXCLUDE_PATTERN"
    fi
    log_info "Команда: $IDEA_PATH inspect ${ARGS[*]}"
fi

# Запуск inspection
log_info "Запуск анализа кода..."
START_TIME=$(date +%s)

if "$IDEA_PATH" inspect "${ARGS[@]}"; then
    END_TIME=$(date +%s)
    DURATION=$((END_TIME - START_TIME))

    log_success "Анализ завершен успешно за ${DURATION}с"
    log_success "Результаты сохранены в: $OUTPUT_FILE"

    # Создание символической ссылки на последний результат
    LATEST_LINK="$OUTPUT_DIR/inspection_results_latest.$FORMAT"
    ln -sf "$(basename "$OUTPUT_FILE")" "$LATEST_LINK" 2>/dev/null || true
    log_info "Ссылка на последние результаты: $LATEST_LINK"

    # Анализ результатов
    if [ "$FORMAT" = "xml" ] && [ -f "$OUTPUT_FILE" ]; then
        if command -v xmllint &> /dev/null; then
            PROBLEM_COUNT=$(xmllint --xpath 'count(//problem)' "$OUTPUT_FILE" 2>/dev/null || echo "0")
            ERROR_COUNT=$(xmllint --xpath 'count(//problem[@severity="ERROR"])' "$OUTPUT_FILE" 2>/dev/null || echo "0")
            WARNING_COUNT=$(xmllint --xpath 'count(//problem[@severity="WARNING"])' "$OUTPUT_FILE" 2>/dev/null || echo "0")

            log_info "Найдено проблем: $PROBLEM_COUNT (ошибки: $ERROR_COUNT, предупреждения: $WARNING_COUNT)"

            if [ "$FAIL_ON_ERROR" = true ] && [ "$ERROR_COUNT" -gt 0 ]; then
                log_error "Найдены ошибки ($ERROR_COUNT). Выполнение прервано."
                exit 1
            fi
        fi
    fi

    # Конвертация в JSON для Cursor (если нужно)
    if [ "$FORMAT" = "xml" ] && command -v python3 &> /dev/null; then
        JSON_OUTPUT="$OUTPUT_DIR/inspection_results_$TIMESTAMP.json"
        if python3 "$SCRIPT_DIR/xml-to-json-converter.py" "$OUTPUT_FILE" "$JSON_OUTPUT" 2>/dev/null; then
            log_info "JSON версия создана: $JSON_OUTPUT"
        fi
    fi

else
    log_error "Анализ завершился с ошибкой"
    exit 1
fi

log_success "=== Code Inspection завершен ==="

