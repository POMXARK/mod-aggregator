import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import {
  migrateUIState,
  saveUIState,
  restoreUIState,
  exportUIState,
  importUIState,
  generateUIStateKey,
  type ParserBuilderUIState
} from '../useParserBuilderState';

// Mock localStorage
const localStorageMock = {
  getItem: vi.fn(),
  setItem: vi.fn(),
  removeItem: vi.fn(),
  clear: vi.fn(),
  length: 0,
  key: vi.fn(),
};

Object.defineProperty(window, 'localStorage', {
  value: localStorageMock,
  writable: true,
});

describe('useParserBuilderState', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorageMock.clear();
  });

  describe('migrateUIState', () => {
    describe('migration from version 0 to 1', () => {
      it('should add missing pageViewerWidth with default value', () => {
        const oldState = {
          version: 0,
          chatPanelWidth: 400,
          showPageViewer: true,
        };

        const migrated = migrateUIState(oldState, 0, 1);

        expect(migrated.pageViewerWidth).toBe(600); // DEFAULT_UI_STATE.pageViewerWidth
        expect(migrated.chatPanelWidth).toBe(400);
        expect(migrated.showPageViewer).toBe(true);
      });

      it('should convert percentage pageViewerWidth to pixels', () => {
        const oldState = {
          version: 0,
          pageViewerWidth: 50, // 50% should become 600px (assuming 1200px container)
        };

        const migrated = migrateUIState(oldState, 0, 1);

        expect(migrated.pageViewerWidth).toBe(600);
      });

      it('should keep pixel values as-is when > 100', () => {
        const oldState = {
          version: 0,
          pageViewerWidth: 800, // Already in pixels
        };

        const migrated = migrateUIState(oldState, 0, 1);

        expect(migrated.pageViewerWidth).toBe(800);
      });

      it('should add missing chatPanelWidth', () => {
        const oldState = {
          version: 0,
          pageViewerWidth: 600,
        };

        const migrated = migrateUIState(oldState, 0, 1);

        expect(migrated.chatPanelWidth).toBe(400); // DEFAULT_UI_STATE.chatPanelWidth
      });
    });

    describe('migration from version 1 to 2', () => {
      it('should add nodes and edges arrays', () => {
        const oldState = {
          version: 1,
          pageViewerWidth: 600,
          chatPanelWidth: 400,
        };

        const migrated = migrateUIState(oldState, 1, 2);

        expect(migrated.nodes).toEqual([]);
        expect(migrated.edges).toEqual([]);
        expect(migrated.stateUrl).toBe('');
      });

      it('should preserve existing values', () => {
        const oldState = {
          version: 1,
          pageViewerWidth: 700,
          nodes: [{ id: '1', type: 'extract' }],
        };

        const migrated = migrateUIState(oldState, 1, 2);

        expect(migrated.pageViewerWidth).toBe(700);
        expect(migrated.nodes).toEqual([{ id: '1', type: 'extract' }]);
        expect(migrated.edges).toEqual([]);
      });
    });

    describe('migration from version 2 to 3', () => {
      it('should add chatId field', () => {
        const oldState = {
          version: 2,
          pageViewerWidth: 600,
          nodes: [],
          edges: [],
          stateUrl: '',
        };

        const migrated = migrateUIState(oldState, 2, 3);

        expect(migrated.chatId).toBeUndefined(); // DEFAULT_UI_STATE.chatId
      });
    });

    it('should handle chained migrations', () => {
      const version0State = {
        version: 0,
        showPageViewer: true,
      };

      const migrated = migrateUIState(version0State, 0, 3);

      // Should have all migrations applied
      expect(migrated.version).toBeUndefined(); // migrateUIState doesn't set version
      expect(migrated.pageViewerWidth).toBe(600);
      expect(migrated.chatPanelWidth).toBe(400);
      expect(migrated.nodes).toEqual([]);
      expect(migrated.edges).toEqual([]);
      expect(migrated.stateUrl).toBe('');
      expect(migrated.chatId).toBeUndefined();
    });

    it('should return state as-is when no migration needed', () => {
      const currentState = {
        version: 3,
        pageViewerWidth: 600,
        chatPanelWidth: 400,
        nodes: [],
        edges: [],
        stateUrl: '',
        chatId: 'chat-123',
      };

      const migrated = migrateUIState(currentState, 3, 3);

      expect(migrated).toEqual(currentState);
    });
  });

  describe('generateUIStateKey', () => {
    it('should return base key when no URL provided', () => {
      expect(generateUIStateKey('', undefined)).toBe('parser-builder-ui-state');
    });

    it('should generate key from URL', () => {
      const key = generateUIStateKey('https://example.com/page');
      expect(key).toMatch(/^parser-builder-ui-state_/);
      expect(key).toContain('ZXhhbXBsZS5jb20vcGFnZQ'); // base64 encoded URL without trailing slash
    });

    it('should normalize URLs by removing trailing slash', () => {
      const key1 = generateUIStateKey('https://example.com/page/');
      const key2 = generateUIStateKey('https://example.com/page');

      expect(key1).toBe(key2);
    });

    it('should include chatId in key when provided', () => {
      const key = generateUIStateKey('https://example.com/page', 'chat-123');
      expect(key).toContain('_chat_chat-123');
    });

    it('should handle special characters in URLs', () => {
      const key = generateUIStateKey('https://example.com/path with spaces & special chars?query=value');
      expect(key).toMatch(/^parser-builder-ui-state_/);
      // Should be base64 encoded
      expect(key.length).toBeGreaterThan('parser-builder-ui-state_'.length);
    });
  });

  describe('saveUIState', () => {
    it('should save state with current version', () => {
      const state = {
        pageViewerWidth: 700,
        currentUrl: 'https://example.com',
        chatId: 'chat-123',
      };

      saveUIState(state);

      expect(localStorageMock.setItem).toHaveBeenCalledTimes(1);
      const [key, savedValue] = localStorageMock.setItem.mock.calls[0];

      expect(key).toContain('parser-builder-ui-state_');
      expect(key).toContain('_chat_chat-123');

      const parsedState = JSON.parse(savedValue);
      expect(parsedState.version).toBe(3); // UI_STATE_VERSION
      expect(parsedState.pageViewerWidth).toBe(700);
      expect(parsedState.currentUrl).toBe('https://example.com');
      expect(parsedState.chatId).toBe('chat-123');
    });

    it('should merge with default state', () => {
      const partialState = {
        pageViewerWidth: 800,
      };

      saveUIState(partialState);

      const [key, savedValue] = localStorageMock.setItem.mock.calls[0];
      const parsedState = JSON.parse(savedValue);

      expect(parsedState.pageViewerWidth).toBe(800);
      expect(parsedState.chatPanelWidth).toBe(400); // default value
      expect(parsedState.showPageViewer).toBe(false); // default value
    });

    it('should handle save errors gracefully', () => {
      localStorageMock.setItem.mockImplementation(() => {
        throw new Error('Storage quota exceeded');
      });

      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      expect(() => saveUIState({ pageViewerWidth: 600 })).not.toThrow();

      expect(consoleSpy).toHaveBeenCalledWith('Failed to save UI state:', expect.any(Error));

      consoleSpy.mockRestore();
    });
  });

  describe('restoreUIState', () => {
    it('should restore state for URL + chatId combination', () => {
      const savedState = {
        version: 3,
        pageViewerWidth: 700,
        chatId: 'chat-123',
        currentUrl: 'https://example.com',
      };

      localStorageMock.getItem.mockReturnValue(JSON.stringify(savedState));

      const restored = restoreUIState('https://example.com', 'chat-123');

      expect(restored).toEqual(savedState);
      expect(localStorageMock.getItem).toHaveBeenCalledTimes(1);
    });

    it('should fallback to URL-only state when chat-specific not found', () => {
      const urlState = {
        version: 3,
        pageViewerWidth: 600,
        currentUrl: 'https://example.com',
      };

      localStorageMock.getItem
        .mockReturnValueOnce(null) // chat-specific key
        .mockReturnValueOnce(JSON.stringify(urlState)); // URL-only key

      const restored = restoreUIState('https://example.com', 'chat-123');

      expect(restored).toEqual(urlState);
      expect(localStorageMock.getItem).toHaveBeenCalledTimes(2);
    });

    it('should fallback to general state when URL-specific not found', () => {
      const generalState = {
        version: 3,
        pageViewerWidth: 500,
      };

      localStorageMock.getItem
        .mockReturnValueOnce(null) // chat-specific
        .mockReturnValueOnce(null) // URL-specific
        .mockReturnValueOnce(JSON.stringify(generalState)); // general

      const restored = restoreUIState('https://example.com', 'chat-123');

      expect(restored).toEqual(generalState);
      expect(localStorageMock.getItem).toHaveBeenCalledTimes(3);
    });

    it('should return null when no state found', () => {
      localStorageMock.getItem.mockReturnValue(null);

      const restored = restoreUIState('https://example.com', 'chat-123');

      expect(restored).toBeNull();
    });

    it('should migrate old state versions', () => {
      const oldState = {
        version: 0,
        showPageViewer: true,
      };

      localStorageMock.getItem.mockReturnValue(JSON.stringify(oldState));

      const restored = restoreUIState('https://example.com', 'chat-123');

      expect(restored?.pageViewerWidth).toBe(600); // migrated
      expect(restored?.showPageViewer).toBe(true); // preserved
    });

    it('should handle corrupted state data', () => {
      localStorageMock.getItem.mockReturnValue('invalid json');

      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      const restored = restoreUIState('https://example.com');

      expect(restored).toBeNull();
      expect(consoleSpy).toHaveBeenCalledWith('Failed to restore UI state:', expect.any(SyntaxError));

      consoleSpy.mockRestore();
    });

    it('should work with URL only (no chatId)', () => {
      const urlState = {
        version: 3,
        pageViewerWidth: 650,
      };

      localStorageMock.getItem.mockReturnValue(JSON.stringify(urlState));

      const restored = restoreUIState('https://example.com');

      expect(restored?.pageViewerWidth).toBe(650);
      expect(localStorageMock.getItem).toHaveBeenCalledTimes(1); // only URL key
    });
  });

  describe('exportUIState and importUIState', () => {
    it('should export and import state correctly', () => {
      const state = {
        version: 3,
        pageViewerWidth: 700,
        showPageViewer: true,
        generatedCode: 'console.log("test");',
      };

      const exported = exportUIState(state);
      expect(typeof exported).toBe('string');

      const parsed = JSON.parse(exported);
      expect(parsed.pageViewerWidth).toBe(700);
      expect(parsed.showPageViewer).toBe(true);
      expect(parsed.generatedCode).toBe('console.log("test");');

      const imported = importUIState(exported);
      expect(imported).toEqual(parsed);
    });

    it('should migrate imported state if needed', () => {
      const oldState = {
        version: 1,
        pageViewerWidth: 50, // percentage
      };

      const exported = JSON.stringify(oldState);
      const imported = importUIState(exported);

      expect(imported?.pageViewerWidth).toBe(600); // migrated to pixels
    });

    it('should handle export errors', () => {
      const circularRef = { self: null };
      circularRef.self = circularRef;

      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      const exported = exportUIState(circularRef as any);

      expect(exported).toBe('');
      expect(consoleSpy).toHaveBeenCalledWith('Failed to export UI state:', expect.any(TypeError));

      consoleSpy.mockRestore();
    });

    it('should handle import errors', () => {
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      const imported = importUIState('invalid json');

      expect(imported).toBeNull();
      expect(consoleSpy).toHaveBeenCalledWith('Failed to import UI state:', expect.any(SyntaxError));

      consoleSpy.mockRestore();
    });
  });
});

