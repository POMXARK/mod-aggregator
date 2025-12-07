/**
 * Утилиты для обработки HTML: удаление скриптов, встраивание скрипта выделения
 * Вынесено из PageViewer для разделения логики
 */

/**
 * Удаляет все JavaScript скрипты из HTML, кроме нашего скрипта выделения
 * @param html - исходный HTML
 * @returns HTML без скриптов
 */
export function removeScripts(html: string): string {
  let modifiedHtml = html;

  // Удаляем все встроенные скрипты, включая наши скрипты навигации и выделения
  // Это важно, чтобы при загрузке из кеша мы могли встроить скрипты заново с правильным URL
  modifiedHtml = modifiedHtml.replace(/<script[^>]*>[\s\S]*?<\/script>/gi, '<!-- Removed script -->');
  modifiedHtml = modifiedHtml.replace(/<script[^>]*>/gi, '<!-- Removed script tag -->');
  modifiedHtml = modifiedHtml.replace(/<\/script>/gi, '<!-- Removed script close tag -->');
  modifiedHtml = modifiedHtml.replace(/<script[^>]*\/\s*>/gi, '<!-- Removed self-closing script -->');
  
  // Также удаляем data-base-url атрибуты, которые мы добавляли ранее
  modifiedHtml = modifiedHtml.replace(/\s+data-base-url\s*=\s*["\'][^"\']*["\']/gi, '');

  // Удаляем inline обработчики событий
  const inlineHandlers = [
    'onclick', 'onmouseover', 'onmouseout', 'onmousedown', 'onmouseup', 'onmousemove',
    'ontouchstart', 'ontouchend', 'ontouchmove', 'onload', 'onerror', 'onchange',
    'onsubmit', 'onfocus', 'onblur', 'onkeydown', 'onkeyup', 'onkeypress',
    'ondblclick', 'oncontextmenu', 'onwheel', 'onscroll', 'onresize'
  ];

  inlineHandlers.forEach(handler => {
    const regex = new RegExp('\\s+' + handler + '\\s*=\\s*["\'][^"\']*["\']', 'gi');
    modifiedHtml = modifiedHtml.replace(regex, '');
  });

  // Удаляем data-атрибуты с JavaScript
  modifiedHtml = modifiedHtml.replace(/\s+data-[^=]*=\s*["\'][^"\']*addEventListener[^"\']*["\']/gi, '');
  modifiedHtml = modifiedHtml.replace(/\s+data-[^=]*=\s*["\'][^"\']*javascript[^"\']*["\']/gi, '');
  modifiedHtml = modifiedHtml.replace(/\s+data-[^=]*=\s*["\'][^"\']*on\w+[^"\']*["\']/gi, '');

  // Удаляем noscript теги
  modifiedHtml = modifiedHtml.replace(/<noscript[^>]*>[\s\S]*?<\/noscript>/gi, '<!-- Removed noscript -->');

  return modifiedHtml;
}

/**
 * Встраивает скрипт выделения, скрипт навигации и стили в HTML
 * @param html - HTML без скриптов
 * @param scriptContent - содержимое скрипта выделения
 * @param navigationScriptContent - содержимое скрипта навигации
 * @param cssStyles - CSS стили для рамки выделения
 * @param baseUrl - базовый URL страницы для определения внутренних ссылок (опционально)
 * @returns HTML со встроенными скриптами и стилями
 */
export function embedSelectionScript(
  html: string, 
  scriptContent: string, 
  navigationScriptContent: string,
  cssStyles: string, 
  baseUrl?: string
): string {
  const scriptCloseTag = '</' + 'script>';
  
  // Экранируем скрипт выделения
  const escapedSelectionScript = scriptContent
    .replace(new RegExp(scriptCloseTag, 'g'), '<\\/script>')
    .replace(/<!--/g, '<\\!--')
    .replace(/-->/g, '--\\>')
    .replace(/<\/script>/gi, '<\\/script>');
  
  // Экранируем скрипт навигации
  const escapedNavigationScript = navigationScriptContent
    .replace(new RegExp(scriptCloseTag, 'g'), '<\\/script>')
    .replace(/<!--/g, '<\\!--')
    .replace(/-->/g, '--\\>')
    .replace(/<\/script>/gi, '<\\/script>');
  
  const selectionScriptTag = '<script id="parser-selection-script" type="text/javascript">' + escapedSelectionScript + '</' + 'script>';
  const navigationScriptTag = '<script id="parser-navigation-script" type="text/javascript">' + escapedNavigationScript + '</' + 'script>';
  const scriptsTag = navigationScriptTag + selectionScriptTag;
  const scriptTagEnd = '</' + 'script>';

  let modifiedHtml = html;
  
  // Добавляем data-атрибут с базовым URL для определения внутренних ссылок
  let baseUrlAttribute = '';
  if (baseUrl) {
    // Экранируем URL для безопасного использования в HTML
    const escapedBaseUrl = baseUrl.replace(/"/g, '&quot;');
    baseUrlAttribute = ` data-base-url="${escapedBaseUrl}"`;
  }

  // Вставляем скрипты и стили как можно раньше
  // Скрипт навигации должен быть первым, чтобы он работал когда режим выделения выключен
  // Важно: добавляем baseUrlAttribute и к html, и к body для надежности
  
  // Сначала добавляем скрипты и стили
  if (modifiedHtml.includes('<!DOCTYPE') || modifiedHtml.includes('<!doctype')) {
    modifiedHtml = modifiedHtml.replace(/(<!DOCTYPE[^>]*>)/i, '$1' + cssStyles + scriptsTag);
  } else if (modifiedHtml.includes('<html')) {
    modifiedHtml = modifiedHtml.replace(/(<html[^>]*>)/i, '$1' + cssStyles + scriptsTag);
  } else if (modifiedHtml.includes('</head>')) {
    modifiedHtml = modifiedHtml.replace('</head>', cssStyles + scriptsTag + '</head>');
  } else if (modifiedHtml.includes('<head>')) {
    modifiedHtml = modifiedHtml.replace('<head>', '<head>' + cssStyles + scriptsTag);
  } else if (modifiedHtml.includes('<body')) {
    modifiedHtml = modifiedHtml.replace(/(<body[^>]*>)/i, '$1' + cssStyles + scriptsTag);
  } else if (modifiedHtml.includes('</body>')) {
    modifiedHtml = modifiedHtml.replace('</body>', cssStyles + scriptsTag + '</body>');
  } else {
    modifiedHtml = cssStyles + scriptsTag + modifiedHtml;
  }
  
  // Затем добавляем data-base-url атрибут к html и body
  if (baseUrlAttribute) {
    // Добавляем к html элементу
    if (modifiedHtml.includes('<html')) {
      const htmlMatch = modifiedHtml.match(/(<html[^>]*>)/i);
      if (htmlMatch && !htmlMatch[1].includes('data-base-url')) {
        modifiedHtml = modifiedHtml.replace(/(<html[^>]*>)/i, `$1${baseUrlAttribute}`);
      }
    }
    
    // Добавляем к body элементу
    if (modifiedHtml.includes('<body')) {
      const bodyMatch = modifiedHtml.match(/(<body[^>]*>)/i);
      if (bodyMatch && !bodyMatch[1].includes('data-base-url')) {
        modifiedHtml = modifiedHtml.replace(/(<body[^>]*>)/i, `$1${baseUrlAttribute}`);
      }
    }
  }

  return modifiedHtml;
}



