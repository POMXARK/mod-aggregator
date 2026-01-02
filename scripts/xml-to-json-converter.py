#!/usr/bin/env python3
"""
Конвертер результатов code inspection из XML в JSON формат
Оптимизирован для использования в Cursor IDE
"""

import xml.etree.ElementTree as ET
import json
import sys
from datetime import datetime
from pathlib import Path
import argparse

def parse_xml_results(xml_file: str) -> dict:
    """Парсит XML файл с результатами inspection и возвращает словарь"""

    try:
        tree = ET.parse(xml_file)
        root = tree.getroot()
    except ET.ParseError as e:
        print(f"Ошибка парсинга XML: {e}", file=sys.stderr)
        return {}

    problems = []

    # Поиск всех проблем в XML
    for problem_elem in root.findall('.//problem'):
        problem = {}

        # Извлечение основных полей
        file_elem = problem_elem.find('file')
        line_elem = problem_elem.find('line')
        column_elem = problem_elem.find('column')
        description_elem = problem_elem.find('description')
        severity_elem = problem_elem.find('severity')
        category_elem = problem_elem.find('category')
        inspection_elem = problem_elem.find('inspection')
        quickfix_elem = problem_elem.find('quickfix')

        problem.update({
            'file': file_elem.text if file_elem is not None else '',
            'line': int(line_elem.text) if line_elem is not None and line_elem.text.isdigit() else 0,
            'column': int(column_elem.text) if column_elem is not None and column_elem.text.isdigit() else 0,
            'message': description_elem.text if description_elem is not None else '',
            'severity': severity_elem.text if severity_elem is not None else 'UNKNOWN',
            'category': category_elem.text if category_elem is not None else '',
            'inspection': inspection_elem.text if inspection_elem is not None else '',
            'quickFix': quickfix_elem.text if quickfix_elem is not None else None,
        })

        # Извлечение дополнительной информации
        hints_elem = problem_elem.find('hints')
        if hints_elem is not None:
            hints = []
            for hint_elem in hints_elem.findall('hint'):
                hints.append(hint_elem.text if hint_elem.text else '')
            problem['hints'] = hints

        # Извлечение атрибутов проблемы
        attributes_elem = problem_elem.find('attributes')
        if attributes_elem is not None:
            attributes = {}
            for attr_elem in attributes_elem.findall('attribute'):
                name = attr_elem.get('name')
                value = attr_elem.get('value') or attr_elem.text
                if name and value:
                    attributes[name] = value
            problem['attributes'] = attributes

        problems.append(problem)

    # Создание итогового отчета
    result = {
        'timestamp': datetime.now().isoformat(),
        'project': str(Path(xml_file).parent.parent.name),  # Имя проекта из пути
        'problems': problems,
        'summary': {
            'total': len(problems),
            'errors': len([p for p in problems if p['severity'] == 'ERROR']),
            'warnings': len([p for p in problems if p['severity'] == 'WARNING']),
            'info': len([p for p in problems if p['severity'] == 'INFO']),
            'weak_warnings': len([p for p in problems if p['severity'] == 'WEAK WARNING']),
        },
        'metadata': {
            'source': 'jetbrains-inspection',
            'format': 'json',
            'converted_at': datetime.now().isoformat(),
            'xml_source': str(Path(xml_file).name)
        }
    }

    return result

def main():
    parser = argparse.ArgumentParser(
        description='Конвертер результатов JetBrains inspection из XML в JSON'
    )
    parser.add_argument('input', help='Путь к входному XML файлу')
    parser.add_argument('output', nargs='?', help='Путь к выходному JSON файлу (опционально)')
    parser.add_argument('--pretty', action='store_true', help='Форматировать JSON с отступами')
    parser.add_argument('--filter-severity', help='Фильтровать по severity (ERROR,WARNING,INFO)')

    args = parser.parse_args()

    # Определение выходного файла
    if args.output:
        output_file = args.output
    else:
        input_path = Path(args.input)
        output_file = str(input_path.parent / f"{input_path.stem}.json")

    # Парсинг XML
    print(f"Парсинг {args.input}...", file=sys.stderr)
    result = parse_xml_results(args.input)

    if not result:
        print("Ошибка: не удалось распарсить XML файл", file=sys.stderr)
        sys.exit(1)

    # Фильтрация по severity
    if args.filter_severity:
        allowed_severities = [s.strip().upper() for s in args.filter_severity.split(',')]
        filtered_problems = [
            p for p in result['problems']
            if p['severity'].upper() in allowed_severities
        ]
        result['problems'] = filtered_problems

        # Пересчет summary
        result['summary'] = {
            'total': len(filtered_problems),
            'errors': len([p for p in filtered_problems if p['severity'] == 'ERROR']),
            'warnings': len([p for p in filtered_problems if p['severity'] == 'WARNING']),
            'info': len([p for p in filtered_problems if p['severity'] == 'INFO']),
            'weak_warnings': len([p for p in filtered_problems if p['severity'] == 'WEAK WARNING']),
        }

    # Сохранение JSON
    indent = 2 if args.pretty else None
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(result, f, indent=indent, ensure_ascii=False)

    print(f"Конвертировано {result['summary']['total']} проблем в {output_file}", file=sys.stderr)

    # Вывод краткой статистики
    summary = result['summary']
    print(f"Всего проблем: {summary['total']}", file=sys.stderr)
    print(f"Ошибки: {summary['errors']}, Предупреждения: {summary['warnings']}, Инфо: {summary['info']}", file=sys.stderr)

if __name__ == '__main__':
    main()

