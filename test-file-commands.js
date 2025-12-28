/**
 * Тестовый скрипт для проверки команд управления файлами
 * 
 * Запуск:
 * 1. В браузере: откройте консоль и выполните команды вручную
 * 2. В Tauri: npm run tauri:dev, затем откройте DevTools и выполните команды
 * 
 * Или используйте этот скрипт как основу для unit-тестов
 */

// Импортируем invoke (в реальном приложении это будет из tauri-wrapper)
// Для тестирования в консоли браузера используйте window.__TAURI__?.core.invoke

async function testFileCommands() {
  console.log('🧪 Начало тестирования команд управления файлами\n');

  try {
    // Тест 1: Создание файла без зависимостей
    console.log('📝 Тест 1: Создание файла без зависимостей');
    const file1 = await invoke('create_file', {
      params: {
        name: 'test-mod',
        version: '1.0.0',
        path: '/path/to/test-mod',
        metadata: { author: 'Test Author', description: 'Test mod' }
      }
    });
    console.log('✅ Файл создан:', file1);
    console.log('   ID:', file1.id);
    console.log('   Name@Version:', `${file1.name}@${file1.version}\n`);

    // Тест 2: Создание файла с зависимостью
    console.log('📝 Тест 2: Создание файла с зависимостью');
    const file2 = await invoke('create_file', {
      params: {
        name: 'dependent-mod',
        version: '1.0.0',
        dependencies: [{
          target_file_name: 'test-mod',
          target_file_version: '1.0.0',
          dependency_type: 'required'
        }]
      }
    });
    console.log('✅ Файл с зависимостью создан:', file2);
    console.log('   ID:', file2.id);
    console.log('   Name@Version:', `${file2.name}@${file2.version}\n`);

    // Тест 3: Получение файла по name@version
    console.log('🔍 Тест 3: Получение файла по name@version');
    const retrievedFile = await invoke('get_file_by_name_version', {
      name: 'test-mod',
      version: '1.0.0'
    });
    console.log('✅ Файл найден:', retrievedFile);
    console.log('   Path:', retrievedFile.path);
    console.log('   Metadata:', retrievedFile.metadata, '\n');

    // Тест 4: Получение всех версий файла
    console.log('📋 Тест 4: Получение всех версий файла');
    const versions = await invoke('get_file_versions', { name: 'test-mod' });
    console.log('✅ Найдено версий:', versions.length);
    versions.forEach(v => {
      console.log(`   - ${v.name}@${v.version} (ID: ${v.id})`);
    });
    console.log();

    // Тест 5: Обновление файла
    console.log('✏️  Тест 5: Обновление файла');
    const updatedFile = await invoke('update_file', {
      params: {
        id: file1.id,
        path: '/new/path/to/test-mod',
        metadata: { author: 'Updated Author', description: 'Updated mod', version: '1.0.1' }
      }
    });
    console.log('✅ Файл обновлен:', updatedFile);
    console.log('   Новый путь:', updatedFile.path);
    console.log('   Новые метаданные:', updatedFile.metadata, '\n');

    // Тест 6: Загрузка новой версии файла
    console.log('📤 Тест 6: Загрузка новой версии файла');
    const newVersion = await invoke('upload_file_version', {
      params: {
        name: 'test-mod',
        version: '2.0.0',
        path: '/path/to/test-mod-v2',
        metadata: { author: 'Test Author', description: 'Version 2.0' }
      }
    });
    console.log('✅ Новая версия загружена:', newVersion);
    console.log('   Name@Version:', `${newVersion.name}@${newVersion.version}\n`);

    // Тест 7: Проверка зависимостей перед удалением
    console.log('⚠️  Тест 7: Попытка удаления файла с зависимостями (без force)');
    const deleteResult1 = await invoke('delete_file', {
      file_id: file1.id,
      force: false
    });
    console.log('✅ Результат удаления:', deleteResult1);
    console.log('   Удален:', deleteResult1.deleted);
    console.log('   Сообщение:', deleteResult1.message);
    if (deleteResult1.dependent_files) {
      console.log('   Зависимые файлы:', deleteResult1.dependent_files.length);
    }
    console.log();

    // Тест 8: Удаление файла с force=true
    console.log('🗑️  Тест 8: Удаление файла с force=true');
    const deleteResult2 = await invoke('delete_file', {
      file_id: file2.id,
      force: true
    });
    console.log('✅ Файл удален:', deleteResult2.deleted);
    console.log('   Сообщение:', deleteResult2.message, '\n');

    // Тест 9: Проверка валидации - дубликат name@version
    console.log('❌ Тест 9: Попытка создать дубликат name@version');
    try {
      await invoke('create_file', {
        params: {
          name: 'test-mod',
          version: '1.0.0', // Уже существует
        }
      });
      console.log('❌ ОШИБКА: Должна была быть ошибка о дубликате!');
    } catch (error) {
      console.log('✅ Правильно обработана ошибка дубликата:', error);
    }
    console.log();

    // Тест 10: Проверка валидации - пустое имя
    console.log('❌ Тест 10: Попытка создать файл с пустым именем');
    try {
      await invoke('create_file', {
        params: {
          name: '',
          version: '1.0.0',
        }
      });
      console.log('❌ ОШИБКА: Должна была быть ошибка валидации!');
    } catch (error) {
      console.log('✅ Правильно обработана ошибка валидации:', error);
    }
    console.log();

    // Тест 11: Получение всех файлов
    console.log('📚 Тест 11: Получение всех файлов');
    const allFiles = await invoke('get_file_versions', { name: '' });
    console.log('✅ Всего файлов в системе:', allFiles.length);
    allFiles.forEach(f => {
      console.log(`   - ${f.name}@${f.version} (ID: ${f.id})`);
    });
    console.log();

    console.log('✅ Все тесты завершены успешно!');
    
    // Очистка тестовых данных (опционально)
    console.log('\n🧹 Очистка тестовых данных...');
    const testFiles = allFiles.filter(f => 
      f.name === 'test-mod' || f.name === 'dependent-mod'
    );
    for (const file of testFiles) {
      try {
        await invoke('delete_file', { file_id: file.id, force: true });
        console.log(`   Удален: ${file.name}@${file.version}`);
      } catch (e) {
        console.log(`   Ошибка при удалении ${file.name}@${file.version}:`, e);
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
  window.testFileCommands = async () => {
    await testFileCommands.call({ invoke: tauriInvoke });
  };
  console.log('✅ Тестовая функция доступна: window.testFileCommands()');
}

// Экспорт для использования в модулях
if (typeof module !== 'undefined' && module.exports) {
  module.exports = { testFileCommands };
}
























