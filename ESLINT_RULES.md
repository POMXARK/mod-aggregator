# ESLint Rules Documentation

This document describes custom ESLint rules used in the Mod Aggregator project.

## Custom Rules

### `require-svelte-extension`

Enforces explicit `.svelte` extensions in component imports.

#### Why?

Vite requires explicit file extensions for proper module resolution. Without `.svelte` extensions, the bundler cannot resolve Svelte component imports, leading to 500 Internal Server Errors.

#### Examples

❌ **Incorrect:**
```typescript
import Component from './Component';
import MyButton from '../ui/MyButton';
import Modal from './components/Modal';
```

✅ **Correct:**
```typescript
import Component from './Component.svelte';
import MyButton from '../ui/MyButton.svelte';
import Modal from './components/Modal.svelte';
```

#### Configuration

```javascript
// eslint.config.js
{
  rules: {
    'custom-rules/require-svelte-extension': 'error'
  }
}
```

#### Auto-fix

The rule provides automatic fixes that add the `.svelte` extension to imports:

```bash
npx eslint --fix src/components/index.ts
```

#### Implementation

Located in `eslint-plugin-custom-rules.js`:

```javascript
export const rules = {
  'require-svelte-extension': {
    meta: {
      type: 'problem',
      docs: {
        description: 'Require explicit .svelte extension in component imports',
        category: 'Best Practices',
        recommended: true,
      },
      fixable: 'code',
    },
    create(context) {
      return {
        ImportDeclaration(node) {
          const source = node.source.value;

          // Check relative imports without extensions
          if (source.startsWith('./') || source.startsWith('../')) {
            const filename = source.split('/').pop();

            if (filename && !filename.includes('.') && !filename.endsWith('.svelte')) {
              const fixedPath = source + '.svelte';

              context.report({
                node: node.source,
                messageId: 'missingSvelteExtension',
                data: { fixed: fixedPath },
                fix(fixer) {
                  return fixer.replaceText(node.source, `'${fixedPath}'`);
                },
              });
            }
          }
        },
      };
    },
  },
};
```

## Usage

To run ESLint with this rule:

```bash
# Check for issues
npm run lint

# Auto-fix issues
npm run lint:fix
```

## Benefits

- **Prevents build errors:** Catches missing extensions before they cause 500 errors
- **Auto-fixable:** Most issues can be fixed automatically
- **Consistent imports:** Ensures all component imports follow the same pattern
- **Better IDE support:** Explicit extensions help IDEs provide better autocomplete and navigation

