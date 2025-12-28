// Скрипт для тестирования компонентов в браузерной консоли
// Использование: скопируйте и вставьте в консоль браузера

console.log('🧪 Тестирование компонентов Mod Aggregator');
console.log('=====================================');

// Проверяем доступность функций
async function testComponents() {
  try {
    // Импортируем helpers
    const helpers = await import('./config/development-helpers.js');

    console.log('✅ Development helpers загружены');

    // Проверяем текущее состояние
    helpers.getCurrentConfig();

    // Тестируем переключение профилей
    console.log('\n🔄 Тестирование профилей...');

    // Включаем только Parser
    console.log('Включаем только Parser...');
    helpers.enableParserOnly();

    await new Promise(resolve => setTimeout(resolve, 1000));

    // Проверяем состояние после изменения
    helpers.getCurrentConfig();

    // Включаем все компоненты обратно
    console.log('\nВключаем все компоненты...');
    helpers.enableAllComponents();

    await new Promise(resolve => setTimeout(resolve, 1000));

    // Финальная проверка
    helpers.getCurrentConfig();

    console.log('\n✅ Тестирование завершено успешно!');
    console.log('🔄 Перезагрузите страницу, чтобы увидеть изменения');

  } catch (error) {
    console.error('❌ Ошибка тестирования:', error);
  }
}

// Запускаем тест
testComponents();



