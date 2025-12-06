<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '../lib/tauri-wrapper';

  interface Props {
    url: string;
    onElementSelect?: (selector: string, element: HTMLElement, data?: any) => void;
  }

  let { url, onElementSelect }: Props = $props();

  let iframeRef: HTMLIFrameElement | null = $state(null);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let selectionMode = $state(false);
  let recentUrls = $state<string[]>([]);
  let showRecentUrls = $state(false);
  let scriptReady = $state(false);
  let panelCollapsed = $state(false);


  onMount(() => {
    // Load recent URLs from localStorage
    loadRecentUrls();
  });

  function loadRecentUrls() {
    try {
      const stored = localStorage.getItem('parser-recent-urls');
      if (stored) {
        recentUrls = JSON.parse(stored);
      }
    } catch (e) {
      console.error('Failed to load recent URLs:', e);
    }
  }

  function saveRecentUrl(url: string) {
    if (!url || url.trim() === '') return;
    
    try {
      // Remove if already exists
      recentUrls = recentUrls.filter(u => u !== url);
      // Add to beginning
      recentUrls = [url, ...recentUrls].slice(0, 10); // Keep only last 10
      localStorage.setItem('parser-recent-urls', JSON.stringify(recentUrls));
    } catch (e) {
      console.error('Failed to save recent URL:', e);
    }
  }

  function removeRecentUrl(urlToRemove: string) {
    recentUrls = recentUrls.filter(u => u !== urlToRemove);
    try {
      localStorage.setItem('parser-recent-urls', JSON.stringify(recentUrls));
    } catch (e) {
      console.error('Failed to remove recent URL:', e);
    }
  }

  function selectRecentUrl(selectedUrl: string) {
    url = selectedUrl;
    showRecentUrls = false;
    loadPage();
  }

  // Функция для получения CSS стилей выделения (определена здесь, чтобы избежать проблем с парсером)
  function getSelectionStyles(): string {
    const styleOpen = String.fromCharCode(60, 115, 116, 121, 108, 101, 32, 105, 100, 61, 34, 112, 97, 114, 115, 101, 114, 45, 115, 101, 108, 101, 99, 116, 105, 111, 110, 45, 115, 116, 121, 108, 101, 115, 34, 62);
    const hoverBoxCss = '#parser-hover-box {position: absolute !important; pointer-events: none !important; z-index: 2147483647 !important; border: 2px solid #4285f4 !important; background: rgba(66, 133, 244, 0.15) !important; box-shadow: 0 0 0 2px rgba(66, 133, 244, 0.4), 0 0 12px rgba(66, 133, 244, 0.3) !important; transition: all 100ms cubic-bezier(0.4, 0, 0.2, 1) !important; display: none !important; box-sizing: border-box !important; margin: 0 !important; padding: 0 !important; visibility: visible !important; opacity: 1 !important;}';
    const overlayCss = '#parser-info-overlay {position: fixed !important; background: #1e1e1e !important; color: #d4d4d4 !important; padding: 6px 10px !important; border-radius: 3px !important; font-size: 11px !important; font-family: "Consolas", "Monaco", "Courier New", monospace !important; pointer-events: none !important; z-index: 2147483647 !important; border: 1px solid #4285f4 !important; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3) !important; display: none !important; line-height: 1.4 !important; max-width: 300px !important; word-wrap: break-word !important; margin: 0 !important; visibility: visible !important; opacity: 1 !important;}';
    const styleClose = String.fromCharCode(60, 47, 115, 116, 121, 108, 101, 62);
    return styleOpen + hoverBoxCss + overlayCss + styleClose;
  }

  function getSelectionScriptContent(): string {
    const SCRIPT_ID = 'parser-selection-' + Date.now();
    return `
(function() {
  'use strict';
  const SCRIPT_ID = '${SCRIPT_ID}';
  
  function log(message) {
    try {
      console.log('[IFRAME] ' + message);
      if (window.parent && window.parent !== window) {
        window.parent.postMessage({ type: 'parser-debug', message: message, scriptId: SCRIPT_ID }, '*');
      }
    } catch (e) {
    }
  }
  
  log('Parser selection script loaded, ID: ' + SCRIPT_ID);
  
  let isSelectionMode = false;
  let selectedElement = null;
  let hoverBox = null;
  let infoOverlay = null;
  let borderWidth = 2;
  let initialized = false;
  
  function waitForBody(callback, maxAttempts) {
    maxAttempts = maxAttempts || 100;
    let attempts = 0;
    
    function check() {
      attempts++;
      log('Checking for document.body, attempt ' + attempts + ', readyState: ' + document.readyState);
      
      if (document.body && typeof document.body.appendChild === 'function') {
        log('document.body found, readyState: ' + document.readyState);
        try {
          callback();
        } catch (e) {
          log('Error in callback: ' + e);
        }
        return true;
      } else if (document.documentElement) {
        log('document.documentElement exists but body not ready yet');
      } else {
        log('document.documentElement not available');
      }
      
      if (attempts < maxAttempts) {
        setTimeout(check, 50);
      } else {
        log('Failed to wait for document.body after ' + maxAttempts + ' attempts, trying anyway');
        try {
          if (document.documentElement) {
            log('Trying to create elements on documentElement instead');
            callback();
          }
        } catch (e) {
          log('Failed to initialize even with documentElement: ' + e);
        }
      }
      return false;
    }
    
    if (document.readyState === 'loading') {
      log('Document still loading, waiting for DOMContentLoaded');
      document.addEventListener('DOMContentLoaded', function() {
        log('DOMContentLoaded fired');
        setTimeout(check, 10);
      });
    } else {
      log('Document already loaded, checking immediately');
      check();
    }
  }
  
  function initSelection() {
    log('initSelection called, initialized: ' + initialized);
    
    function tryInit() {
      try {
        let targetElement = document.body;
        if (!targetElement && document.documentElement) {
          log('Using documentElement as fallback');
          targetElement = document.documentElement;
        }
        
        if (!targetElement) {
          log('No target element available for initialization');
          return false;
        }
        
        if (!hoverBox) {
          hoverBox = document.createElement('div');
          hoverBox.id = 'parser-hover-box';
          // Используем setProperty с !important для максимального приоритета
          hoverBox.style.setProperty('position', 'absolute', 'important');
          hoverBox.style.setProperty('pointer-events', 'none', 'important');
          hoverBox.style.setProperty('z-index', '2147483647', 'important');
          hoverBox.style.setProperty('border', borderWidth + 'px solid #4285f4', 'important');
          hoverBox.style.setProperty('background', 'rgba(66, 133, 244, 0.15)', 'important');
          hoverBox.style.setProperty('box-shadow', '0 0 0 2px rgba(66, 133, 244, 0.4), 0 0 12px rgba(66, 133, 244, 0.3)', 'important');
          hoverBox.style.setProperty('transition', 'all 100ms cubic-bezier(0.4, 0, 0.2, 1)', 'important');
          hoverBox.style.setProperty('display', 'none', 'important');
          hoverBox.style.setProperty('box-sizing', 'border-box', 'important');
          hoverBox.style.setProperty('margin', '0', 'important');
          hoverBox.style.setProperty('padding', '0', 'important');
          try {
            targetElement.appendChild(hoverBox);
            log('Hover box created and appended');
          } catch (e) {
            log('Failed to append hoverBox: ' + e);
            return false;
          }
        }
        
        if (!infoOverlay) {
          infoOverlay = document.createElement('div');
          infoOverlay.id = 'parser-info-overlay';
          // Используем setProperty с !important для максимального приоритета
          infoOverlay.style.setProperty('position', 'fixed', 'important');
          infoOverlay.style.setProperty('background', '#1e1e1e', 'important');
          infoOverlay.style.setProperty('color', '#d4d4d4', 'important');
          infoOverlay.style.setProperty('padding', '6px 10px', 'important');
          infoOverlay.style.setProperty('border-radius', '3px', 'important');
          infoOverlay.style.setProperty('font-size', '11px', 'important');
          infoOverlay.style.setProperty('font-family', '"Consolas", "Monaco", "Courier New", monospace', 'important');
          infoOverlay.style.setProperty('pointer-events', 'none', 'important');
          infoOverlay.style.setProperty('z-index', '2147483647', 'important');
          infoOverlay.style.setProperty('border', '1px solid #4285f4', 'important');
          infoOverlay.style.setProperty('box-shadow', '0 2px 8px rgba(0, 0, 0, 0.3)', 'important');
          infoOverlay.style.setProperty('display', 'none', 'important');
          infoOverlay.style.setProperty('line-height', '1.4', 'important');
          infoOverlay.style.setProperty('max-width', '300px', 'important');
          infoOverlay.style.setProperty('word-wrap', 'break-word', 'important');
          infoOverlay.style.setProperty('margin', '0', 'important');
          try {
            targetElement.appendChild(infoOverlay);
            log('Info overlay created and appended');
          } catch (e) {
            log('Failed to append infoOverlay: ' + e);
            return false;
          }
        }
        
        initialized = true;
        log('Selection initialized successfully');
        return true;
      } catch (e) {
        log('Error in tryInit: ' + e);
        return false;
      }
    }
    
    if (document.body && typeof document.body.appendChild === 'function') {
      log('document.body ready, initializing immediately');
      tryInit();
    } else {
      log('document.body not ready, waiting...');
      waitForBody(function() {
        tryInit();
      }, 100);
    }
  }
  
  function updateHoverBox(element) {
    if (!hoverBox || !element) return;
    try {
      const rect = element.getBoundingClientRect();
      const scrollX = window.pageXOffset || document.documentElement.scrollLeft || 0;
      const scrollY = window.pageYOffset || document.documentElement.scrollTop || 0;
      
      // Используем setProperty с !important для гарантии отображения
      hoverBox.style.setProperty('display', 'block', 'important');
      hoverBox.style.setProperty('left', (rect.left + scrollX) + 'px', 'important');
      hoverBox.style.setProperty('top', (rect.top + scrollY) + 'px', 'important');
      hoverBox.style.setProperty('width', rect.width + 'px', 'important');
      hoverBox.style.setProperty('height', rect.height + 'px', 'important');
      hoverBox.style.setProperty('visibility', 'visible', 'important');
      hoverBox.style.setProperty('opacity', '1', 'important');
    } catch (e) {
      log('Error updating hoverBox: ' + e);
    }
  }
  
  function hideHoverBox() {
    if (hoverBox) {
      hoverBox.style.setProperty('display', 'none', 'important');
    }
  }
  
  function showElementInfo(element, x, y) {
    if (!infoOverlay) return;
    const tagName = element.tagName.toLowerCase();
    const id = element.id ? '#' + element.id : '';
    const classes = element.className && typeof element.className === 'string' 
      ? '.' + element.className.split(' ').filter(c => c && !c.includes('parser-')).slice(0, 3).join('.') 
      : '';
    const text = (element.textContent || '').trim().substring(0, 40);
    
    let infoText = '<div style="color: #569cd6; font-weight: bold;">' + tagName + id + classes + '</div>';
    if (text) {
      infoText += '<div style="color: #ce9178; margin-top: 4px;">"' + text + (element.textContent && element.textContent.trim().length > 40 ? '...' : '') + '"</div>';
    }
    
    infoOverlay.innerHTML = infoText;
    infoOverlay.style.setProperty('display', 'block', 'important');
    infoOverlay.style.setProperty('left', (x + 10) + 'px', 'important');
    infoOverlay.style.setProperty('top', (y + 10) + 'px', 'important');
    infoOverlay.style.setProperty('visibility', 'visible', 'important');
    infoOverlay.style.setProperty('opacity', '1', 'important');
  }
  
  function hideElementInfo() {
    if (infoOverlay) {
      infoOverlay.style.setProperty('display', 'none', 'important');
    }
  }
  
  function generateSelector(element) {
    if (element.id) {
      return '#' + element.id;
    }
    
    if (element.className && typeof element.className === 'string') {
      const classes = element.className.split(' ').filter(c => c && !c.includes('parser-')).map(c => '.' + c).join('');
      if (classes) {
        return element.tagName.toLowerCase() + classes;
      }
    }
    
    const path = [];
    let current = element;
    
    while (current && current !== document.body) {
      let selector = current.tagName.toLowerCase();
      
      if (current.id) {
        selector = '#' + current.id;
        path.unshift(selector);
        break;
      }
      
      if (current.className && typeof current.className === 'string') {
        const classes = current.className.split(' ').filter(c => c).map(c => '.' + c).join('');
        if (classes) {
          selector += classes;
        }
      }
      
      const parent = current.parentElement;
      if (parent) {
        const siblings = Array.from(parent.children).filter(el => el.tagName === current.tagName);
        if (siblings.length > 1) {
          const index = siblings.indexOf(current) + 1;
          selector += ':nth-of-type(' + index + ')';
        }
      }
      
      path.unshift(selector);
      current = current.parentElement;
    }
    
    return path.join(' > ');
  }
  
  function handleMouseMove(e) {
    if (!isSelectionMode) return;
    
    // Останавливаем распространение события с максимальным приоритетом
    try {
      e.preventDefault();
      e.stopPropagation();
      e.stopImmediatePropagation();
    } catch (err) {
      // Игнорируем ошибки, если событие уже обработано
    }
    
    const target = e.target;
    if (!target || 
        target === document.body || 
        target === document.documentElement || 
        target === hoverBox || 
        target === infoOverlay ||
        target.id === 'parser-hover-box' ||
        target.id === 'parser-info-overlay') {
      return;
    }
    
    // Принудительно обновляем рамку
    updateHoverBox(target);
    showElementInfo(target, e.clientX, e.clientY);
    
    // Дополнительная проверка: если рамка не видна, показываем её принудительно
    if (hoverBox && hoverBox.style.display === 'none') {
      log('HoverBox was hidden, forcing display');
      hoverBox.style.setProperty('display', 'block', 'important');
    }
  }
  
  function handleMouseOut(e) {
    if (!isSelectionMode) return;
    const target = e.target;
    if (target && 
        target !== selectedElement && 
        target !== hoverBox && 
        target !== infoOverlay &&
        target.id !== 'parser-hover-box' &&
        target.id !== 'parser-info-overlay') {
      if (e.relatedTarget === document.body || e.relatedTarget === document.documentElement || !e.relatedTarget) {
        hideHoverBox();
        hideElementInfo();
      }
    }
  }
  
  function handleClick(e) {
    if (!isSelectionMode) return;
    e.preventDefault();
    e.stopPropagation();
    e.stopImmediatePropagation();
    
    const target = e.target;
    if (!target || 
        target === document.body || 
        target === document.documentElement || 
        target === hoverBox || 
        target === infoOverlay ||
        target.id === 'parser-hover-box' ||
        target.id === 'parser-info-overlay') {
      return;
    }
    
    selectedElement = target;
    
    if (hoverBox) {
      hoverBox.style.borderColor = '#34a853';
      hoverBox.style.background = 'rgba(52, 168, 83, 0.25)';
      hoverBox.style.boxShadow = '0 0 0 2px rgba(52, 168, 83, 0.5), 0 0 15px rgba(52, 168, 83, 0.4)';
    }
    
    const selector = generateSelector(target);
    const similarCount = document.querySelectorAll(selector).length;
    const attributes = {};
    if (target.attributes) {
      for (let i = 0; i < target.attributes.length; i++) {
        const attr = target.attributes[i];
        attributes[attr.name] = attr.value;
      }
    }
    
    console.log('[IFRAME] Element selected, selector:', selector, 'tag:', target.tagName);
    
    try {
      window.parent.postMessage({
        type: 'element-selected',
        selector: selector,
        tagName: target.tagName,
        text: (target.textContent || '').trim().substring(0, 100),
        attributes: attributes,
        similarElements: similarCount,
      }, '*');
      console.log('[IFRAME] Element selection message sent to parent');
    } catch (err) {
      console.error('[IFRAME] Failed to send element selection message:', err);
    }
    
    hideElementInfo();
  }
  
  function enableSelection() {
    if (isSelectionMode) {
      log('Selection mode already enabled');
      return;
    }
    
    log('Enabling selection mode');
    isSelectionMode = true;
    
    if (!initialized) {
      log('Not initialized yet, initializing now');
      initSelection();
    }
    
    function attachListeners() {
      const target = document.body || document.documentElement;
      if (target) {
        if (document.body) {
          document.body.style.cursor = 'crosshair';
        }
        document.addEventListener('mousemove', handleMouseMove, { capture: true, passive: false });
        document.addEventListener('click', handleClick, { capture: true, passive: false });
        document.addEventListener('mouseout', handleMouseOut, { capture: true, passive: false });
        document.addEventListener('mouseover', handleMouseMove, { capture: true, passive: false });
        log('Selection event listeners attached');
        return true;
      } else {
        log('No target element available for listeners');
        return false;
      }
    }
    
    if (document.body || document.documentElement) {
      attachListeners();
    } else {
      log('Waiting for body/documentElement to attach listeners');
      waitForBody(function() {
        attachListeners();
      }, 50);
    }
  }
  
  function disableSelection() {
    if (!isSelectionMode) {
      return;
    }
    
    console.log('[IFRAME] Disabling selection mode');
    isSelectionMode = false;
    
    if (document.body) {
      document.body.style.cursor = '';
    }
    
    document.removeEventListener('mousemove', handleMouseMove, { capture: true });
    document.removeEventListener('click', handleClick, { capture: true });
    document.removeEventListener('mouseout', handleMouseOut, { capture: true });
    document.removeEventListener('mouseover', handleMouseMove, { capture: true });
    
    selectedElement = null;
    hideHoverBox();
    hideElementInfo();
    
    if (hoverBox) {
      hoverBox.style.borderColor = '#4285f4';
      hoverBox.style.background = 'rgba(66, 133, 244, 0.1)';
      hoverBox.style.boxShadow = '0 0 0 1px rgba(66, 133, 244, 0.3), 0 0 8px rgba(66, 133, 244, 0.2)';
    }
    
    console.log('[IFRAME] Selection disabled and event listeners removed');
  }
  
  function notifyReady() {
    log('Notifying parent that script is ready, ID: ' + SCRIPT_ID);
    
    let attempts = 0;
    const maxAttempts = 10;
    
    function sendReady() {
      attempts++;
      try {
        if (window.parent && window.parent !== window) {
          window.parent.postMessage({
            type: 'parser-script-ready',
            scriptId: SCRIPT_ID,
            attempt: attempts
          }, '*');
          log('Ready message sent successfully (attempt ' + attempts + ')');
          
          if (attempts < maxAttempts) {
            setTimeout(sendReady, 100);
          }
        } else {
          log('Cannot access window.parent');
          if (attempts < maxAttempts) {
            setTimeout(sendReady, 100);
          }
        }
      } catch (e) {
        log('Failed to send ready message (attempt ' + attempts + '): ' + e);
        if (attempts < maxAttempts) {
          setTimeout(sendReady, 100);
        }
      }
    }
    
    sendReady();
    
    if (!initialized) {
      initialized = true;
    }
  }
  
  window.addEventListener('message', function(event) {
    try {
      if (!event.data || typeof event.data !== 'object') {
        return;
      }
      
      const validTypes = ['enable-selection', 'disable-selection'];
      if (event.data.type && validTypes.includes(event.data.type)) {
        log('Received valid message: ' + event.data.type + ' from origin: ' + (event.origin || 'unknown'));
        
        if (event.data.type === 'enable-selection') {
          log('Enabling selection from message');
          setTimeout(function() {
            try {
              enableSelection();
            } catch (e) {
              log('Error enabling selection from message: ' + e);
            }
          }, 10);
        } else if (event.data.type === 'disable-selection') {
          log('Disabling selection from message');
          setTimeout(function() {
            try {
              disableSelection();
            } catch (e) {
              log('Error disabling selection from message: ' + e);
            }
          }, 10);
        }
      }
    } catch (e) {
      log('Error in message handler: ' + e);
    }
  });
  
  log('Parser selection script loaded and message listener registered, ID: ' + SCRIPT_ID);
  
  function initAndNotify() {
    log('Initializing selection script');
    initSelection();
    notifyReady();
  }
  
  if (typeof window !== 'undefined') {
    window.__parserSelectionEnabled = true;
    window.__parserEnableSelection = enableSelection;
    window.__parserDisableSelection = disableSelection;
  }
  
  // Send immediate notification that script is loaded (before initialization)
  notifyReady();
  
  // Try to initialize immediately
  try {
    log('Attempting immediate initialization');
    initAndNotify();
  } catch (e) {
    log('Immediate initialization failed: ' + e);
    // Still notify that script is loaded even if init fails
    notifyReady();
  }
  
  // Also try on DOMContentLoaded
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', function() {
      log('DOM loaded, initializing selection script');
      try {
        initAndNotify();
      } catch (e) {
        log('DOMContentLoaded initialization failed: ' + e);
      }
    });
  } else {
    log('DOM already loaded, initializing again');
    setTimeout(function() {
      try {
        initAndNotify();
      } catch (e) {
        log('Delayed initialization failed: ' + e);
      }
    }, 50);
  }
  
  // Also try on window load
  if (window.addEventListener) {
    window.addEventListener('load', function() {
      log('Window loaded, ensuring script is ready');
      setTimeout(function() {
        try {
          initAndNotify();
        } catch (e) {
          log('Window load initialization failed: ' + e);
        }
      }, 100);
    });
  }
  
  // Multiple fallback attempts with increasing delays
  setTimeout(function() {
    log('Fallback attempt 1 - forcing initialization');
    try {
      initAndNotify();
    } catch (e) {
      log('Fallback 1 failed: ' + e);
    }
  }, 500);
  
  setTimeout(function() {
    log('Fallback attempt 2 - forcing initialization');
    try {
      initAndNotify();
    } catch (e) {
      log('Fallback 2 failed: ' + e);
    }
  }, 1000);
  
  setTimeout(function() {
    log('Fallback attempt 3 - forcing initialization');
    try {
      initAndNotify();
    } catch (e) {
      log('Fallback 3 failed: ' + e);
    }
  }, 2000);
  
  setTimeout(function() {
    log('Final fallback - forcing initialization');
    try {
      initAndNotify();
    } catch (e) {
      log('Final fallback failed: ' + e);
    }
  }, 5000);
})();
`.trim();
  }

  // Функция для обработки HTML и встраивания внешних ресурсов (CSS, изображения) как base64
  // Это делает страницу полностью автономной (как SingleFile)
  async function embedResources(html: string, baseUrl: string, pageFolder: string): Promise<string> {
    console.log('[PageViewer] ===== START embedResources =====');
    console.log('[PageViewer] Processing resources for base URL:', baseUrl, 'folder:', pageFolder);
    console.log('[PageViewer] HTML length:', html.length, 'chars');
    let processedHtml = html;
    
    try {
      // Находим все внешние CSS файлы (улучшенное регулярное выражение)
      // Поддерживает различные варианты: с кавычками, без кавычек, одинарные/двойные кавычки
      // Также ищем через rel="stylesheet" и type="text/css"
      // Более гибкое регулярное выражение, которое находит href в любом порядке атрибутов
      const cssLinkRegex1 = /<link[^>]*rel\s*=\s*["']?stylesheet["']?[^>]*href\s*=\s*["']([^"']+)["'][^>]*>/gi;
      const cssLinkRegex2 = /<link[^>]*type\s*=\s*["']?text\/css["']?[^>]*href\s*=\s*["']([^"']+)["'][^>]*>/gi;
      // Также ищем любые link с href, которые могут быть CSS
      const cssLinkRegex3 = /<link[^>]*href\s*=\s*["']([^"']+\.css[^"']*)["'][^>]*>/gi;
      const cssLinks: Map<string, string> = new Map(); // Map<absoluteUrl, originalUrl>
      let match: RegExpExecArray | null = null;
      
      // Ищем через rel="stylesheet"
      while ((match = cssLinkRegex1.exec(html)) !== null) {
        const cssUrl = match[1];
        // Пропускаем data URLs и blob URLs
        if (!cssUrl.startsWith('data:') && !cssUrl.startsWith('blob:') && !cssUrl.startsWith('http://localhost')) {
          try {
            const absoluteUrl = new URL(cssUrl, baseUrl).href;
            if (!cssLinks.has(absoluteUrl)) {
              cssLinks.set(absoluteUrl, cssUrl);
              console.log('[PageViewer] Found CSS link (rel):', cssUrl, '->', absoluteUrl);
            }
          } catch (e) {
            console.warn('[PageViewer] Invalid CSS URL:', cssUrl, e);
          }
        }
      }
      
      // Ищем через type="text/css"
      match = null;
      while ((match = cssLinkRegex2.exec(html)) !== null) {
        const cssUrl = match[1];
        if (!cssUrl.startsWith('data:') && !cssUrl.startsWith('blob:') && !cssUrl.startsWith('http://localhost')) {
          try {
            const absoluteUrl = new URL(cssUrl, baseUrl).href;
            if (!cssLinks.has(absoluteUrl)) {
              cssLinks.set(absoluteUrl, cssUrl);
              console.log('[PageViewer] Found CSS link (type):', cssUrl, '->', absoluteUrl);
            }
          } catch (e) {
            console.warn('[PageViewer] Invalid CSS URL:', cssUrl, e);
          }
        }
      }
      
      // Ищем любые link с .css в URL
      match = null;
      while ((match = cssLinkRegex3.exec(html)) !== null) {
        const cssUrl = match[1];
        if (!cssUrl.startsWith('data:') && !cssUrl.startsWith('blob:') && !cssUrl.startsWith('http://localhost') && cssUrl.includes('.css')) {
          try {
            const absoluteUrl = new URL(cssUrl, baseUrl).href;
            if (!cssLinks.has(absoluteUrl)) {
              cssLinks.set(absoluteUrl, cssUrl);
              console.log('[PageViewer] Found CSS link (.css):', cssUrl, '->', absoluteUrl);
            }
          } catch (e) {
            console.warn('[PageViewer] Invalid CSS URL:', cssUrl, e);
          }
        }
      }
      
      console.log('[PageViewer] Found', cssLinks.size, 'external CSS files total');
      if (cssLinks.size === 0) {
        console.warn('[PageViewer] WARNING: No CSS files found! HTML snippet:', html.substring(0, 2000));
      }
      
      // Загружаем и сохраняем CSS в папку
      for (const [absoluteUrl, originalUrl] of cssLinks.entries()) {
        try {
          console.log('[PageViewer] Fetching CSS:', absoluteUrl);
          const cssBytes = await invoke<number[]>('fetch_resource', { url: absoluteUrl });
          let decodedCss = new TextDecoder('utf-8').decode(new Uint8Array(cssBytes));
          
          // Обрабатываем относительные URL внутри CSS (url() для изображений, шрифтов и т.д.)
          const cssBaseUrl = new URL(absoluteUrl);
          const cssDir = cssBaseUrl.href.substring(0, cssBaseUrl.href.lastIndexOf('/') + 1);
          
          // Заменяем относительные URL в CSS на абсолютные (для последующей загрузки)
          const urlRegex = /url\(["']?([^"')]+)["']?\)/gi;
          decodedCss = decodedCss.replace(urlRegex, (match, url) => {
            // Пропускаем data URLs, blob URLs и абсолютные URL
            if (url.startsWith('data:') || url.startsWith('blob:') || url.startsWith('http://') || url.startsWith('https://') || url.startsWith('//')) {
              return match;
            }
            try {
              // Пробуем сначала относительно директории CSS файла
              let absoluteCssUrl: string;
              try {
                absoluteCssUrl = new URL(url, cssDir).href;
              } catch (e1) {
                // Если не получилось, пробуем относительно базового URL страницы
                try {
                  absoluteCssUrl = new URL(url, baseUrl).href;
                } catch (e2) {
                  console.warn('[PageViewer] Invalid URL in CSS:', url, 'CSS dir:', cssDir, 'Base URL:', baseUrl);
                  return match;
                }
              }
              return match.replace(url, absoluteCssUrl);
            } catch (e) {
              console.warn('[PageViewer] Error processing URL in CSS:', url, e);
              return match;
            }
          });
          
          // Встраиваем CSS как inline стиль (base64 не нужен для текста)
          // Используем String.fromCharCode для формирования тегов
          const styleOpenTag = [60, 115, 116, 121, 108, 101, 32, 116, 121, 112, 101, 61, 34, 116, 101, 120, 116, 47, 99, 115, 115, 34, 62].map(c => String.fromCharCode(c)).join('');
          const styleCloseTag = [60, 47, 115, 116, 121, 108, 101, 62].map(c => String.fromCharCode(c)).join('');
          const inlineStyle = styleOpenTag + decodedCss + styleCloseTag;
          
          // Также сохраняем CSS в папку для локального сохранения
          try {
            const cssSubfolder = `${pageFolder}/css`;
            await invoke<string>('save_resource', {
              url: absoluteUrl,
              data: cssBytes,
              subfolder: cssSubfolder
            });
            console.log('[PageViewer] Also saved CSS to folder:', cssSubfolder);
          } catch (e) {
            console.warn('[PageViewer] Failed to save CSS to folder:', e);
          }
          
          // Заменяем ссылку на CSS на inline стиль
          const escapedOriginalUrl = originalUrl.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
          const escapedAbsoluteUrl = absoluteUrl.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
          
          // Ищем и заменяем по обоим вариантам URL
          const linkRegex1 = new RegExp(`<link[^>]*href\\s*=\\s*["']${escapedOriginalUrl}["'][^>]*>`, 'gi');
          const linkRegex2 = new RegExp(`<link[^>]*href\\s*=\\s*["']${escapedAbsoluteUrl}["'][^>]*>`, 'gi');
          
          const beforeReplace = processedHtml;
          processedHtml = processedHtml.replace(linkRegex1, inlineStyle);
          processedHtml = processedHtml.replace(linkRegex2, inlineStyle);
          
          if (beforeReplace === processedHtml) {
            console.warn('[PageViewer] CSS link not found for replacement:', originalUrl, 'or', absoluteUrl);
            // Попробуем более широкий поиск
            const wideRegex = new RegExp(`<link[^>]*${escapedOriginalUrl}[^>]*>`, 'gi');
            processedHtml = processedHtml.replace(wideRegex, inlineStyle);
          }
          
          console.log('[PageViewer] Embedded CSS inline:', absoluteUrl, 'Size:', decodedCss.length, 'chars');
        } catch (e) {
          console.warn('[PageViewer] Failed to fetch/save CSS:', absoluteUrl, e);
        }
      }
      
      // Находим все изображения в уже обработанном HTML (после встраивания CSS)
      // Более гибкое регулярное выражение для поиска изображений
      const imgRegex1 = /<img[^>]*src\s*=\s*["']([^"']+)["'][^>]*>/gi;
      const imgRegex2 = /<img[^>]*src\s*=\s*([^\s>]+)[^>]*>/gi; // Без кавычек
      const imgUrls: Map<string, string> = new Map(); // Map<absoluteUrl, originalUrl>
      match = null;
      
      // Ищем изображения в HTML тегах (с кавычками)
      while ((match = imgRegex1.exec(processedHtml)) !== null) {
        const imgUrl = match[1];
        // Пропускаем data URLs и blob URLs
        if (!imgUrl.startsWith('data:') && !imgUrl.startsWith('blob:') && !imgUrl.startsWith('http://localhost')) {
          try {
            const absoluteUrl = new URL(imgUrl, baseUrl).href;
            // Сохраняем оригинальный URL для замены
            if (!imgUrls.has(absoluteUrl)) {
              imgUrls.set(absoluteUrl, imgUrl);
              console.log('[PageViewer] Found image (img tag):', imgUrl, '->', absoluteUrl);
            }
          } catch (e) {
            console.warn('[PageViewer] Invalid image URL:', imgUrl, e);
          }
        }
      }
      
      // Ищем изображения без кавычек
      match = null;
      while ((match = imgRegex2.exec(processedHtml)) !== null) {
        const imgUrl = match[1];
        if (!imgUrl.startsWith('data:') && !imgUrl.startsWith('blob:') && !imgUrl.startsWith('http://localhost') && !imgUrl.includes('=')) {
          try {
            const absoluteUrl = new URL(imgUrl, baseUrl).href;
            if (!imgUrls.has(absoluteUrl)) {
              imgUrls.set(absoluteUrl, imgUrl);
              console.log('[PageViewer] Found image (img tag, no quotes):', imgUrl, '->', absoluteUrl);
            }
          } catch (e) {
            // Игнорируем ошибки для этого варианта
          }
        }
      }
      
      // Также ищем изображения в CSS (background-image) - теперь в processedHtml
      const bgImgRegex = /background-image\s*:\s*url\(["']?([^"')]+)["']?\)/gi;
      match = null;
      while ((match = bgImgRegex.exec(processedHtml)) !== null) {
        const imgUrl = match[1];
        if (!imgUrl.startsWith('data:') && !imgUrl.startsWith('blob:') && !imgUrl.startsWith('http://localhost')) {
          try {
            const absoluteUrl = new URL(imgUrl, baseUrl).href;
            if (!imgUrls.has(absoluteUrl)) {
              imgUrls.set(absoluteUrl, imgUrl);
              console.log('[PageViewer] Found image (CSS background):', imgUrl, '->', absoluteUrl);
            }
          } catch (e) {
            console.warn('[PageViewer] Invalid background image URL:', imgUrl, e);
          }
        }
      }
      
      console.log('[PageViewer] Found', imgUrls.size, 'images to embed total');
      if (imgUrls.size === 0) {
        console.warn('[PageViewer] WARNING: No images found!');
      }
      
      // Загружаем и встраиваем изображения как base64 data URLs
      for (const [absoluteUrl, originalUrl] of imgUrls.entries()) {
        try {
          console.log('[PageViewer] Fetching image:', absoluteUrl);
          const imgBytes = await invoke<number[]>('fetch_resource', { url: absoluteUrl });
          
          // Конвертируем в base64
          const uint8Array = new Uint8Array(imgBytes);
          let binary = '';
          const chunkSize = 8192;
          for (let i = 0; i < uint8Array.length; i += chunkSize) {
            const chunk = uint8Array.slice(i, i + chunkSize);
            binary += String.fromCharCode.apply(null, Array.from(chunk));
          }
          const base64 = btoa(binary);
          
          // Определяем MIME тип
          let mimeType = 'image/png';
          const urlLower = absoluteUrl.toLowerCase();
          if (urlLower.match(/\.(jpg|jpeg)$/)) mimeType = 'image/jpeg';
          else if (urlLower.match(/\.gif$/)) mimeType = 'image/gif';
          else if (urlLower.match(/\.webp$/)) mimeType = 'image/webp';
          else if (urlLower.match(/\.svg$/)) mimeType = 'image/svg+xml';
          else if (urlLower.match(/\.ico$/)) mimeType = 'image/x-icon';
          
          const dataUrl = `data:${mimeType};base64,${base64}`;
          
          // Также сохраняем изображение в папку для локального сохранения
          try {
            const imgSubfolder = `${pageFolder}/images`;
            await invoke<string>('save_resource', {
              url: absoluteUrl,
              data: imgBytes,
              subfolder: imgSubfolder
            });
            console.log('[PageViewer] Also saved image to folder:', imgSubfolder);
          } catch (e) {
            console.warn('[PageViewer] Failed to save image to folder:', e);
          }
          
          // Заменяем все вхождения оригинального URL на data URL
          const escapedOriginalUrl = originalUrl.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
          const escapedAbsoluteUrl = absoluteUrl.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
          
          // Заменяем в атрибутах src
          const imgSrcRegex1 = new RegExp(`(src\\s*=\\s*["'])${escapedOriginalUrl}(["'])`, 'gi');
          processedHtml = processedHtml.replace(imgSrcRegex1, `$1${dataUrl}$2`);
          
          const imgSrcRegex2 = new RegExp(`(src\\s*=\\s*["'])${escapedAbsoluteUrl}(["'])`, 'gi');
          processedHtml = processedHtml.replace(imgSrcRegex2, `$1${dataUrl}$2`);
          
          // Также заменяем в CSS background-image
          const bgImgUrlRegex1 = new RegExp(`(background-image\\s*:\\s*url\\(["']?)${escapedOriginalUrl}(["']?\\))`, 'gi');
          processedHtml = processedHtml.replace(bgImgUrlRegex1, `$1${dataUrl}$2`);
          
          const bgImgUrlRegex2 = new RegExp(`(background-image\\s*:\\s*url\\(["']?)${escapedAbsoluteUrl}(["']?\\))`, 'gi');
          processedHtml = processedHtml.replace(bgImgUrlRegex2, `$1${dataUrl}$2`);
          
          console.log('[PageViewer] Embedded image as base64:', absoluteUrl, 'Size:', imgBytes.length, 'bytes');
        } catch (e) {
          console.warn('[PageViewer] Failed to fetch/embed image:', absoluteUrl, e);
        }
      }
      
      console.log('[PageViewer] Resources embedded successfully');
      console.log('[PageViewer] Final HTML length:', processedHtml.length, 'chars');
      console.log('[PageViewer] ===== END embedResources =====');
    } catch (e) {
      console.error('[PageViewer] Error embedding resources:', e);
      console.error('[PageViewer] Error stack:', e instanceof Error ? e.stack : 'No stack');
      // Возвращаем оригинальный HTML в случае ошибки
      return html;
    }
    
    return processedHtml;
  }

  async function loadPage() {
    if (!url) {
      console.warn('[PageViewer] loadPage called with empty URL');
      return;
    }
    
    console.log('[PageViewer] loadPage called with URL:', url);
    loading = true;
    error = null;
    scriptReady = false;

      try {
        try {
          new URL(url);
          console.log('[PageViewer] URL validated:', url);
        } catch (e) {
          console.error('[PageViewer] Invalid URL format:', url, e);
          error = 'Неверный формат URL';
          loading = false;
          return;
        }
        
        // Создаем папку для страницы один раз (используем для всех операций)
        const urlObj = new URL(url);
        const timestamp = Date.now();
        const pageFolder = `page_${timestamp}_${urlObj.hostname.replace(/[^a-z0-9]/gi, '_')}`;
        console.log('[PageViewer] Created page folder:', pageFolder);
        
        console.log('[PageViewer] Fetching page via Tauri backend...');
        let html = await invoke<string>('fetch_page', { url });
        console.log('[PageViewer] Page fetched, HTML length:', html.length, 'chars');
        
        // Обрабатываем и сохраняем внешние ресурсы (CSS, изображения) в папку
        console.log('[PageViewer] Processing external resources in folder:', pageFolder);
        html = await embedResources(html, url, pageFolder);
        console.log('[PageViewer] Resources processed, HTML length:', html.length, 'chars');
        
        const maxSize = 10 * 1024 * 1024;
      const htmlSizeMB = Math.round(html.length / 1024 / 1024);
      console.log('[PageViewer] HTML size:', htmlSizeMB, 'MB');
      
      if (html.length > maxSize) {
        console.error('[PageViewer] HTML too large:', htmlSizeMB, 'MB');
        error = `Страница слишком большая (${htmlSizeMB}MB). Максимальный размер: 10MB`;
        loading = false;
        return;
      }
      
      console.log('[PageViewer] Embedding selection script...');
      
      // Удаляем ВСЕ JavaScript скрипты из HTML, кроме нашего
      let modifiedHtml = html;
      
      // Улучшенное удаление скриптов - обрабатываем все варианты:
      // 1. Встроенные скрипты
      // 2. Внешние скрипты через src
      // 3. Самозакрывающиеся теги
      // 4. Скрипты с различными атрибутами (type, async, defer и т.д.)
      // 5. Скрипты с параметрами в URL
      
      // Сначала удаляем все встроенные скрипты (содержимое между тегами)
      modifiedHtml = modifiedHtml.replace(/<script[^>]*>[\s\S]*?<\/script>/gi, '<!-- Removed script -->');
      
      // Затем удаляем все оставшиеся теги script (внешние, самозакрывающиеся и т.д.)
      // Это обрабатывает случаи, когда первый regex не сработал
      modifiedHtml = modifiedHtml.replace(/<script[^>]*>/gi, '<!-- Removed script tag -->');
      modifiedHtml = modifiedHtml.replace(/<\/script>/gi, '<!-- Removed script close tag -->');
      
      // Удаляем самозакрывающиеся теги script (на случай, если они есть)
      modifiedHtml = modifiedHtml.replace(/<script[^>]*\/\s*>/gi, '<!-- Removed self-closing script -->');
      
      // Удаляем inline обработчики событий из всех тегов (onclick, onmouseover и т.д.)
      const inlineHandlers = [
        'onclick', 'onmouseover', 'onmouseout', 'onmousedown', 'onmouseup', 'onmousemove',
        'ontouchstart', 'ontouchend', 'ontouchmove', 'onload', 'onerror', 'onchange',
        'onsubmit', 'onfocus', 'onblur', 'onkeydown', 'onkeyup', 'onkeypress',
        'ondblclick', 'oncontextmenu', 'onwheel', 'onscroll', 'onresize'
      ];
      
      inlineHandlers.forEach(handler => {
        // Удаляем атрибуты вида onclick="..." или onclick='...'
        const regex = new RegExp('\\s+' + handler + '\\s*=\\s*["\'][^"\']*["\']', 'gi');
        modifiedHtml = modifiedHtml.replace(regex, '');
      });
      
      // Удаляем обработчики событий в стиле addEventListener из оставшегося HTML
      // (на случай, если они были встроены в атрибуты или data-атрибуты)
      modifiedHtml = modifiedHtml.replace(/\s+data-[^=]*=\s*["\'][^"\']*addEventListener[^"\']*["\']/gi, '');
      
      // Удаляем data-атрибуты, которые могут содержать JavaScript код
      modifiedHtml = modifiedHtml.replace(/\s+data-[^=]*=\s*["\'][^"\']*javascript[^"\']*["\']/gi, '');
      modifiedHtml = modifiedHtml.replace(/\s+data-[^=]*=\s*["\'][^"\']*on\w+[^"\']*["\']/gi, '');
      
      // Удаляем noscript теги (они могут содержать альтернативный контент со скриптами)
      modifiedHtml = modifiedHtml.replace(/<noscript[^>]*>[\s\S]*?<\/noscript>/gi, '<!-- Removed noscript -->');
      
      console.log('[PageViewer] Removed all JavaScript scripts and event handlers (except our selection script)');
      
      const scriptContent = getSelectionScriptContent();
      
      const scriptCloseTag = '</' + 'script>';
      const escapedScript = scriptContent
        .replace(new RegExp(scriptCloseTag, 'g'), '<\\/script>')
        .replace(/<!--/g, '<\\!--')
        .replace(/-->/g, '--\\>')
        .replace(/<\/script>/gi, '<\\/script>');
      const scriptTagStart = '<script id="parser-selection-script" type="text/javascript">';
      const scriptTagEnd = '</' + 'script>';
      const fullScriptTag = scriptTagStart + escapedScript + scriptTagEnd;
      
      console.log('[PageViewer] Script tag length:', fullScriptTag.length, 'chars');
      console.log('[PageViewer] Script content preview:', scriptContent.substring(0, 200) + '...');
      
      // Добавляем CSS стили для гарантированного отображения рамки
      const cssStyles = getSelectionStyles();
      
      // Insert script and styles as early as possible - right after <!DOCTYPE> or at the very beginning
      // Используем modifiedHtml (уже без скриптов), а не html
      if (modifiedHtml.includes('<!DOCTYPE') || modifiedHtml.includes('<!doctype')) {
        // Insert right after DOCTYPE
        modifiedHtml = modifiedHtml.replace(/(<!DOCTYPE[^>]*>)/i, '$1' + cssStyles + fullScriptTag);
        console.log('[PageViewer] Script and styles inserted after DOCTYPE');
      } else if (modifiedHtml.includes('<html')) {
        // Insert right after <html> tag
        modifiedHtml = modifiedHtml.replace(/(<html[^>]*>)/i, '$1' + cssStyles + fullScriptTag);
        console.log('[PageViewer] Script and styles inserted after <html> tag');
      } else if (modifiedHtml.includes('</head>')) {
        modifiedHtml = modifiedHtml.replace('</head>', cssStyles + fullScriptTag + '</head>');
        console.log('[PageViewer] Script and styles inserted before </head>');
      } else if (modifiedHtml.includes('<head>')) {
        modifiedHtml = modifiedHtml.replace('<head>', '<head>' + cssStyles + fullScriptTag);
        console.log('[PageViewer] Script and styles inserted after <head>');
      } else if (modifiedHtml.includes('<body')) {
        modifiedHtml = modifiedHtml.replace(/(<body[^>]*>)/i, '$1' + cssStyles + fullScriptTag);
        console.log('[PageViewer] Script and styles inserted after <body>');
      } else if (modifiedHtml.includes('</body>')) {
        modifiedHtml = modifiedHtml.replace('</body>', cssStyles + fullScriptTag + '</body>');
        console.log('[PageViewer] Script and styles inserted before </body>');
      } else {
        // Last resort: prepend to HTML
        modifiedHtml = cssStyles + fullScriptTag + modifiedHtml;
        console.log('[PageViewer] Script and styles prepended to HTML');
      }
      
      if (!modifiedHtml.includes('parser-selection-script')) {
        console.error('[PageViewer] WARNING: Script was not inserted into HTML!');
      } else {
        console.log('[PageViewer] Script successfully inserted into HTML');
      }
      
      if (!iframeRef) {
        console.error('[PageViewer] Iframe not initialized');
        error = 'Iframe не инициализирован';
        loading = false;
        return;
      }
      
      // Declare variables outside try block for error handling
      let blobUrl: string | null = null;
      let blobCleaned = false;
      
      // Save page locally in the page folder (используем ту же папку, что и для ресурсов)
      let fileUrl: string | null = null;
      if (modifiedHtml.length < 10 * 1024 * 1024) {
        try {
          const filename = `${pageFolder}/index.html`;
          console.log('[PageViewer] Saving page locally in folder:', pageFolder);
          fileUrl = await invoke<string>('save_page_local', { 
            html: modifiedHtml, 
            filename 
          });
          console.log('[PageViewer] Page saved locally, file URL:', fileUrl);
        } catch (e) {
          console.warn('[PageViewer] Failed to save page locally:', e);
        }
      } else {
        console.log('[PageViewer] Page too large for local save');
      }
      
      // Use Blob URL for iframe (file:// URLs are blocked by browser security)
      
      try {
        console.log('[PageViewer] Creating Blob URL for iframe...');
        const blob = new Blob([modifiedHtml], { type: 'text/html;charset=utf-8' });
        blobUrl = URL.createObjectURL(blob);
        console.log('[PageViewer] Blob URL created:', blobUrl.substring(0, 50) + '...');
      } catch (e) {
        console.error('[PageViewer] Failed to create Blob URL:', e);
        error = 'Не удалось создать URL для загрузки страницы.';
        loading = false;
        return;
      }
      
      const cleanupBlob = () => {
        if (!blobCleaned && blobUrl) {
          try {
            URL.revokeObjectURL(blobUrl);
            console.log('[PageViewer] Blob URL revoked');
          } catch (e) {
            console.warn('[PageViewer] Failed to revoke blob URL:', e);
          }
          blobCleaned = true;
        }
      };
      
      // Wait for iframe to fully load
      let loadResolve: (() => void) | null = null;
      let loadReject: ((error: Error) => void) | null = null;
      const loadPromise = new Promise<void>((resolve, reject) => {
        loadResolve = resolve;
        loadReject = reject;
      });
      
      let loadTimeout: ReturnType<typeof setTimeout> | null = null;
      const LOAD_TIMEOUT = 60000; // 60 seconds timeout (some pages load slowly)
      
      const checkIframeLoaded = () => {
        try {
          const iframeDoc = iframeRef?.contentDocument || iframeRef?.contentWindow?.document;
          if (iframeDoc) {
            // Check readyState - if complete, page is loaded
            if (iframeDoc.readyState === 'complete') {
              console.log('[PageViewer] Iframe document readyState is complete');
              // Don't check body.children.length - some pages load content dynamically
              if (loadTimeout) {
                clearTimeout(loadTimeout);
                loadTimeout = null;
              }
              if (loadResolve) {
                loadResolve();
              }
              return true;
            } else {
              console.log('[PageViewer] Iframe document readyState:', iframeDoc.readyState);
            }
          } else {
            console.log('[PageViewer] Cannot access iframe document (CORS) - assuming loaded after onload event');
          }
        } catch (e) {
          // CORS error - assume page is loaded after onload event
          console.log('[PageViewer] CORS error checking iframe, will rely on onload event');
        }
        return false;
      };
      
      iframeRef.onerror = () => {
        console.error('[PageViewer] Iframe load error');
        if (loadTimeout) {
          clearTimeout(loadTimeout);
          loadTimeout = null;
        }
        cleanupBlob();
        if (loadReject) {
          loadReject(new Error('Failed to load iframe'));
        }
        error = 'Не удалось загрузить страницу';
        loading = false;
      };
      
      iframeRef.onload = () => {
        console.log('[PageViewer] Iframe onload event fired');
        // For Blob URLs, onload usually means page is ready
        // Wait a bit for scripts to execute (especially on slow-loading pages like synthira.ru)
        setTimeout(() => {
          if (checkIframeLoaded()) {
            return;
          }
          // If checkIframeLoaded didn't resolve, resolve anyway after onload
          // (some pages may have CORS restrictions or load content dynamically)
          console.log('[PageViewer] Resolving load promise after onload event (CORS or dynamic content)');
          if (loadTimeout) {
            clearTimeout(loadTimeout);
            loadTimeout = null;
          }
          if (loadResolve) {
            loadResolve();
          }
        }, 2000); // Increased delay for slow-loading pages
      };
      
      try {
        console.log('[PageViewer] Setting iframe src to Blob URL...');
        iframeRef.src = blobUrl;
        console.log('[PageViewer] Iframe src set, waiting for full load...');
        
        // Set timeout for loading
        loadTimeout = setTimeout(() => {
          console.warn('[PageViewer] Iframe load timeout after', LOAD_TIMEOUT, 'ms');
          if (loadReject) {
            loadReject(new Error('Load timeout'));
          }
          error = 'Таймаут загрузки страницы';
          loading = false;
          cleanupBlob();
        }, LOAD_TIMEOUT);
        
        // Wait for page to fully load
        await loadPromise;
        
        console.log('[PageViewer] Page fully loaded');
        loading = false;
        saveRecentUrl(url);
        
        // Wait for script to send ready message via postMessage
        // Don't try to access iframe document directly due to CORS
        console.log('[PageViewer] Waiting for script ready message from iframe...');
        
        const checkReady = setInterval(() => {
          if (scriptReady) {
            clearInterval(checkReady);
            console.log('[PageViewer] Script confirmed ready, checking if selection mode should be enabled:', selectionMode);
            if (selectionMode) {
              enableSelectionInIframe();
            }
          }
        }, 100);
        
        // Give script more time to initialize (some pages load slowly)
        setTimeout(() => {
          clearInterval(checkReady);
          if (!scriptReady) {
            console.warn('[PageViewer] Script ready timeout after 10 seconds - attempting to enable selection anyway');
            console.warn('[PageViewer] Script may not have executed or may be blocked by CSP');
            scriptReady = true; // Mark as ready to allow enabling
            if (selectionMode) {
              enableSelectionInIframe();
            }
          }
        }, 10000);
        
        // Don't cleanup blob URL - keep it for the lifetime of the page
        // The blob will be cleaned up when the component is destroyed
      } catch (e) {
        console.error('[PageViewer] Error in load promise handler:', e);
        if (loadReject) {
          loadReject(e as Error);
        }
        cleanupBlob();
        error = 'Ошибка при загрузке страницы';
        loading = false;
      }
      
    } catch (err: any) {
      console.error('[PageViewer] Error in loadPage:', err);
      console.error('[PageViewer] Error stack:', err?.stack);
      error = err?.message || err?.toString() || 'Неизвестная ошибка';
      loading = false;
      if (blobUrl) {
        try {
          URL.revokeObjectURL(blobUrl);
        } catch (e) {
          // Ignore cleanup errors
        }
      }
    }
  }
  
  function enableSelectionInIframe() {
    if (!iframeRef?.contentWindow) {
      console.warn('[PageViewer] Iframe contentWindow not available, cannot enable selection');
      return;
    }
    
    console.log('[PageViewer] Sending enable-selection message to iframe');
    
    try {
      const iframeWin = iframeRef.contentWindow as any;
      if (iframeWin.__parserEnableSelection) {
        console.log('[PageViewer] Calling enableSelection function directly');
        iframeWin.__parserEnableSelection();
        return;
      }
    } catch (e) {
      console.log('[PageViewer] Cannot call function directly (CORS), using postMessage');
    }
    
    try {
      iframeRef.contentWindow.postMessage({ type: 'enable-selection' }, '*');
      console.log('[PageViewer] Enable-selection message sent via postMessage');
      
      setTimeout(() => {
        iframeRef?.contentWindow?.postMessage({ type: 'enable-selection' }, '*');
        console.log('[PageViewer] Enable-selection message sent again (retry)');
      }, 200);
    } catch (e) {
      console.error('[PageViewer] Failed to send enable-selection message:', e);
    }
  }
  
  function disableSelectionInIframe() {
    if (!iframeRef?.contentWindow) {
      console.warn('[PageViewer] Iframe contentWindow not available, cannot disable selection');
      return;
    }
    
    console.log('[PageViewer] Sending disable-selection message to iframe');
    
    try {
      const iframeWin = iframeRef.contentWindow as any;
      if (iframeWin.__parserDisableSelection) {
        iframeWin.__parserDisableSelection();
        return;
      }
    } catch (e) {
    }
    
    try {
      iframeRef.contentWindow.postMessage({ type: 'disable-selection' }, '*');
    } catch (e) {
      console.error('[PageViewer] Failed to send disable-selection message:', e);
    }
  }
  
  $effect(() => {
    function handleMessage(event: MessageEvent) {
      if (!event.data || typeof event.data !== 'object') {
        return;
      }
      
      const validTypes = ['parser-script-ready', 'element-selected', 'parser-debug'];
      if (!validTypes.includes(event.data.type)) {
        return;
      }
      
      console.log('[PageViewer] Received valid message from iframe:', event.data.type, 'origin:', event.origin);
      
      if (event.data.type === 'parser-debug') {
        console.log('[PageViewer] [IFRAME DEBUG]:', event.data.message);
        return;
      }
      
      if (event.data.type === 'parser-script-ready') {
        console.log('[PageViewer] Script confirmed ready in iframe, scriptId:', event.data.scriptId, 'attempt:', event.data.attempt);
        scriptReady = true;
        if (selectionMode) {
          console.log('[PageViewer] Selection mode is active, enabling selection in iframe');
          setTimeout(() => {
            enableSelectionInIframe();
          }, 100);
        }
        return;
      }
      
      if (event.data.type === 'element-selected') {
        const { selector, tagName, text } = event.data;
        console.log('[PageViewer] Element selected:', { selector, tagName, text });
        console.log('[PageViewer] Element data:', event.data);
        
        if (onElementSelect) {
          console.log('[PageViewer] Calling onElementSelect callback');
          const mockElement = {
            tagName,
            textContent: text,
          } as HTMLElement;
          onElementSelect(selector, mockElement, {
            tagName,
            text,
            attributes: event.data.attributes || {},
            similarElements: event.data.similarElements,
          });
          console.log('[PageViewer] onElementSelect callback completed');
        } else {
          console.warn('[PageViewer] onElementSelect callback not provided');
        }
        
        selectionMode = false;
        disableSelectionInIframe();
      }
    }
    
    console.log('[PageViewer] Setting up message listener');
    window.addEventListener('message', handleMessage);
    return () => {
      console.log('[PageViewer] Cleaning up message listener');
      window.removeEventListener('message', handleMessage);
    };
  });


  function toggleSelectionMode() {
    const oldState = selectionMode;
    selectionMode = !selectionMode;
    console.log('[PageViewer] toggleSelectionMode called, changed from', oldState, 'to', selectionMode);
    if (selectionMode) {
      // Try to enable immediately
      enableSelectionInIframe();
      
      // Also set up periodic checks
      if (!scriptReady) {
        console.log('[PageViewer] Script not ready yet, setting up periodic checks...');
        let attempts = 0;
        const maxAttempts = 50; // 5 seconds with 100ms intervals
        
        const checkReady = setInterval(() => {
          attempts++;
          
          // Try to enable selection on each check
          enableSelectionInIframe();
          
          // Check if script is ready by trying to access functions
          if (iframeRef?.contentWindow) {
            try {
              const iframeWin = iframeRef.contentWindow as any;
              if (iframeWin.__parserEnableSelection || iframeWin.__parserSelectionEnabled) {
                scriptReady = true;
                clearInterval(checkReady);
                console.log('[PageViewer] Script confirmed ready via function check');
                enableSelectionInIframe();
                return;
              }
            } catch (e) {
              // CORS error, continue checking
            }
          }
          
          if (scriptReady) {
            clearInterval(checkReady);
            console.log('[PageViewer] Script became ready, enabling selection');
            enableSelectionInIframe();
          } else if (attempts >= maxAttempts) {
            clearInterval(checkReady);
            console.warn('[PageViewer] Script ready timeout after', attempts, 'attempts, continuing anyway');
            // Continue trying anyway - script might still work
          }
        }, 100);
      }
    } else {
      console.log('[PageViewer] Disabling selection mode');
      disableSelectionInIframe();
    }
  }

  $effect(() => {
    function handleClickOutside(e: MouseEvent) {
      const target = e.target as HTMLElement;
      if (!target.closest('.url-input-wrapper')) {
        showRecentUrls = false;
      }
    }
    
    if (showRecentUrls) {
      document.addEventListener('click', handleClickOutside);
      return () => {
        document.removeEventListener('click', handleClickOutside);
      };
    }
  });
</script>

<div class="page-viewer">
  <div class="viewer-header" class:collapsed={panelCollapsed}>
    <button 
      class="btn-collapse" 
      onclick={() => panelCollapsed = !panelCollapsed}
      aria-label={panelCollapsed ? 'Развернуть панель' : 'Свернуть панель'}
      title={panelCollapsed ? 'Развернуть панель' : 'Свернуть панель'}
    >
      {panelCollapsed ? '▼' : '▲'}
    </button>
    <div class="header-content">
      <div class="url-input-group">
        <div class="url-input-wrapper">
          <input 
            type="url" 
            value={url}
            oninput={(e) => url = e.currentTarget.value}
            onfocus={() => showRecentUrls = recentUrls.length > 0}
            placeholder="https://example.com"
            class="url-input"
          />
          {#if showRecentUrls && recentUrls.length > 0}
            <div class="recent-urls-dropdown">
              <div class="recent-urls-header">Последние открытые:</div>
              {#each recentUrls as recentUrl}
                <div class="recent-url-item">
                  <button 
                    class="recent-url-link"
                    onclick={() => selectRecentUrl(recentUrl)}
                  >
                    {recentUrl}
                  </button>
                  <button 
                    class="recent-url-remove"
                    onclick={() => removeRecentUrl(recentUrl)}
                    aria-label="Удалить"
                  >
                    ×
                  </button>
                </div>
              {/each}
            </div>
          {/if}
        </div>
        <button class="btn-load" onclick={loadPage} disabled={loading}>
          {loading ? '⏳' : '📥'}
        </button>
      </div>
      <button 
        class="btn-select" 
        class:active={selectionMode}
        onclick={toggleSelectionMode}
      >
        {selectionMode ? '✓' : '🎯'}
      </button>
      {#if selectionMode}
        <div class="selection-mode-hint">
          🎯 Режим выделения активен
        </div>
      {/if}
    </div>
  </div>

  {#if error}
    <div class="error-message">
      Ошибка: {error}
    </div>
  {/if}

  <div class="iframe-container">
    {#if loading}
      <div class="loading">Загрузка страницы...</div>
    {/if}
    <iframe
      bind:this={iframeRef}
      class="page-iframe"
      sandbox="allow-same-origin allow-scripts allow-forms allow-popups allow-top-navigation"
    ></iframe>
  </div>
</div>

<style>
  .page-viewer {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: #0f172a;
  }

  .viewer-header {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem;
    border-bottom: 1px solid #334155;
    align-items: center;
    transition: all 0.3s ease;
    min-height: 3rem;
  }
  
  .viewer-header.collapsed {
    min-height: 2.5rem;
  }
  
  .viewer-header.collapsed .header-content {
    display: none;
  }
  
  .btn-collapse {
    padding: 0.25rem 0.5rem;
    background: #334155;
    color: #e2e8f0;
    border: none;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    cursor: pointer;
    transition: all 0.2s;
    min-width: 1.5rem;
    height: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .btn-collapse:hover {
    background: #475569;
  }
  
  .header-content {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    flex: 1;
    transition: opacity 0.3s ease;
  }

  .url-input-group {
    display: flex;
    gap: 0.375rem;
    flex: 1;
  }

  .url-input-wrapper {
    position: relative;
    flex: 1;
  }

  .url-input {
    width: 100%;
    padding: 0.375rem 0.5rem;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    color: #e2e8f0;
    font-size: 0.8125rem;
  }

  .url-input:focus {
    outline: none;
    border-color: #0ea5e9;
  }

  .recent-urls-dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    margin-top: 0.25rem;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    max-height: 300px;
    overflow-y: auto;
    z-index: 1000;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .recent-urls-header {
    padding: 0.375rem 0.5rem;
    font-size: 0.6875rem;
    color: #94a3b8;
    border-bottom: 1px solid #334155;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .recent-url-item {
    display: flex;
    align-items: center;
    padding: 0.375rem 0.5rem;
    border-bottom: 1px solid #334155;
    transition: background 0.2s;
  }

  .recent-url-item:last-child {
    border-bottom: none;
  }

  .recent-url-item:hover {
    background: #0f172a;
  }

  .recent-url-link {
    flex: 1;
    text-align: left;
    background: none;
    border: none;
    color: #e2e8f0;
    font-size: 0.8125rem;
    cursor: pointer;
    padding: 0.125rem 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .recent-url-link:hover {
    color: #0ea5e9;
  }

  .recent-url-remove {
    background: none;
    border: none;
    color: #94a3b8;
    font-size: 1rem;
    cursor: pointer;
    padding: 0.125rem 0.375rem;
    line-height: 1;
    border-radius: 0.25rem;
    transition: all 0.2s;
    min-width: 1.5rem;
    height: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .recent-url-remove:hover {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.1);
  }

  .btn-load,
  .btn-select {
    padding: 0.375rem 0.75rem;
    background: #0ea5e9;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-weight: 500;
    font-size: 0.8125rem;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
    min-width: 2.5rem;
    height: 1.875rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .btn-load:hover:not(:disabled),
  .btn-select:hover:not(:disabled) {
    background: #0284c7;
  }

  .btn-load:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-select.active {
    background: #10b981;
  }

  .error-message {
    padding: 1rem;
    background: #7f1d1d;
    color: #fca5a5;
    border-bottom: 1px solid #334155;
  }

  .iframe-container {
    flex: 1;
    position: relative;
    overflow: hidden;
  }

  .loading {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: #94a3b8;
    font-size: 1.1rem;
  }

  .page-iframe {
    width: 100%;
    height: 100%;
    border: none;
    background: white;
  }

  .selection-mode-hint {
    padding: 0.25rem 0.5rem;
    background: rgba(16, 185, 129, 0.15);
    border: 1px solid rgba(16, 185, 129, 0.3);
    border-radius: 0.375rem;
    color: #10b981;
    font-size: 0.75rem;
    font-weight: 500;
    animation: pulse-hint 2s ease-in-out infinite;
    white-space: nowrap;
  }

  @keyframes pulse-hint {
    0%, 100% {
      opacity: 1;
      box-shadow: 0 0 0 0 rgba(16, 185, 129, 0.4);
    }
    50% {
      opacity: 0.8;
      box-shadow: 0 0 0 4px rgba(16, 185, 129, 0);
    }
  }
</style>

