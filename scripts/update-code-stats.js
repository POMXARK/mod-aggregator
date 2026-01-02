#!/usr/bin/env node

/**
 * Скрипт для автоматического обновления статистики строк кода в README.md
 * Использование: node scripts/update-code-stats.js
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

function analyzeRefactoringCandidates(files) {
  const candidates = {
    critical: [], // > 2000 строк - срочно рефакторить
    high: [],     // 1000-2000 строк - рекомендуется рефакторить
    medium: [],   // 500-1000 строк - рассмотреть рефакторинг
    low: []       // 200-500 строк - мониторить
  };

  const byLanguage = {};

  files.forEach(file => {
    // Категоризация по размеру
    if (file.lines > 2000) {
      candidates.critical.push(file);
    } else if (file.lines > 1000) {
      candidates.high.push(file);
    } else if (file.lines > 500) {
      candidates.medium.push(file);
    } else if (file.lines > 200) {
      candidates.low.push(file);
    }

    // Статистика по языкам
    if (!byLanguage[file.extension]) {
      byLanguage[file.extension] = { files: [], totalLines: 0 };
    }
    byLanguage[file.extension].files.push(file);
    byLanguage[file.extension].totalLines += file.lines;
  });

  // Сортировка кандидатов по размеру
  Object.keys(candidates).forEach(key => {
    candidates[key].sort((a, b) => b.lines - a.lines);
  });

  return { candidates, byLanguage };
}

function generateRefactoringReport(candidates, byLanguage) {
  let report = '# 📊 Анализ кандидатов на рефакторинг\n\n';
  report += `**Дата генерации:** ${new Date().toLocaleDateString('ru-RU')}\n\n`;

  // Критические кандидаты
  if (candidates.critical.length > 0) {
    report += '## 🚨 КРИТИЧЕСКИЕ КАНДИДАТЫ (>2000 строк)\n\n';
    report += 'Эти файлы требуют **немедленного рефакторинга**:\n\n';
    candidates.critical.forEach((file, i) => {
      report += `${i + 1}. **${file.file}** - ${formatNumber(file.lines)} строк\n`;
    });
    report += '\n**Рекомендации:**\n';
    report += '- Разбить на несколько модулей/компонентов\n';
    report += '- Вынести общую логику в отдельные функции\n';
    report += '- Рассмотреть паттерн "Extract Class" или "Extract Module"\n\n';
  }

  // Высокий приоритет
  if (candidates.high.length > 0) {
    report += '## ⚠️ ВЫСОКИЙ ПРИОРИТЕТ (1000-2000 строк)\n\n';
    report += 'Рекомендуется рефакторинг в ближайшее время:\n\n';
    candidates.high.slice(0, 10).forEach((file, i) => {
      report += `${i + 1}. **${file.file}** - ${formatNumber(file.lines)} строк\n`;
    });
    report += '\n**Рекомендации:**\n';
    report += '- Вынести части логики в отдельные функции\n';
    report += '- Рассмотреть разделение на подкомпоненты\n';
    report += '- Проверить на наличие дублированного кода\n\n';
  }

  // Средний приоритет
  if (candidates.medium.length > 0) {
    report += '## 📋 СРЕДНИЙ ПРИОРИТЕТ (500-1000 строк)\n\n';
    report += 'Рассмотреть рефакторинг при следующей модификации:\n\n';
    candidates.medium.slice(0, 15).forEach((file, i) => {
      report += `${i + 1}. **${file.file}** - ${formatNumber(file.lines)} строк\n`;
    });
    report += '\n**Рекомендации:**\n';
    report += '- Мониторить рост файла\n';
    report += '- Рассмотреть извлечение утилитарных функций\n';
    report += '- Проверить single responsibility principle\n\n';
  }

  // Статистика по языкам
  report += '## 📈 Статистика по языкам\n\n';
  Object.entries(byLanguage)
    .sort(([,a], [,b]) => b.totalLines - a.totalLines)
    .forEach(([ext, data]) => {
      const langName = ext === '.rs' ? 'Rust' :
                      ext === '.ts' ? 'TypeScript' :
                      ext === '.tsx' ? 'TSX' :
                      ext === '.svelte' ? 'Svelte' :
                      ext === '.js' ? 'JavaScript' : ext;

      const avgSize = Math.round(data.totalLines / data.files.length);
      const largeFiles = data.files.filter(f => f.lines > 500).length;

      report += `### ${langName}\n`;
      report += `- **Файлов:** ${data.files.length}\n`;
      report += `- **Всего строк:** ${formatNumber(data.totalLines)}\n`;
      report += `- **Средний размер:** ${avgSize} строк\n`;
      report += `- **Крупных файлов (>500 строк):** ${largeFiles}\n\n`;

      if (largeFiles > 0) {
        report += '**Крупные файлы:**\n';
        data.files.filter(f => f.lines > 500).slice(0, 5).forEach(file => {
          report += `- ${file.file} (${formatNumber(file.lines)} строк)\n`;
        });
        report += '\n';
      }
    });

  return report;
}

function updateReadme(stats, topFiles, totalFiles, totalLines, refactoringReport) {
  const readmePath = path.join(process.cwd(), 'README.md');
  let readme = fs.readFileSync(readmePath, 'utf8');

  // Обновляем общую статистику
  const totalStatsRegex = /- \*\*Всего файлов\*\*: \d+([\d,]*)\n- \*\*Всего строк кода\*\*: [\d,]+/;
  const newTotalStats = `- **Всего файлов**: ${totalFiles}\n- **Всего строк кода**: ${formatNumber(totalLines)}`;
  readme = readme.replace(totalStatsRegex, newTotalStats);

  // Обновляем распределение по языкам
  const langStats = {};
  stats.forEach(file => {
    langStats[file.extension] = (langStats[file.extension] || 0) + file.lines;
  });

  const sortedLangs = Object.entries(langStats)
    .sort(([,a], [,b]) => b - a)
    .map(([ext, lines]) => {
      const langName = ext === '.rs' ? 'Rust' :
                      ext === '.ts' ? 'TypeScript' :
                      ext === '.tsx' ? 'TSX' :
                      ext === '.svelte' ? 'Svelte' :
                      ext === '.js' ? 'JavaScript' : ext;
      return `| ${langName} | ${formatNumber(lines)} | ${(lines / totalLines * 100).toFixed(1)}% |`;
    });

  const tableContent = sortedLangs.join('\n');
  const langTableRegex = /\| Язык \| Строк кода \| Процент \|\n\|------\|------------\|---------\|\n([\s\S]*?)(?=\n\n### Крупнейшие файлы)/;
  readme = readme.replace(langTableRegex, `| Язык | Строк кода | Процент |\n|------|------------|---------|\n${tableContent}`);

  // Обновляем таблицу крупнейших файлов
  const topFilesTable = topFiles.slice(0, 6).map(file => {
    const langName = file.extension === '.rs' ? 'Rust' :
                    file.extension === '.ts' ? 'TypeScript' :
                    file.extension === '.tsx' ? 'TSX' :
                    file.extension === '.svelte' ? 'Svelte' :
                    file.extension === '.js' ? 'JavaScript' : file.extension;
    return `| \`${file.file.replace(/\\/g, '/')}\` | ${formatNumber(file.lines)} | ${langName} |`;
  }).join('\n');

  const topFilesRegex = /\| Файл \| Строк \| Язык \|\n\|------\|-------\|------\|\n([\s\S]*?)(?=\n\n\*Статистика обновлена автоматически)/;
  readme = readme.replace(topFilesRegex, `| Файл | Строк | Язык |\n|------|-------|------|\n${topFilesTable}`);

  // Обновляем дату
  const today = new Date().toISOString().split('T')[0];
  const dateRegex = /\*Статистика обновлена автоматически\. Последнее обновление: .*?\*/;
  readme = readme.replace(dateRegex, `*Статистика обновлена автоматически. Последнее обновление: ${today}*`);

  fs.writeFileSync(readmePath, readme, 'utf8');
  console.log('✅ README.md успешно обновлен со свежей статистикой кода!');

  // Сохраняем отчет по рефакторингу
  const reportPath = path.join(process.cwd(), 'REFACTORING-ANALYSIS.md');
  fs.writeFileSync(reportPath, refactoringReport, 'utf8');
  console.log('📋 Отчет по рефакторингу сохранен в REFACTORING-ANALYSIS.md');
}

// Основная логика
console.log('🔍 Анализ строк кода в проекте...\n');

const extensions = ['.rs', '.ts', '.tsx', '.svelte', '.js', '.jsx'];
const excludeDirs = ['node_modules', 'target', '.cursor', '.claude', '.git', '.storybook', 'dist'];

const files = walkDirectory('.', extensions, excludeDirs);
const sortedFiles = files.sort((a, b) => b.lines - a.lines);
const totalLines = files.reduce((sum, file) => sum + file.lines, 0);
const totalFiles = files.length;

// Анализ кандидатов на рефакторинг
console.log('🔧 Анализ кандидатов на рефакторинг...\n');
const { candidates, byLanguage } = analyzeRefactoringCandidates(files);

// Генерация отчета по рефакторингу
const refactoringReport = generateRefactoringReport(candidates, byLanguage);

// Вывод краткой информации в консоль
console.log('📋 Краткий анализ рефакторинга:');
console.log(`   🚨 Критические файлы (>2000 строк): ${candidates.critical.length}`);
console.log(`   ⚠️  Высокий приоритет (1000-2000 строк): ${candidates.high.length}`);
console.log(`   📋 Средний приоритет (500-1000 строк): ${candidates.medium.length}`);
console.log(`   👁️  Наблюдение (200-500 строк): ${candidates.low.length}`);
console.log('');

// Обновляем README и сохраняем отчет
updateReadme(files, sortedFiles, totalFiles, totalLines, refactoringReport);

console.log(`📊 Обработано ${totalFiles} файлов, ${formatNumber(totalLines)} строк кода`);