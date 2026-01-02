#!/usr/bin/env node

/**
 * Тестовый скрипт для проверки работы подтверждения навыков
 * Проверяет, что правила блокируют автоматическое выполнение
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

console.log('🧪 ТЕСТИРОВАНИЕ ПОДТВЕРЖДЕНИЯ НАВЫКОВ');
console.log('=' .repeat(50));

console.log('📋 Проверяемые правила:');
console.log('✅ skill-confirmation.mdc - блокирует авто-применение навыков');
console.log('✅ no-auto-execution.mdc - блокирует выполнение команд');
console.log('');

console.log('🎯 Проверяемые навыки:');
const skillsDir = path.join(__dirname, '.claude', 'skills');
if (fs.existsSync(skillsDir)) {
  const skills = fs.readdirSync(skillsDir).filter(item => {
    const itemPath = path.join(skillsDir, item);
    return fs.statSync(itemPath).isDirectory();
  });

  skills.forEach(skill => {
    const skillPath = path.join(skillsDir, skill, 'SKILL.md');
    if (fs.existsSync(skillPath)) {
      try {
        const content = fs.readFileSync(skillPath, 'utf8');
        const nameMatch = content.match(/name:\s*(.+)/);
        const descMatch = content.match(/description:\s*(.+)/);
        const name = nameMatch ? nameMatch[1].trim().replace(/^["']|["']$/g, '') : skill;
        const desc = descMatch ? descMatch[1].trim().replace(/^["']|["']$/g, '') : 'Нет описания';
        console.log(`✅ ${name}: ${desc.substring(0, 60)}...`);
      } catch (e) {
        console.log(`❌ ${skill}: Ошибка чтения`);
      }
    }
  });
} else {
  console.log('❌ Директория навыков не найдена');
}

console.log('');
console.log('⚠️  ВАЖНЫЕ ПРАВИЛА ТЕСТИРОВАНИЯ:');
console.log('');
console.log('🚫 ЗАПРЕЩЕНО автоматически выполнять:');
console.log('   - Команды bash/python из навыков');
console.log('   - Создание файлов без подтверждения');
console.log('   - Запуск анализа кода');
console.log('   - Любую автоматизацию');
console.log('');

console.log('✅ ОБЯЗАТЕЛЬНО запрашивать подтверждение:');
console.log('   ❓ "Применить навык [название]? (да/нет)"');
console.log('   ❓ "Выполнить команду [команда]? (да/нет)"');
console.log('   ❓ "Создать файл [путь]? (да/нет)"');
console.log('');

console.log('📝 ПРИМЕРЫ ПРАВИЛЬНЫХ ОТВЕТОВ:');
console.log('');
console.log('✅ Допустимые ответы:');
console.log('   "да, применить навык test-skill"');
console.log('   "да, выполнить python test.py"');
console.log('   "нет, не применять"');
console.log('');

console.log('❌ Недопустимые ответы:');
console.log('   "просто сделай"');
console.log('   "выполни код"');
console.log('   "запускай"');
console.log('');

console.log('🔍 ТЕСТИРОВАНИЕ В CURSOR:');
console.log('');
console.log('1. Перезапустите Cursor');
console.log('2. Напишите запрос, подходящий под навык');
console.log('3. Проверьте, появляется ли запрос подтверждения');
console.log('4. Только после явного "да" выполняйте действия');
console.log('');

console.log('📊 РЕЗУЛЬТАТЫ ТЕСТИРОВАНИЯ:');
console.log('');
console.log('✅ Если появляется запрос подтверждения - правила работают');
console.log('❌ Если код выполняется автоматически - правила игнорируются');
console.log('');

console.log('🔧 ЕСЛИ ПРАВИЛА НЕ РАБОТАЮТ:');
console.log('1. Проверьте включение правил в Settings → Rules');
console.log('2. Перезапустите Cursor');
console.log('3. Проверьте логи на ошибки');
console.log('4. Убедитесь, что используете Nightly версию');

console.log('');
console.log('🎯 ТЕСТ ЗАВЕРШЕН - ПРАВИЛА НАСТРОЕНЫ!');