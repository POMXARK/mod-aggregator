#!/usr/bin/env node

/**
 * Скрипт валидации навыка Claude Code
 * Проверяет правильность структуры и формата SKILL.md файла
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Получить путь к проверяемому навыку из аргументов
const skillPath = process.argv[2];

if (!skillPath) {
  console.log('❌ Укажите путь к навыку:');
  console.log('node validate-skill.js /path/to/skill/directory');
  process.exit(1);
}

const skillDir = path.resolve(skillPath);
const skillMdPath = path.join(skillDir, 'SKILL.md');

console.log(`🔍 Проверка навыка: ${path.basename(skillDir)}`);
console.log(`📁 Путь: ${skillDir}`);
console.log('=' .repeat(50));

try {
  // Проверка существования файла
  if (!fs.existsSync(skillMdPath)) {
    console.log('❌ SKILL.md не найден');
    process.exit(1);
  }

  // Проверка директории навыка
  if (!fs.statSync(skillDir).isDirectory()) {
    console.log('❌ Указанный путь не является директорией');
    process.exit(1);
  }

  const content = fs.readFileSync(skillMdPath, 'utf8');

  // Проверка YAML frontmatter
  if (!content.startsWith('---')) {
    console.log('❌ Файл должен начинаться с ---');
    process.exit(1);
  }

  const frontmatterEnd = content.indexOf('---', 3);
  if (frontmatterEnd === -1) {
    console.log('❌ Не найден закрывающий ---');
    process.exit(1);
  }

  const frontmatter = content.substring(3, frontmatterEnd);
  const body = content.substring(frontmatterEnd + 3).trim();

  // Парсинг метаданных
  const nameMatch = frontmatter.match(/name:\s*(.+)/);
  const descriptionMatch = frontmatter.match(/description:\s*(.+)/);
  const versionMatch = frontmatter.match(/version:\s*(.+)/);

  let errors = [];
  let warnings = [];

  // Проверка обязательных полей
  if (!nameMatch) {
    errors.push('Отсутствует поле "name"');
  } else {
    const name = nameMatch[1].trim().replace(/^["']|["']$/g, '');
    console.log(`✅ name: ${name}`);

    // Проверка соответствия имени директории
    const dirName = path.basename(skillDir);
    if (name !== dirName) {
      warnings.push(`Имя навыка "${name}" не совпадает с именем директории "${dirName}"`);
    }
  }

  if (!descriptionMatch) {
    errors.push('Отсутствует поле "description"');
  } else {
    const description = descriptionMatch[1].trim().replace(/^["']|["']$/g, '');
    console.log(`✅ description: ${description}`);

    // Проверка качества описания
    if (description.length < 20) {
      warnings.push('Описание слишком короткое (минимум 20 символов)');
    }

    if (!description.includes('Использовать')) {
      warnings.push('Описание должно содержать слово "Использовать" для указания сценариев применения');
    }
  }

  if (!versionMatch) {
    warnings.push('Отсутствует поле "version"');
  } else {
    const version = versionMatch[1].trim().replace(/^["']|["']$/g, '');
    console.log(`✅ version: ${version}`);
  }

  console.log(`✅ Содержимое: ${body.length} символов`);

  // Дополнительные проверки
  if (body.length < 100) {
    warnings.push('Содержимое навыка слишком короткое (минимум 100 символов)');
  }

  if (!body.includes('##')) {
    warnings.push('Рекомендуется использовать заголовки для структурирования содержимого');
  }

  // Вывод результатов
  console.log('=' .repeat(50));

  if (errors.length > 0) {
    console.log('❌ ОШИБКИ:');
    errors.forEach(error => console.log(`  - ${error}`));
    console.log('');
    process.exit(1);
  }

  if (warnings.length > 0) {
    console.log('⚠️  ПРЕДУПРЕЖДЕНИЯ:');
    warnings.forEach(warning => console.log(`  - ${warning}`));
    console.log('');
  }

  if (errors.length === 0) {
    console.log('🎉 Навык прошел валидацию!');
    console.log('');
    console.log('📋 Следующие шаги:');
    console.log('1. Протестируйте навык в Claude Code');
    console.log('2. Проверьте активацию командой "What Skills are available?"');
    console.log('3. Попробуйте запросы из описания навыка');
  }

} catch (error) {
  console.log(`❌ Ошибка при проверке: ${error.message}`);
  process.exit(1);
}