#!/usr/bin/env node

/**
 * Скрипт валидации навыков Claude Code
 * Проверяет правильность структуры и формата навыков в .claude/skills/
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const skillsDir = path.join(__dirname, '.claude', 'skills');

function validateSkill(skillPath) {
  const skillDir = path.dirname(skillPath);
  const skillName = path.basename(skillDir);

  console.log(`🔍 Проверка навыка: ${skillName}`);
  console.log(`📁 Путь: ${path.relative(__dirname, skillPath)}`);

  try {
    const content = fs.readFileSync(skillPath, 'utf8');

    // Проверка YAML frontmatter
    if (!content.startsWith('---')) {
      console.log('❌ ОШИБКА: Файл должен начинаться с ---');
      return false;
    }

    // Извлечение frontmatter
    const frontmatterEnd = content.indexOf('---', 3);
    if (frontmatterEnd === -1) {
      console.log('❌ ОШИБКА: Не найден закрывающий ---');
      return false;
    }

    const frontmatter = content.substring(3, frontmatterEnd);
    const body = content.substring(frontmatterEnd + 3).trim();

    // Парсинг метаданных
    const nameMatch = frontmatter.match(/name:\s*(.+)/);
    const descriptionMatch = frontmatter.match(/description:\s*(.+)/);
    const versionMatch = frontmatter.match(/version:\s*(.+)/);

    if (!nameMatch) {
      console.log('❌ ОШИБКА: Отсутствует поле name');
      return false;
    }

    if (!descriptionMatch) {
      console.log('❌ ОШИБКА: Отсутствует поле description');
      return false;
    }

    if (!versionMatch) {
      console.log('⚠️  ПРЕДУПРЕЖДЕНИЕ: Отсутствует поле version');
    }

    const name = nameMatch[1].trim().replace(/^["']|["']$/g, '');
    const description = descriptionMatch[1].trim().replace(/^["']|["']$/g, '');
    const version = versionMatch ? versionMatch[1].trim().replace(/^["']|["']$/g, '') : 'не указана';

    console.log(`✅ name: ${name}`);
    console.log(`✅ description: ${description}`);
    console.log(`✅ version: ${version}`);
    console.log(`✅ Содержимое: ${body.length} символов`);

    // Проверка соответствия имени
    if (name !== skillName) {
      console.log(`⚠️  Несоответствие имени: "${name}" vs "${skillName}"`);
    }

    return true;

  } catch (error) {
    console.log(`❌ ОШИБКА чтения: ${error.message}`);
    return false;
  }
}

function main() {
  console.log('🚀 Валидация навыков Claude Code');
  console.log('=' .repeat(50));
  console.log(`Директория: ${path.relative(__dirname, skillsDir)}`);
  console.log('=' .repeat(50));

  if (!fs.existsSync(skillsDir)) {
    console.log(`❌ Директория .claude/skills не найдена: ${skillsDir}`);
    process.exit(1);
  }

  const skillDirs = fs.readdirSync(skillsDir)
    .filter(item => {
      const itemPath = path.join(skillsDir, item);
      return fs.statSync(itemPath).isDirectory();
    });

  console.log(`📂 Найдено навыков: ${skillDirs.length}`);
  console.log('');

  let validSkills = 0;
  let invalidSkills = 0;

  for (const skillDir of skillDirs) {
    const skillPath = path.join(skillsDir, skillDir, 'SKILL.md');

    if (!fs.existsSync(skillPath)) {
      console.log(`❌ SKILL.md не найден в ${skillDir}`);
      invalidSkills++;
      continue;
    }

    if (validateSkill(skillPath)) {
      validSkills++;
    } else {
      invalidSkills++;
    }
    console.log('');
  }

  console.log('=' .repeat(50));
  console.log('📊 Результаты валидации:');
  console.log(`✅ Корректных навыков: ${validSkills}`);
  console.log(`❌ Некорректных навыков: ${invalidSkills}`);

  if (invalidSkills > 0) {
    console.log('\n🔧 Исправьте ошибки перед использованием в Claude Code');
    process.exit(1);
  } else {
    console.log('\n🎉 Все навыки корректны!');
    console.log('📋 Для тестирования в Claude Code:');
    console.log('   1. Перезапустите Claude Code/Cursor');
    console.log('   2. Спросите: "What Skills are available?"');
    console.log('   3. Проверьте автоматическую активацию навыков');
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  main();
}