/**
 * Custom ESLint rules for the project
 */

function checkImportOrExport(source, context) {
  const filename = context.getFilename();

  // Apply to .svelte files and index.ts files in components directories or lib/composables
  const normalizedFilename = filename.replace(/\\/g, '/');
  const isValidFile = normalizedFilename.endsWith('.svelte') ||
                     (normalizedFilename.endsWith('index.ts') &&
                      (normalizedFilename.includes('/components/') || normalizedFilename.includes('/lib/composables/')));


  if (!isValidFile) {
    return;
  }

  if (!source || !source.value) {
    return;
  }

  const sourceValue = source.value;

  // Check if it's an import of a Svelte component or composable with runes
  // Either relative import or import from components directories
  let shouldCheckImport = false;

  if (sourceValue.startsWith('./') || sourceValue.startsWith('../')) {
    // For relative imports, check component-like imports
    // Allow composables directory since those may contain .svelte.ts files
    // Exclude other lib directories that are definitely utilities
    shouldCheckImport = !sourceValue.includes('/types/') &&
                       !sourceValue.includes('/config/') &&
                       !sourceValue.includes('/framework/') &&
                       !sourceValue.includes('/nodes/') &&
                       !sourceValue.includes('index');

    // Special case: if we're in lib/composables/index.ts, allow checking composables
    if (filename.includes('/lib/composables/index.ts')) {
      shouldCheckImport = true;
    } else if (sourceValue.includes('/lib/') && !sourceValue.includes('/lib/composables/')) {
      // Exclude other lib directories
      shouldCheckImport = false;
    }
  } else if (sourceValue.includes('@/components/')) {
    // For absolute imports from @/components/, check exclusions
    shouldCheckImport = !sourceValue.includes('/types/') &&
                       !sourceValue.includes('/config/') &&
                       !sourceValue.includes('/framework/') &&
                       !sourceValue.includes('/nodes/') &&
                       !sourceValue.endsWith('/icons') &&
                       !sourceValue.includes('index');
  }

  if (shouldCheckImport) {
    const importFilename = sourceValue.split('/').pop();

    // Check if the import looks like it should have an extension but doesn't have the correct one
    if (importFilename && !importFilename.includes('.')) {
      // This is an import without extension
      // For index.ts files, these are typically directory imports, so don't require extension
      // Only check for actual component files, not directory imports in index.ts

      // Skip checking in index.ts files as they typically import directories
      if (!filename.includes('index.ts')) {
        // Skip utility files that should remain as .ts
        const utilityFiles = ['useAIChat', 'useAISettings', 'useChatStorage', 'useMods', 'useNotifications',
                             'useParserBuilderState', 'useParserCodeGenerator', 'useParserRunner', 'useSites', 'useUI'];

        if (!utilityFiles.includes(importFilename)) {
          // Only for component-like files suggest .svelte
          const fixedPath = sourceValue + '.svelte';
          context.report({
            node: source,
            messageId: 'missingSvelteExtension',
            data: {
              fixed: fixedPath,
            },
            fix(fixer) {
              return fixer.replaceText(source, `'${fixedPath}'`);
            },
          });
        }
      }
    } else if (importFilename && importFilename.endsWith('.svelte')) {
      // This import already has .svelte extension - check if it should be .svelte.ts
      // Files with Svelte runes should use .svelte.ts extension

      const svelteTsFiles = ['useParserSettings', 'usePageLoader', 'useRecentUrls', 'useSelection'];
      const filenameWithoutExtension = importFilename.replace('.svelte', '');

      if (svelteTsFiles.includes(filenameWithoutExtension)) {
        const fixedPath = sourceValue.replace('.svelte', '.svelte.ts');

        context.report({
          node: source,
          messageId: 'wrongSvelteExtension',
          data: {
            current: sourceValue,
            fixed: fixedPath,
          },
          fix(fixer) {
            return fixer.replaceText(source, `'${fixedPath}'`);
          },
        });
      }
      // For other .svelte imports, assume they are correct
      // File existence validation should be done by build tools like Vite
    }
  }
}

export const rules = {
  'require-svelte-extension': {
    meta: {
      type: 'problem',
      docs: {
        description: 'Require explicit .svelte extension in component imports',
        category: 'Best Practices',
        recommended: true,
      },
      schema: [],
      messages: {
        missingSvelteExtension: 'Svelte component imports must include the .svelte extension. Use "{{fixed}}" instead.',
        wrongSvelteExtension: 'Incorrect Svelte file extension. Use "{{fixed}}" instead of "{{current}}".',
        incorrectSvelteImport: 'Incorrect import path. Use "{{fixed}}" instead of "{{current}}" (likely importing a directory).',
      },
      fixable: 'code',
    },
    create(context) {
      return {
        ImportDeclaration(node) {
          checkImportOrExport(node.source, context);
        },
        ExportAllDeclaration(node) {
          checkImportOrExport(node.source, context);
        },
      };
    },
  },
};

function checkBindFunction(node, context) {
  // Only check .svelte files
  if (!context.getFilename().endsWith('.svelte')) {
    return;
  }

  if (node.type === 'SvelteDirective' && node.kind === 'bind' && node.expression) {
    // Check if the expression is a function call or function reference
    const expression = node.expression;

    // If it's a function call (like getAddAIMessageToChat())
    if (expression.type === 'CallExpression') {
      context.report({
        node,
        message: 'Cannot bind to function values. Use regular props instead of bind: for functions.',
      });
      return;
    }

    // If it's an identifier, check if it refers to a function
    if (expression.type === 'Identifier') {
      const scope = context.getScope();
      const variable = scope.variables.find(v => v.name === expression.name);

      if (variable && variable.defs && variable.defs.length > 0) {
        const def = variable.defs[0];
        if (def.node.type === 'FunctionDeclaration' ||
            def.node.type === 'VariableDeclarator' &&
            def.node.init &&
            (def.node.init.type === 'FunctionExpression' ||
             def.node.init.type === 'ArrowFunctionExpression')) {
          context.report({
            node,
            message: 'Cannot bind to function values. Use regular props instead of bind: for functions.',
          });
        }
      }
    }
  }
}

const eslintRules = {
  'require-svelte-extension': {
    meta: {
      type: 'problem',
      docs: {
        description: 'Require explicit .svelte extension in component imports',
        category: 'Best Practices',
        recommended: true,
      },
      schema: [],
      messages: {
        missingSvelteExtension: 'Svelte component imports must include the .svelte extension. Use "{{fixed}}" instead.',
        wrongSvelteExtension: 'Incorrect Svelte file extension. Use "{{fixed}}" instead of "{{current}}".',
        incorrectSvelteImport: 'Incorrect import path. Use "{{fixed}}" instead of "{{current}}" (likely importing a directory).',
      },
      fixable: 'code',
    },
    create(context) {
      return {
        ImportDeclaration(node) {
          checkImportOrExport(node.source, context);
        },
        ExportAllDeclaration(node) {
          checkImportOrExport(node.source, context);
        },
      };
    },
  },

  'no-bind-functions': {
    meta: {
      type: 'problem',
      docs: {
        description: 'Disallow binding to function values with bind: directive',
        category: 'Best Practices',
        recommended: true,
      },
      schema: [],
      messages: {
        noBindFunction: 'Cannot bind to function values. Use regular props instead of bind: for functions.',
      },
    },
    create(context) {
      return {
        // Use source code analysis to find bind: patterns
        Program() {
          const sourceCode = context.sourceCode.getText();
          const lines = sourceCode.split('\n');

          lines.forEach((line, index) => {
            // Look for bind: patterns that might contain function calls
            const bindMatch = line.match(/bind:(\w+)\s*=\s*\{([^}]+)\}/);
            if (bindMatch) {
              const propName = bindMatch[1];
              const expression = bindMatch[2].trim();

              // Check if expression looks like a function call or function reference
              if (expression.includes('(') && expression.includes(')') ||
                  expression.match(/\b(function|=>)\b/)) {
                context.report({
                  loc: {
                    start: { line: index + 1, column: line.indexOf(`bind:${propName}`) },
                    end: { line: index + 1, column: line.indexOf(`bind:${propName}`) + `bind:${propName}={${expression}}`.length }
                  },
                  message: 'Cannot bind to function values. Use regular props instead of bind: for functions.',
                });
              }
            }
          });
        }
      };
    },
  },
};

export default { rules: eslintRules };
