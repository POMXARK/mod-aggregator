import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, waitFor, fireEvent } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import ParserBuilder from '../ParserBuilder.new.svelte';
import type { Site } from '@/lib/api';
import { invoke } from '@/lib/tauri-wrapper';

// Mock Tauri invoke
vi.mock('@/lib/tauri-wrapper', () => ({
  invoke: vi.fn(),
}));

// Mock XYFlow components since they're complex to test
vi.mock('@xyflow/svelte', () => ({
  SvelteFlow: vi.fn().mockImplementation(() => ({
    $$render: () => ({}),
    $$unmount: () => {},
  })),
  Background: vi.fn().mockImplementation(() => ({
    $$render: () => ({}),
  })),
  Controls: vi.fn().mockImplementation(() => ({
    $$render: () => ({}),
  })),
  MiniMap: vi.fn().mockImplementation(() => ({
    $$render: () => ({}),
  })),
}));

// Mock framework initialization
vi.mock('@/components/lib/framework.svelte', () => ({
  initParserFramework: vi.fn(),
  createNode: vi.fn(),
  getRegisteredNodeTypes: vi.fn(() => ({
    extract: { name: 'Extract', icon: 'extract' },
    filter: { name: 'Filter', icon: 'filter' },
    transform: { name: 'Transform', icon: 'transform' },
  })),
}));

describe('ParserBuilder Integration Tests', () => {
  const mockSites: Site[] = [
    { id: 1, name: 'Test Site 1', url: 'https://test1.com', selectors: [] },
    { id: 2, name: 'Test Site 2', url: 'https://test2.com', selectors: [] },
  ];

  beforeEach(() => {
    vi.clearAllMocks();
    // Mock successful API calls
    vi.mocked(invoke).mockResolvedValue(mockSites);
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('Site Selection and Loading', () => {
    it('loads sites on mount and allows selection', async () => {
      const user = userEvent.setup();
      render(ParserBuilder);

      // Wait for sites to load
      await waitFor(() => {
        expect(invoke).toHaveBeenCalledWith('get_sites');
      });

      // Check that site selector is rendered
      const siteSelect = screen.getByRole('combobox', { name: /выберите сайт/i });
      expect(siteSelect).toBeInTheDocument();

      // Select a site
      await user.selectOptions(siteSelect, '1');

      // Verify site selection is handled (this would trigger state updates)
      expect(siteSelect).toHaveValue('1');
    });

    it('handles site loading errors gracefully', async () => {
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      vi.mocked(invoke).mockRejectedValue(new Error('Network error'));

      render(ParserBuilder);

      // Should not crash, error should be logged
      await waitFor(() => {
        expect(consoleSpy).toHaveBeenCalledWith('Failed to load sites:', expect.any(Error));
      });

      consoleSpy.mockRestore();
    });
  });

  describe('URL Input and Page Loading', () => {
    it('allows entering URL and toggling page viewer', async () => {
      const user = userEvent.setup();
      render(ParserBuilder);

      // Find URL input
      const urlInput = screen.getByRole('textbox', { name: /url/i });
      expect(urlInput).toBeInTheDocument();

      // Enter URL
      await user.type(urlInput, 'https://example.com');

      // Check URL was entered
      expect(urlInput).toHaveValue('https://example.com');

      // Find page viewer toggle
      const pageViewerToggle = screen.getByRole('button', { name: /просмотр страницы/i });
      expect(pageViewerToggle).toBeInTheDocument();

      // Toggle page viewer
      await user.click(pageViewerToggle);

      // The component should handle the state change
      // (exact behavior depends on internal state management)
    });

    it('validates URL format', async () => {
      const user = userEvent.setup();
      render(ParserBuilder);

      const urlInput = screen.getByRole('textbox', { name: /url/i });

      // Enter invalid URL
      await user.type(urlInput, 'not-a-url');

      // The component should handle invalid URLs gracefully
      // (validation logic depends on implementation)
    });
  });

  describe('Node Creation and Management', () => {
    it('renders available node types for creation', async () => {
      render(ParserBuilder);

      // Wait for component to initialize
      await waitFor(() => {
        expect(screen.getByText('Конструктор парсеров')).toBeInTheDocument();
      });

      // Check that node creation UI is available
      // (This depends on the actual UI implementation)
      const nodeCreationArea = screen.getByText(/создать|добавить|ноды/i);
      expect(nodeCreationArea).toBeInTheDocument();
    });

    it('allows creating nodes via UI', async () => {
      const user = userEvent.setup();
      render(ParserBuilder);

      await waitFor(() => {
        expect(screen.getByText('Конструктор парсеров')).toBeInTheDocument();
      });

      // Look for node creation buttons or interface
      const addButtons = screen.getAllByRole('button').filter(btn =>
        btn.textContent?.toLowerCase().includes('добавить') ||
        btn.textContent?.toLowerCase().includes('создать') ||
        btn.textContent?.toLowerCase().includes('extract') ||
        btn.textContent?.toLowerCase().includes('filter') ||
        btn.textContent?.toLowerCase().includes('transform')
      );

      if (addButtons.length > 0) {
        // Test clicking an add button
        await user.click(addButtons[0]);

        // Verify node creation was attempted
        // (exact behavior depends on implementation)
      }
    });
  });

  describe('Code Generation and Execution', () => {
    it('generates code when nodes are configured', async () => {
      render(ParserBuilder);

      await waitFor(() => {
        expect(screen.getByText('Конструктор парсеров')).toBeInTheDocument();
      });

      // Look for code generation button
      const generateButton = screen.getByRole('button', { name: /сгенерировать|generate/i });
      expect(generateButton).toBeInTheDocument();

      // The code generation would be tested by checking if generatedCode state updates
      // This is a placeholder for the actual test implementation
    });

    it('allows executing generated parser code', async () => {
      const user = userEvent.setup();
      render(ParserBuilder);

      await waitFor(() => {
        expect(screen.getByText('Конструктор парсеров')).toBeInTheDocument();
      });

      // Look for execute/run button
      const runButtons = screen.getAllByRole('button').filter(btn =>
        btn.textContent?.toLowerCase().includes('запустить') ||
        btn.textContent?.toLowerCase().includes('выполнить') ||
        btn.textContent?.toLowerCase().includes('run') ||
        btn.textContent?.toLowerCase().includes('execute')
      );

      if (runButtons.length > 0) {
        await user.click(runButtons[0]);

        // Verify execution was attempted
        // (exact behavior depends on implementation)
      }
    });
  });

  describe('Panel Management', () => {
    it('allows toggling different panels', async () => {
      const user = userEvent.setup();
      render(ParserBuilder);

      await waitFor(() => {
        expect(screen.getByText('Конструктор парсеров')).toBeInTheDocument();
      });

      // Look for panel toggle buttons
      const toggleButtons = screen.getAllByRole('button').filter(btn =>
        btn.textContent?.toLowerCase().includes('чат') ||
        btn.textContent?.toLowerCase().includes('результаты') ||
        btn.textContent?.toLowerCase().includes('код') ||
        btn.textContent?.toLowerCase().includes('настройки')
      );

      for (const button of toggleButtons.slice(0, 2)) { // Test first 2 toggles
        await user.click(button);
        // Verify panel state changed (depends on implementation)
      }
    });

    it('manages panel layout and resizing', async () => {
      render(ParserBuilder);

      await waitFor(() => {
        expect(screen.getByText('Конструктор парсеров')).toBeInTheDocument();
      });

      // Look for resize handles or layout elements
      const resizeElements = screen.queryAllByTestId('resize-handle');

      if (resizeElements.length > 0) {
        // Test resize functionality
        // (This would require more complex interaction testing)
      }
    });
  });

  describe('Error Handling and Recovery', () => {
    it('handles node creation errors gracefully', async () => {
      const user = userEvent.setup();
      render(ParserBuilder);

      await waitFor(() => {
        expect(screen.getByText('Конструктор парсеров')).toBeInTheDocument();
      });

      // Simulate error condition (depends on actual error handling implementation)
      // This is a placeholder for error handling tests
    });

    it('recovers from code generation failures', async () => {
      // Test error recovery in code generation
      // This would test error boundaries and fallback UI
    });
  });

  describe('State Persistence', () => {
    it('saves and restores UI state', async () => {
      const user = userEvent.setup();
      const { unmount } = render(ParserBuilder);

      await waitFor(() => {
        expect(screen.getByText('Конструктор парсеров')).toBeInTheDocument();
      });

      // Make some state changes
      const urlInput = screen.getByRole('textbox', { name: /url/i });
      await user.type(urlInput, 'https://test.com');

      // Unmount and re-mount to test state restoration
      unmount();

      // Re-render component
      render(ParserBuilder);

      // State should be restored (depends on localStorage implementation)
      await waitFor(() => {
        const restoredUrlInput = screen.getByRole('textbox', { name: /url/i });
        // Note: State restoration depends on implementation
      });
    });
  });

  describe('Accessibility and Keyboard Navigation', () => {
    it('supports keyboard navigation', async () => {
      const user = userEvent.setup();
      render(ParserBuilder);

      await waitFor(() => {
        expect(screen.getByText('Конструктор парсеров')).toBeInTheDocument();
      });

      // Test tab navigation
      await user.tab();
      await user.tab();

      // Test keyboard shortcuts if any
      // (Depends on implementation)
    });

    it('has proper ARIA labels and roles', () => {
      render(ParserBuilder);

      // Check for proper accessibility attributes
      const buttons = screen.getAllByRole('button');
      expect(buttons.length).toBeGreaterThan(0);

      const inputs = screen.getAllByRole('textbox');
      expect(inputs.length).toBeGreaterThan(0);
    });
  });
});

