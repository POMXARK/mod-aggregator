/**
 * Mock implementation of Tauri commands for browser development
 * This allows the app to run in a regular browser without Tauri backend
 */

// Check if running in Tauri
export function isTauri(): boolean {
  if (typeof window === 'undefined') return false;
  
  // Check for Tauri 2.0 - multiple ways to detect
  try {
    // Method 1: Check for __TAURI__ object
    // @ts-ignore
    const tauri = window.__TAURI__;
    if (tauri !== undefined && tauri !== null) {
      return true;
    }
    
    // Method 2: Check for __TAURI_INTERNALS__
    // @ts-ignore
    if (window.__TAURI_INTERNALS__) {
      return true;
    }
    
    // Method 3: Check for __TAURI_METADATA__
    // @ts-ignore
    if ((window as any).__TAURI_METADATA__) {
      return true;
    }
    
    // Method 4: Try to access Tauri API through import (will fail in browser)
    // This is checked at runtime in tauri-wrapper.ts
  } catch (e) {
    // If any check throws, we're not in Tauri
    return false;
  }
  
  return false;
}

// Mock data for browser development
const mockSites = [
  {
    id: 1,
    name: 'Example Site 1',
    url: 'https://example.com',
    parser_config: { list_selector: '.item', title_selector: 'h2' },
  },
  {
    id: 2,
    name: 'Example Site 2',
    url: 'https://example.org',
    parser_config: { list_selector: '.mod', title_selector: '.title' },
  },
];

const mockMods = [
  {
    id: 1,
    site_id: 1,
    title: 'Example Mod 1',
    version: '1.0.0',
    author: 'Author 1',
    description: 'This is an example mod',
    download_url: 'https://example.com/mod1.zip',
    updated_at: new Date().toISOString(),
  },
  {
    id: 2,
    site_id: 1,
    title: 'Example Mod 2',
    version: '2.0.0',
    author: 'Author 2',
    description: 'Another example mod',
    download_url: 'https://example.com/mod2.zip',
    updated_at: new Date().toISOString(),
  },
];

const mockNotifications = [
  {
    id: 1,
    title: 'Update available',
    message: 'New version of Mod 1 is available',
    type: 'update',
    read: false,
    created_at: new Date().toISOString(),
  },
];

// Mock invoke function
export async function mockInvoke<T = any>(cmd: string, args?: any): Promise<T> {
  console.log('[MOCK] invoke called:', cmd, args);
  
  // Simulate network delay
  await new Promise(resolve => setTimeout(resolve, 100));
  
  switch (cmd) {
    case 'get_sites':
      return mockSites as T;
    
    case 'get_mods':
      const siteId = args?.siteId;
      if (siteId) {
        return mockMods.filter(m => m.site_id === siteId) as T;
      }
      return mockMods as T;
    
    case 'add_site':
      const newSite = {
        id: mockSites.length + 1,
        name: args?.name || 'New Site',
        url: args?.url || 'https://example.com',
        parser_config: args?.parser_config || {},
      };
      mockSites.push(newSite);
      return newSite as T;
    
    case 'update_site':
      const siteIndex = mockSites.findIndex(s => s.id === args?.id);
      if (siteIndex >= 0) {
        mockSites[siteIndex] = { ...mockSites[siteIndex], ...args };
        return mockSites[siteIndex] as T;
      }
      throw new Error('Site not found');
    
    case 'delete_site':
      const deleteIndex = mockSites.findIndex(s => s.id === args?.id);
      if (deleteIndex >= 0) {
        mockSites.splice(deleteIndex, 1);
      }
      return undefined as T;
    
    case 'check_updates':
      // Simulate checking for updates
      return [] as T;
    
    case 'get_notifications':
      return mockNotifications as T;
    
    case 'mark_notification_read':
      const notifIndex = mockNotifications.findIndex(n => n.id === args?.id);
      if (notifIndex >= 0) {
        mockNotifications[notifIndex].read = true;
      }
      return undefined as T;
    
    case 'fetch_page':
      // In browser, we can't bypass CORS, so return a simple HTML page
      const url = args?.url || 'https://example.com';
      return `
        <!DOCTYPE html>
        <html>
        <head>
          <title>Mock Page</title>
        </head>
        <body>
          <h1>Mock Page for ${url}</h1>
          <p>This is a mock page loaded in browser mode.</p>
          <div class="item">
            <h2>Item 1</h2>
            <p>Description 1</p>
          </div>
          <div class="item">
            <h2>Item 2</h2>
            <p>Description 2</p>
          </div>
        </body>
        </html>
      ` as T;
    
    case 'save_page_local':
      console.warn('[MOCK] save_page_local called - not implemented in browser mode');
      return `file:///mock/path/${args?.filename || 'page.html'}` as T;
    
    case 'build_parser':
    case 'test_parser':
      console.warn(`[MOCK] ${cmd} called - not implemented in browser mode`);
      return [] as T;
    
    default:
      console.warn(`[MOCK] Unknown command: ${cmd}`);
      throw new Error(`Unknown command: ${cmd}`);
  }
}

