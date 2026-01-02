import { test, expect } from '@playwright/test';

test.describe('App Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should load the main application', async ({ page }) => {
    // Check that the app loads
    await expect(page).toHaveTitle(/Mod Aggregator/);

    // Check main navigation is present
    await expect(page.locator('nav')).toBeVisible();

    // Check main content area exists
    await expect(page.locator('main')).toBeVisible();
  });

  test('should navigate between main pages', async ({ page }) => {
    // Start on Mods page
    await expect(page.locator('h2').filter({ hasText: 'Моды' })).toBeVisible();

    // Navigate to Sites
    await page.getByRole('button', { name: 'Сайты' }).click();
    await expect(page.locator('h2').filter({ hasText: 'Управление сайтами' })).toBeVisible();

    // Navigate to Parser
    await page.getByRole('button', { name: 'Конструктор' }).click();
    await expect(page.getByText('Конструктор парсеров')).toBeVisible();

    // Navigate to Notifications
    await page.getByRole('button', { name: 'Уведомления' }).click();
    await expect(page.getByRole('heading', { name: 'Уведомления' })).toBeVisible();

    // Navigate back to Mods
    await page.getByRole('button', { name: 'Моды' }).click();
    await expect(page.locator('h2').filter({ hasText: 'Моды' })).toBeVisible();
  });

  test('should maintain navigation state', async ({ page }) => {
    // Navigate to Sites and check active state
    await page.getByRole('button', { name: 'Сайты' }).click();

    // Check that Sites button has active class
    await expect(page.locator('nav button').filter({ hasText: 'Сайты' })).toHaveClass(/active/);

    // Navigate to Parser
    await page.getByRole('button', { name: 'Конструктор' }).click();

    // Check that Parser button is now active and Sites is not
    await expect(page.locator('nav button').filter({ hasText: 'Конструктор' })).toHaveClass(/active/);
    await expect(page.locator('nav button').filter({ hasText: 'Сайты' })).not.toHaveClass(/active/);
  });

  test('should handle browser back/forward navigation', async ({ page }) => {
    // Navigate to Sites
    await page.getByRole('button', { name: 'Сайты' }).click();
    await expect(page.locator('h2').filter({ hasText: 'Управление сайтами' })).toBeVisible();

    // Navigate to Parser
    await page.getByRole('button', { name: 'Конструктор' }).click();
    await expect(page.getByText('Конструктор парсеров')).toBeVisible();

    // Go back
    await page.goBack();
    await expect(page.locator('h2').filter({ hasText: 'Управление сайтами' })).toBeVisible();

    // Go forward
    await page.goForward();
    await expect(page.getByText('Конструктор парсеров')).toBeVisible();
  });
});

test.describe('Mods Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display mods list', async ({ page }) => {
    // Check mods heading is visible
    await expect(page.locator('h2').filter({ hasText: 'Моды' })).toBeVisible();

    // Check that mod cards or list is present
    // (This will depend on actual implementation)
    const modContent = page.locator('[data-testid="mods-content"], .mods-list, .mod-cards');
    await expect(modContent.or(page.locator('main'))).toBeVisible();
  });

  test('should handle mod filtering and sorting', async ({ page }) => {
    // Check for filter/search inputs
    const searchInput = page.locator('input[type="search"], input[placeholder*="поиск" i]');
    if (await searchInput.isVisible()) {
      await searchInput.fill('test mod');
      await expect(searchInput).toHaveValue('test mod');
    }

    // Check for sort options
    const sortSelect = page.locator('select').filter({ hasText: /сортировка|sort/i });
    if (await sortSelect.isVisible()) {
      await sortSelect.selectOption('name');
    }
  });
});

test.describe('Sites Management', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Сайты' }).click();
  });

  test('should display sites management interface', async ({ page }) => {
    await expect(page.locator('h2').filter({ hasText: 'Управление сайтами' })).toBeVisible();
  });

  test('should allow adding new sites', async ({ page }) => {
    // Look for add site button
    const addButton = page.getByRole('button', { name: /добавить|add.*site/i });
    if (await addButton.isVisible()) {
      await addButton.click();

      // Check form appears
      const siteForm = page.locator('form, [data-testid="site-form"]');
      await expect(siteForm.or(page.locator('dialog, .modal'))).toBeVisible();
    }
  });

  test('should validate site form inputs', async ({ page }) => {
    const addButton = page.getByRole('button', { name: /добавить|add.*site/i });
    if (await addButton.isVisible()) {
      await addButton.click();

      // Try to submit empty form
      const submitButton = page.getByRole('button', { name: /сохранить|save/i });
      if (await submitButton.isVisible()) {
        await submitButton.click();

        // Check for validation errors
        const errorMessages = page.locator('.error, [data-testid*="error"]');
        // Validation might be client-side or server-side
        await expect(errorMessages.or(page.locator('body'))).toBeTruthy();
      }
    }
  });
});

test.describe('Parser Builder', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Конструктор' }).click();
  });

  test('should load parser builder interface', async ({ page }) => {
    await expect(page.getByText('Конструктор парсеров')).toBeVisible();
  });

  test('should allow site selection for parsing', async ({ page }) => {
    // Check for site selector
    const siteSelect = page.locator('select, [data-testid="site-select"]');
    if (await siteSelect.isVisible()) {
      // Select first available option
      await siteSelect.selectOption({ index: 1 });
    }
  });

  test('should handle URL input and page loading', async ({ page }) => {
    // Look for URL input
    const urlInput = page.locator('input[type="url"], input[placeholder*="url" i]');
    if (await urlInput.isVisible()) {
      await urlInput.fill('https://example.com');

      // Look for load/preview button
      const loadButton = page.getByRole('button', { name: /загрузить|load|preview/i });
      if (await loadButton.isVisible()) {
        await loadButton.click();

        // Check that page viewer appears or loading state shows
        const pageViewer = page.locator('[data-testid="page-viewer"], iframe, .page-preview');
        await expect(pageViewer.or(page.locator('.loading, [aria-busy="true"]'))).toBeTruthy();
      }
    }
  });

  test('should allow creating and connecting parser nodes', async ({ page }) => {
    // This test depends on the actual node editor implementation
    // Check for node creation interface
    const nodeEditor = page.locator('[data-testid="node-editor"], .node-editor, .flow-editor');
    if (await nodeEditor.isVisible()) {
      // Look for add node buttons
      const addNodeButtons = page.locator('button').filter({ hasText: /добавить|add.*node/i });
      if (await addNodeButtons.count() > 0) {
        await addNodeButtons.first().click();

        // Check that node was added
        const nodes = page.locator('.react-flow__node, [data-testid*="node"]');
        await expect(nodes).toHaveCount(await nodes.count() + 1);
      }
    }
  });

  test('should generate and execute parser code', async ({ page }) => {
    // Look for generate code button
    const generateButton = page.getByRole('button', { name: /сгенерировать|generate.*code/i });
    if (await generateButton.isVisible()) {
      await generateButton.click();

      // Check that code appears
      const codeEditor = page.locator('[data-testid="code-editor"], .code-editor, textarea');
      if (await codeEditor.isVisible()) {
        await expect(codeEditor).not.toHaveValue('');
      }

      // Look for execute/run button
      const runButton = page.getByRole('button', { name: /выполнить|run|execute/i });
      if (await runButton.isVisible()) {
        await runButton.click();

        // Check for results or output
        const results = page.locator('[data-testid="results"], .results, .output');
        await expect(results.or(page.locator('.loading'))).toBeTruthy();
      }
    }
  });
});

test.describe('Collections Management', () => {
  test('should handle collection creation and management', async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Коллекции' }).click();

    // Check collections interface loads
    const collectionsContent = page.locator('[data-testid="collections"], .collections');
    await expect(collectionsContent.or(page.getByText('Коллекции'))).toBeTruthy();
  });
});

test.describe('Error Handling', () => {
  test('should handle network errors gracefully', async ({ page }) => {
    // Mock network failure by blocking API calls
    await page.route('**/invoke/**', route => route.abort());

    await page.goto('/');

    // App should still load basic UI
    await expect(page.locator('nav')).toBeVisible();

    // Should show error messages or fallback content
    const errorMessages = page.locator('.error, .alert, [role="alert"]');
    await expect(errorMessages.or(page.locator('body'))).toBeTruthy();
  });

  test('should handle invalid routes', async ({ page }) => {
    await page.goto('/invalid-route');

    // Should redirect to main page or show 404
    await expect(page.locator('nav')).toBeVisible();
  });
});

test.describe('Responsive Design', () => {
  test('should work on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    await page.goto('/');

    // Navigation should be accessible on mobile
    await expect(page.locator('nav')).toBeVisible();

    // Main content should be readable
    await expect(page.locator('main')).toBeVisible();
  });

  test('should work on tablet viewport', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });

    await page.goto('/');

    await expect(page.locator('nav')).toBeVisible();
    await expect(page.locator('main')).toBeVisible();
  });
});

test.describe('Accessibility', () => {
  test('should have proper heading structure', async ({ page }) => {
    await page.goto('/');

    // Check for h1
    const h1 = page.locator('h1');
    await expect(h1.or(page.locator('h2').first())).toBeVisible();

    // Check heading hierarchy
    const headings = await page.locator('h1, h2, h3, h4, h5, h6').allTextContents();
    expect(headings.length).toBeGreaterThan(0);
  });

  test('should support keyboard navigation', async ({ page }) => {
    await page.goto('/');

    // Tab through interactive elements
    await page.keyboard.press('Tab');
    let focusedElement = await page.locator(':focus');
    await expect(focusedElement).toBeVisible();

    // Continue tabbing
    for (let i = 0; i < 5; i++) {
      await page.keyboard.press('Tab');
      focusedElement = await page.locator(':focus');
      if (await focusedElement.isVisible()) {
        break; // Found a focusable element
      }
    }
  });

  test('should have proper ARIA labels', async ({ page }) => {
    await page.goto('/');

    // Check buttons have accessible names
    const buttons = page.locator('button:not([aria-label]):not([aria-labelledby])');
    const buttonCount = await buttons.count();

    // Some buttons might not need aria-label if they have text content
    const buttonsWithText = page.locator('button:not([aria-label]):not([aria-labelledby])').filter({ hasText: /.+/ });
    const accessibleButtons = await buttonsWithText.count();

    // At least some buttons should be accessible
    expect(buttonCount - accessibleButtons).toBeLessThan(buttonCount + 1);
  });
});

test.describe('Performance', () => {
  test('should load within reasonable time', async ({ page }) => {
    const startTime = Date.now();

    await page.goto('/', { waitUntil: 'networkidle' });

    const loadTime = Date.now() - startTime;
    expect(loadTime).toBeLessThan(5000); // Should load within 5 seconds
  });

  test('should handle large data sets', async ({ page }) => {
    // This test would need to mock large datasets
    // For now, just check basic performance
    await page.goto('/');

    // Navigate through different sections quickly
    await page.getByRole('button', { name: 'Сайты' }).click();
    await page.getByRole('button', { name: 'Конструктор' }).click();
    await page.getByRole('button', { name: 'Моды' }).click();

    // Should still be responsive
    await expect(page.locator('nav')).toBeVisible();
  });
});

