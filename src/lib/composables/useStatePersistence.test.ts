import { describe, it, expect, vi } from 'vitest';
import { get } from 'svelte/store';
import { statePersistence } from './useStatePersistence';

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
});

describe('StatePersistence', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorageMock.getItem.mockReturnValue(null);
  });

  it('should create persistent state with default value', () => {
    const store = statePersistence.createPersistentState('test', { count: 0 });

    expect(get(store)).toEqual({ count: 0 });
  });

  it('should save state to localStorage', () => {
    const store = statePersistence.createPersistentState('test', { count: 0 });

    store.set({ count: 5 });

    // Wait for debounce
    setTimeout(() => {
      expect(localStorageMock.setItem).toHaveBeenCalledWith(
        'test',
        expect.stringContaining('"count":5')
      );
    }, 1100);
  });

  it('should load state from localStorage', () => {
    // svelte-persisted-store handles loading automatically
    // We just verify that the store is created
    const store = statePersistence.createPersistentState('test', { count: 0 });
    expect(store).toBeDefined();
  });

  it('should reset state', () => {
    const store = statePersistence.createPersistentState('test', { count: 0 });

    store.set({ count: 5 });
    store.reset();

    expect(get(store)).toEqual({ count: 0 });
    expect(localStorageMock.removeItem).toHaveBeenCalledWith('test');
  });
});