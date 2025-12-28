/**
 * JSON Serializer utility for import/export operations
 *
 * Provides utilities for serializing and deserializing files, collections, and builds
 * to/from JSON format for sharing, backup, and transfer between systems.
 */

import type { File } from '@/types/file';
import type { FileDependency } from '@/types/dependency';

export const SCHEMA_VERSION = '1.0';

export type ExportType = 'file' | 'collection' | 'build';

/**
 * Exported dependency structure
 */
export interface ExportedDependency {
  target_file_name: string;
  target_file_version?: string;
  dependency_type: 'required' | 'optional' | 'peer';
}

/**
 * File export structure
 */
export interface FileExport {
  version: string;
  type: 'file';
  data: {
    file: File;
    dependencies: ExportedDependency[];
  };
}

/**
 * Import result
 */
export interface ImportResult {
  file_id?: number;
  collection_id?: number;
  created: boolean;
  missing_dependencies: MissingDependency[];
  missing_files: MissingFile[];
  warnings: string[];
}

/**
 * Missing dependency
 */
export interface MissingDependency {
  target_file_name: string;
  target_file_version?: string;
  available_versions: string[];
}

/**
 * Missing file
 */
export interface MissingFile {
  file_name: string;
  file_version: string;
}

/**
 * Import validation result
 */
export interface ImportValidationResult {
  valid: boolean;
  type?: string;
  version?: string;
  errors: string[];
  warnings: string[];
  preview?: ImportPreview;
}

/**
 * Import preview
 */
export interface ImportPreview {
  files_count: number;
  dependencies_count: number;
  collections_count: number;
  missing_dependencies: MissingDependency[];
}

/**
 * Serialize file to JSON export format
 */
export function serializeFile(file: File, dependencies: FileDependency[]): string {
  const exportedDeps: ExportedDependency[] = dependencies.map(dep => ({
    target_file_name: dep.targetFileName,
    target_file_version: dep.targetFileVersion,
    dependency_type: dep.dependencyType,
  }));

  const exportData: FileExport = {
    version: SCHEMA_VERSION,
    type: 'file',
    data: {
      file,
      dependencies: exportedDeps,
    },
  };

  return JSON.stringify(exportData, null, 2);
}

/**
 * Deserialize file from JSON export format
 */
export function deserializeFile(jsonData: string): {
  file: File;
  dependencies: ExportedDependency[];
} {
  const exportData: FileExport = JSON.parse(jsonData);

  // Validate version
  if (exportData.version !== SCHEMA_VERSION) {
    throw new Error(
      `Invalid schema version: expected ${SCHEMA_VERSION}, got ${exportData.version}`
    );
  }

  // Validate type
  if (exportData.type !== 'file') {
    throw new Error(`Invalid type: expected file, got ${exportData.type}`);
  }

  // Validate data
  if (!exportData.data.file.name || !exportData.data.file.version) {
    throw new Error('File name and version are required');
  }

  return {
    file: exportData.data.file,
    dependencies: exportData.data.dependencies,
  };
}

/**
 * Validate JSON before import
 */
export function validateImportJson(jsonData: string): ImportValidationResult {
  const result: ImportValidationResult = {
    valid: false,
    type: undefined,
    version: undefined,
    errors: [],
    warnings: [],
    preview: undefined,
  };

  try {
    const json = JSON.parse(jsonData);

    if (typeof json !== 'object' || json === null) {
      result.errors.push('JSON must be an object');
      return result;
    }

    // Check version
    if (json.version) {
      result.version = json.version;
      if (json.version !== SCHEMA_VERSION) {
        result.warnings.push(
          `Schema version mismatch: expected ${SCHEMA_VERSION}, got ${json.version}`
        );
      }
    } else {
      result.errors.push('Missing required field: version');
    }

    // Check type
    if (json.type) {
      result.type = json.type;
      if (!['file', 'collection', 'build'].includes(json.type)) {
        result.errors.push(`Unknown import type: ${json.type}`);
      }
    } else {
      result.errors.push('Missing required field: type');
    }

    // Check data
    if (!json.data) {
      result.errors.push('Missing required field: data');
    } else if (result.type === 'file') {
      // Create preview for file import
      const fileData = json.data.file;
      const deps = json.data.dependencies || [];

      result.preview = {
        files_count: fileData ? 1 : 0,
        dependencies_count: Array.isArray(deps) ? deps.length : 0,
        collections_count: 0,
        missing_dependencies: [],
      };
    } else if (['collection', 'build'].includes(result.type || '')) {
      result.warnings.push(`Import type '${result.type}' is not yet fully supported`);
    }

    // If no errors, validation passed
    result.valid = result.errors.length === 0;
  } catch (error) {
    result.errors.push(
      `Invalid JSON format: ${error instanceof Error ? error.message : String(error)}`
    );
  }

  return result;
}

/**
 * Download JSON as file
 */
export function downloadJson(json: string, filename: string): void {
  const blob = new Blob([json], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

/**
 * Read JSON from file input
 */
export function readJsonFromFile(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = e => {
      if (e.target?.result && typeof e.target.result === 'string') {
        resolve(e.target.result);
      } else {
        reject(new Error('Failed to read file'));
      }
    };
    reader.onerror = () => reject(new Error('Failed to read file'));
    reader.readAsText(file);
  });
}
