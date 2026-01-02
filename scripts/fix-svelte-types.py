#!/usr/bin/env python3
"""
Python скрипт для автоматического исправления проблем типизации в Svelte/TypeScript файлах.
Исправляет ошибки типа "Cannot assign store value to variable of type T" путем изменения типа на Writable<T>.
"""

import os
import re
import sys
import shutil
from typing import Dict, List, Set, Tuple, Optional
import json

class SvelteTypeFixer:
    def __init__(self):
        # Паттерны для поиска типов
        self.writable_pattern = re.compile(r'Writable<([^>]+)>')
        self.variable_declaration_pattern = re.compile(r'(let|const|var)\s+(\w+)\s*:\s*([^=;\n]+?)\s*=\s*(.+?);', re.DOTALL)

        # Svelte store patterns
        self.store_patterns = [
            re.compile(r'writable\s*\('),
            re.compile(r'readable\s*\('),
            re.compile(r'derived\s*\('),
            re.compile(r'\$derived\s*\('),
        ]

        # Встроенные типы TypeScript
        self.builtin_types = {
            'string', 'number', 'boolean', 'any', 'unknown', 'never',
            'void', 'null', 'undefined', 'object', 'Array', 'Record',
            'Promise', 'Date', 'RegExp', 'Map', 'Set'
        }

    def fix_file(self, file_path: str, create_backup: bool = True) -> Dict:
        """Исправление проблем типизации в файле"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            return {
                'file': file_path,
                'error': f'Cannot read file: {str(e)}',
                'fixes_applied': 0
            }

        original_content = content
        fixes_applied = 0

        print(f"\n=== Fixing {file_path} ===")

        # Анализ импортов типов
        imported_types = self.extract_imported_types(content)

        # Создаем временный анализатор для методов
        temp_analyzer = type('TempAnalyzer', (), {})()
        temp_analyzer.extract_parameters_and_variables = self.extract_parameters_and_variables.__get__(temp_analyzer)
        temp_analyzer.extract_used_variables = self.extract_used_variables.__get__(temp_analyzer)

        # Анализ проблем - запускаем анализатор через subprocess
        import subprocess
        import json

        result = subprocess.run([sys.executable, os.path.join(os.path.dirname(__file__), 'analyze-svelte-types.py'), file_path],
                              capture_output=True, text=True)

        # Парсим JSON отчет
        report_file = 'svelte-type-analysis-results.json'
        if os.path.exists(report_file):
            with open(report_file, 'r', encoding='utf-8') as f:
                report_data = json.load(f)
                # Находим проблемы для этого файла
                for item in report_data:
                    if item['file'] == file_path:
                        all_problems = item['problems']
                        break
                else:
                    all_problems = []
        else:
            all_problems = []

        # Создаем временный анализатор для методов
        temp_analyzer = type('TempAnalyzer', (), {})()
        temp_analyzer.extract_parameters_and_variables = self.extract_parameters_and_variables.__get__(temp_analyzer)
        temp_analyzer.extract_used_variables = self.extract_used_variables.__get__(temp_analyzer)

        # Отладка: покажем все проблемы
        print(f"All problems: {[p.get('variable', p.get('type', p)) for p in all_problems]}")
        print(f"Problem types: {[type(p) for p in all_problems]}")

        # Поиск и исправление проблем типизации
        content, file_fixes = self.fix_typing_issues(content, imported_types, all_problems, temp_analyzer)
        fixes_applied += len(file_fixes)

        # Сохранение файла с бэкапом
        if content != original_content:
            if create_backup:
                backup_path = f"{file_path}.bak"
                shutil.copy2(file_path, backup_path)
                print(f"Backup created: {backup_path}")

            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"File updated with {fixes_applied} fixes")
        else:
            print("No fixes needed")

        return {
            'file': file_path,
            'fixes_applied': fixes_applied,
            'fixes': file_fixes
        }

    def extract_imported_types(self, content: str) -> Dict[str, Dict]:
        """Извлечение импортированных типов с информацией о строках"""
        types = {}
        import_pattern = re.compile(r'import\s+(?:type\s+)?{([^}]+)}\s+from\s+[\'"]([^\'"]+)[\'"];?')

        for match in import_pattern.finditer(content):
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

    def fix_typing_issues(self, content: str, imported_types: Dict[str, Dict], problems: List[Dict], temp_analyzer=None) -> Tuple[str, List[Dict]]:
        """Исправление проблем типизации"""
        fixes = []

        # Анализ использования типов
        used_types = self.extract_used_types(content)

        # Исправление неиспользуемых импортов
        for import_name, import_info in imported_types.items():
            if import_name not in used_types:
                # Находим строку импорта
                lines = content.split('\n')
                if import_info['line'] <= len(lines):
                    import_line = lines[import_info['line'] - 1].strip()

                    # Создаем новую строку без этого импорта
                    # Это упрощенная логика - в реальности нужно правильно парсить импорты
                    if f'{import_name},' in import_line:
                        # Убираем импорт с запятой
                        new_line = import_line.replace(f'{import_name},', '').replace(', ,', ',')
                    elif f',{import_name}' in import_line:
                        # Убираем импорт в конце списка
                        new_line = import_line.replace(f',{import_name}', '')
                    else:
                        # Убираем единственный импорт
                        new_line = import_line.replace(import_name, '')

                    # Если остались пустые скобки или только пробелы, удаляем всю строку
                    if '{}' in new_line or '{ }' in new_line or '{  }' in new_line:
                        new_line = ''

                    if new_line != import_line:
                        lines[import_info['line'] - 1] = new_line if new_line else ''
                        content = '\n'.join(lines)

                        fixes.append({
                            'type': 'remove-unused-import',
                            'import_name': import_name,
                            'old_line': import_line,
                            'new_line': new_line,
                            'line': import_info['line'],
                            'reason': f'Removed unused import {import_name}'
                        })

                        print(f"Removed unused import: {import_name}")

        # Исправление проблем shorthand properties с undefined переменными
        shorthand_problems = [p for p in problems if p.get('type') == 'shorthand-property-undefined']

        for problem in shorthand_problems:
            var_name = problem['variable']

            # Заменяем shorthand property на полную форму с undefined
            # Ищем в script частях, где находятся объекты
            script_matches = re.findall(r'<script[^>]*>(.*?)</script>', content, re.DOTALL)
            for script_content in script_matches:
                # Ищем и заменяем shorthand property в объектах
                # Ищем { var } и заменяем на { var: undefined }
                pattern = r'(?<![.\w])\{\s*([^}]*?)\b' + re.escape(var_name) + r'\b([^}]*?)\}'
                replacement = r'{\1' + var_name + r': undefined\2}'

                if re.search(pattern, script_content):
                    new_script_content = re.sub(pattern, replacement, script_content)
                    content = content.replace(script_content, new_script_content)
                    print(f"Fixed undefined shorthand property: {var_name} -> {var_name}: undefined")
                    fixes.append({
                        'type': 'fix-shorthand-property-undefined',
                        'variable': var_name,
                        'line': problem['line'],
                        'reason': f'Changed shorthand property {var_name} to {var_name}: undefined'
                    })
                    break  # Выходим после первой замены

        # Исправление неиспользуемых переменных на основе проблем из анализатора
        unused_var_problems = [p for p in problems if p.get('type') == 'unused-variable']

        for problem in unused_var_problems:
            var_name = problem['variable']
            var_type = problem.get('type', 'variable')  # Это поле 'type' проблемы, не тип переменной

            # Находим информацию о переменной в текущем контенте
            parameters_and_vars = temp_analyzer.extract_parameters_and_variables(content)
            var_info = parameters_and_vars.get(var_name)

            if not var_info:
                print(f"Variable {var_name} not found in current content, skipping")
                continue

            print(f"Processing unused var from analyzer: {var_name} (type: {var_info['type']})")

            if var_info['type'] == 'variable':
                # Для простых переменных попробуем найти и удалить строку объявления
                lines = content.split('\n')
                if var_info['line'] <= len(lines):
                    line_content = lines[var_info['line'] - 1].strip()
                    # Проверяем, что это простое объявление переменной
                    if re.match(r'(let|const|var)\s+' + re.escape(var_name) + r'\s*=.*;', line_content):
                        # Удаляем строку
                        lines[var_info['line'] - 1] = ''
                        content = '\n'.join(lines)
                        print(f"Removed unused variable: {var_name}")
                        fixes.append({
                            'type': 'remove-unused-variable',
                            'variable': var_name,
                            'line': var_info['line'],
                            'reason': f'Removed unused variable {var_name}'
                        })
                    else:
                        print(f"Cannot safely remove unused variable: {var_name} (complex declaration)")
            elif var_info['type'] == 'destructured':
                # Для деструктурированных переменных попробуем удалить из деструктуризации
                # Это безопасно для опциональных props
                print(f"Trying to remove destructured variable: {var_name} (type: {var_info.get('type')})")
                new_content, success = self.try_remove_from_destructuring(content, var_name)
                if success:
                    content = new_content
                    print(f"Removed unused destructured variable: {var_name}")
                    fixes.append({
                        'type': 'remove-unused-destructured-variable',
                        'variable': var_name,
                        'line': var_info['line'],
                        'reason': f'Removed unused destructured variable {var_name}'
                    })
                else:
                    print(f"Cannot safely remove unused destructured variable: {var_name}")

        # Исправление орфографических ошибок
        spelling_problems = [p for p in problems if p.get('type') == 'spelling-error']
        for problem in spelling_problems:
            word = problem['word']
            correction = problem['correction']
            line_num = problem['line']

            lines = content.split('\n')
            if line_num <= len(lines):
                line = lines[line_num - 1]

                # Заменяем слово в комментарии
                if '//' in line:
                    before_comment, comment = line.split('//', 1)
                    corrected_comment = comment.replace(word, correction)
                    corrected_line = before_comment + '//' + corrected_comment
                    lines[line_num - 1] = corrected_line
                    content = '\n'.join(lines)
                    print(f"Fixed spelling: '{word}' -> '{correction}'")
                    fixes.append({
                        'type': 'fix-spelling',
                        'word': word,
                        'correction': correction,
                        'line': line_num,
                        'reason': f'Fixed spelling error {word} -> {correction}'
                    })

        # Находим все объявления переменных с присваиванием
        variables = self.extract_variable_declarations(content)

        for var_name, var_info in variables.items():
            var_type = var_info['type']
            assigned_value = var_info.get('assigned_value')

            if assigned_value:
                # Проверяем, присваивается ли store значение переменной обычного типа
                is_store_value = self.is_store_value(assigned_value)

                if var_type.startswith('Writable<'):
                    # Проверяем, присваивается ли обычное значение переменной типа Writable<T>
                    if not is_store_value:
                        # Это проблема - переменной типа Writable<T> присваивается обычное значение
                        writable_match = self.writable_pattern.search(var_type)
                        if writable_match:
                            inner_type = writable_match.group(1).strip()
                            # Исправляем тип на T
                            new_type = inner_type

                            # Находим и заменяем объявление
                            old_declaration = var_info['full_match']
                            new_declaration = old_declaration.replace(f': {var_type}', f': {new_type}', 1)

                            content = content.replace(old_declaration, new_declaration, 1)

                            fixes.append({
                                'type': 'change-variable-type',
                                'variable': var_name,
                                'old_type': var_type,
                                'new_type': new_type,
                                'line': var_info['line'],
                                'reason': 'Variable typed as Writable<T> but assigned plain value - changed to T'
                            })

                            print(f"Fixed: {var_name}: {var_type} -> {new_type}")

                elif is_store_value and not assigned_value.startswith('$') and not var_type.startswith('Writable<'):
                    # Это проблема - переменной обычного типа присваивается store значение
                    if var_type in [imp for imp in imported_types.keys()] or not self.is_builtin_type(var_type):
                        # Исправляем тип на Writable<T>
                        new_type = f'Writable<{var_type}>'

                        # Находим и заменяем объявление
                        old_declaration = var_info['full_match']
                        new_declaration = old_declaration.replace(f': {var_type}', f': {new_type}', 1)

                        content = content.replace(old_declaration, new_declaration, 1)

                        fixes.append({
                            'type': 'change-variable-type',
                            'variable': var_name,
                            'old_type': var_type,
                            'new_type': new_type,
                            'line': var_info['line'],
                            'reason': 'Variable typed as T but assigned store value - changed to Writable<T>'
                        })

                        print(f"Fixed: {var_name}: {var_type} -> {new_type}")

        return content, fixes

    def extract_variable_declarations(self, content: str) -> Dict:
        """Извлечение объявлений переменных с типами"""
        variables = {}

        # Сначала ищем переменные с присваиванием
        for match in self.variable_declaration_pattern.finditer(content):
            var_keyword = match.group(1)
            var_name = match.group(2)
            var_type = match.group(3).strip()
            assigned_value = match.group(4).strip()

            # Определяем номер строки
            line_num = content[:match.start()].count('\n') + 1
            full_match = match.group(0)

            variables[var_name] = {
                'type': var_type,
                'line': line_num,
                'assigned_value': assigned_value,
                'full_match': full_match
            }

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
                # Пропускаем строки с объявлениями переменных
                if re.match(r'(let|const|var)\s+', line):
                    continue

                # Пропускаем объявления интерфейсов и типов
                if re.match(r'(interface|type)\s+', line):
                    continue
                # Пропускаем строки с объявлениями свойств интерфейса (имя: Тип)
                if ':' in line and re.search(r'\w+\s*:\s*([A-Z]|\(\s*\)|Promise|void|boolean|number|string)', line):
                    continue

                # Пропускаем деструктуризацию $props()
                if '$props()' in line:
                    continue

                # Пропускаем комментарии
                if line.startswith('//') or '/*' in line:
                    continue

                # Пропускаем строковые литералы (простая проверка)
                # Убираем содержимое строк из анализа
                line_no_strings = re.sub(r"'[^']*'|\"[^\"]*\"", '', line)
                # Также убираем содержимое шаблонных строк
                line_no_strings = re.sub(r'`[^`]*`', '', line_no_strings)

                # Ищем переменные в строке (не в начале строки с объявлением)
                # Ищем слова, которые могут быть переменными
                words = re.findall(r'\b([a-zA-Z_$][a-zA-Z0-9_$]*)\b', line_no_strings)

                for word in words:
                    # Исключаем ключевые слова
                    if word not in ['let', 'const', 'var', 'function', 'if', 'else', 'for', 'while', 'return', 'true', 'false', 'null', 'undefined', 'console', 'log', 'error', 'warn', 'info', 'debug', 'typeof', 'instanceof', 'this', 'super', 'new', 'delete', 'void', 'typeof', 'in', 'of']:
                        # Исключаем имена атрибутов (слова, за которыми следует '=')
                        if '=' in line_no_strings and line_no_strings.find(word) < line_no_strings.find('='):
                            continue
                        # Проверяем, что это не часть объявления (нет '=' перед словом в этой строке)
                        word_pos = line_no_strings.find(word)
                        if word_pos != -1 and '=' not in line_no_strings[:word_pos]:
                            used_vars.add(word)

            # Анализируем template часть
            # Ищем использования в {{ var }}
            template_vars = re.findall(r'\{\{\s*([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\}\}', template_content)
            used_vars.update(template_vars)

            # Ищем shorthand properties в объектах: { var }
            shorthand_vars = re.findall(r'(?<![.\w])\{\s*([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\}', template_content)
            used_vars.update(shorthand_vars)

                # Ищем использования в атрибутах компонентов: attr={var}
            # Но НЕ считаем имена атрибутов переменными (только значения в {})
            attr_matches = re.findall(r'(\w+)\s*=\s*\{\s*([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\}', template_content)
            for attr_name, var_name in attr_matches:
                # Добавляем только значение в {}, но не имя атрибута
                used_vars.add(var_name)

            # НЕ считаем передачу как prop "использованием"
            # var={value} - это не использование переменной var

        return used_vars

    def try_remove_from_destructuring(self, content: str, var_name: str) -> tuple[str, bool]:
        """Пытается удалить переменную из деструктуризации $props()"""
        # Ищем let { ... }: Props = $props();
        props_match = re.search(r'(let\s*\{\s*)([^}]+)(\s*\}\s*:\s*Props\s*=\s*\$props\(\))', content, re.DOTALL)
        if not props_match:
            return content, False

        prefix = props_match.group(1)
        vars_part = props_match.group(2)
        suffix = props_match.group(3)

        # Разбираем переменные
        vars_text = re.sub(r'\s+', ' ', vars_part)  # Нормализуем пробелы
        vars_list = [v.strip().split(':')[0].strip().split('=')[0].strip() for v in vars_text.split(',')]

        # Проверяем, есть ли переменная
        if var_name not in vars_list:
            return content, False

        # Удаляем переменную
        vars_list.remove(var_name)

        # Если список пустой, это странно, но оставим как есть
        if not vars_list:
            return content, False

        # Собираем обратно
        new_vars_text = ',\n    '.join(vars_list)
        new_destructuring = f"{prefix}{new_vars_text}{suffix}"

        # Заменяем в контенте
        old_destructuring = props_match.group(0)
        new_content = content.replace(old_destructuring, new_destructuring)

        return new_content, True

    def is_builtin_type(self, type_name: str) -> bool:
        """Проверка, является ли тип встроенным"""
        # Убираем generic параметры
        base_type = re.sub(r'<.*>', '', type_name).strip()
        return base_type in self.builtin_types

def main():
    if len(sys.argv) < 2:
        print("Usage: python fix-svelte-types.py <file1> [file2] ... [--no-backup]")
        print("Options:")
        print("  --no-backup    Don't create backup files")
        sys.exit(1)

    create_backup = True
    files_to_fix = []

    for arg in sys.argv[1:]:
        if arg == '--no-backup':
            create_backup = False
        else:
            files_to_fix.append(arg)

    if not files_to_fix:
        print("No files specified")
        sys.exit(1)

    fixer = SvelteTypeFixer()
    all_results = []
    total_fixes = 0

    for file_path in files_to_fix:
        if os.path.exists(file_path):
            result = fixer.fix_file(file_path, create_backup)
            all_results.append(result)
            total_fixes += result['fixes_applied']
        else:
            print(f"File not found: {file_path}")

    # Сохранение отчета
    if all_results:
        report_file = 'svelte-type-fixes-report.json'
        with open(report_file, 'w', encoding='utf-8') as f:
            json.dump(all_results, f, indent=2, ensure_ascii=False)
        print(f"\nFix report saved to: {report_file}")

        # Общая статистика
        print("\n=== FIX SUMMARY ===")
        print(f"Files processed: {len(all_results)}")
        print(f"Total fixes applied: {total_fixes}")

        for result in all_results:
            if result['fixes_applied'] > 0:
                print(f"  {result['file']}: {result['fixes_applied']} fixes")

if __name__ == '__main__':
    main()
