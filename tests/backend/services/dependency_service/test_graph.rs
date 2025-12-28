use mod_aggregator::models::{File, FileDependency, DependencyType};
use mod_aggregator::services::dependency_service::DependencyGraph;
use chrono::Utc;

/// Создать тестовый файл
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

/// Создать тестовую зависимость
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
    assert_eq!(order.len(), 3);
    assert_eq!(order[0], 3); // C первым (нет зависимостей)
    assert_eq!(order[1], 2); // B вторым
    assert_eq!(order[2], 1); // A последним
}

#[test]
fn test_get_installation_order_complex() {
    let mut graph = DependencyGraph::new();
    let file_a = create_test_file(1, "modA", "1.0.0");
    let file_b = create_test_file(2, "modB", "1.0.0");
    let file_c = create_test_file(3, "modC", "1.0.0");
    let file_d = create_test_file(4, "modD", "1.0.0");

    graph.add_file(file_a.clone());
    graph.add_file(file_b.clone());
    graph.add_file(file_c.clone());
    graph.add_file(file_d.clone());

    // A -> B, A -> C
    // B -> D
    // C -> D
    let dep_a_to_b = create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
    graph.add_dependency(dep_a_to_b);

    let dep_a_to_c = create_test_dependency(1, "modC", Some("1.0.0"), DependencyType::Required);
    graph.add_dependency(dep_a_to_c);

    let dep_b_to_d = create_test_dependency(2, "modD", Some("1.0.0"), DependencyType::Required);
    graph.add_dependency(dep_b_to_d);

    let dep_c_to_d = create_test_dependency(3, "modD", Some("1.0.0"), DependencyType::Required);
    graph.add_dependency(dep_c_to_d);

    let order = graph.get_installation_order(&[1, 2, 3, 4]).unwrap();
    
    // D должен быть первым (нет зависимостей)
    // B и C должны быть перед A
    assert_eq!(order.len(), 4);
    assert_eq!(order[0], 4); // D первым
    assert!(order.contains(&2)); // B
    assert!(order.contains(&3)); // C
    assert_eq!(order[3], 1); // A последним
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
    let dep_a1_to_b = create_test_dependency(1, "modB", Some("1.0.0"), DependencyType::Required);
    graph.add_dependency(dep_a1_to_b);

    // B -> A v2 (не должно создавать цикл с A v1)
    let dep_b_to_a2 = create_test_dependency(3, "modA", Some("2.0.0"), DependencyType::Required);
    graph.add_dependency(dep_b_to_a2);

    // Проверяем, что нет цикла
    assert!(!graph.has_cycle_from(1));
    assert!(!graph.has_cycle_from(2));
    assert!(!graph.has_cycle_from(3));
}
























