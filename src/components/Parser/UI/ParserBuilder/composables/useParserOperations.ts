import { invoke } from '@/lib/tauri-wrapper';
import type { Node, Edge } from '@xyflow/svelte';
import type { Site, ParserConfig } from '@/lib/api';
import type { ParserSettings, ParserResult, SelectedElementInfo } from '../types/parser-builder.types';

/**
 * Composable для операций с парсером
 * Управляет генерацией кода, тестированием, сохранением и загрузкой
 */
export function useParserOperations() {
  /**
   * Генерирует код парсера на Rust из узлов графа
   */
  function generateParserCode(
    nodes: Node[],
    edges: Edge[],
    parserSettings: ParserSettings,
    onCodeGenerated: (code: string) => void
  ) {
    // Ensure nodes and edges are arrays
    const nodesArray = Array.isArray(nodes) ? nodes : [];
    const edgesArray = Array.isArray(edges) ? edges : [];

    // Generate Rust parser code from nodes with settings
    let code = '// Generated parser code\n';
    code += `// Settings: max_elements=${parserSettings.maxElements}, timeout=${parserSettings.timeoutSeconds}s, slow_mode=${parserSettings.slowMode}, delay=${parserSettings.delayPerElement}ms\n\n`;
    code += 'use scraper::{Html, Selector};\n';
    code += 'use std::error::Error;\n';
    if (parserSettings.timeoutSeconds > 0 || parserSettings.slowMode) {
      code += 'use std::time::{Instant, Duration};\n';
    }
    code += '\n';

    // Добавляем константы настроек
    if (parserSettings.maxElements > 0) {
      code += `const MAX_ELEMENTS: usize = ${parserSettings.maxElements};\n`;
    }
    if (parserSettings.timeoutSeconds > 0) {
      code += `const TIMEOUT_SECONDS: u64 = ${parserSettings.timeoutSeconds};\n`;
    }
    if (parserSettings.slowMode && parserSettings.delayPerElement > 0) {
      code += `const DELAY_PER_ELEMENT_MS: u64 = ${parserSettings.delayPerElement};\n`;
    }
    if (
      parserSettings.maxElements > 0 ||
      parserSettings.timeoutSeconds > 0 ||
      (parserSettings.slowMode && parserSettings.delayPerElement > 0)
    ) {
      code += '\n';
    }

    code +=
      'pub fn parse_page(html: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {\n';
    code += '    let document = Html::parse_document(html);\n';
    code += '    let mut results = Vec::new();\n';
    if (parserSettings.timeoutSeconds > 0 || parserSettings.slowMode) {
      code += '    let start_time = Instant::now();\n';
    }
    code += '\n';

    // Find root selector node
    const rootNode = nodesArray.find(n => n.type === 'selector' && !edgesArray.some(e => e.target === n.id));

    if (rootNode) {
      const selector = rootNode.data.selector || '';
      code += `    let selector = Selector::parse("${selector}")?;`;

      // Собираем все элементы с учетом ограничения
      if (parserSettings.maxElements > 0) {
        code += '    let all_elements: Vec<_> = document.select(&selector).collect();\n';
        code += `    let elements_to_process: Vec<_> = if all_elements.len() > MAX_ELEMENTS {\n`;
        code += `        all_elements.into_iter().take(MAX_ELEMENTS).collect()\n`;
        code += '    } else {\n';
        code += '        all_elements\n';
        code += '    };\n\n';
        code += '    for element_ref in elements_to_process.iter() {\n';
        code += '        let element = element_ref;\n';
      } else {
        code += '    for element in document.select(&selector) {\n';
      }

      code += '        let mut item = serde_json::json!({});\n';

      // Проверка таймаута в цикле
      if (parserSettings.timeoutSeconds > 0) {
        code += `        if start_time.elapsed() > Duration::from_secs(TIMEOUT_SECONDS) {\n`;
        code += `            break;\n`;
        code += `        }\n`;
      }

      // Process connected extract nodes
      const extractEdges = edgesArray.filter(e => e.source === rootNode.id);
      for (const edge of extractEdges) {
        const extractNode = nodesArray.find(n => n.id === edge.target);
        if (extractNode && extractNode.type === 'extract') {
          const attribute = extractNode.data.attribute || 'text';
          if (attribute === 'text') {
            code += `        item["${attribute}"] = serde_json::json!(element.text().collect::<String>().trim());\n`;
          } else if (attribute === 'href') {
            code += `        if let Some(href) = element.value().attr("href") {\n`;
            code += `            item["url"] = serde_json::json!(href);\n`;
            code += `        }\n`;
          } else if (attribute === 'src') {
            code += `        if let Some(src) = element.value().attr("src") {\n`;
            code += `            item["image"] = serde_json::json!(src);\n`;
            code += `        }\n`;
          }
        }
      }

      code += '        results.push(item);\n';

      // Задержка в медленном режиме
      if (parserSettings.slowMode && parserSettings.delayPerElement > 0) {
        code += `        std::thread::sleep(Duration::from_millis(DELAY_PER_ELEMENT_MS));\n`;
      }

      code += '    }\n\n';
    }

    code += '    Ok(results)\n';
    code += '}\n';

    onCodeGenerated(code);
  }

  /**
   * Тестирует парсер на текущей странице
   */
  async function testParser(
    currentUrl: string,
    nodes: Node[],
    parserSettings: ParserSettings,
    onResults: (results: ParserResult[], diagnostics: unknown[], stats: unknown) => void,
    onError: (error: string) => void
  ) {
    if (nodes.length === 0) {
      onError('Создайте ноды парсера перед тестированием');
      return;
    }

    if (!currentUrl) {
      onError('Загрузите страницу для тестирования');
      return;
    }

    try {
      const result = await invoke('run_parser_test', {
        url: currentUrl,
        nodes: nodes,
        settings: parserSettings,
      });

      // Преобразуем результаты
      const results = result.data?.map((r: unknown) => ({
        data: r,
        expanded: false,
      })) || [];

      onResults(results, result.diagnostics || [], result.extractionStats || {});
    } catch (error: unknown) {
      console.error('Parser test error:', error);
      const errorMessage = error instanceof Error ? error.message : String(error);
      onError(`Ошибка тестирования парсера: ${errorMessage}`);
    }
  }

  /**
   * Автоматически определяет похожие элементы на странице
   */
  async function autoDetectElements(
    selectedElementInfo: SelectedElementInfo,
    onSuggestionsFound: (suggestions: unknown[]) => void
  ) {
    // Try to find similar elements and suggest extraction nodes
    const iframe = document.querySelector('iframe');
    if (iframe?.contentWindow) {
      iframe.contentWindow.postMessage(
        {
          type: 'find-similar',
          selector: selectedElementInfo.selector,
        },
        '*'
      );
    }

    // Also try AI detection if available
    try {
      const suggestions = await detectWithAI(
        selectedElementInfo.selector,
        selectedElementInfo.elementInfo
      );
      if (suggestions.length > 0) {
        onSuggestionsFound(suggestions);
      }
    } catch (error) {
      console.error('AI detection failed:', error);
      // Fallback to simple detection
      const suggestions = simpleAutoDetect(selectedElementInfo.elementInfo);
      onSuggestionsFound(suggestions);
    }
  }

  /**
   * Определяет похожие элементы с помощью AI
   */
  async function detectWithAI(selector: string, elementInfo: SelectedElementInfo['elementInfo']): Promise<unknown[]> {
    try {
      const { detectElementsWithAI } = await import('@/lib/ai-detector');

      // Get HTML from iframe if possible
      const iframe = document.querySelector('iframe');
      let html = '';
      if (iframe?.contentWindow?.document) {
        html = iframe.contentWindow.document.documentElement.outerHTML;
      }

      const elementData = {
        tagName: elementInfo.tagName!,
        text: elementInfo.text!,
        attributes: elementInfo.attributes!,
        selector,
      };

      const suggestions = await detectElementsWithAI(html, elementData);

      return suggestions.map(s => ({
        type: s.type,
        data: s.data,
      }));
    } catch (error) {
      console.error('AI detection error:', error);
      return [];
    }
  }

  /**
   * Простое эвристическое определение похожих элементов
   */
  function simpleAutoDetect(elementInfo: SelectedElementInfo['elementInfo']): unknown[] {
    const suggestions: unknown[] = [];

    // If element has href, suggest extract node for URL
    if (elementInfo.attributes?.href) {
      suggestions.push({
        type: 'extract',
        data: {
          label: 'Extract URL',
          attribute: 'href',
          selector: '.selected-element a', // placeholder
        },
      });
    }

    // If element has src, suggest extract node for image
    if (elementInfo.attributes?.src) {
      suggestions.push({
        type: 'extract',
        data: {
          label: 'Extract Image',
          attribute: 'src',
          selector: '.selected-element img', // placeholder
        },
      });
    }

    // Always suggest text extraction
    suggestions.push({
      type: 'extract',
      data: {
        label: 'Extract Text',
        attribute: 'text',
        selector: '.selected-element', // placeholder
      },
    });

    return suggestions;
  }

  /**
   * Сохраняет конфигурацию парсера в базу данных
   */
  async function saveParser(
    selectedSite: Site | null,
    nodes: Node[],
    onSuccess: () => void,
    onError: (error: string) => void
  ) {
    if (!selectedSite) {
      onError('Выберите сайт для сохранения');
      return;
    }

    try {
      // Generate config from nodes
      const rootNode = nodes.find(n => n.type === 'selector');
      const config: ParserConfig = {
        list_selector: String(rootNode?.data?.selector || ''),
      };

      // Extract other selectors from extract nodes
      const extractNodes = nodes.filter(n => n.type === 'extract');
      for (const node of extractNodes) {
        if (node.data.selector) {
          const attribute = node.data.attribute || 'text';
          config[`${attribute}_selector`] = node.data.selector;
        }
      }

      await invoke('update_site', {
        id: selectedSite.id,
        name: selectedSite.name,
        url: selectedSite.url,
        parserConfig: config,
      });

      onSuccess();
    } catch (error: unknown) {
      console.error('Save parser error:', error);
      const errorMessage = error instanceof Error ? error.message : String(error);
      onError(`Ошибка сохранения: ${errorMessage}`);
    }
  }

  /**
   * Загружает сайт и его конфигурацию
   */
  async function loadSite(
    selectedSite: Site | null,
    onSiteLoaded: (url: string, nodes: Node[], edges: Edge[]) => void,
    onError: (error: string) => void
  ) {
    try {
      if (!selectedSite) {
        onSiteLoaded('', [], []);
        return;
      }

      // Validate site object
      if (!selectedSite.url) {
        onError('Выбранный сайт не имеет URL');
        return;
      }

      // Validate URL format
      try {
        new URL(selectedSite.url);
      } catch {
        onError('Неверный формат URL');
        return;
      }

      // Load parser config from site
      const config = selectedSite.parser_config || {};
      const nodes: Node[] = [];
      const edges: Edge[] = [];

      // Create nodes from config
      if (config.list_selector) {
        nodes.push({
          id: 'list-selector',
          type: 'selector',
          position: { x: 100, y: 100 },
          data: {
            label: 'List Selector',
            selector: config.list_selector,
          },
        });
      }

      if (config.title_selector) {
        nodes.push({
          id: 'title-selector',
          type: 'extract',
          position: { x: 100, y: 200 },
          data: {
            label: 'Title Extract',
            attribute: 'text',
            selector: config.title_selector,
          },
        });

        if (nodes.length > 1) {
          edges.push({
            id: 'edge-1',
            source: 'list-selector',
            target: 'title-selector',
            type: 'smoothstep',
          });
        }
      }

      onSiteLoaded(selectedSite.url, nodes, edges);
    } catch (err: unknown) {
      console.error('Error loading site:', err);
      const errorMessage = err instanceof Error ? err.message : 'Ошибка загрузки сайта';
      onError(errorMessage);
    }
  }

  return {
    generateParserCode,
    testParser,
    autoDetectElements,
    saveParser,
    loadSite,
  };
}

