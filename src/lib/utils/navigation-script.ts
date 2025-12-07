/**
 * Скрипт для обработки навигации по внутренним ссылкам
 * Работает независимо от скрипта выделения, активен когда режим выделения выключен
 */

/**
 * Генерирует скрипт для обработки навигации по внутренним ссылкам
 * @param baseUrl - базовый URL страницы для определения внутренних ссылок
 * @returns JavaScript код для встраивания в HTML
 */
export function getNavigationScriptContent(baseUrl?: string): string {
  const SCRIPT_ID = 'parser-navigation-' + Date.now();
  // Экранируем baseUrl для безопасного использования в JavaScript
  const escapedBaseUrl = baseUrl ? baseUrl.replace(/\\/g, '\\\\').replace(/'/g, "\\'").replace(/"/g, '\\"') : '';
  const baseUrlConst = baseUrl ? `'${escapedBaseUrl}'` : 'null';
  
  return `
(function() {
  'use strict';
  const NAVIGATION_SCRIPT_ID = '${SCRIPT_ID}';
  const BASE_URL = ${baseUrlConst};
  
  function log(message) {
    try {
      console.log('[NAVIGATION] ' + message);
      if (window.parent && window.parent !== window) {
        window.parent.postMessage({ 
          type: 'parser-debug', 
          message: '[NAVIGATION] ' + message, 
          scriptId: NAVIGATION_SCRIPT_ID 
        }, '*');
      }
    } catch (e) {
      // Игнорируем ошибки логирования
    }
  }
  
  log('Navigation script loaded, ID: ' + NAVIGATION_SCRIPT_ID + ', base URL: ' + BASE_URL);
  
  let isSelectionModeActive = false;
  let linksWithIndicator = new Set();
  
  /**
   * Получает базовый URL страницы
   */
  function getBaseUrl() {
    // Сначала используем константу из скрипта
    if (BASE_URL) {
      log('Using base URL from script constant: ' + BASE_URL);
      return BASE_URL;
    }
    
    // Затем проверяем data-атрибуты
    if (document.body) {
      const bodyUrl = document.body.getAttribute('data-base-url');
      if (bodyUrl) {
        log('Found base URL in body: ' + bodyUrl);
        return bodyUrl;
      }
    }
    
    if (document.documentElement) {
      const htmlUrl = document.documentElement.getAttribute('data-base-url');
      if (htmlUrl) {
        log('Found base URL in html: ' + htmlUrl);
        return htmlUrl;
      }
    }
    
    log('No base URL found');
    return null;
  }
  
  /**
   * Проверяет, является ли ссылка внутренней
   */
  function isInternalLink(linkElement, baseUrl) {
    if (!linkElement || !linkElement.href || !baseUrl) return false;
    
    try {
      const linkUrl = new URL(linkElement.href, baseUrl);
      const baseUrlObj = new URL(baseUrl);
      
      // Ссылка внутренняя, если hostname и protocol совпадают
      return linkUrl.hostname === baseUrlObj.hostname && 
             linkUrl.protocol === baseUrlObj.protocol;
    } catch (e) {
      return false;
    }
  }
  
  /**
   * Добавляет визуальную индикацию для внутренних ссылок
   */
  function addLinkIndicator(link) {
    if (!link || linksWithIndicator.has(link)) return;
    
    try {
      link.style.cursor = 'pointer';
      link.style.transition = 'all 0.2s ease';
      
      const mouseEnterHandler = function() {
        if (!isSelectionModeActive) {
          link.style.opacity = '0.85';
          link.style.textDecoration = 'underline';
          link.style.backgroundColor = 'rgba(66, 133, 244, 0.1)';
          link.style.borderRadius = '2px';
          link.style.padding = '1px 2px';
        }
      };
      
      const mouseLeaveHandler = function() {
        if (!isSelectionModeActive) {
          link.style.opacity = '';
          link.style.textDecoration = '';
          link.style.backgroundColor = '';
          link.style.borderRadius = '';
          link.style.padding = '';
        }
      };
      
      link.addEventListener('mouseenter', mouseEnterHandler, { passive: true });
      link.addEventListener('mouseleave', mouseLeaveHandler, { passive: true });
      
      linksWithIndicator.add(link);
    } catch (e) {
      log('Error adding link indicator: ' + e);
    }
  }
  
  /**
   * Обрабатывает клик по ссылке
   */
  function handleLinkClick(e) {
    // Если режим выделения активен, не обрабатываем ссылки
    if (isSelectionModeActive) {
      log('Selection mode active, ignoring link click');
      return;
    }
    
    let target = e.target;
    while (target && target.tagName !== 'A') {
      target = target.parentElement;
    }
    
    if (!target || !target.href) {
      log('No link target found');
      return;
    }
    
    log('Link clicked: ' + target.href + ', selection mode: ' + isSelectionModeActive);
    
    try {
      const baseUrl = getBaseUrl();
      if (!baseUrl) {
        log('No base URL found, cannot determine internal links');
        return;
      }
      
      log('Base URL: ' + baseUrl + ', link href: ' + target.href);
      
      // Проверяем, является ли ссылка внутренней
      if (!isInternalLink(target, baseUrl)) {
        log('External link, allowing default behavior: ' + target.href);
        return;
      }
      
      // Нормализуем URL ссылки
      // target.href может быть абсолютным или относительным
      let linkUrl;
      try {
        // Если target.href уже абсолютный URL, используем его напрямую
        if (target.href.startsWith('http://') || target.href.startsWith('https://')) {
          linkUrl = new URL(target.href);
        } else {
          // Если относительный, создаем полный URL относительно baseUrl
          linkUrl = new URL(target.href, baseUrl);
        }
      } catch (e) {
        log('Error parsing link URL: ' + target.href + ', error: ' + e);
        return;
      }
      
      const fullUrl = linkUrl.href;
      log('Internal link clicked: ' + fullUrl + ' (original: ' + target.href + ')');
      
      // Предотвращаем стандартное поведение
      e.preventDefault();
      e.stopPropagation();
      e.stopImmediatePropagation();
      
      // Отправляем сообщение в родительское окно с полным URL
      if (window.parent && window.parent !== window) {
        const message = {
          type: 'navigate-internal-link',
          url: fullUrl,
          scriptId: NAVIGATION_SCRIPT_ID
        };
        log('Sending message to parent: ' + JSON.stringify(message));
        try {
          window.parent.postMessage(message, '*');
          log('Message sent successfully');
        } catch (err) {
          log('Error sending message: ' + err);
        }
      } else {
        log('No parent window found');
      }
      
      return false;
    } catch (e) {
      log('Error handling link click: ' + e + ', stack: ' + (e.stack || ''));
    }
  }
  
  /**
   * Обрабатывает наведение на ссылки для визуальной индикации
   */
  function handleLinkHover(e) {
    if (isSelectionModeActive) return;
    
    let target = e.target;
    while (target && target.tagName !== 'A') {
      target = target.parentElement;
    }
    
    if (!target || !target.href) return;
    
    try {
      const baseUrl = getBaseUrl();
      if (!baseUrl) return;
      
      // Если ссылка внутренняя, добавляем визуальную индикацию
      if (isInternalLink(target, baseUrl)) {
        addLinkIndicator(target);
      }
    } catch (e) {
      // Игнорируем ошибки при наведении
    }
  }
  
  /**
   * Обрабатывает существующие ссылки на странице
   */
  function processLinks() {
    try {
      const allLinks = document.querySelectorAll('a[href]');
      const baseUrl = getBaseUrl();
      
      if (baseUrl) {
        let internalCount = 0;
        allLinks.forEach(link => {
          if (isInternalLink(link, baseUrl)) {
            addLinkIndicator(link);
            internalCount++;
          }
        });
        log('Processed ' + allLinks.length + ' links, found ' + internalCount + ' internal links');
      }
    } catch (e) {
      log('Error processing existing links: ' + e);
    }
  }
  
  /**
   * Инициализирует обработчики событий
   */
  let navigationInitialized = false;
  
  function initNavigation() {
    if (navigationInitialized) {
      log('Navigation already initialized, skipping');
      return;
    }
    
    log('Initializing navigation handlers');
    
    // Обработчик кликов по ссылкам
    // Используем capture: true и passive: false, чтобы перехватить клик до других обработчиков
    document.addEventListener('click', handleLinkClick, { capture: true, passive: false });
    log('Click handler attached');
    
    // Обработчик наведения на ссылки для визуальной индикации
    document.addEventListener('mouseover', handleLinkHover, { capture: true, passive: true });
    log('Hover handler attached');
    
    // Обрабатываем существующие ссылки
    processLinks();
    
    navigationInitialized = true;
    
    // Наблюдаем за добавлением новых ссылок в DOM
    try {
      const observer = new MutationObserver(function(mutations) {
        let hasNewLinks = false;
        mutations.forEach(function(mutation) {
          if (mutation.addedNodes.length > 0) {
            mutation.addedNodes.forEach(function(node) {
              if (node.nodeType === 1) { // Element node
                if (node.tagName === 'A' && node.href) {
                  hasNewLinks = true;
                } else if (node.querySelectorAll) {
                  const links = node.querySelectorAll('a[href]');
                  if (links.length > 0) {
                    hasNewLinks = true;
                  }
                }
              }
            });
          }
        });
        
        if (hasNewLinks) {
          // Небольшая задержка, чтобы дать время DOM обновиться
          setTimeout(processLinks, 100);
        }
      });
      
      observer.observe(document.body || document.documentElement, {
        childList: true,
        subtree: true
      });
      
      log('MutationObserver set up for new links');
    } catch (e) {
      log('Error setting up MutationObserver: ' + e);
    }
    
    log('Navigation listeners attached');
  }
  
  /**
   * Обрабатывает сообщения от родительского окна
   */
  window.addEventListener('message', function(event) {
    try {
      if (!event.data || typeof event.data !== 'object') {
        return;
      }
      
      log('Navigation script received message: ' + JSON.stringify(event.data));
      
      // Обновляем состояние режима выделения
      if (event.data.type === 'enable-selection') {
        isSelectionModeActive = true;
        log('Selection mode enabled, navigation disabled');
      } else if (event.data.type === 'disable-selection') {
        isSelectionModeActive = false;
        log('Selection mode disabled, navigation enabled');
      }
    } catch (e) {
      log('Error handling message: ' + e);
    }
  });
  
  // Инициализируем навигацию
  function waitForBody(callback, maxAttempts) {
    maxAttempts = maxAttempts || 100;
    let attempts = 0;
    
    function check() {
      if (document.body || document.documentElement) {
        callback();
      } else if (attempts < maxAttempts) {
        attempts++;
        setTimeout(check, 50);
      }
    }
    
    check();
  }
  
  // Инициализируем сразу, если DOM готов
  if (document.body || document.documentElement) {
    log('DOM ready, initializing navigation immediately');
    initNavigation();
  } else {
    log('DOM not ready, waiting for body');
    waitForBody(function() {
      log('Body found, initializing navigation');
      initNavigation();
    }, 50);
  }
  
  // Также инициализируем при полной загрузке
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', function() {
      log('DOMContentLoaded, re-initializing navigation');
      initNavigation();
    });
  } else {
    log('DOM already loaded');
  }
  
  log('Navigation script initialized, selection mode active: ' + isSelectionModeActive);
})();
`;
}

