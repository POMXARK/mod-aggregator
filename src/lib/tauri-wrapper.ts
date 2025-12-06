/**
 * Wrapper for Tauri invoke that automatically uses mock in browser mode
 */
import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import { isTauri, mockInvoke } from './tauri-mock';

// Cache the Tauri check result
let tauriCheckCache: boolean | null = null;

/**
 * Check if we're in Tauri by trying to use the API
 */
function checkTauri(): boolean {
  if (tauriCheckCache !== null) return tauriCheckCache;
  
  // First try the simple check
  if (isTauri()) {
    tauriCheckCache = true;
    return true;
  }
  
  // If simple check fails, try to actually use the API
  // This will fail in browser but work in Tauri
  try {
    // @ts-ignore - check if Tauri API is available
    if (typeof window !== 'undefined' && window.__TAURI__) {
      tauriCheckCache = true;
      return true;
    }
  } catch (e) {
    // Not in Tauri
  }
  
  tauriCheckCache = false;
  return false;
}

/**
 * Universal invoke function that works in both Tauri and browser
 */
export async function invoke<T = any>(cmd: string, args?: any): Promise<T> {
  // Check if we're in Tauri
  if (checkTauri()) {
    try {
      // Try to use real Tauri invoke
      return await tauriInvoke<T>(cmd, args);
    } catch (error: any) {
      // If error is about Tauri not being available, use mock
      if (error?.message?.includes('Tauri') || error?.message?.includes('__TAURI__')) {
        console.warn('[tauri-wrapper] Tauri not available, using mock');
        tauriCheckCache = false;
        return mockInvoke<T>(cmd, args);
      }
      // Otherwise, re-throw the error (it's a real Tauri error)
      throw error;
    }
  }
  
  // Running in browser - use mock
  return mockInvoke<T>(cmd, args);
}

