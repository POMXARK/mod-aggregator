/**
 * Тестовый скрипт для проверки команд импорта/экспорта файлов
 * 
 * Запуск:
 * 1. В Tauri: npm run tauri:dev, затем откройте DevTools и выполните команды
 * 2. В браузере: npm run dev, затем откройте консоль
 */

// Импортируем invoke (в реальном приложении это будет из tauri-wrapper)
// Для тестирования в консоли браузера используйте window.__TAURI__?.core.invoke

async function testImportExport() {
  console.log('🧪 Начало тестирования импорта/экспорта файлов\n');

  try {
    // Тест 1: Создание файла для экспорта
    console.log('📝 Тест 1: Создание файла для экспорта');
    const testFile = await invoke('create_file', {
      params: {
        name: 'export-test-mod',
        version: '1.0.0',
        path: '/path/to/export-test',
        metadata: { author: 'Test Author', description: 'Test mod for export' },
        dependencies: [{
          target_file_name: 'base-mod',
          target_file_version: '1.0.0',
          dependency_type: 'required'
        }]
      }
    });
    console.log('✅ Файл создан:', testFile);
    console.log('   ID:', testFile.id);
    console.log('   Name@Version:', `${testFile.name}@${testFile.version}\n`);

    // Сначала создадим базовый файл, если его нет
    console.log('📝 Создание базового файла для зависимости');
    try {
      const baseFile = await invoke('create_file', {
        params: {
          name: 'base-mod',
          version: '1.0.0',
          path: '/path/to/base',
          metadata: { author: 'Base Author' }
        }
      });
      console.log('✅ Базовый файл создан:', baseFile.id, '\n');
    } catch (e) {
      console.log('ℹ️  Базовый файл уже существует или ошибка:', e.message, '\n');
    }

    // Тест 2: Экспорт файла
    console.log('📤 Тест 2: Экспорт файла в JSON');
    const exportedJson = await invoke('export_file', {
      file_id: testFile.id
    });
    console.log('✅ Файл экспортирован');
    console.log('   JSON длина:', exportedJson.length, 'символов');
    console.log('   JSON превью:', exportedJson.substring(0, 200) + '...\n');

    // Парсим JSON для проверки
    const exportData = JSON.parse(exportedJson);
    console.log('   Версия схемы:', exportData.version);
    console.log('   Тип:', exportData.type);
    console.log('   Файл:', exportData.data.file.name + '@' + exportData.data.file.version);
    console.log('   Зависимостей:', exportData.data.dependencies.length, '\n');

    // Тест 3: Валидация JSON перед импортом
    console.log('✅ Тест 3: Валидация JSON перед импортом');
    const validation = await invoke('validate_import_json', {
      json_data: exportedJson
    });
    console.log('✅ Валидация завершена');
    console.log('   Валиден:', validation.valid);
    console.log('   Тип:', validation.type);
    console.log('   Версия:', validation.version);
    if (validation.preview) {
      console.log('   Превью:');
      console.log('     - Файлов:', validation.preview.files_count);
      console.log('     - Зависимостей:', validation.preview.dependencies_count);
    }
    if (validation.errors.length > 0) {
      console.log('   Ошибки:', validation.errors);
    }
    if (validation.warnings.length > 0) {
      console.log('   Предупреждения:', validation.warnings);
    }
    console.log();

    // Тест 4: Импорт файла (создание нового)
    console.log('📥 Тест 4: Импорт файла (создание нового)');
    
    // Модифицируем JSON для создания нового файла
    const importJson = JSON.parse(exportedJson);
    importJson.data.file.name = 'imported-mod';
    importJson.data.file.version = '2.0.0';
    importJson.data.file.id = 0; // Новый файл
    
    const importResult1 = await invoke('import_file', {
      json_data: JSON.stringify(importJson)
    });
    console.log('✅ Файл импортирован');
    console.log('   Создан:', importResult1.created);
    console.log('   ID:', importResult1.file_id);
    console.log('   Отсутствующих зависимостей:', importResult1.missing_dependencies.length);
    if (importResult1.missing_dependencies.length > 0) {
      importResult1.missing_dependencies.forEach(dep => {
        console.log(`     - ${dep.target_file_name}${dep.target_file_version ? '@' + dep.target_file_version : ''}`);
      });
    }
    if (importResult1.warnings.length > 0) {
      console.log('   Предупреждения:', importResult1.warnings);
    }
    console.log();

    // Тест 5: Импорт файла (обновление существующего)
    console.log('📥 Тест 5: Импорт файла (обновление существующего)');
    
    // Используем оригинальный JSON для обновления
    const importResult2 = await invoke('import_file', {
      json_data: exportedJson
    });
    console.log('✅ Файл импортирован');
    console.log('   Создан:', importResult2.created);
    console.log('   ID:', importResult2.file_id);
    console.log('   Отсутствующих зависимостей:', importResult2.missing_dependencies.length);
    console.log();

    // Тест 6: Валидация некорректного JSON
    console.log('❌ Тест 6: Валидация некорректного JSON');
    try {
      const invalidValidation = await invoke('validate_import_json', {
        json_data: '{"invalid": "json"}'
      });
      console.log('✅ Валидация обработана');
      console.log('   Валиден:', invalidValidation.valid);
      console.log('   Ошибки:', invalidValidation.errors);
    } catch (error) {
      console.log('✅ Правильно обработана ошибка:', error);
    }
    console.log();

    // Тест 7: Импорт с отсутствующими зависимостями
    console.log('⚠️  Тест 7: Импорт с отсутствующими зависимостями');
    const missingDepJson = JSON.parse(exportedJson);
    missingDepJson.data.dependencies.push({
      target_file_name: 'non-existent-mod',
      target_file_version: '1.0.0',
      dependency_type: 'required'
    });
    
    const importResult3 = await invoke('import_file', {
      json_data: JSON.stringify(missingDepJson)
    });
    console.log('✅ Импорт завершен с предупреждениями');
    console.log('   Отсутствующих зависимостей:', importResult3.missing_dependencies.length);
    importResult3.missing_dependencies.forEach(dep => {
      console.log(`     - ${dep.target_file_name}${dep.target_file_version ? '@' + dep.target_file_version : ''}`);
      if (dep.available_versions.length > 0) {
        console.log(`       Доступны версии: ${dep.available_versions.join(', ')}`);
      }
    });
    console.log();

    console.log('✅ Все тесты завершены успешно!');
    
    // Очистка тестовых данных (опционально)
    console.log('\n🧹 Очистка тестовых данных...');
    const testFiles = [
      { id: testFile.id, name: testFile.name },
      ...(importResult1.file_id ? [{ id: importResult1.file_id, name: 'imported-mod' }] : [])
    ];
    
    for (const file of testFiles) {
      try {
        await invoke('delete_file', { file_id: file.id, force: true });
        console.log(`   Удален: ${file.name} (ID: ${file.id})`);
      } catch (e) {
        console.log(`   Ошибка при удалении ${file.name}:`, e.message);
      }
    }
    console.log('✅ Очистка завершена');

  } catch (error) {
    console.error('❌ Ошибка при тестировании:', error);
    throw error;
  }
}

// Функция для использования в браузере (если доступен Tauri)
if (typeof window !== 'undefined' && window.__TAURI__) {
  const { invoke: tauriInvoke } = window.__TAURI__.core;
  window.testImportExport = async () => {
    await testImportExport.call({ invoke: tauriInvoke });
  };
  console.log('✅ Тестовая функция доступна: window.testImportExport()');
}

// Экспорт для использования в модулях
if (typeof module !== 'undefined' && module.exports) {
  module.exports = { testImportExport };
}



































