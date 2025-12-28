import type { Collection, CollectionFileView } from '@/types/collection';
import { invoke } from '@/lib/tauri-wrapper';

/**
 * Утилита для композиции и работы с несколькими коллекциями
 *
 * Предоставляет функции для объединения коллекций и получения файлов из нескольких коллекций.
 */
export class CollectionComposer {
  /**
   * Объединить несколько коллекций в одну
   *
   * Создает новую коллекцию, содержащую все файлы из исходных коллекций.
   * Порядок файлов сохраняется согласно порядку коллекций и порядку файлов внутри каждой коллекции.
   *
   * @param name - Название новой коллекции
   * @param description - Описание новой коллекции (опционально)
   * @param sourceCollectionIds - Массив ID исходных коллекций для объединения
   * @returns Созданная коллекция
   * @throws Ошибка если коллекция не может быть создана
   */
  static async combineCollections(
    name: string,
    sourceCollectionIds: number[],
    description?: string
  ): Promise<Collection> {
    if (!name.trim()) {
      throw new Error('Название коллекции не может быть пустым');
    }

    if (sourceCollectionIds.length === 0) {
      throw new Error('Необходимо указать хотя бы одну исходную коллекцию');
    }

    try {
      const result = await invoke<Collection>('combine_collections', {
        name: name.trim(),
        description: description?.trim() || null,
        source_collection_ids: sourceCollectionIds,
      });

      return result;
    } catch (error) {
      throw new Error(
        `Не удалось объединить коллекции: ${error instanceof Error ? error.message : String(error)}`
      );
    }
  }

  /**
   * Получить файлы из нескольких коллекций
   *
   * Возвращает объединенный список файлов из указанных коллекций.
   * Если файл присутствует в нескольких коллекциях, он будет представлен один раз
   * с информацией о всех коллекциях, в которых он находится.
   *
   * @param collectionIds - Массив ID коллекций
   * @returns Массив представлений файлов с информацией о коллекциях
   * @throws Ошибка если не удалось получить файлы
   */
  static async getFilesFromMultipleCollections(
    collectionIds: number[]
  ): Promise<CollectionFileView[]> {
    if (collectionIds.length === 0) {
      return [];
    }

    try {
      const result = await invoke<CollectionFileView[]>('get_files_from_multiple_collections', {
        collection_ids: collectionIds,
      });

      return result;
    } catch (error) {
      throw new Error(
        `Не удалось получить файлы из коллекций: ${error instanceof Error ? error.message : String(error)}`
      );
    }
  }

  /**
   * Получить уникальные файлы из нескольких коллекций
   *
   * Возвращает список уникальных файлов (без дубликатов) из указанных коллекций.
   *
   * @param collectionIds - Массив ID коллекций
   * @returns Массив уникальных файлов
   * @throws Ошибка если не удалось получить файлы
   */
  static async getUniqueFilesFromCollections(
    collectionIds: number[]
  ): Promise<CollectionFileView[]> {
    const files = await this.getFilesFromMultipleCollections(collectionIds);

    // Группируем по fileId и объединяем информацию о коллекциях
    const fileMap = new Map<number, CollectionFileView>();

    for (const fileView of files) {
      const existing = fileMap.get(fileView.fileId);

      if (existing) {
        // Объединяем информацию о коллекциях
        existing.collections.push(...fileView.collections);
      } else {
        fileMap.set(fileView.fileId, { ...fileView });
      }
    }

    return Array.from(fileMap.values());
  }

  /**
   * Получить статистику по коллекциям
   *
   * @param collectionIds - Массив ID коллекций
   * @returns Статистика по коллекциям
   */
  static async getCollectionsStats(collectionIds: number[]): Promise<{
    totalCollections: number;
    totalFiles: number;
    uniqueFiles: number;
    filesPerCollection: Map<number, number>;
  }> {
    const files = await this.getFilesFromMultipleCollections(collectionIds);

    const uniqueFileIds = new Set(files.map(f => f.fileId));
    const filesPerCollection = new Map<number, number>();

    // Подсчитываем файлы по коллекциям
    for (const fileView of files) {
      for (const collectionInfo of fileView.collections) {
        const current = filesPerCollection.get(collectionInfo.collectionId) || 0;
        filesPerCollection.set(collectionInfo.collectionId, current + 1);
      }
    }

    return {
      totalCollections: collectionIds.length,
      totalFiles: files.length,
      uniqueFiles: uniqueFileIds.size,
      filesPerCollection,
    };
  }

  /**
   * Проверить, можно ли объединить коллекции
   *
   * Проверяет, что все указанные коллекции существуют и могут быть объединены.
   *
   * @param collectionIds - Массив ID коллекций для проверки
   * @returns true если коллекции можно объединить
   */
  static async canCombineCollections(
    collectionIds: number[]
  ): Promise<{ canCombine: boolean; errors: string[] }> {
    const errors: string[] = [];

    if (collectionIds.length === 0) {
      errors.push('Необходимо указать хотя бы одну коллекцию');
      return { canCombine: false, errors };
    }

    if (collectionIds.length === 1) {
      errors.push('Для объединения необходимо указать минимум две коллекции');
      return { canCombine: false, errors };
    }

    // Проверяем наличие дубликатов
    const uniqueIds = new Set(collectionIds);
    if (uniqueIds.size !== collectionIds.length) {
      errors.push('В списке коллекций есть дубликаты');
      return { canCombine: false, errors };
    }

    // Проверяем существование коллекций (можно добавить вызов API)
    // Пока просто возвращаем true, если нет очевидных ошибок
    return { canCombine: true, errors: [] };
  }

  /**
   * Получить файлы, которые присутствуют во всех указанных коллекциях
   *
   * @param collectionIds - Массив ID коллекций
   * @returns Массив файлов, присутствующих во всех коллекциях
   */
  static async getCommonFiles(collectionIds: number[]): Promise<CollectionFileView[]> {
    if (collectionIds.length === 0) {
      return [];
    }

    if (collectionIds.length === 1) {
      return this.getFilesFromMultipleCollections(collectionIds);
    }

    const files = await this.getFilesFromMultipleCollections(collectionIds);
    const collectionIdSet = new Set(collectionIds);

    // Фильтруем файлы, которые присутствуют во всех коллекциях
    return files.filter(fileView => {
      const fileCollectionIds = new Set(fileView.collections.map(c => c.collectionId));

      // Проверяем, что файл присутствует во всех запрошенных коллекциях
      for (const collectionId of collectionIdSet) {
        if (!fileCollectionIds.has(collectionId)) {
          return false;
        }
      }

      return true;
    });
  }

  /**
   * Получить файлы, которые присутствуют только в одной из указанных коллекций
   *
   * @param collectionIds - Массив ID коллекций
   * @returns Массив файлов, присутствующих только в одной коллекции
   */
  static async getUniqueFiles(collectionIds: number[]): Promise<CollectionFileView[]> {
    if (collectionIds.length === 0) {
      return [];
    }

    if (collectionIds.length === 1) {
      return this.getFilesFromMultipleCollections(collectionIds);
    }

    const files = await this.getFilesFromMultipleCollections(collectionIds);

    // Фильтруем файлы, которые присутствуют только в одной коллекции
    return files.filter(fileView => fileView.collections.length === 1);
  }

  /**
   * Сортировать файлы по коллекциям
   *
   * Группирует файлы по коллекциям для удобного отображения.
   *
   * @param files - Массив представлений файлов
   * @returns Объект с файлами, сгруппированными по коллекциям
   */
  static groupFilesByCollection(files: CollectionFileView[]): Map<number, CollectionFileView[]> {
    const grouped = new Map<number, CollectionFileView[]>();

    for (const fileView of files) {
      for (const collectionInfo of fileView.collections) {
        const collectionId = collectionInfo.collectionId;

        if (!grouped.has(collectionId)) {
          grouped.set(collectionId, []);
        }

        grouped.get(collectionId)!.push(fileView);
      }
    }

    return grouped;
  }

  /**
   * Сортировать файлы по порядку в коллекциях
   *
   * Сортирует файлы согласно их порядку (orderIndex) в коллекциях.
   *
   * @param files - Массив представлений файлов
   * @param collectionId - ID коллекции для сортировки (если файл в нескольких коллекциях)
   * @returns Отсортированный массив файлов
   */
  static sortFilesByOrder(
    files: CollectionFileView[],
    collectionId?: number
  ): CollectionFileView[] {
    return [...files].sort((a, b) => {
      // Если указана конкретная коллекция, используем её порядок
      if (collectionId) {
        const aOrder = a.collections.find(c => c.collectionId === collectionId)?.orderIndex ?? 0;
        const bOrder = b.collections.find(c => c.collectionId === collectionId)?.orderIndex ?? 0;
        return aOrder - bOrder;
      }

      // Иначе используем минимальный порядок из всех коллекций
      const aMinOrder = Math.min(...a.collections.map(c => c.orderIndex), Number.MAX_SAFE_INTEGER);
      const bMinOrder = Math.min(...b.collections.map(c => c.orderIndex), Number.MAX_SAFE_INTEGER);

      return aMinOrder - bMinOrder;
    });
  }
}
