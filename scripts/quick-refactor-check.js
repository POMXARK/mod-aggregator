#!/usr/bin/env node

/**
 * Быстрая проверка кандидатов на рефакторинг
 * Показывает только самые проблемные файлы
 * Использование: node scripts/quick-refactor-check.js
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function countLines(filePath) {
  try {
    const content = fs.readFileSync(filePath, 'utf8');
    return content.split('\n').length;
  } catch (error) {
    return 0;
  }
}

function walkDirectory(dir, extensions, excludeDirs) {
  const results = [];

  function walk(currentPath) {
    const items = fs.readdirSync(currentPath);

    for (const item of items) {
      const fullPath = path.join(currentPath, item);
      const stat = fs.statSync(fullPath);

      if (stat.isDirectory()) {
        if (!excludeDirs.some(exclude => fullPath.includes(exclude))) {
          walk(fullPath);
        }
      } else if (stat.isFile()) {
        const ext = path.extname(item);
        if (extensions.includes(ext)) {
          const lines = countLines(fullPath);
          results.push({
            file: path.relative(process.cwd(), fullPath),
            lines: lines,
            extension: ext
          });
        }
      }
    }
  }

  walk(dir);
  return results;
}

function formatNumber(num) {
  return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',');
}

// Основная логика
console.log('🚀 Быстрая проверка кандидатов на рефакторинг\n');

const extensions = ['.rs', '.ts', '.tsx', '.svelte', '.js', '.jsx'];
const excludeDirs = ['node_modules', 'target', '.cursor', '.claude', '.git', '.storybook', 'dist'];

const files = walkDirectory('.', extensions, excludeDirs);
const sortedFiles = files.sort((a, b) => b.lines - a.lines);

// Анализ по категориям
const critical = sortedFiles.filter(f => f.lines > 2000);
const high = sortedFiles.filter(f => f.lines > 1000 && f.lines <= 2000);
const medium = sortedFiles.filter(f => f.lines > 500 && f.lines <= 1000);

// Вывод результатов
if (critical.length > 0) {
  console.log('🚨 КРИТИЧЕСКИЕ ФАЙЛЫ (>2000 строк):');
  critical.forEach((file, i) => {
    console.log(`  ${i + 1}. ${file.file} - ${formatNumber(file.lines)} строк`);
  });
  console.log('');
}

if (high.length > 0) {
  console.log('⚠️ ВЫСОКИЙ ПРИОРИТЕТ (1000-2000 строк):');
  high.forEach((file, i) => {
    console.log(`  ${i + 1}. ${file.file} - ${formatNumber(file.lines)} строк`);
  });
  console.log('');
}

console.log('📊 Краткая статистика:');
console.log(`  • Критические файлы: ${critical.length}`);
console.log(`  • Высокий приоритет: ${high.length}`);
console.log(`  • Средний приоритет: ${medium.length}`);
console.log(`  • Всего файлов: ${files.length}`);

const totalLines = files.reduce((sum, file) => sum + file.lines, 0);
console.log(`  • Всего строк кода: ${formatNumber(totalLines)}`);

if (critical.length > 0 || high.length > 0) {
  console.log('\n💡 Для детального анализа выполните: npm run analyze-refactoring');
}