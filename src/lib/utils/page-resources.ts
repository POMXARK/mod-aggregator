/**
 * Утилиты для обработки ресурсов страницы (CSS, изображения)
 * Вынесено из PageViewer для разделения логики
 */
import { invoke } from '../tauri-wrapper';

/**
 * Обрабатывает HTML и встраивает внешние ресурсы (CSS, изображения) как base64
 * Это делает страницу полностью автономной (как SingleFile)
 * @param html - исходный HTML
 * @param baseUrl - базовый URL страницы
 * @param pageFolder - папка для сохранения ресурсов
 * @returns обработанный HTML с встроенными ресурсами
 */
export async function embedResources(
  html: string,
  baseUrl: string,
  pageFolder: string
): Promise<string> {
  console.log('[PageResources] ===== START embedResources =====');
  console.log('[PageResources] Processing resources for base URL:', baseUrl, 'folder:', pageFolder);
  console.log('[PageResources] HTML length:', html.length, 'chars');
  let processedHtml = html;

  try {
    // Обрабатываем CSS файлы
    processedHtml = await processCssFiles(processedHtml, baseUrl, pageFolder);
    
    // Обрабатываем изображения
    processedHtml = await processImages(processedHtml, baseUrl, pageFolder);

    console.log('[PageResources] Resources embedded successfully');
    console.log('[PageResources] Final HTML length:', processedHtml.length, 'chars');
    console.log('[PageResources] ===== END embedResources =====');
  } catch (e) {
    console.error('[PageResources] Error embedding resources:', e);
    console.error('[PageResources] Error stack:', e instanceof Error ? e.stack : 'No stack');
    return html;
  }

  return processedHtml;
}

/**
 * Обрабатывает CSS файлы: находит, загружает и встраивает их
 */
async function processCssFiles(
  html: string,
  baseUrl: string,
  pageFolder: string
): Promise<string> {
  const cssLinkRegex1 = /<link[^>]*rel\s*=\s*["']?stylesheet["']?[^>]*href\s*=\s*["']([^"']+)["'][^>]*>/gi;
  const cssLinkRegex2 = /<link[^>]*type\s*=\s*["']?text\/css["']?[^>]*href\s*=\s*["']([^"']+)["'][^>]*>/gi;
  const cssLinkRegex3 = /<link[^>]*href\s*=\s*["']([^"']+\.css[^"']*)["'][^>]*>/gi;
  const cssLinks: Map<string, string> = new Map();
  let match: RegExpExecArray | null = null;

  // Ищем CSS файлы
  while ((match = cssLinkRegex1.exec(html)) !== null) {
    const cssUrl = match[1];
    if (!cssUrl.startsWith('data:') && !cssUrl.startsWith('blob:') && !cssUrl.startsWith('http://localhost')) {
      try {
        const absoluteUrl = new URL(cssUrl, baseUrl).href;
        if (!cssLinks.has(absoluteUrl)) {
          cssLinks.set(absoluteUrl, cssUrl);
        }
      } catch (e) {
        console.warn('[PageResources] Invalid CSS URL:', cssUrl, e);
      }
    }
  }

  match = null;
  while ((match = cssLinkRegex2.exec(html)) !== null) {
    const cssUrl = match[1];
    if (!cssUrl.startsWith('data:') && !cssUrl.startsWith('blob:') && !cssUrl.startsWith('http://localhost')) {
      try {
        const absoluteUrl = new URL(cssUrl, baseUrl).href;
        if (!cssLinks.has(absoluteUrl)) {
          cssLinks.set(absoluteUrl, cssUrl);
        }
      } catch (e) {
        console.warn('[PageResources] Invalid CSS URL:', cssUrl, e);
      }
    }
  }

  match = null;
  while ((match = cssLinkRegex3.exec(html)) !== null) {
    const cssUrl = match[1];
    if (!cssUrl.startsWith('data:') && !cssUrl.startsWith('blob:') && !cssUrl.startsWith('http://localhost') && cssUrl.includes('.css')) {
      try {
        const absoluteUrl = new URL(cssUrl, baseUrl).href;
        if (!cssLinks.has(absoluteUrl)) {
          cssLinks.set(absoluteUrl, cssUrl);
        }
      } catch (e) {
        console.warn('[PageResources] Invalid CSS URL:', cssUrl, e);
      }
    }
  }

  console.log('[PageResources] Found', cssLinks.size, 'external CSS files total');

  // Загружаем и встраиваем CSS
  let processedHtml = html;
  for (const [absoluteUrl, originalUrl] of cssLinks.entries()) {
    try {
      // fetch_resource теперь автоматически получает app_handle из контекста Tauri
      const cssBytes = await invoke<number[]>('fetch_resource', { url: absoluteUrl });
      let decodedCss = new TextDecoder('utf-8').decode(new Uint8Array(cssBytes));

      // Обрабатываем относительные URL внутри CSS
      const cssBaseUrl = new URL(absoluteUrl);
      const cssDir = cssBaseUrl.href.substring(0, cssBaseUrl.href.lastIndexOf('/') + 1);
      const urlRegex = /url\(["']?([^"')]+)["']?\)/gi;
      decodedCss = decodedCss.replace(urlRegex, (match, url) => {
        if (url.startsWith('data:') || url.startsWith('blob:') || url.startsWith('http://') || url.startsWith('https://') || url.startsWith('//')) {
          return match;
        }
        try {
          let absoluteCssUrl: string;
          try {
            absoluteCssUrl = new URL(url, cssDir).href;
          } catch (e1) {
            try {
              absoluteCssUrl = new URL(url, baseUrl).href;
            } catch (e2) {
              return match;
            }
          }
          return match.replace(url, absoluteCssUrl);
        } catch (e) {
          return match;
        }
      });

      // Встраиваем CSS как inline стиль
      const styleOpenTag = [60, 115, 116, 121, 108, 101, 32, 116, 121, 112, 101, 61, 34, 116, 101, 120, 116, 47, 99, 115, 115, 34, 62].map(c => String.fromCharCode(c)).join('');
      const styleCloseTag = [60, 47, 115, 116, 121, 108, 101, 62].map(c => String.fromCharCode(c)).join('');
      const inlineStyle = styleOpenTag + decodedCss + styleCloseTag;

      // Сохраняем CSS в папку
      try {
        const cssSubfolder = `${pageFolder}/css`;
        await invoke<string>('save_resource', {
          url: absoluteUrl,
          data: cssBytes,
          subfolder: cssSubfolder
        });
      } catch (e) {
        console.warn('[PageResources] Failed to save CSS to folder:', e);
      }

      // Заменяем ссылку на CSS
      const escapedOriginalUrl = originalUrl.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
      const escapedAbsoluteUrl = absoluteUrl.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
      const linkRegex1 = new RegExp(`<link[^>]*href\\s*=\\s*["']${escapedOriginalUrl}["'][^>]*>`, 'gi');
      const linkRegex2 = new RegExp(`<link[^>]*href\\s*=\\s*["']${escapedAbsoluteUrl}["'][^>]*>`, 'gi');
      processedHtml = processedHtml.replace(linkRegex1, inlineStyle);
      processedHtml = processedHtml.replace(linkRegex2, inlineStyle);

      console.log('[PageResources] Embedded CSS inline:', absoluteUrl);
    } catch (e) {
      console.warn('[PageResources] Failed to fetch/save CSS:', absoluteUrl, e);
    }
  }

  return processedHtml;
}

/**
 * Обрабатывает изображения: находит, загружает и встраивает их как base64
 */
async function processImages(
  html: string,
  baseUrl: string,
  pageFolder: string
): Promise<string> {
  const imgRegex1 = /<img[^>]*src\s*=\s*["']([^"']+)["'][^>]*>/gi;
  const imgRegex2 = /<img[^>]*src\s*=\s*([^\s>]+)[^>]*>/gi;
  const bgImgRegex = /background-image\s*:\s*url\(["']?([^"')]+)["']?\)/gi;
  const imgUrls: Map<string, string> = new Map();
  let match: RegExpExecArray | null = null;

  // Ищем изображения
  while ((match = imgRegex1.exec(html)) !== null) {
    const imgUrl = match[1];
    if (!imgUrl.startsWith('data:') && !imgUrl.startsWith('blob:') && !imgUrl.startsWith('http://localhost')) {
      try {
        const absoluteUrl = new URL(imgUrl, baseUrl).href;
        if (!imgUrls.has(absoluteUrl)) {
          imgUrls.set(absoluteUrl, imgUrl);
        }
      } catch (e) {
        console.warn('[PageResources] Invalid image URL:', imgUrl, e);
      }
    }
  }

  match = null;
  while ((match = imgRegex2.exec(html)) !== null) {
    const imgUrl = match[1];
    if (!imgUrl.startsWith('data:') && !imgUrl.startsWith('blob:') && !imgUrl.startsWith('http://localhost') && !imgUrl.includes('=')) {
      try {
        const absoluteUrl = new URL(imgUrl, baseUrl).href;
        if (!imgUrls.has(absoluteUrl)) {
          imgUrls.set(absoluteUrl, imgUrl);
        }
      } catch (e) {
        // Игнорируем ошибки
      }
    }
  }

  match = null;
  while ((match = bgImgRegex.exec(html)) !== null) {
    const imgUrl = match[1];
    if (!imgUrl.startsWith('data:') && !imgUrl.startsWith('blob:') && !imgUrl.startsWith('http://localhost')) {
      try {
        const absoluteUrl = new URL(imgUrl, baseUrl).href;
        if (!imgUrls.has(absoluteUrl)) {
          imgUrls.set(absoluteUrl, imgUrl);
        }
      } catch (e) {
        console.warn('[PageResources] Invalid background image URL:', imgUrl, e);
      }
    }
  }

  console.log('[PageResources] Found', imgUrls.size, 'images to embed total');

  // Загружаем и встраиваем изображения
  let processedHtml = html;
  for (const [absoluteUrl, originalUrl] of imgUrls.entries()) {
    try {
      // fetch_resource теперь автоматически получает app_handle из контекста Tauri
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

      // Сохраняем изображение в папку
      try {
        const imgSubfolder = `${pageFolder}/images`;
        await invoke<string>('save_resource', {
          url: absoluteUrl,
          data: imgBytes,
          subfolder: imgSubfolder
        });
      } catch (e) {
        console.warn('[PageResources] Failed to save image to folder:', e);
      }

      // Заменяем URL на data URL
      const escapedOriginalUrl = originalUrl.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
      const escapedAbsoluteUrl = absoluteUrl.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

      const imgSrcRegex1 = new RegExp(`(src\\s*=\\s*["'])${escapedOriginalUrl}(["'])`, 'gi');
      processedHtml = processedHtml.replace(imgSrcRegex1, `$1${dataUrl}$2`);

      const imgSrcRegex2 = new RegExp(`(src\\s*=\\s*["'])${escapedAbsoluteUrl}(["'])`, 'gi');
      processedHtml = processedHtml.replace(imgSrcRegex2, `$1${dataUrl}$2`);

      const bgImgUrlRegex1 = new RegExp(`(background-image\\s*:\\s*url\\(["']?)${escapedOriginalUrl}(["']?\\))`, 'gi');
      processedHtml = processedHtml.replace(bgImgUrlRegex1, `$1${dataUrl}$2`);

      const bgImgUrlRegex2 = new RegExp(`(background-image\\s*:\\s*url\\(["']?)${escapedAbsoluteUrl}(["']?\\))`, 'gi');
      processedHtml = processedHtml.replace(bgImgUrlRegex2, `$1${dataUrl}$2`);

      console.log('[PageResources] Embedded image as base64:', absoluteUrl);
    } catch (e) {
      console.warn('[PageResources] Failed to fetch/embed image:', absoluteUrl, e);
    }
  }

  return processedHtml;
}



