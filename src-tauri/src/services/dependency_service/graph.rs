use crate::models::{File, FileDependency};
use std::collections::{HashMap, HashSet, VecDeque};

/// Граф зависимостей файлов
///
/// Представляет направленный граф, где узлы - файлы, а рёбра - зависимости.
/// Используется для валидации, обнаружения циклов и определения порядка установки.
pub struct DependencyGraph {
    /// Карта файлов по их ID
    nodes: HashMap<i64, File>,
    /// Карта зависимостей: source_file_id -> Vec<FileDependency>
    edges: HashMap<i64, Vec<FileDependency>>,
    /// Обратные зависимости: target_file_name@version -> Vec<source_file_id>
    reverse_edges: HashMap<String, Vec<i64>>,
}

impl DependencyGraph {
    /// Создать новый граф зависимостей
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            reverse_edges: HashMap::new(),
        }
    }

    /// Добавить файл в граф
    pub fn add_file(&mut self, file: File) {
        self.nodes.insert(file.id, file);
    }

    /// Добавить зависимость в граф
    pub fn add_dependency(&mut self, dependency: FileDependency) {
        // Добавляем прямую зависимость
        self.edges
            .entry(dependency.source_file_id)
            .or_insert_with(Vec::new)
            .push(dependency.clone());

        // Добавляем обратную зависимость для быстрого поиска зависимых файлов
        let target_key = if let Some(version) = &dependency.target_file_version {
            format!("{}@{}", dependency.target_file_name, version)
        } else {
            dependency.target_file_name.clone()
        };
        self.reverse_edges
            .entry(target_key)
            .or_insert_with(Vec::new)
            .push(dependency.source_file_id);
    }

    /// Получить все файлы, зависящие от данного файла
    ///
    /// # Параметры
    /// * `file_id` - ID файла
    ///
    /// # Возвращает
    /// Вектор ID файлов, которые зависят от данного
    #[allow(dead_code)]
    pub fn get_dependents(&self, file_id: i64) -> Vec<i64> {
        let file = match self.nodes.get(&file_id) {
            Some(f) => f,
            None => return Vec::new(),
        };

        let file_key = format!("{}@{}", file.name, file.version);
        self.reverse_edges
            .get(&file_key)
            .cloned()
            .unwrap_or_default()
    }

    /// Получить все зависимости файла
    ///
    /// # Параметры
    /// * `file_id` - ID файла
    ///
    /// # Возвращает
    /// Вектор зависимостей файла
    pub fn get_dependencies(&self, file_id: i64) -> Vec<&FileDependency> {
        self.edges
            .get(&file_id)
            .map(|deps| deps.iter().collect())
            .unwrap_or_default()
    }

    /// Проверить наличие циклической зависимости
    ///
    /// Использует DFS для обнаружения циклов в графе.
    ///
    /// # Параметры
    /// * `file_id` - ID файла для проверки
    /// * `target_file_name` - имя целевого файла
    /// * `target_file_version` - версия целевого файла (опционально)
    ///
    /// # Возвращает
    /// true, если добавление зависимости создаст цикл
    pub fn check_circular(
        &self,
        file_id: i64,
        target_file_name: &str,
        target_file_version: Option<&str>,
    ) -> bool {
        // Создаем временный граф с новой зависимостью для проверки
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        // Начинаем DFS от целевого файла
        let _target_key = if let Some(version) = target_file_version {
            format!("{}@{}", target_file_name, version)
        } else {
            target_file_name.to_string()
        };

        // Ищем файл с таким именем и версией
        let target_file_id = self.nodes.values().find_map(|f| {
            if f.name == target_file_name {
                if let Some(version) = target_file_version {
                    if f.version == version {
                        return Some(f.id);
                    }
                } else {
                    return Some(f.id);
                }
            }
            None
        });

        if let Some(target_id) = target_file_id {
            // Проверяем, есть ли путь от target к file_id (это создаст цикл)
            self.dfs_check_cycle(target_id, file_id, &mut visited, &mut rec_stack)
        } else {
            false // Файл не найден, цикла нет
        }
    }

    /// DFS для проверки цикла
    fn dfs_check_cycle(
        &self,
        current: i64,
        target: i64,
        visited: &mut HashSet<i64>,
        rec_stack: &mut HashSet<i64>,
    ) -> bool {
        if current == target {
            return true; // Найден цикл
        }

        visited.insert(current);
        rec_stack.insert(current);

        if let Some(dependencies) = self.edges.get(&current) {
            for dep in dependencies {
                // Находим ID целевого файла зависимости
                if let Some(target_file) = self.find_file_by_name_version(
                    &dep.target_file_name,
                    dep.target_file_version.as_deref(),
                ) {
                    if !visited.contains(&target_file.id) {
                        if self.dfs_check_cycle(target_file.id, target, visited, rec_stack) {
                            return true;
                        }
                    } else if rec_stack.contains(&target_file.id) {
                        return true; // Найден цикл
                    }
                }
            }
        }

        rec_stack.remove(&current);
        false
    }

    /// Найти файл по имени и версии
    fn find_file_by_name_version(&self, name: &str, version: Option<&str>) -> Option<&File> {
        self.nodes
            .values()
            .find(|f| f.name == name && version.map_or(true, |v| f.version == v))
    }

    /// Получить порядок установки файлов (топологическая сортировка)
    ///
    /// Использует алгоритм Kahn для топологической сортировки.
    ///
    /// # Параметры
    /// * `file_ids` - список ID файлов для установки
    ///
    /// # Возвращает
    /// Вектор ID файлов в порядке установки (зависимости первыми)
    pub fn get_installation_order(&self, file_ids: &[i64]) -> Result<Vec<i64>, String> {
        // Алгоритм Kahn сам обнаружит циклы, если не все файлы будут обработаны

        // Алгоритм Kahn для топологической сортировки
        let mut in_degree: HashMap<i64, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Инициализируем степени входа
        for &file_id in file_ids {
            in_degree.insert(file_id, 0);
        }

        // Подсчитываем степени входа (сколько зависимостей имеет каждый файл)
        // Если A -> B (A зависит от B), то A имеет зависимость, B должен быть установлен первым
        // in_degree для файла = количество файлов, от которых он зависит
        for &file_id in file_ids {
            if let Some(dependencies) = self.edges.get(&file_id) {
                for dep in dependencies {
                    if let Some(target_file) = self.find_file_by_name_version(
                        &dep.target_file_name,
                        dep.target_file_version.as_deref(),
                    ) {
                        if file_ids.contains(&target_file.id) {
                            // file_id зависит от target_file
                            *in_degree.entry(file_id).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        // Находим файлы без зависимостей
        for &file_id in file_ids {
            if in_degree.get(&file_id).copied().unwrap_or(0) == 0 {
                queue.push_back(file_id);
            }
        }

        // Топологическая сортировка
        while let Some(file_id) = queue.pop_front() {
            result.push(file_id);

            // Уменьшаем степени входа для файлов, которые зависят от file_id
            // file_id только что установлен, поэтому все файлы, зависящие от него,
            // теперь имеют на одну зависимость меньше
            // Ищем все файлы, которые имеют зависимость на file_id
            for &other_file_id in file_ids {
                if let Some(dependencies) = self.edges.get(&other_file_id) {
                    for dep in dependencies {
                        if let Some(target_file) = self.find_file_by_name_version(
                            &dep.target_file_name,
                            dep.target_file_version.as_deref(),
                        ) {
                            if target_file.id == file_id {
                                // other_file_id зависит от file_id (target_file)
                                if let Some(degree) = in_degree.get_mut(&other_file_id) {
                                    *degree -= 1;
                                    if *degree == 0 {
                                        queue.push_back(other_file_id);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Проверяем, что все файлы обработаны
        if result.len() != file_ids.len() {
            return Err("Circular dependencies detected".to_string());
        }

        Ok(result)
    }

    /// Проверить наличие цикла от файла
    #[allow(dead_code)]
    pub fn has_cycle_from(&self, file_id: i64) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        self.dfs_cycle(file_id, &mut visited, &mut rec_stack)
    }

    /// DFS для обнаружения цикла
    #[allow(dead_code)]
    fn dfs_cycle(
        &self,
        current: i64,
        visited: &mut HashSet<i64>,
        rec_stack: &mut HashSet<i64>,
    ) -> bool {
        visited.insert(current);
        rec_stack.insert(current);

        if let Some(dependencies) = self.edges.get(&current) {
            for dep in dependencies {
                if let Some(target_file) = self.find_file_by_name_version(
                    &dep.target_file_name,
                    dep.target_file_version.as_deref(),
                ) {
                    if !visited.contains(&target_file.id) {
                        if self.dfs_cycle(target_file.id, visited, rec_stack) {
                            return true;
                        }
                    } else if rec_stack.contains(&target_file.id) {
                        return true; // Найден цикл
                    }
                }
            }
        }

        rec_stack.remove(&current);
        false
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{DependencyType, File, FileDependency};
    use chrono::Utc;

    fn create_test_file(id: i64, name: &str, version: &str) -> File {
        File {
            id,
            name: name.to_string(),
            version: version.to_string(),
            path: None,
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_test_dependency(
        source_id: i64,
        target_name: &str,
        target_version: Option<&str>,
        dep_type: DependencyType,
    ) -> FileDependency {
        FileDependency {
            id: 0,
            source_file_id: source_id,
            target_file_name: target_name.to_string(),
            target_file_version: target_version.map(|s| s.to_string()),
            dependency_type: dep_type,
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_add_file_and_dependency() {
        let mut graph = DependencyGraph::new();
        let file = create_test_file(1, "mod1", "1.0.0");

        graph.add_file(file.clone());

        let dep = create_test_dependency(1, "mod2", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep.clone());

        let dependencies = graph.get_dependencies(1);
        assert_eq!(dependencies.len(), 1);
        assert_eq!(dependencies[0].target_file_name, "mod2");
    }

    #[test]
    fn test_get_dependents() {
        let mut graph = DependencyGraph::new();
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");
        let file_c = create_test_file(3, "modC", "1.0.0");

        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());
        graph.add_file(file_c.clone());

        // B -> A
        let dep_b_to_a = create_test_dependency(2, "modA", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_b_to_a);

        // C -> A
        let dep_c_to_a = create_test_dependency(3, "modA", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_c_to_a);

        let dependents = graph.get_dependents(1);
        assert_eq!(dependents.len(), 2);
        assert!(dependents.contains(&2));
        assert!(dependents.contains(&3));
    }

    #[test]
    fn test_has_cycle_from() {
        let mut graph = DependencyGraph::new();
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");

        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());

        // A -> B
        let dep_a_to_b = create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_a_to_b);

        // B -> A (цикл)
        let dep_b_to_a = create_test_dependency(2, "modA", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_b_to_a);

        assert!(graph.has_cycle_from(1));
        assert!(graph.has_cycle_from(2));
    }

    #[test]
    fn test_no_cycle_linear() {
        let mut graph = DependencyGraph::new();
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");
        let file_c = create_test_file(3, "modC", "1.0.0");

        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());
        graph.add_file(file_c.clone());

        // A -> B -> C (линейная цепочка, нет циклов)
        let dep_a_to_b = create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_a_to_b);

        let dep_b_to_c = create_test_dependency(2, "modC", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_b_to_c);

        assert!(!graph.has_cycle_from(1));
        assert!(!graph.has_cycle_from(2));
        assert!(!graph.has_cycle_from(3));
    }

    #[test]
    fn test_get_installation_order_linear() {
        let mut graph = DependencyGraph::new();
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");
        let file_c = create_test_file(3, "modC", "1.0.0");

        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());
        graph.add_file(file_c.clone());

        // A -> B -> C
        let dep_a_to_b = create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_a_to_b);

        let dep_b_to_c = create_test_dependency(2, "modC", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_b_to_c);

        let order = graph.get_installation_order(&[1, 2, 3]).unwrap();

        // Порядок должен быть: C, B, A (зависимости первыми)
        // A -> B -> C означает: A зависит от B, B зависит от C
        // Поэтому порядок установки: C (нет зависимостей), затем B, затем A
        assert_eq!(order.len(), 3);
        // Проверяем, что C идет первым (нет зависимостей)
        assert_eq!(order[0], 3);
        // Проверяем, что A идет последним (зависит от всех)
        assert_eq!(order[2], 1);
        // B должен быть в середине
        assert_eq!(order[1], 2);
    }

    #[test]
    fn test_get_installation_order_with_cycle() {
        let mut graph = DependencyGraph::new();
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");

        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());

        // A -> B -> A (цикл)
        let dep_a_to_b = create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_a_to_b);

        let dep_b_to_a = create_test_dependency(2, "modA", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_b_to_a);

        let result = graph.get_installation_order(&[1, 2]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Circular"));
    }

    #[test]
    fn test_get_installation_order_independent() {
        let mut graph = DependencyGraph::new();
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");
        let file_c = create_test_file(3, "modC", "1.0.0");

        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());
        graph.add_file(file_c.clone());

        // Нет зависимостей между файлами
        let order = graph.get_installation_order(&[1, 2, 3]).unwrap();

        assert_eq!(order.len(), 3);
        assert!(order.contains(&1));
        assert!(order.contains(&2));
        assert!(order.contains(&3));
    }

    #[test]
    fn test_check_circular_before_adding() {
        let mut graph = DependencyGraph::new();
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");

        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());

        // A -> B
        let dep_a_to_b = create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_a_to_b);

        // Проверяем, создаст ли B -> A цикл
        assert!(graph.check_circular(2, "modA", Some("1.0.0")));
    }

    #[test]
    fn test_check_circular_no_cycle() {
        let mut graph = DependencyGraph::new();
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");
        let file_c = create_test_file(3, "modC", "1.0.0");

        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());
        graph.add_file(file_c.clone());

        // A -> B
        let dep_a_to_b = create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_a_to_b);

        // Проверяем, создаст ли C -> A цикл (не должно)
        assert!(!graph.check_circular(3, "modA", Some("1.0.0")));
    }

    #[test]
    fn test_multiple_versions_same_name() {
        let mut graph = DependencyGraph::new();
        let file_a_v1 = create_test_file(1, "modA", "1.0.0");
        let file_a_v2 = create_test_file(2, "modA", "2.0.0");
        let file_b = create_test_file(3, "modB", "1.0.0");

        graph.add_file(file_a_v1.clone());
        graph.add_file(file_a_v2.clone());
        graph.add_file(file_b.clone());

        // A v1 -> B
        let dep_a1_to_b =
            create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_a1_to_b);

        // B -> A v2 (не должно создавать цикл с A v1)
        let dep_b_to_a2 =
            create_test_dependency(3, "modA", Some("2.0.0"), DependencyType::Required);
        graph.add_dependency(dep_b_to_a2);

        // Проверяем, что нет цикла
        assert!(!graph.has_cycle_from(1));
        assert!(!graph.has_cycle_from(2));
        assert!(!graph.has_cycle_from(3));
    }
}
