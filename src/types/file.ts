/**
 * Идентификатор файла в формате name@version
 */
export type FileId = string;

/**
 * Модель файла/мода в системе
 *
 * Файл идентифицируется комбинацией name@version (как в npm пакетах).
 * Множественные версии одного файла могут сосуществовать.
 */
export interface File {
  id: number;
  name: string;
  version: string;
  path?: string;
  metadata: Record<string, unknown>;
  createdAt: string;
  updatedAt: string;
}

/**
 * Создать идентификатор файла из имени и версии
 */
export function createFileId(name: string, version: string): FileId {
  return `${name}@${version}`;
}

/**
 * Парсить идентификатор файла на имя и версию
 */
export function parseFileId(fileId: FileId): { name: string; version: string } | null {
  const parts = fileId.split('@');
  if (parts.length !== 2) {
    return null;
  }
  return { name: parts[0], version: parts[1] };
}

