import type { Node, Edge } from '@xyflow/svelte';

export interface ParserCodeGeneratorSettings {
  maxElements: number;
  timeoutSeconds: number;
  slowMode: boolean;
  delayPerElement: number;
}

/**
 * Composable для генерации Rust кода парсера из узлов графа
 *
 * Преобразует визуальный граф парсера в код на языке Rust,
 * который можно использовать для парсинга веб-страниц.
 *
 * @param nodes - узлы графа парсера
 * @param edges - связи между узлами
 * @param settings - настройки парсера
 * @returns Функция для генерации кода
 */
export function useParserCodeGenerator(
  nodes: () => Node[],
  edges: () => Edge[],
  settings: () => ParserCodeGeneratorSettings
) {
  /**
   * Генерирует код парсера на Rust из узлов графа
   */
  function generateParserCode(): string {
    const currentNodes = nodes();
    const currentEdges = edges();
    const currentSettings = settings();

    // Generate Rust parser code from nodes with settings
    let code = '// Generated parser code\n';
    code += `// Settings: max_elements=${currentSettings.maxElements}, timeout=${currentSettings.timeoutSeconds}s, slow_mode=${currentSettings.slowMode}, delay=${currentSettings.delayPerElement}ms\n\n`;
    code += 'use scraper::{Html, Selector};\n';
    if (currentSettings.timeoutSeconds > 0 || currentSettings.slowMode) {
      code += 'use std::time::{Instant, Duration};\n';
    }
    code += '\n';

    // Добавляем константы настроек
    if (currentSettings.maxElements > 0) {
      code += `const MAX_ELEMENTS: usize = ${currentSettings.maxElements};\n`;
    }
    if (currentSettings.timeoutSeconds > 0) {
      code += `const TIMEOUT_SECONDS: u64 = ${currentSettings.timeoutSeconds};\n`;
    }
    if (currentSettings.slowMode && currentSettings.delayPerElement > 0) {
      code += `const DELAY_PER_ELEMENT_MS: u64 = ${currentSettings.delayPerElement};\n`;
    }
    if (
      currentSettings.maxElements > 0 ||
      currentSettings.timeoutSeconds > 0 ||
      (currentSettings.slowMode && currentSettings.delayPerElement > 0)
    ) {
      code += '\n';
    }

    code +=
      'pub fn parse_page(html: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {\n';
    code += '    let document = Html::parse_document(html);\n';
    code += '    let mut results = Vec::new();\n';
    if (currentSettings.timeoutSeconds > 0 || currentSettings.slowMode) {
      code += '    let start_time = Instant::now();\n';
    }
    code += '\n';

    // Find root selector node
    const rootNode = currentNodes.find(
      n => n.type === 'selector' && !currentEdges.some(e => e.target === n.id)
    );

    if (rootNode) {
      const selector = rootNode.data.selector || '';
      code += `    let selector = Selector::parse("${selector}")?;\n`;

      // Собираем все элементы с учетом ограничения
      if (currentSettings.maxElements > 0) {
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
      if (currentSettings.timeoutSeconds > 0) {
        code += `        if start_time.elapsed() > Duration::from_secs(TIMEOUT_SECONDS) {\n`;
        code += `            break;\n`;
        code += `        }\n`;
      }

      // Process connected extract nodes
      const extractEdges = currentEdges.filter(e => e.source === rootNode.id);
      for (const edge of extractEdges) {
        const extractNode = currentNodes.find(n => n.id === edge.target);
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
      if (currentSettings.slowMode && currentSettings.delayPerElement > 0) {
        code += `        std::thread::sleep(Duration::from_millis(DELAY_PER_ELEMENT_MS));\n`;
      }

      code += '    }\n\n';
    }

    code += '    Ok(results)\n';
    code += '}\n';

    return code;
  }

  /**
   * Генерирует JSON конфигурацию парсера из узлов графа
   */
  function generateParserConfig(): any {
    const currentNodes = nodes();
    const currentEdges = edges();
    const config: any = {};

    // Находим корневой selector node
    const rootNode = currentNodes.find(
      n => n.type === 'selector' && !currentEdges.some(e => e.target === n.id)
    );

    if (rootNode) {
      config.list_selector = rootNode.data.selector || '';

      // Извлекаем селекторы из связанных extract nodes
      const extractEdges = currentEdges.filter(e => e.source === rootNode.id);
      for (const edge of extractEdges) {
        const extractNode = currentNodes.find(n => n.id === edge.target);
        if (extractNode && extractNode.type === 'extract') {
          const attribute = extractNode.data.attribute || 'text';
          if (!config[`${attribute}_selector`]) {
            config[`${attribute}_selector`] = extractNode.data.selector || '';
          }
        }
      }
    }

    return config;
  }

  return {
    generateParserCode,
    generateParserConfig,
  };
}

