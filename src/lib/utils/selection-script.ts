/**
 * Утилиты для генерации скрипта выделения элементов
 * Вынесено из PageViewer для разделения логики
 */

/**
 * Получает CSS стили для рамки выделения элементов
 * @returns HTML строка со стилями
 */
export function getSelectionStyles(): string {
  const styleOpen = String.fromCharCode(60, 115, 116, 121, 108, 101, 32, 105, 100, 61, 34, 112, 97, 114, 115, 101, 114, 45, 115, 101, 108, 101, 99, 116, 105, 111, 110, 45, 115, 116, 121, 108, 101, 115, 34, 62);
  const hoverBoxCss = '#parser-hover-box {position: absolute !important; pointer-events: none !important; z-index: 2147483647 !important; border: 2px solid #4285f4 !important; background: rgba(66, 133, 244, 0.15) !important; box-shadow: 0 0 0 2px rgba(66, 133, 244, 0.4), 0 0 12px rgba(66, 133, 244, 0.3) !important; transition: all 100ms cubic-bezier(0.4, 0, 0.2, 1) !important; display: none !important; box-sizing: border-box !important; margin: 0 !important; padding: 0 !important; visibility: visible !important; opacity: 1 !important;}';
  const overlayCss = '#parser-info-overlay {position: fixed !important; background: #1e1e1e !important; color: #d4d4d4 !important; padding: 6px 10px !important; border-radius: 3px !important; font-size: 11px !important; font-family: "Consolas", "Monaco", "Courier New", monospace !important; pointer-events: none !important; z-index: 2147483647 !important; border: 1px solid #4285f4 !important; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3) !important; display: none !important; line-height: 1.4 !important; max-width: 300px !important; word-wrap: break-word !important; margin: 0 !important; visibility: visible !important; opacity: 1 !important;}';
  const styleClose = String.fromCharCode(60, 47, 115, 116, 121, 108, 101, 62);
  return styleOpen + hoverBoxCss + overlayCss + styleClose;
}

/**
 * Получает JavaScript код для функционала выделения элементов на странице
 * @returns JavaScript код в виде строки
 */
export function getSelectionScriptContent(): string {
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
    
    try {
      e.preventDefault();
      e.stopPropagation();
      e.stopImmediatePropagation();
    } catch (err) {
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
    
    updateHoverBox(target);
    showElementInfo(target, e.clientX, e.clientY);
    
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
  
  notifyReady();
  
  try {
    log('Attempting immediate initialization');
    initAndNotify();
  } catch (e) {
    log('Immediate initialization failed: ' + e);
    notifyReady();
  }
  
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
