#!/usr/bin/env python3
"""
Анализатор проблем с типизацией stores в Svelte компонентах

Ищет ошибки типа:
- Type Writable<T> is missing the following properties from type T
- Неправильное использование $store синтаксиса
- Неправильное присваивание Writable<T> к T
"""

import os
import re
import json
from pathlib import Path
from typing import List, Dict, Any, Tuple

class SvelteStoreAnalyzer:
    def __init__(self, project_root: str):
        self.project_root = Path(project_root)
        self.issues: List[Dict[str, Any]] = []

    def analyze_file(self, file_path: str) -> None:
        """Анализирует Svelte файл на проблемы с stores"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            return

        lines = content.split('\n')
        relative_path = os.path.relpath(file_path, self.project_root)

        # Анализ script блока
        script_match = re.search(r'<script[^>]*>(.*?)</script>', content, re.DOTALL)
        if not script_match:
            return

        script_content = script_match.group(1)

        # Найти импорты stores
        store_imports = self._find_store_imports(script_content)

        # Найти объявления переменных
        variable_declarations = self._find_variable_declarations(script_content)

        # Найти использования stores в эффектах
        self._analyze_store_usage_in_effects(script_content, lines, relative_path, store_imports, variable_declarations)

    def _find_store_imports(self, content: str) -> Dict[str, str]:
        """Находит импорты stores из composables"""
        stores = {}

        # Ищем destructuring из composables
        # const { storeName, ... } = useSomething();
        matches = re.finditer(r'const\s*{\s*([^}]+)\s*}\s*=\s*(\w+)\(\)', content)
        for match in matches:
            destructured = match.group(1)
            composable = match.group(2)

            # Разбираем destructured переменные
            vars_list = [var.strip().split(':')[0].strip() for var in destructured.split(',')]

            for var in vars_list:
                if 'Store' in var or var.endswith('Store'):
                    stores[var] = composable

        return stores

    def _find_variable_declarations(self, content: str) -> Dict[str, str]:
        """Находит объявления переменных с типами"""
        declarations = {}

        # Ищем let/const переменные с типами
        matches = re.finditer(r'\b(?:let|const)\s+(\w+)\s*:\s*([^;=]+)', content)
        for match in matches:
            var_name = match.group(1)
            var_type = match.group(2).strip()
            declarations[var_name] = var_type

        return declarations

    def _analyze_store_usage_in_effects(self, content: str, lines: List[str], file_path: str,
                                      store_imports: Dict[str, str], declarations: Dict[str, str]) -> None:
        """Анализирует использование stores в $effect"""

        # Найти все $effect блоки
        effect_matches = re.finditer(r'\$effect\s*\(\s*\(\)\s*=>\s*{([^}]*(?:\{[^}]*\}[^}]*)*)\}', content, re.DOTALL)

        for effect_match in effect_matches:
            effect_content = effect_match.group(1)

            # Найти присваивания типа variable = $storeName
            assignments = re.finditer(r'(\w+)\s*=\s*\$(\w+)', effect_content)

            for assignment in assignments:
                var_name = assignment.group(1)
                store_ref = assignment.group(2)

                # Проверить, есть ли такая переменная в объявлениях
                if var_name in declarations:
                    var_type = declarations[var_name]

                    # Проверить, есть ли store в импортах
                    if store_ref in store_imports:
                        # Найти строку с присваиванием
                        line_match = re.search(re.escape(f'{var_name} = ${store_ref}'), effect_content)
                        if line_match:
                            # Найти номер строки
                            effect_start = effect_match.start()
                            assignment_start = effect_start + line_match.start()
                            line_number = content[:assignment_start].count('\n') + 1

                            # Это проблематичное присваивание - $storeName не является reactive binding
                            # В Svelte 5 нужно использовать $derived(storeName) для получения значения из store
                            self.issues.append({
                                'file': file_path,
                                'line': line_number,
                                'column': 0,
                                'message': f'Type Writable<{var_type}> is missing the following properties from type {var_type}: store value access error',
                                'severity': 'ERROR',
                                'category': 'Svelte',
                                'rule': 'svelte-store-reactive-access',
                                'fix': {
                                    'type': 'replace-assignment',
                                    'old': f'{var_name} = ${store_ref}',
                                    'new': f'{var_name} = $derived({store_ref})'
                                }
                            })

            # Также ищем другие проблематичные использования stores
            # variable = store.get() или подобные
            other_assignments = re.finditer(r'(\w+)\s*=\s*(\w+)\.get\(\)', effect_content)
            for assignment in other_assignments:
                var_name = assignment.group(1)
                store_name = assignment.group(2)

                if var_name in declarations and store_name in store_imports:
                    var_type = declarations[var_name]
                    line_match = re.search(re.escape(f'{var_name} = {store_name}.get()'), effect_content)
                    if line_match:
                        effect_start = effect_match.start()
                        assignment_start = effect_start + line_match.start()
                        line_number = content[:assignment_start].count('\n') + 1

                        self.issues.append({
                            'file': file_path,
                            'line': line_number,
                            'column': 0,
                            'message': f'Incorrect store access pattern: use $derived({store_name}) instead of {store_name}.get()',
                            'severity': 'WARNING',
                            'category': 'Svelte',
                            'rule': 'svelte-store-get-usage',
                            'fix': {
                                'type': 'replace-assignment',
                                'old': f'{var_name} = {store_name}.get()',
                                'new': f'{var_name} = $derived({store_name})'
                            }
                        })

    def _is_problematic_assignment(self, var_type: str, store_name: str) -> bool:
        """Проверяет, является ли присваивание проблематичным"""

        # Если переменная имеет тип без Writable<>, а store присваивается с $
        # это может быть проблемой
        if not var_type.startswith('Writable<') and store_name.endswith('Store'):
            return True

        return False

    def generate_report(self) -> Dict[str, Any]:
        """Генерирует отчет о найденных проблемах"""
        return {
            'summary': {
                'total_files_analyzed': 0,  # TODO: implement
                'errors': len([i for i in self.issues if i['severity'] == 'ERROR']),
                'warnings': len([i for i in self.issues if i['severity'] == 'WARNING']),
                'info': len([i for i in self.issues if i['severity'] == 'INFO'])
            },
            'problems': self.issues
        }

def main():
    # Создаем анализатор
    analyzer = SvelteStoreAnalyzer('.')

    # Ищем все Svelte файлы
    svelte_files = []
    for root, dirs, files in os.walk('.'):
        for file in files:
            if file.endswith('.svelte'):
                svelte_files.append(os.path.join(root, file))

    # Анализируем файлы
    for file_path in svelte_files:
        analyzer.analyze_file(file_path)

    # Генерируем отчет
    report = analyzer.generate_report()

    # Выводим результаты
    print(json.dumps(report, indent=2, ensure_ascii=False))

    # Сохраняем отчет
    with open('store-analysis-report.json', 'w', encoding='utf-8') as f:
        json.dump(report, f, indent=2, ensure_ascii=False)

if __name__ == '__main__':
    main()
