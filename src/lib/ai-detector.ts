/**
 * AI-powered element detection for parser builder
 * Supports multiple free AI APIs
 */

interface ElementInfo {
  tagName: string;
  text: string;
  attributes: Record<string, string>;
  selector: string;
}

interface AISuggestion {
  type: string;
  data: any;
  confidence: number;
}

/**
 * Detect parsing elements using AI
 * Currently supports Hugging Face Inference API (free)
 */
export async function detectElementsWithAI(
  html: string,
  selectedElement: ElementInfo
): Promise<AISuggestion[]> {
  const suggestions: AISuggestion[] = [];

  try {
    // Try Hugging Face Inference API (free, no key required for some models)
    const response = await fetch('https://api-inference.huggingface.co/models/microsoft/DialoGPT-medium', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        inputs: `Analyze this HTML element and suggest CSS selectors for parsing:
Tag: ${selectedElement.tagName}
Text: ${selectedElement.text.substring(0, 200)}
Attributes: ${JSON.stringify(selectedElement.attributes)}
Current selector: ${selectedElement.selector}

Suggest selectors for:
1. List container (if this is a list item)
2. Title extraction
3. URL extraction (if links present)
4. Image extraction (if images present)
5. Description/text extraction`,
      }),
    });

    if (response.ok) {
      const data = await response.json();
      // Parse AI response and create suggestions
      // This is a simplified version - in production, you'd parse the AI response properly
      suggestions.push(...parseAIResponse(data, selectedElement));
    }
  } catch (error) {
    console.warn('AI detection failed, using heuristics:', error);
  }

  // Fallback to heuristic-based detection
  if (suggestions.length === 0) {
    suggestions.push(...heuristicDetection(selectedElement));
  }

  return suggestions;
}

function parseAIResponse(aiData: any, element: ElementInfo): AISuggestion[] {
  const suggestions: AISuggestion[] = [];
  
  // Simple heuristic parsing of AI response
  // In production, use proper NLP parsing
  
  // Check for list patterns
  if (element.tagName.toLowerCase().match(/li|div|article|section/)) {
    suggestions.push({
      type: 'selector',
      data: {
        label: 'List Container',
        selector: element.selector.replace(/:nth-of-type\([0-9]+\)/g, ''),
      },
      confidence: 0.7,
    });
  }
  
  return suggestions;
}

function heuristicDetection(element: ElementInfo): AISuggestion[] {
  const suggestions: AISuggestion[] = [];
  const tagName = element.tagName.toLowerCase();
  
  // Detect list items
  if (tagName === 'li' || element.attributes.class?.includes('item')) {
    // Find parent container
    const parentSelector = element.selector.split(' > ').slice(0, -1).join(' > ') || 'ul, ol, div';
    suggestions.push({
      type: 'selector',
      data: {
        label: 'List Container',
        selector: parentSelector,
      },
      confidence: 0.8,
    });
  }
  
  // Detect links
  if (tagName === 'a' || element.attributes.href) {
    suggestions.push({
      type: 'extract',
      data: {
        label: 'Extract URL',
        attribute: 'href',
        selector: element.selector,
      },
      confidence: 0.9,
    });
  }
  
  // Detect images
  if (tagName === 'img' || element.attributes.src) {
    suggestions.push({
      type: 'extract',
      data: {
        label: 'Extract Image',
        attribute: 'src',
        selector: element.selector,
      },
      confidence: 0.9,
    });
  }
  
  // Detect headings (likely titles)
  if (tagName.match(/^h[1-6]$/)) {
    suggestions.push({
      type: 'extract',
      data: {
        label: 'Extract Title',
        attribute: 'text',
        selector: element.selector,
      },
      confidence: 0.85,
    });
  }
  
  // Always suggest text extraction
  suggestions.push({
    type: 'extract',
    data: {
      label: 'Extract Text',
      attribute: 'text',
      selector: element.selector,
    },
    confidence: 0.7,
  });
  
  return suggestions;
}

/**
 * Use OpenAI API (requires API key, but has free tier)
 */
export async function detectWithOpenAI(
  html: string,
  selectedElement: ElementInfo,
  apiKey?: string
): Promise<AISuggestion[]> {
  if (!apiKey) {
    return heuristicDetection(selectedElement);
  }

  try {
    const response = await fetch('https://api.openai.com/v1/chat/completions', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${apiKey}`,
      },
      body: JSON.stringify({
        model: 'gpt-3.5-turbo',
        messages: [
          {
            role: 'system',
            content: 'You are an expert web scraping assistant. Analyze HTML elements and suggest CSS selectors for parsing.',
          },
          {
            role: 'user',
            content: `Analyze this HTML element:
Tag: ${selectedElement.tagName}
Text: ${selectedElement.text.substring(0, 200)}
Attributes: ${JSON.stringify(selectedElement.attributes)}
Current selector: ${selectedElement.selector}

Suggest CSS selectors for parsing similar elements.`,
          },
        ],
        max_tokens: 500,
      }),
    });

    if (response.ok) {
      const data = await response.json();
      // Parse OpenAI response
      return parseOpenAIResponse(data, selectedElement);
    }
  } catch (error) {
    console.error('OpenAI API error:', error);
  }

  return heuristicDetection(selectedElement);
}

function parseOpenAIResponse(data: any, element: ElementInfo): AISuggestion[] {
  const suggestions: AISuggestion[] = [];
  const content = data.choices?.[0]?.message?.content || '';
  
  // Simple parsing - in production, use proper JSON parsing
  // For now, fallback to heuristics
  return heuristicDetection(element);
}


