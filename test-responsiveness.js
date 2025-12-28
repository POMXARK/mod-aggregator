/**
 * Скрипт для тестирования адаптивности приложения
 * Запускается в консоли браузера приложения (DevTools)
 * 
 * Использование:
 * 1. Откройте DevTools в приложении (F12)
 * 2. Вставьте этот скрипт в консоль
 * 3. Запустите: testResponsiveness()
 */

async function testResponsiveness() {
  // Проверяем наличие Tauri API
  if (typeof window === 'undefined' || !window.__TAURI__) {
    console.error('Tauri API не доступен. Запустите скрипт в Tauri приложении.');
    return;
  }
  
  // Для Tauri 2.x используем правильный API
  let window_handle;
  try {
    // Попытка 1: Использовать глобальный объект Tauri
    if (window.__TAURI__?.window) {
      const windowModule = window.__TAURI__.window;
      // Попробуем разные способы получения окна
      if (windowModule.getCurrent) {
        window_handle = windowModule.getCurrent();
      } else if (windowModule.getCurrentWindow) {
        window_handle = windowModule.getCurrentWindow();
      } else if (windowModule.appWindow) {
        window_handle = windowModule.appWindow;
      } else {
        // Используем динамический импорт
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        window_handle = getCurrentWindow();
      }
    } else {
      // Используем динамический импорт как fallback
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      window_handle = getCurrentWindow();
    }
  } catch (e) {
    console.error('Ошибка при получении окна:', e);
    console.log('Попробуйте использовать ручное изменение размера окна');
    return;
  }
  
  const testSizes = [
    { width: 800, height: 600, name: 'Маленький (800x600)' },
    { width: 1024, height: 768, name: 'Средний (1024x768)' },
    { width: 1280, height: 720, name: 'HD (1280x720)' },
    { width: 1920, height: 1080, name: 'Full HD (1920x1080)' },
    { width: 1536, height: 864, name: 'Большой (1536x864)' },
  ];
  
  console.log('=== Тестирование адаптивности ===\n');
  
  for (const size of testSizes) {
    console.log(`Тестирую размер: ${size.name}`);
    console.log(`Изменяю размер окна на ${size.width}x${size.height}...`);
    
    try {
      // Используем Tauri API для изменения размера
      await window_handle.setSize({ width: size.width, height: size.height });
      await window_handle.center();
      
      // Ждем завершения анимации
      await new Promise(resolve => setTimeout(resolve, 500));
      
      // Проверяем текущий размер
      const currentSize = await window_handle.innerSize();
      console.log(`✓ Размер установлен: ${currentSize.width}x${currentSize.height}`);
      
      // Проверяем элементы на странице
      const sidebar = document.querySelector('.sidebar');
      const mainContent = document.querySelector('.main-content');
      const modsGrid = document.querySelector('.mods-grid');
      
      if (sidebar) {
        const sidebarWidth = sidebar.offsetWidth;
        const sidebarComputed = getComputedStyle(sidebar);
        console.log(`  Sidebar: ${sidebarWidth}px (${sidebarComputed.width})`);
        
        // Проверяем иконки
        const icons = sidebar.querySelectorAll('.icon, .icon-small');
        if (icons.length > 0) {
          const firstIcon = icons[0];
          const iconSize = getComputedStyle(firstIcon).width;
          console.log(`  Иконки: ${iconSize}`);
        }
      }
      
      if (mainContent) {
        const mainWidth = mainContent.offsetWidth;
        console.log(`  Main Content: ${mainWidth}px`);
      }
      
      if (modsGrid) {
        const gridComputed = getComputedStyle(modsGrid);
        const columns = gridComputed.gridTemplateColumns;
        console.log(`  Mods Grid: ${columns}`);
        
        // Подсчитываем количество колонок
        const columnCount = columns.split(' ').filter(c => c && c !== 'repeat').length || 
                           (columns.includes('auto-fill') ? 'auto-fill' : '1');
        console.log(`  Колонок: ${columnCount}`);
      }
      
      console.log('  Ожидаю 2 секунды для визуальной проверки...\n');
      await new Promise(resolve => setTimeout(resolve, 2000));
      
    } catch (error) {
      console.error(`✗ Ошибка при изменении размера:`, error);
    }
  }
  
  console.log('=== Тестирование завершено ===');
  console.log('\nПроверьте визуально каждый размер и убедитесь, что:');
  console.log('1. Sidebar адаптируется корректно');
  console.log('2. Иконки имеют правильный размер');
  console.log('3. Текст читаем и не переполняется');
  console.log('4. Кнопки доступны и кликабельны');
  console.log('5. Grid адаптируется под размер экрана');
}

// Экспорт для использования
if (typeof window !== 'undefined') {
  window.testResponsiveness = testResponsiveness;
  console.log('✓ Скрипт загружен. Запустите: testResponsiveness()');
}

