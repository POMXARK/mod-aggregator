import type {
  DependencyGraph as DependencyGraphType,
  GraphNode,
  GraphEdge,
} from '@/types/dependency';

/**
 * Утилита для работы с графом зависимостей на frontend
 */
export class DependencyGraph {
  private nodes: Map<number, GraphNode> = new Map();
  private edges: GraphEdge[] = [];

  /**
   * Построить граф из данных
   */
  static fromData(data: DependencyGraphType): DependencyGraph {
    const graph = new DependencyGraph();

    for (const node of data.nodes) {
      graph.nodes.set(node.fileId, node);
    }

    graph.edges = [...data.edges];

    return graph;
  }

  /**
   * Получить все узлы графа
   */
  getNodes(): GraphNode[] {
    return Array.from(this.nodes.values());
  }

  /**
   * Получить все рёбра графа
   */
  getEdges(): GraphEdge[] {
    return [...this.edges];
  }

  /**
   * Получить зависимости файла
   */
  getDependencies(fileId: number): GraphEdge[] {
    return this.edges.filter(edge => edge.fromFileId === fileId);
  }

  /**
   * Получить файлы, зависящие от указанного файла
   */
  getDependents(fileId: number): GraphEdge[] {
    return this.edges.filter(edge => edge.toFileId === fileId);
  }

  /**
   * Проверить, есть ли у файла отсутствующие зависимости
   */
  hasMissingDependencies(fileId: number): boolean {
    const node = this.nodes.get(fileId);
    return node?.hasMissingDependencies ?? false;
  }

  /**
   * Получить все файлы с отсутствующими зависимостями
   */
  getFilesWithMissingDependencies(): GraphNode[] {
    return this.getNodes().filter(node => node.hasMissingDependencies);
  }

  /**
   * Получить топологический порядок файлов (для установки)
   */
  getTopologicalOrder(): number[] {
    const visited = new Set<number>();
    const result: number[] = [];
    const inDegree = new Map<number, number>();

    // Инициализируем степени входа
    for (const node of this.nodes.keys()) {
      inDegree.set(node, 0);
    }

    // Подсчитываем степени входа
    for (const edge of this.edges) {
      if (edge.satisfied) {
        const current = inDegree.get(edge.toFileId) ?? 0;
        inDegree.set(edge.toFileId, current + 1);
      }
    }

    // Находим файлы без зависимостей
    const queue: number[] = [];
    for (const [fileId, degree] of inDegree.entries()) {
      if (degree === 0) {
        queue.push(fileId);
      }
    }

    // Топологическая сортировка
    while (queue.length > 0) {
      const fileId = queue.shift()!;
      if (visited.has(fileId)) {
        continue;
      }

      visited.add(fileId);
      result.push(fileId);

      // Уменьшаем степени входа для зависимых файлов
      for (const edge of this.getDependencies(fileId)) {
        if (edge.satisfied) {
          const current = inDegree.get(edge.toFileId) ?? 0;
          inDegree.set(edge.toFileId, current - 1);
          if (inDegree.get(edge.toFileId) === 0) {
            queue.push(edge.toFileId);
          }
        }
      }
    }

    // Добавляем оставшиеся файлы (без зависимостей)
    for (const fileId of this.nodes.keys()) {
      if (!visited.has(fileId)) {
        result.push(fileId);
      }
    }

    return result;
  }

  /**
   * Найти циклы в графе (упрощенная версия)
   */
  findCycles(): number[][] {
    const cycles: number[][] = [];
    const visited = new Set<number>();
    const recStack = new Set<number>();

    const dfs = (fileId: number, path: number[]): void => {
      if (recStack.has(fileId)) {
        // Найден цикл
        const cycleStart = path.indexOf(fileId);
        if (cycleStart !== -1) {
          cycles.push([...path.slice(cycleStart), fileId]);
        }
        return;
      }

      if (visited.has(fileId)) {
        return;
      }

      visited.add(fileId);
      recStack.add(fileId);

      for (const edge of this.getDependencies(fileId)) {
        if (edge.satisfied) {
          dfs(edge.toFileId, [...path, fileId]);
        }
      }

      recStack.delete(fileId);
    };

    for (const fileId of this.nodes.keys()) {
      if (!visited.has(fileId)) {
        dfs(fileId, []);
      }
    }

    return cycles;
  }
}
