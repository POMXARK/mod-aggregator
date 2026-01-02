#!/usr/bin/env python3
"""
Python скрипт для глубокого анализа типов в Svelte/TypeScript файлах.
Обнаруживает проблемы с Writable<T> vs T типизацией.
"""

import os
import re
import sys
from typing import Dict, List, Set, Tuple
import json

class SvelteTypeAnalyzer:
    def __init__(self):
        # Паттерны для поиска типов
        self.writable_pattern = re.compile(r'Writable<([^>]+)>')
        self.variable_declaration_pattern = re.compile(r'(?:let|const|var)\s+(\w+)\s*:\s*([^=]+?)\s*=\s*(.+?);', re.DOTALL)
        self.simple_variable_pattern = re.compile(r'(?:let|const|var)\s+(\w+)\s*:\s*([^;=\n]+)(?:\s*=|\s*;|\n|;)')
        self.import_pattern = re.compile(r'import\s+(?:type\s+)?{([^}]+)}\s+from\s+[\'"]([^\'"]+)[\'"];?')

        # Svelte store patterns
        self.store_patterns = [
            re.compile(r'writable\s*\('),  # writable(...)
            re.compile(r'readable\s*\('),  # readable(...)
            re.compile(r'derived\s*\('),   # derived(...)
            re.compile(r'\$derived\s*\('), # $derived(...)
        ]

        # Встроенные типы TypeScript
        self.builtin_types = {
            'string', 'number', 'boolean', 'any', 'unknown', 'never',
            'void', 'null', 'undefined', 'object', 'Array', 'Record',
            'Promise', 'Date', 'RegExp', 'Map', 'Set'
        }

    def analyze_file(self, file_path: str) -> Dict:
        """Анализ одного файла на проблемы типизации"""
        try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        except Exception as e:
            return {
                'file': file_path,
                'error': f'Cannot read file: {str(e)}',
                'problems': []
            }

        problems = []

        # Анализ импортов типов
        imported_types = self.extract_imported_types(content)

        # Анализ использования типов
        used_types = self.extract_used_types(content)

        # Проверка неиспользуемых импортов
        for import_name, import_info in imported_types.items():
            if import_name not in used_types:
                problems.append({
                    'type': 'unused-import',
                    'message': f'{import_name} is imported but never used',
                    'import_name': import_name,
                    'import_line': import_info['line'],
                    'severity': 'WARNING',
                    'fix': {
                        'type': 'remove-unused-import',
                        'import_name': import_name,
                        'import_line': import_info['line']
                    }
                })

        # Анализ объявлений переменных
        variables = self.extract_variable_declarations(content)

        # Анализ орфографии в комментариях
        self.analyze_spelling(content, problems)

        # Анализ параметров функций и переменных
        parameters_and_vars = self.extract_parameters_and_variables(content)
        used_vars = self.extract_used_variables(content)
        print(f"Used vars: {sorted(used_vars)}")
        print(f"Parameters and vars: {sorted(parameters_and_vars.keys())}")

        # Проверка неиспользуемых переменных/параметров
        # Исключаем переменные, которые только деструктурированы из props
        destructured_vars = {name for name, info in parameters_and_vars.items() if info['type'] == 'destructured'}

        for var_name, var_info in parameters_and_vars.items():
            # Пропускаем переменные, которые начинаются с _ или только деструктурированы
            if var_name.startswith('_'):
                continue

            # Для деструктурированных переменных проверяем, используются ли они за пределами деструктуризации
            if var_info['type'] == 'destructured':
                # Ищем реальные использования (вызовы функций, присваивания и т.д.)
                real_usage_found = False

                # Ищем вызовы функций: varName(
                if re.search(r'\b' + re.escape(var_name) + r'\s*\(', content):
                    real_usage_found = True
                    print(f"Found function call usage for {var_name}")

                # Ищем присваивания: varName =, varName +=, etc. (но НЕ varName={value})
                assignment_pattern = r'\b' + re.escape(var_name) + r'\s*[\+\-\*\/]?=\s*(?!\{)'
                if re.search(assignment_pattern, content):
                    real_usage_found = True
                    print(f"Found assignment usage for {var_name}")

                # Ищем использование в template как {{ var }}
                if re.search(r'\{\{\s*' + re.escape(var_name) + r'\s*\}\}', content):
                    real_usage_found = True
                    print(f"Found template usage for {var_name}")

                # Имена атрибутов в template НЕ считаются использованием переменных
                # var={value} - только value считается использованием, var - это имя атрибута
                # НО: shorthand properties в объектах считаются использованием
                # {var} в объектах - это var: var, т.е. использование
                if re.search(r'(?<![.\w])\{\s*' + re.escape(var_name) + r'\s*\}', content):
                    real_usage_found = True
                    print(f"Found shorthand property usage for {var_name}")

                    # Имена атрибутов в template НЕ считаются использованием переменных
                # var={value} - только value считается использованием, var - это имя атрибута

                # Проверяем shorthand properties в объектах
                shorthand_match = re.search(r'(?<![.\w])\{\s*' + re.escape(var_name) + r'\s*\}', content)
                if shorthand_match:
                    real_usage_found = True
                    print(f"Found shorthand property usage for {var_name}")

                if not real_usage_found:
                    print(f"Found unused destructured variable: {var_name}")
                    problems.append({
                        'type': 'unused-variable',
                        'message': f'{var_name} is declared but its value is never read',
                        'variable': var_name,
                        'line': var_info['line'],
                        'severity': 'WARNING',
                        'fix': {
                            'type': 'remove-unused-variable',
                            'variable': var_name,
                            'line': var_info['line']
                        }
                    })
            elif var_info['type'] == 'variable' and var_name not in used_vars:
                problems.append({
                    'type': 'unused-variable',
                    'message': f'{var_name} is declared but its value is never read',
                    'variable': var_name,
                    'line': var_info['line'],
                    'severity': 'WARNING',
                    'fix': {
                        'type': 'remove-unused-variable',
                        'variable': var_name,
                        'line': var_info['line']
                    }
                })

        # Проверка проблем типизации
        for var_name, var_info in variables.items():
            var_type = var_info['type']
            var_line = var_info['line']
            assigned_value = var_info.get('assigned_value')

            # Проверка на Writable<T> типизацию
            if 'Writable<' in var_type:
                writable_match = self.writable_pattern.search(var_type)
                if writable_match:
                    inner_type = writable_match.group(1).strip()

                    if assigned_value:
                        # Проверка, присваивается ли не-store значение переменной типа Writable<T>
                        is_store_value = self.is_store_value(assigned_value)
                        if not is_store_value:
                            problems.append({
                                'type': 'incorrect-writable-typing',
                                'message': f'Type Writable<{inner_type}> is missing the following properties from type {inner_type}',
                                'variable': var_name,
                                'declared_type': var_type,
                                'expected_type': inner_type,
                                'assigned_value': assigned_value,
                                'line': var_line,
                                'severity': 'ERROR',
                                'fix': {
                                    'type': 'change-variable-type',
                                    'from': var_type,
                                    'to': inner_type
                                }
                            })

            # Проверка на присваивание обычного значения переменной типа Writable<T>
            if 'Writable<' in var_type:
                writable_match = self.writable_pattern.search(var_type)
                if writable_match:
                    inner_type = writable_match.group(1).strip()

                    if assigned_value:
                        # Проверка, присваивается ли обычное значение переменной типа Writable<T>
                        is_store_value = self.is_store_value(assigned_value)
                        if not is_store_value:
                            problems.append({
                                'type': 'writable-assigned-plain-value',
                                'message': f'Type Writable<{inner_type}> is missing the following properties from type {inner_type}',
                                'variable': var_name,
                                'declared_type': var_type,
                                'expected_type': inner_type,
                                'assigned_value': assigned_value,
                                'line': var_line,
                                'severity': 'ERROR',
                                'fix': {
                                    'type': 'change-variable-type',
                                    'from': var_type,
                                    'to': inner_type
                                }
                            })

            # Проверка на присваивание store значения переменной типа T
            elif var_type in imported_types or not self.is_builtin_type(var_type):
                if assigned_value:
                    # Проверка, присваивается ли store значение переменной обычного типа
                    is_store_value = self.is_store_value(assigned_value)
                    if is_store_value and not assigned_value.startswith('$'):
                        problems.append({
                            'type': 'missing-writable-wrapper',
                            'message': f'Cannot assign store value to variable of type {var_type}',
                            'variable': var_name,
                            'declared_type': var_type,
                            'expected_type': f'Writable<{var_type}>',
                            'assigned_value': assigned_value,
                            'line': var_line,
                            'severity': 'ERROR',
                            'fix': {
                                'type': 'change-variable-type',
                                'from': var_type,
                                'to': f'Writable<{var_type}>'
                            }
                        })

        # Анализ shorthand properties в объектах, которые ссылаются на неопределенные переменные
        shorthand_problems = []
        # Ищем в script части (между <script> и </script>)
        script_matches = re.findall(r'<script[^>]*>(.*?)</script>', content, re.DOTALL)
        for script_content in script_matches:
            # Ищем объекты и разбираем их свойства
            # Находим все объекты: { ... }
            object_matches = re.findall(r'\{([^}]+)\}', script_content)
            for obj_content in object_matches:
                # Разбираем свойства объекта, обрабатывая многострочные конструкции
                # Разделяем по запятым, но учитываем вложенные структуры
                properties = []
                current_prop = ""
                brace_count = 0

                for char in obj_content:
                    if char == '{':
                        brace_count += 1
                    elif char == '}':
                        brace_count -= 1
                    elif char == ',' and brace_count == 0:
                        properties.append(current_prop.strip())
                        current_prop = ""
                        continue
                    current_prop += char

                if current_prop.strip():
                    properties.append(current_prop.strip())

                for prop in properties:
                    prop = prop.strip()
                    # Убираем комментарии
                    prop = re.sub(r'//.*', '', prop).strip()
                    # Проверяем, является ли это shorthand property (только имя без двоеточия)
                    if re.match(r'^[a-zA-Z_$][a-zA-Z0-9_$]*$', prop):
                        var_name = prop
                        # Находим позицию в полном контенте
                        obj_start = content.find('{' + obj_content + '}')
                        if obj_start != -1:
                            line_num = content[:obj_start].count('\n') + 1
                            shorthand_problems.append({
                                'var': var_name,
                                'line': line_num
                            })

        available_vars = set(parameters_and_vars.keys())

        for problem in shorthand_problems:
            var_name = problem['var']
            if var_name not in available_vars:
                problems.append({
                    'type': 'shorthand-property-undefined',
                    'severity': 'ERROR',
                    'message': f'No value exists in scope for the shorthand property {var_name}',
                    'variable': var_name,
                    'line': problem['line']
                })

        return {
            'file': file_path,
            'problems': problems,
            'summary': {
                'total_problems': len(problems),
                'errors': len([p for p in problems if p['severity'] == 'ERROR']),
                'warnings': len([p for p in problems if p['severity'] == 'WARNING'])
            }
        }

    def extract_imported_types(self, content: str) -> Dict[str, Dict]:
        """Извлечение импортированных типов с информацией о строках"""
        types = {}
        for match in self.import_pattern.finditer(content):
            import_block = match.group(1)
            line_num = content[:match.start()].count('\n') + 1
            # Разбор импортов
            imports = [imp.strip() for imp in import_block.split(',')]
            for imp in imports:
                # Убираем 'type' и 'as alias'
                imp = re.sub(r'\s+type\s+', '', imp)
                imp = re.sub(r'\s+as\s+\w+', '', imp)
                clean_imp = imp.strip()
                types[clean_imp] = {'line': line_num}

        return types

    def extract_used_types(self, content: str) -> Set[str]:
        """Извлечение использованных типов"""
        used_types = set()

        # Типы в объявлениях переменных
        var_type_matches = re.findall(r'(?:let|const|var)\s+\w+\s*:\s*([A-Z][a-zA-Z0-9_]*(?:<[^>]*>)?)', content)
        used_types.update(var_type_matches)

        # Типы в параметрах функций
        param_type_matches = re.findall(r'\w+\s*:\s*([A-Z][a-zA-Z0-9_]*(?:<[^>]*>)?)', content)
        used_types.update(param_type_matches)

        # Типы в generic
        generic_matches = re.findall(r'<([A-Z][a-zA-Z0-9_]*(?:<[^>]*>)?)>', content)
        used_types.update(generic_matches)

        # Анализ store bindings - переменные типа $storeName НЕ указывают на типы
        # Типы выводятся автоматически, поэтому store bindings не считаются использованием типов
        # store_binding_matches = re.findall(r'\$([a-zA-Z_][a-zA-Z0-9_]*)(?:Store)?', content)
        # (Убрана логика вывода типов из store bindings)

        # Анализ деструктуризации результатов функций
        destructuring_matches = re.findall(r'const\s*{\s*[^}]*}\s*=\s*(\w+)\s*\(', content)
        for func_name in destructuring_matches:
            used_types.add(func_name)

        # Очистка от generic параметров
        cleaned_types = set()
        for type_name in used_types:
            # Убираем generic параметры для простых типов
            clean_type = re.sub(r'<.*>', '', type_name)
            cleaned_types.add(clean_type)

        return cleaned_types

    def extract_variable_declarations(self, content: str) -> Dict:
        """Извлечение объявлений переменных с типами"""
        variables = {}

        # Ищем все объявления переменных
        lines = content.split('\n')
        i = 0
        while i < len(lines):
            line = lines[i]
            # Ищем начало объявления переменной
            var_match = re.search(r'(?:let|const|var)\s+(\w+)\s*:\s*([^=;\n]+)', line)
            if var_match:
                var_name = var_match.group(1)
                var_type = var_match.group(2).strip()
                line_num = i + 1

                # Проверяем, есть ли присваивание на этой строке
                assign_match = re.search(r'=\s*(.+)', line)
                if assign_match:
                    # Присваивание на той же строке
                    assigned_value = assign_match.group(1).strip()

                    # Собираем значение, считая скобки
                    assigned_lines = [assigned_value]
                    brace_count = assigned_value.count('{') - assigned_value.count('}')
                    paren_count = assigned_value.count('(') - assigned_value.count(')')
                    bracket_count = assigned_value.count('[') - assigned_value.count(']')

                    # Продолжаем собирать строки, пока не сбалансируем все скобки
                    i += 1
                    while i < len(lines) and (brace_count > 0 or paren_count > 0 or bracket_count > 0):
                        next_line = lines[i]
                        assigned_lines.append(next_line)
                        brace_count += next_line.count('{') - next_line.count('}')
                        paren_count += next_line.count('(') - next_line.count(')')
                        bracket_count += next_line.count('[') - next_line.count(']')

                        # Проверяем, закончилась ли инструкция
                        stripped = next_line.strip()
                        if stripped.endswith(';') and brace_count <= 0 and paren_count <= 0 and bracket_count <= 0:
                            break
                        i += 1

                    assigned_value = ''.join(assigned_lines).strip()
                    # Убираем завершающую точку с запятой
                    if assigned_value.endswith(';'):
                        assigned_value = assigned_value[:-1].strip()
                else:
                    # Нет присваивания
                    assigned_value = None
                    i += 1

                variables[var_name] = {
                    'type': var_type,
                    'line': line_num,
                    'assigned_value': assigned_value
                }
            else:
                i += 1

        return variables

    def is_store_value(self, value: str) -> bool:
        """Проверка, является ли значение store"""
        value = value.strip()

        # Прямые вызовы store функций
        for pattern in self.store_patterns:
            if pattern.search(value):
                return True

        # $derived(store) - это не store значение, это обычное значение
        if value.startswith('$derived('):
            return False

        # Переменные, которые могут быть stores (простая эвристика)
        if re.match(r'^[a-zA-Z_$][a-zA-Z0-9_$]*$', value):
            # Проверяем, не является ли это вызовом функции
            if not value.endswith('()'):
                # Предполагаем, что переменные без () могут быть stores
                return True

        return False

    def extract_parameters_and_variables(self, content: str) -> Dict[str, Dict]:
        """Извлечение объявлений параметров функций и переменных"""
        declarations = {}

        # Параметры функций
        # Ищем function name(param: type, param2: type)
        func_param_matches = re.findall(r'(?:function\s+\w+|(?:let|const|var)\s+\w+\s*=.*=>).*?\(([^)]*)\)', content, re.DOTALL)
        for params_str in func_param_matches:
            # Разбираем параметры: param: type, param2: type
            param_matches = re.findall(r'(\w+)\s*:\s*[^,]+', params_str)
            for param in param_matches:
                if param not in declarations:
                    # Находим строку (приблизительно)
                    line_num = content.find(params_str) // 100 + 1  # Приблизительная оценка
                    declarations[param] = {'line': line_num, 'type': 'parameter'}

        # Переменные из деструктуризации $props()
        # Ищем let { ... }: Props = $props();
        props_match = re.search(r'let\s*\{\s*(.*?)\s*\}\s*:\s*Props\s*=\s*\$props\(\)', content, re.DOTALL)
        if props_match:
            destructured_vars = props_match.group(1)
            # Разбираем переменные, убирая переносы строк и пробелы
            vars_text = re.sub(r'\s+', ' ', destructured_vars)  # Заменяем whitespace на пробелы
            vars_list = []
            for var in vars_text.split(','):
                var_name = var.strip().split(':')[0].strip()
                var_name = var_name.split('=')[0].strip()  # Убираем default values
                if var_name:
                    vars_list.append(var_name)

            # Находим строку начала деструктуризации
            start_pos = props_match.start()
            line_num = content[:start_pos].count('\n') + 1

            for var_name in vars_list:
                if var_name not in declarations:
                    declarations[var_name] = {'line': line_num, 'type': 'destructured'}

        # Обычные переменные
        lines = content.split('\n')
        for line_num, line in enumerate(lines, 1):
            var_match = re.search(r'(?:let|const|var)\s+(\w+)\s*[:=]', line)
            if var_match:
                var_name = var_match.group(1)
                if var_name not in declarations:
                    declarations[var_name] = {'line': line_num, 'type': 'variable'}

        return declarations

    def extract_used_variables(self, content: str) -> Set[str]:
        """Извлечение использованных переменных"""
        used_vars = set()

        # Разделим контент на части: script и template
        script_match = re.search(r'<script[^>]*>(.*?)</script>', content, re.DOTALL)
        template_content = content
        if script_match:
            script_content = script_match.group(1)
            template_content = content.replace(script_match.group(0), '')

            # Анализируем script часть
            # Ищем использования переменных в выражениях (не в объявлениях)
            # Используем более точную логику: переменная, за которой нет '=' (не объявление)

            # Разделим script на строки для построчного анализа
            script_lines = script_content.split('\n')

            for line in script_lines:
                line = line.strip()
                # Пропускаем строки с объявлениями
                if re.match(r'(let|const|var)\s+', line):
                    continue

                # Ищем переменные в строке (не в начале строки с объявлением)
                # Ищем слова, которые могут быть переменными
                words = re.findall(r'\b([a-zA-Z_$][a-zA-Z0-9_$]*)\b', line)

                for word in words:
                    # Исключаем ключевые слова
                    if word not in ['let', 'const', 'var', 'function', 'if', 'else', 'for', 'while', 'return', 'true', 'false', 'null', 'undefined', 'console', 'log', 'error', 'warn', 'info', 'debug', 'typeof', 'instanceof', 'this', 'super', 'new', 'delete', 'void', 'typeof', 'in', 'of']:
                        # Проверяем, что это не часть объявления (нет '=' перед словом в этой строке)
                        if '=' not in line[:line.find(word)]:
                            used_vars.add(word)

            # Анализируем template часть
            # Ищем использования в {{ var }}
            template_vars = re.findall(r'\{\{\s*([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\}\}', template_content)
            used_vars.update(template_vars)

            # Ищем использования в атрибутах компонентов: attr={var}
            attr_vars = re.findall(r'\w+\s*=\s*\{\s*([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\}', template_content)
            used_vars.update(attr_vars)

        return used_vars

    def analyze_spelling(self, content: str, problems: List[Dict]) -> None:
        """Анализ орфографии в комментариях"""
        # Словарь известных опечаток из WebStorm XML
        spelling_corrections = {
            'логируем': 'логируем',  # Возможно правильное
            'ноду': 'node',         # Опечатка: должно быть "node" (английский термин)
            'нодой': 'node',        # Опечатка: должно быть "node" (английский термин)
            'парсера': 'парсера'    # Возможно правильное
        }

        lines = content.split('\n')
        for line_num, line in enumerate(lines, 1):
            # Ищем комментарии (// и /* */)
            comment_text = ""

            # Однострочные комментарии
            if '//' in line:
                comment_text = line.split('//', 1)[1].strip()

            # Многострочные комментарии (простая проверка)
            elif '/*' in line and '*/' in line:
                start = line.find('/*')
                end = line.find('*/', start)
                if start != -1 and end != -1:
                    comment_text = line[start+2:end].strip()

            if comment_text:
                # Проверяем на известные опечатки
                for wrong_word, correct_word in spelling_corrections.items():
                    if wrong_word in comment_text:
                        problems.append({
                            'type': 'spelling-error',
                            'message': f'Typo in comment: "{wrong_word}" should be "{correct_word}"',
                            'word': wrong_word,
                            'correction': correct_word,
                            'line': line_num,
                            'severity': 'WARNING',
                            'fix': {
                                'type': 'fix-spelling',
                                'word': wrong_word,
                                'correction': correct_word,
                                'line': line_num
                            }
                        })

    def is_builtin_type(self, type_name: str) -> bool:
        """Проверка, является ли тип встроенным"""
        # Убираем generic параметры
        base_type = re.sub(r'<.*>', '', type_name).strip()
        return base_type in self.builtin_types

def main():
    if len(sys.argv) < 2:
        print("Usage: python analyze-svelte-types.py <file1> [file2] ...")
        sys.exit(1)

    analyzer = SvelteTypeAnalyzer()
    all_results = []

    for file_path in sys.argv[1:]:
        if not os.path.exists(file_path):
            print(f"File not found: {file_path}")
            continue

        print(f"\n=== Processing {file_path} ===")

        # Анализ файла
        result = analyzer.analyze_file(file_path)

        # Вывод проблем
        if result['problems']:
            print(f"\n=== {file_path} ===")
            for problem in result['problems']:
                line_info = problem.get('line') or problem.get('import_line', 'N/A')
                print(f"[{problem['severity']}] Line {line_info}: {problem['message']}")

        all_results.append(result)

    # Сохранение отчета
    if all_results:
        report_file = 'svelte-type-analysis-results.json'
        with open(report_file, 'w', encoding='utf-8') as f:
            json.dump(all_results, f, indent=2, ensure_ascii=False)
        print(f"\nFinal report saved to: {report_file}")

if __name__ == '__main__':
    main()
