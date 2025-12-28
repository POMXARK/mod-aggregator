/**
 * Mock implementation of Tauri commands for browser development
 * This allows the app to run in a regular browser without Tauri backend
 */

// Check if running in Tauri
export function isTauri(): boolean {
  if (typeof window === 'undefined') {
    return false;
  }

  // Check for Tauri 2.0 - multiple ways to detect
  try {
    // Method 1: Check for __TAURI__ object
    const tauri = (window as any).__TAURI__;
    if (tauri !== undefined && tauri !== null) {
      return true;
    }

    // Method 2: Check for __TAURI_INTERNALS__
    // @ts-expect-error: Tauri internal properties may not be defined in type definitions
    if (window.__TAURI_INTERNALS__) {
      return true;
    }

    // Method 3: Check for __TAURI_METADATA__
    // @ts-expect-error: Tauri metadata properties may not be defined in type definitions
    if ((window as any).__TAURI_METADATA__) {
      return true;
    }

    // Method 4: Try to access Tauri API through import (will fail in browser)
    // This is checked at runtime in tauri-wrapper.ts
  } catch {
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

// Mock files storage
const mockFiles: Array<{
  id: number;
  name: string;
  version: string;
  path?: string;
  metadata: Record<string, unknown>;
  createdAt: string;
  updatedAt: string;
}> = [];

let nextFileId = 1;

// Mock collections storage
const mockCollections: Array<{
  id: number;
  name: string;
  description?: string;
  created_at: string;
  updated_at: string;
}> = [];

let nextCollectionId = 1;

// Mock collection files storage
const mockCollectionFiles: Array<{
  id: number;
  collection_id: number;
  file_id: number;
  logic_rule_id?: number;
  order_index: number;
  created_at: string;
}> = [];

let nextCollectionFileId = 1;

// Mock collection logic rules storage
const mockCollectionLogicRules: Array<{
  id: number;
  collection_id: number;
  name: string;
  condition_type: string;
  condition_params: Record<string, unknown>;
  action: string;
  created_at: string;
}> = [];

let nextLogicRuleId = 1;

// Mock invoke function
export async function mockInvoke<T = any>(cmd: string, args?: any): Promise<T> {
  console.log('[MOCK] invoke called:', cmd, args);

  // Simulate network delay
  await new Promise(resolve => setTimeout(resolve, 100));

  switch (cmd) {
    case 'get_sites':
      return mockSites as T;

    case 'get_mods': {
      const siteId = args?.siteId;
      if (siteId) {
        return mockMods.filter(m => m.site_id === siteId) as T;
      }
      return mockMods as T;
    }

    case 'add_site': {
      const newSite = {
        id: mockSites.length + 1,
        name: args?.name || 'New Site',
        url: args?.url || 'https://example.com',
        parser_config: args?.parser_config || {},
      };
      mockSites.push(newSite);
      return newSite as T;
    }

    case 'update_site':
      {
        const siteIndex = mockSites.findIndex(s => s.id === args?.id);
        if (siteIndex >= 0) {
          mockSites[siteIndex] = { ...mockSites[siteIndex], ...args };
          return mockSites[siteIndex] as T;
        }
      }
      throw new Error('Site not found');

    case 'delete_site': {
      const deleteIndex = mockSites.findIndex(s => s.id === args?.id);
      if (deleteIndex >= 0) {
        mockSites.splice(deleteIndex, 1);
      }
      return undefined as T;
    }

    case 'check_updates':
      // Simulate checking for updates
      return [] as T;

    case 'get_notifications':
      return mockNotifications as T;

    case 'mark_notification_read': {
      const notifIndex = mockNotifications.findIndex(n => n.id === args?.id);
      if (notifIndex >= 0) {
        mockNotifications[notifIndex].read = true;
      }
      return undefined as T;
    }

    case 'fetch_page': {
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
    }

    case 'save_page_local':
      console.warn('[MOCK] save_page_local called - not implemented in browser mode');
      return `file:///mock/path/${args?.filename || 'page.html'}` as T;

    case 'build_parser':
    case 'test_parser':
      console.warn(`[MOCK] ${cmd} called - not implemented in browser mode`);
      return [] as T;

    // File management commands
    case 'create_file': {
      const params = args?.params || args || {};
      if (!params.name || !params.version) {
        throw new Error('Name and version are required');
      }

      // Check for duplicate
      const existing = mockFiles.find(f => f.name === params.name && f.version === params.version);
      if (existing) {
        throw new Error(`File with name@version already exists: ${params.name}@${params.version}`);
      }

      const newFile = {
        id: nextFileId++,
        name: params.name,
        version: params.version,
        path: params.path,
        metadata: params.metadata || {},
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      mockFiles.push(newFile);
      return newFile as T;
    }

    case 'update_file': {
      const params = args?.params || args || {};
      const file = mockFiles.find(f => f.id === params.id);
      if (!file) {
        throw new Error('File not found');
      }

      // Check for duplicate if name/version changed
      if (params.name || params.version) {
        const newName = params.name || file.name;
        const newVersion = params.version || file.version;
        const duplicate = mockFiles.find(
          f => f.id !== params.id && f.name === newName && f.version === newVersion
        );
        if (duplicate) {
          throw new Error(`File with name@version already exists: ${newName}@${newVersion}`);
        }
      }

      if (params.name) {
        file.name = params.name;
      }
      if (params.version) {
        file.version = params.version;
      }
      if (params.path !== undefined) {
        file.path = params.path;
      }
      if (params.metadata) {
        file.metadata = params.metadata;
      }
      file.updatedAt = new Date().toISOString();

      return file as T;
    }

    case 'delete_file': {
      const fileId = args?.file_id || args?.id;
      const force = args?.force || false;

      const fileIndex = mockFiles.findIndex(f => f.id === fileId);
      if (fileIndex < 0) {
        throw new Error('File not found');
      }

      // Check for dependent files (simplified mock)
      const dependentFiles = mockFiles.filter(_f => {
        // In real implementation, this would check dependencies
        return false; // Mock: no dependencies
      });

      if (dependentFiles.length > 0 && !force) {
        return {
          deleted: false,
          dependent_files: dependentFiles,
          message: `Cannot delete file: ${dependentFiles.length} file(s) depend on it`,
        } as T;
      }

      mockFiles.splice(fileIndex, 1);
      return {
        deleted: true,
        dependent_files: force && dependentFiles.length > 0 ? dependentFiles : undefined,
        message: 'File deleted successfully',
      } as T;
    }

    case 'upload_file_version': {
      const params = args?.params || args || {};
      if (!params.name || !params.version) {
        throw new Error('Name and version are required');
      }

      // Check for duplicate
      const existing = mockFiles.find(f => f.name === params.name && f.version === params.version);
      if (existing) {
        throw new Error(`File with name@version already exists: ${params.name}@${params.version}`);
      }

      const newFile = {
        id: nextFileId++,
        name: params.name,
        version: params.version,
        path: params.path,
        metadata: params.metadata || {},
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      mockFiles.push(newFile);
      return newFile as T;
    }

    case 'get_file_by_name_version': {
      const name = args?.name;
      const version = args?.version;
      const file = mockFiles.find(f => f.name === name && f.version === version);
      if (!file) {
        throw new Error(`File not found: ${name}@${version}`);
      }
      return file as T;
    }

    case 'get_file_versions': {
      const name = args?.name || '';
      if (name === '') {
        return mockFiles as T;
      }
      return mockFiles.filter(f => f.name === name) as T;
    }

    // Import/Export commands
    case 'export_file': {
      const fileId = args?.file_id;
      const file = mockFiles.find(f => f.id === fileId);
      if (!file) {
        throw new Error('File not found');
      }

      // Mock export JSON
      const exportData = {
        version: '1.0',
        type: 'file',
        data: {
          file: {
            ...file,
            createdAt: file.createdAt,
            updatedAt: file.updatedAt,
          },
          dependencies: [],
        },
      };
      return JSON.stringify(exportData, null, 2) as T;
    }

    case 'import_file': {
      const jsonData = args?.json_data;
      if (!jsonData) {
        throw new Error('JSON data is required');
      }

      try {
        const importData = JSON.parse(jsonData);
        if (importData.type !== 'file') {
          throw new Error('Invalid type: expected file');
        }

        const fileData = importData.data.file;
        const existing = mockFiles.find(
          f => f.name === fileData.name && f.version === fileData.version
        );

        if (existing) {
          // Update existing
          Object.assign(existing, {
            path: fileData.path,
            metadata: fileData.metadata || {},
            updatedAt: new Date().toISOString(),
          });
          return {
            file_id: existing.id,
            created: false,
            missing_dependencies: [],
            missing_files: [],
            warnings: [],
          } as T;
        } else {
          // Create new
          const newFile = {
            id: nextFileId++,
            name: fileData.name,
            version: fileData.version,
            path: fileData.path,
            metadata: fileData.metadata || {},
            createdAt: new Date().toISOString(),
            updatedAt: new Date().toISOString(),
          };
          mockFiles.push(newFile);
          return {
            file_id: newFile.id,
            created: true,
            missing_dependencies: [],
            missing_files: [],
            warnings: [],
          } as T;
        }
      } catch (_e) {
        throw new Error(`Invalid JSON format: ${_e instanceof Error ? _e.message : String(_e)}`);
      }
    }

    case 'validate_import_json': {
      const jsonData = args?.json_data;
      if (!jsonData) {
        return {
          valid: false,
          errors: ['JSON data is required'],
          warnings: [],
        } as T;
      }

      try {
        const json = JSON.parse(jsonData);
        const result: any = {
          valid: true,
          type: json.type,
          version: json.version,
          errors: [],
          warnings: [],
        };

        if (!json.version || json.version !== '1.0') {
          result.warnings.push(
            `Schema version mismatch: expected 1.0, got ${json.version || 'unknown'}`
          );
        }

        if (!json.type) {
          result.valid = false;
          result.errors.push('Missing required field: type');
        }

        if (!json.data) {
          result.valid = false;
          result.errors.push('Missing required field: data');
        } else if (json.type === 'file' && json.data.file) {
          result.preview = {
            files_count: 1,
            dependencies_count: (json.data.dependencies || []).length,
            collections_count: 0,
            missing_dependencies: [],
          };
        }

        return result as T;
      } catch (_e) {
        return {
          valid: false,
          errors: [`Invalid JSON format: ${_e instanceof Error ? _e.message : String(_e)}`],
          warnings: [],
        } as T;
      }
    }

    case 'export_collection':
    case 'export_build':
    case 'import_collection':
    case 'import_build':
      console.warn(`[MOCK] ${cmd} called - not yet implemented`);
      throw new Error(`${cmd} is not yet implemented`);

    // Collection commands
    case 'get_collections': {
      return mockCollections as T;
    }

    case 'create_collection': {
      const params = args?.params || args || {};
      if (!params.name) {
        throw new Error('Name is required');
      }

      const existing = mockCollections.find(c => c.name === params.name);
      if (existing) {
        throw new Error(`Collection name already exists: ${params.name}`);
      }

      const now = new Date().toISOString();
      const newCollection = {
        id: nextCollectionId++,
        name: params.name,
        description: params.description,
        created_at: now,
        updated_at: now,
      };
      mockCollections.push(newCollection);
      return newCollection as T;
    }

    case 'update_collection': {
      const params = args?.params || args || {};
      const collection = mockCollections.find(c => c.id === params.id);
      if (!collection) {
        throw new Error('Collection not found');
      }

      if (params.name) {
        const duplicate = mockCollections.find(c => c.id !== params.id && c.name === params.name);
        if (duplicate) {
          throw new Error(`Collection name already exists: ${params.name}`);
        }
        collection.name = params.name;
      }
      if (params.description !== undefined) {
        collection.description = params.description;
      }
      collection.updated_at = new Date().toISOString();

      return collection as T;
    }

    case 'delete_collection': {
      const collectionId = args?.collection_id || args?.id;
      const index = mockCollections.findIndex(c => c.id === collectionId);
      if (index < 0) {
        throw new Error('Collection not found');
      }

      // Remove collection files
      const filesToRemove = mockCollectionFiles.filter(cf => cf.collection_id === collectionId);
      filesToRemove.forEach(cf => {
        const fileIndex = mockCollectionFiles.findIndex(f => f.id === cf.id);
        if (fileIndex >= 0) {
          mockCollectionFiles.splice(fileIndex, 1);
        }
      });

      mockCollections.splice(index, 1);
      return undefined as T;
    }

    case 'get_collection_files': {
      const collectionId = args?.collection_id;
      const collectionFiles = mockCollectionFiles
        .filter(cf => cf.collection_id === collectionId)
        .map(cf => {
          const file = mockFiles.find(f => f.id === cf.file_id);
          const rule = cf.logic_rule_id
            ? mockCollectionLogicRules.find(r => r.id === cf.logic_rule_id)
            : undefined;

          return {
            id: cf.id,
            collection_id: cf.collection_id,
            file_id: cf.file_id,
            file: file || null,
            logic_rule_id: cf.logic_rule_id,
            logic_rule: rule || null,
            order_index: cf.order_index,
            created_at: cf.created_at,
          };
        });
      return collectionFiles as T;
    }

    case 'add_file_to_collection': {
      const params = args?.params || args || {};
      if (!params.collection_id || !params.file_id) {
        throw new Error('Collection ID and file ID are required');
      }

      const collection = mockCollections.find(c => c.id === params.collection_id);
      if (!collection) {
        throw new Error('Collection not found');
      }

      const file = mockFiles.find(f => f.id === params.file_id);
      if (!file) {
        throw new Error('File not found');
      }

      const existing = mockCollectionFiles.find(
        cf => cf.collection_id === params.collection_id && cf.file_id === params.file_id
      );
      if (existing) {
        throw new Error('File already in collection');
      }

      const now = new Date().toISOString();
      const newCollectionFile = {
        id: nextCollectionFileId++,
        collection_id: params.collection_id,
        file_id: params.file_id,
        logic_rule_id: params.logic_rule_id,
        order_index: params.order_index || 0,
        created_at: now,
      };
      mockCollectionFiles.push(newCollectionFile);

      const rule = newCollectionFile.logic_rule_id
        ? mockCollectionLogicRules.find(r => r.id === newCollectionFile.logic_rule_id)
        : undefined;

      return {
        id: newCollectionFile.id,
        collection_id: newCollectionFile.collection_id,
        file_id: newCollectionFile.file_id,
        file: file,
        logic_rule_id: newCollectionFile.logic_rule_id,
        logic_rule: rule || null,
        order_index: newCollectionFile.order_index,
        created_at: newCollectionFile.created_at,
      } as T;
    }

    case 'remove_file_from_collection': {
      const collectionId = args?.collection_id;
      const fileId = args?.file_id;

      const index = mockCollectionFiles.findIndex(
        cf => cf.collection_id === collectionId && cf.file_id === fileId
      );
      if (index < 0) {
        throw new Error('File not in collection');
      }

      mockCollectionFiles.splice(index, 1);
      return undefined as T;
    }

    case 'reorder_collection_files': {
      const collectionId = args?.collection_id;
      const fileOrders = args?.file_orders || [];

      fileOrders.forEach((fo: { file_id: number; order_index: number }) => {
        const cf = mockCollectionFiles.find(
          c => c.collection_id === collectionId && c.file_id === fo.file_id
        );
        if (cf) {
          cf.order_index = fo.order_index;
        }
      });

      return undefined as T;
    }

    case 'get_collection_logic_rules': {
      const collectionId = args?.collection_id;
      const rules = mockCollectionLogicRules
        .filter(r => r.collection_id === collectionId)
        .map(r => ({
          id: r.id,
          collection_id: r.collection_id,
          name: r.name,
          condition_type: r.condition_type,
          condition_params: r.condition_params,
          action: r.action,
          created_at: r.created_at,
        }));
      return rules as T;
    }

    case 'create_collection_logic_rule': {
      const params = args?.params || args || {};
      if (!params.collection_id || !params.name || !params.condition_type || !params.action) {
        throw new Error('Collection ID, name, condition type, and action are required');
      }

      const collection = mockCollections.find(c => c.id === params.collection_id);
      if (!collection) {
        throw new Error('Collection not found');
      }

      const now = new Date().toISOString();
      const newRule = {
        id: nextLogicRuleId++,
        collection_id: params.collection_id,
        name: params.name,
        condition_type: params.condition_type,
        condition_params: params.condition_params || {},
        action: params.action,
        created_at: now,
      };
      mockCollectionLogicRules.push(newRule);

      return {
        id: newRule.id,
        collection_id: newRule.collection_id,
        name: newRule.name,
        condition_type: newRule.condition_type,
        condition_params: newRule.condition_params,
        action: newRule.action,
        created_at: newRule.created_at,
      } as T;
    }

    case 'update_collection_logic_rule': {
      const params = args?.params || args || {};
      const rule = mockCollectionLogicRules.find(r => r.id === params.id);
      if (!rule) {
        throw new Error('Rule not found');
      }

      if (params.name) {
        rule.name = params.name;
      }
      if (params.condition_type) {
        rule.condition_type = params.condition_type;
      }
      if (params.condition_params) {
        rule.condition_params = params.condition_params;
      }
      if (params.action) {
        rule.action = params.action;
      }

      return {
        id: rule.id,
        collection_id: rule.collection_id,
        name: rule.name,
        condition_type: rule.condition_type,
        condition_params: rule.condition_params,
        action: rule.action,
        created_at: rule.created_at,
      } as T;
    }

    case 'delete_collection_logic_rule': {
      const ruleId = args?.rule_id || args?.id;
      const index = mockCollectionLogicRules.findIndex(r => r.id === ruleId);
      if (index < 0) {
        throw new Error('Rule not found');
      }

      mockCollectionLogicRules.splice(index, 1);
      return undefined as T;
    }

    case 'evaluate_collection_logic': {
      const collectionId = args?.collection_id;
      const collectionFiles = mockCollectionFiles.filter(cf => cf.collection_id === collectionId);

      const enabledFiles: number[] = [];
      const disabledFiles: number[] = [];
      const evaluationDetails: Array<{
        file_id: number;
        enabled: boolean;
        applied_rules: number[];
      }> = [];

      collectionFiles.forEach(cf => {
        let enabled = true;
        const appliedRules: number[] = [];

        if (cf.logic_rule_id) {
          const rule = mockCollectionLogicRules.find(r => r.id === cf.logic_rule_id);
          if (rule) {
            appliedRules.push(rule.id);
            // Simplified evaluation: boolean rules only
            if (rule.condition_type === 'boolean') {
              const value = (rule.condition_params as any).value;
              enabled = rule.action === 'enable' ? value : !value;
            }
          }
        }

        if (enabled) {
          enabledFiles.push(cf.file_id);
        } else {
          disabledFiles.push(cf.file_id);
        }

        evaluationDetails.push({
          file_id: cf.file_id,
          enabled,
          applied_rules: appliedRules,
        });
      });

      return {
        collection_id: collectionId,
        enabled_files: enabledFiles,
        disabled_files: disabledFiles,
        evaluation_details: evaluationDetails,
      } as T;
    }

    case 'combine_collections': {
      const params = args?.params || args || {};
      if (
        !params.name ||
        !params.source_collection_ids ||
        params.source_collection_ids.length === 0
      ) {
        throw new Error('Name and source collection IDs are required');
      }

      const existing = mockCollections.find(c => c.name === params.name);
      if (existing) {
        throw new Error(`Collection name already exists: ${params.name}`);
      }

      // Check all source collections exist
      params.source_collection_ids.forEach((id: number) => {
        if (!mockCollections.find(c => c.id === id)) {
          throw new Error(`Source collection not found: ${id}`);
        }
      });

      // Create new collection
      const now = new Date().toISOString();
      const newCollection = {
        id: nextCollectionId++,
        name: params.name,
        description: params.description,
        created_at: now,
        updated_at: now,
      };
      mockCollections.push(newCollection);

      // Collect all files from source collections
      const allFileIds = new Set<number>();
      params.source_collection_ids.forEach((collectionId: number) => {
        mockCollectionFiles
          .filter(cf => cf.collection_id === collectionId)
          .forEach(cf => allFileIds.add(cf.file_id));
      });

      // Filter by selected files if provided
      const fileIdsToAdd = params.selected_file_ids || Array.from(allFileIds);

      // Add files to new collection
      fileIdsToAdd.forEach((fileId: number, index: number) => {
        if (allFileIds.has(fileId)) {
          const now = new Date().toISOString();
          mockCollectionFiles.push({
            id: nextCollectionFileId++,
            collection_id: newCollection.id,
            file_id: fileId,
            logic_rule_id: undefined,
            order_index: index,
            created_at: now,
          });
        }
      });

      return newCollection as T;
    }

    case 'get_files_from_multiple_collections': {
      const collectionIds = args?.collection_ids || [];
      const filesMap = new Map<
        number,
        {
          file_id: number;
          file: any;
          collections: Array<{
            collection_id: number;
            collection_name: string;
            order_index: number;
            logic_rule_id?: number;
          }>;
        }
      >();

      collectionIds.forEach((collectionId: number) => {
        const collection = mockCollections.find(c => c.id === collectionId);
        if (!collection) {
          throw new Error(`Collection not found: ${collectionId}`);
        }

        mockCollectionFiles
          .filter(cf => cf.collection_id === collectionId)
          .forEach(cf => {
            const file = mockFiles.find(f => f.id === cf.file_id);
            if (!file) {
              return;
            }

            if (!filesMap.has(cf.file_id)) {
              filesMap.set(cf.file_id, {
                file_id: cf.file_id,
                file: file,
                collections: [],
              });
            }

            const entry = filesMap.get(cf.file_id)!;
            entry.collections.push({
              collection_id: collectionId,
              collection_name: collection.name,
              order_index: cf.order_index,
              logic_rule_id: cf.logic_rule_id,
            });
          });
      });

      return Array.from(filesMap.values()) as T;
    }

    default:
      console.warn(`[MOCK] Unknown command: ${cmd}`);
      throw new Error(`Unknown command: ${cmd}`);
  }
}
