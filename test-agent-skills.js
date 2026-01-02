// Тест для проверки работы Agent Skills
// Запустите этот файл для проверки статуса навыков

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

console.log('🔍 Проверка Agent Skills...\n');

// Проверяем наличие папки skills
const skillsDir = path.join(__dirname, 'skills');
if (fs.existsSync(skillsDir)) {
    console.log('✅ Папка skills найдена');

    const skillFiles = fs.readdirSync(skillsDir);
    console.log(`📁 Найдено навыков: ${skillFiles.length}`);

    skillFiles.forEach(file => {
        console.log(`  - ${file}`);
    });
} else {
    console.log('❌ Папка skills не найдена');
}

// Проверяем наличие папки .cursor/rules
const rulesDir = path.join(__dirname, '.cursor', 'rules');
if (fs.existsSync(rulesDir)) {
    console.log('✅ Папка .cursor/rules найдена');

    const ruleFiles = fs.readdirSync(rulesDir);
    console.log(`📁 Найдено правил: ${ruleFiles.length}`);

    ruleFiles.forEach(file => {
        console.log(`  - ${file}`);
    });
} else {
    console.log('❌ Папка .cursor/rules не найдена');
}

console.log('\n💡 Рекомендации:');
console.log('1. Убедитесь, что используете Cursor версии 2.3.x или новее');
console.log('2. Проверьте настройки в: Cursor Settings → AI → Agent Skills');
console.log('3. Если настройка не найдена, попробуйте перезапустить Cursor');
console.log('4. Проверьте раздел Import Settings в Rules');