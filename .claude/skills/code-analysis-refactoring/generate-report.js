#!/usr/bin/env node

/**
 * Вспомогательный скрипт для генерации отчетов анализа рефакторинга
 * Используется навыком code-analysis-refactoring
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
    try {
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
            results.push({ file: path.relative(process.cwd(), fullPath), lines, extension: ext });
          }
        }
      }
    } catch (error) {
      // Игнорируем ошибки доступа к директориям
    }
  }
  walk(dir);
  return results;
}

// Основная логика анализа
console.log('🔍 Запуск анализа кода для рефакторинга...\n');

const extensions = ['.rs', '.ts', '.tsx', '.svelte', '.js', '.jsx'];
const excludeDirs = ['node_modules', 'target', '.cursor', '.claude', '.git', '.storybook', 'dist'];

const files = walkDirectory('.', extensions, excludeDirs);
const sortedFiles = files.sort((a, b) => b.lines - a.lines);

const critical = sortedFiles.filter(f => f.lines > 2000);
const high = sortedFiles.filter(f => f.lines > 1000 && f.lines <= 2000);
const medium = sortedFiles.filter(f => f.lines > 500 && f.lines <= 1000);

console.log('📋 РЕЗУЛЬТАТЫ АНАЛИЗА КОДА:');
console.log('='.repeat(50));
console.log(`🚨 Критические файлы (>2000 строк): ${critical.length}`);
console.log(`⚠️ Высокий приоритет (1000-2000 строк): ${high.length}`);
console.log(`📋 Средний приоритет (500-1000 строк): ${medium.length}`);
console.log(`📊 Всего файлов: ${files.length}`);

if (critical.length > 0) {
  console.log('\n🚨 КРИТИЧЕСКИЕ КАНДИДАТЫ НА РЕФАКТОРИНГ:');
  critical.forEach((file, i) => {
    console.log(`  ${i + 1}. ${file.file} (${file.lines} строк)`);
  });
}

if (high.length > 0) {
  console.log('\n⚠️ ВЫСОКИЙ ПРИОРИТЕТ:');
  high.slice(0, 5).forEach((file, i) => {
    console.log(`  ${i + 1}. ${file.file} (${file.lines} строк)`);
  });
}

const totalLines = files.reduce((sum, file) => sum + file.lines, 0);
console.log(`\n📈 ОБЩАЯ СТАТИСТИКА:`);
console.log(`  Всего строк кода: ${totalLines.toLocaleString()}`);
console.log(`  Средний размер файла: ${Math.round(totalLines / files.length)} строк`);

// Генерируем отчет
const report = `# 📊 Анализ кандидатов на рефакторинг

**Дата генерации:** ${new Date().toLocaleDateString('ru-RU')}

## 🚨 Критические кандидаты (>2000 строк)

${critical.length > 0 ? critical.map((f, i) => `${i + 1}. **${f.file}** - ${f.lines} строк`).join('\n') : 'Критических файлов не найдено'}

## ⚠️ Высокий приоритет (1000-2000 строк)

${high.length > 0 ? high.slice(0, 10).map((f, i) => `${i + 1}. **${f.file}** - ${f.lines} строк`).join('\n') : 'Файлов высокого приоритета не найдено'}

## 📊 Общая статистика

- **Всего файлов:** ${files.length}
- **Всего строк кода:** ${totalLines.toLocaleString()}
- **Средний размер файла:** ${Math.round(totalLines / files.length)} строк

## 📈 Распределение по языкам

${Object.entries(files.reduce((acc, file) => {
  acc[file.extension] = (acc[file.extension] || 0) + file.lines;
  return acc;
}, {})).sort(([,a], [,b]) => b - a).map(([ext, lines]) => `- ${ext}: ${lines.toLocaleString()} строк (${((lines/totalLines)*100).toFixed(1)}%)`).join('\n')}
`;

fs.writeFileSync('REFACTORING-ANALYSIS.md', report);
console.log('\n📋 Отчет сохранен в REFACTORING-ANALYSIS.md');
console.log('✅ Анализ завершен!');