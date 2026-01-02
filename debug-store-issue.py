#!/usr/bin/env python3

import re

# Читаем файл ParserWorkspace.svelte
with open('src/components/Parser/UI/ParserBuilder/components/ParserWorkspace.svelte', 'r', encoding='utf-8') as f:
    content = f.read()

print("=== Анализ файла ParserWorkspace.svelte ===")

# Ищем импорты stores
store_imports = {}
matches = re.finditer(r'const\s*{\s*([^}]+)\s*}\s*=\s*(\w+)\(\)', content)
for match in matches:
    destructured = match.group(1)
    composable = match.group(2)
    print(f"Найден composable: {composable}")
    print(f"Destructured: {destructured}")

    vars_list = [var.strip().split(':')[0].strip() for var in destructured.split(',')]
    for var in vars_list:
        if 'Store' in var:
            store_imports[var] = composable
            print(f"  Store: {var} из {composable}")

print(f"\nНайденные stores: {store_imports}")

# Ищем объявления переменных
declarations = {}
matches = re.finditer(r'\b(?:let|const)\s+(\w+)\s*:\s*([^;=]+)', content)
for match in matches:
    var_name = match.group(1)
    var_type = match.group(2).strip()
    declarations[var_name] = var_type
    print(f"Объявление: {var_name}: {var_type}")

print(f"\nОбъявления: {declarations}")

# Ищем $effect блоки
effect_matches = re.finditer(r'\$effect\s*\(\s*\(\)\s*=>\s*{([^}]*(?:\{[^}]*\}[^}]*)*)\}', content, re.DOTALL)

for i, effect_match in enumerate(effect_matches):
    effect_content = effect_match.group(1)
    print(f"\n$effect {i+1}: {effect_content.strip()[:100]}...")

    # Ищем присваивания
    assignments = re.finditer(r'(\w+)\s*=\s*\$(\w+)', effect_content)
    for assignment in assignments:
        var_name = assignment.group(1)
        store_ref = assignment.group(2)
        print(f"  Присваивание: {var_name} = ${store_ref}")

        if var_name in declarations:
            var_type = declarations[var_name]
            print(f"    Тип переменной: {var_type}")
            print(f"    Store в импортах: {store_ref in store_imports}")

            if store_ref in store_imports:
                print(f"    ❌ ПРОБЛЕМА НАЙДЕНА: {var_name} = ${store_ref}")
                print(f"    Переменная типа {var_type}, но присваивается ${store_ref}")
                print("    Нужно заменить на: {var_name} = $derived({store_ref})"
