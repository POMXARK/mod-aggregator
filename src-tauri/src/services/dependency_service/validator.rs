use super::graph::DependencyGraph;
#[allow(unused_imports)]
use crate::models::{File, FileDependency};

/// Валидатор зависимостей
///
/// Проверяет корректность зависимостей, обнаруживает циклические зависимости
/// и проверяет наличие требуемых файлов.
pub struct DependencyValidator {
    graph: DependencyGraph,
}

impl DependencyValidator {
    /// Создать новый валидатор
    pub fn new(graph: DependencyGraph) -> Self {
        Self { graph }
    }

    /// Проверить, создаст ли добавление зависимости циклическую зависимость
    ///
    /// # Параметры
    /// * `source_file_id` - ID исходного файла
    /// * `target_file_name` - имя целевого файла
    /// * `target_file_version` - версия целевого файла (опционально)
    ///
    /// # Возвращает
    /// true, если добавление создаст цикл
    pub fn check_circular_dependency(
        &self,
        source_file_id: i64,
        target_file_name: &str,
        target_file_version: Option<&str>,
    ) -> bool {
        self.graph
            .check_circular(source_file_id, target_file_name, target_file_version)
    }

    /// Проверить, не является ли зависимость самозависимостью
    ///
    /// # Параметры
    /// * `source_file_id` - ID исходного файла
    /// * `target_file_name` - имя целевого файла
    /// * `target_file_version` - версия целевого файла (опционально)
    ///
    /// # Возвращает
    /// true, если это самозависимость
    pub fn check_self_dependency(
        &self,
        source_file: &File,
        target_file_name: &str,
        target_file_version: Option<&str>,
    ) -> bool {
        if source_file.name != target_file_name {
            return false;
        }

        match target_file_version {
            Some(version) => source_file.version == version,
            None => true, // Если версия не указана, считаем самозависимостью
        }
    }

    /// Валидировать зависимость перед добавлением
    ///
    /// # Параметры
    /// * `source_file` - исходный файл
    /// * `target_file_name` - имя целевого файла
    /// * `target_file_version` - версия целевого файла (опционально)
    ///
    /// # Возвращает
    /// Ok(()) если валидация прошла, Err с описанием ошибки
    pub fn validate_dependency(
        &self,
        source_file: &File,
        target_file_name: &str,
        target_file_version: Option<&str>,
    ) -> Result<(), String> {
        // Проверка самозависимости
        if self.check_self_dependency(source_file, target_file_name, target_file_version) {
            return Err("Self-dependency not allowed".to_string());
        }

        // Проверка циклических зависимостей
        if self.check_circular_dependency(source_file.id, target_file_name, target_file_version) {
            return Err("Circular dependency detected".to_string());
        }

        Ok(())
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
    fn test_self_dependency_detection() {
        let file = create_test_file(1, "mod1", "1.0.0");
        let mut graph = DependencyGraph::new();
        graph.add_file(file.clone());
        let validator = DependencyValidator::new(graph);

        // Самозависимость с той же версией
        assert!(validator.check_self_dependency(&file, "mod1", Some("1.0.0")));

        // Самозависимость без указания версии
        assert!(validator.check_self_dependency(&file, "mod1", None));

        // Не самозависимость - другое имя
        assert!(!validator.check_self_dependency(&file, "mod2", Some("1.0.0")));

        // Не самозависимость - другая версия
        assert!(!validator.check_self_dependency(&file, "mod1", Some("2.0.0")));
    }

    #[test]
    fn test_circular_dependency_detection_simple() {
        // Создаем простой цикл: A -> B -> A
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");

        let mut graph = DependencyGraph::new();
        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());

        // Добавляем зависимость A -> B
        let dep_a_to_b = create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_a_to_b);

        let validator = DependencyValidator::new(graph);

        // Попытка добавить B -> A должна обнаружить цикл
        assert!(validator.check_circular_dependency(2, "modA", Some("1.0.0")));
    }

    #[test]
    fn test_circular_dependency_detection_complex() {
        // Создаем сложный цикл: A -> B -> C -> A
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");
        let file_c = create_test_file(3, "modC", "1.0.0");

        let mut graph = DependencyGraph::new();
        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());
        graph.add_file(file_c.clone());

        // A -> B
        let dep_a_to_b = create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_a_to_b);

        // B -> C
        let dep_b_to_c = create_test_dependency(2, "modC", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_b_to_c);

        let validator = DependencyValidator::new(graph);

        // Попытка добавить C -> A должна обнаружить цикл
        assert!(validator.check_circular_dependency(3, "modA", Some("1.0.0")));
    }

    #[test]
    fn test_no_circular_dependency_linear() {
        // Линейная цепочка без циклов: A -> B -> C
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");
        let file_c = create_test_file(3, "modC", "1.0.0");

        let mut graph = DependencyGraph::new();
        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());
        graph.add_file(file_c.clone());

        // A -> B
        let dep_a_to_b = create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_a_to_b);

        // B -> C
        let dep_b_to_c = create_test_dependency(2, "modC", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_b_to_c);

        let validator = DependencyValidator::new(graph);

        // C -> D (новый файл) не должно создавать цикл
        assert!(!validator.check_circular_dependency(3, "modD", Some("1.0.0")));
    }

    #[test]
    fn test_validate_dependency_self_dependency() {
        let file = create_test_file(1, "mod1", "1.0.0");
        let mut graph = DependencyGraph::new();
        graph.add_file(file.clone());
        let validator = DependencyValidator::new(graph);

        let result = validator.validate_dependency(&file, "mod1", Some("1.0.0"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Self-dependency"));
    }

    #[test]
    fn test_validate_dependency_circular() {
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");

        let mut graph = DependencyGraph::new();
        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());

        // A -> B
        let dep_a_to_b = create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_a_to_b);

        let validator = DependencyValidator::new(graph);

        // Попытка добавить B -> A должна вернуть ошибку
        let result = validator.validate_dependency(&file_b, "modA", Some("1.0.0"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Circular"));
    }

    #[test]
    fn test_validate_dependency_valid() {
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");

        let mut graph = DependencyGraph::new();
        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());

        let validator = DependencyValidator::new(graph);

        // Валидная зависимость A -> B (нет циклов)
        let result = validator.validate_dependency(&file_a, "modB", Some("1.0.0"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_circular_dependency_with_different_versions() {
        // Тест с разными версиями одного файла
        let file_a_v1 = create_test_file(1, "modA", "1.0.0");
        let file_a_v2 = create_test_file(2, "modA", "2.0.0");
        let file_b = create_test_file(3, "modB", "1.0.0");

        let mut graph = DependencyGraph::new();
        graph.add_file(file_a_v1.clone());
        graph.add_file(file_a_v2.clone());
        graph.add_file(file_b.clone());

        // A v1 -> B
        let dep_a1_to_b =
            create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
        graph.add_dependency(dep_a1_to_b);

        let validator = DependencyValidator::new(graph);

        // Проверяем, что A v2 -> B не создаст цикл с A v1 -> B
        // (разные версии одного файла не должны создавать цикл друг с другом)
        assert!(!validator.check_circular_dependency(2, "modB", Some("1.0.0")));
    }

    #[test]
    fn test_circular_dependency_without_version() {
        // Тест с зависимостью без указания версии
        let file_a = create_test_file(1, "modA", "1.0.0");
        let file_b = create_test_file(2, "modB", "1.0.0");

        let mut graph = DependencyGraph::new();
        graph.add_file(file_a.clone());
        graph.add_file(file_b.clone());

        // A -> B (без версии)
        let dep_a_to_b = create_test_dependency(1, "modB", None, DependencyType::Required);
        graph.add_dependency(dep_a_to_b);

        let validator = DependencyValidator::new(graph);

        // B -> A должно обнаружить цикл (даже без версии)
        assert!(validator.check_circular_dependency(2, "modA", Some("1.0.0")));
    }
}
