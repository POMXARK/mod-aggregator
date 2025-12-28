// Unit tests for collection logic evaluator
// Tests the evaluation of collection logic rules to determine which files should be enabled/disabled

use mod_aggregator::database::Database;
use mod_aggregator::services::collection_service::CollectionLogicEvaluator;
use mod_aggregator::models::collection_logic::{ConditionType, Action};
use mod_aggregator::models::file::File;
use chrono::Utc;

/// Helper function to create a test database
/// Creates a new database instance for testing
async fn create_test_db() -> Database {
    Database::new().await.unwrap()
}

/// Helper function to create a test file
async fn create_test_file(db: &Database, name: &str, version: &str) -> i64 {
    let file = db.create_file(name, version, None, None).await.unwrap();
    file.id
}

/// Helper function to create a test collection
async fn create_test_collection(db: &Database, name: &str) -> i64 {
    let collection = db.create_collection(name, None).await.unwrap();
    collection.id
}

/// Helper function to create a test logic rule
async fn create_test_rule(
    db: &Database,
    collection_id: i64,
    name: &str,
    condition_type: ConditionType,
    condition_params: serde_json::Value,
    action: Action,
) -> i64 {
    let rule = db.create_collection_logic_rule(collection_id, name, condition_type, &condition_params, action).await.unwrap();
    rule.id
}

#[tokio::test]
async fn test_boolean_condition_enable() {
    let db = create_test_db().await;
    let collection_id = create_test_collection(&db, "Test Collection").await;
    let file_id = create_test_file(&db, "test-mod", "1.0.0").await;
    
    // Create a boolean rule that enables the file when condition is true
    let rule_id = create_test_rule(
        &db,
        collection_id,
        "Enable if true",
        ConditionType::Boolean,
        serde_json::json!({"value": true}),
        Action::Enable,
    ).await;
    
    // Add file to collection with the rule
    db.add_file_to_collection(collection_id, file_id, Some(rule_id), None).await.unwrap();
    
    // Evaluate collection
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator.evaluate_collection(collection_id).await.unwrap();
    
    // File should be enabled
    assert!(result.enabled_files.contains(&file_id));
    assert!(!result.disabled_files.contains(&file_id));
}

#[tokio::test]
async fn test_boolean_condition_disable() {
    let db = create_test_db().await;
    let collection_id = create_test_collection(&db, "Test Collection").await;
    let file_id = create_test_file(&db, "test-mod", "1.0.0").await;
    
    // Create a boolean rule that disables the file when condition is false
    let rule_id = create_test_rule(
        &db,
        collection_id,
        "Disable if false",
        ConditionType::Boolean,
        serde_json::json!({"value": false}),
        Action::Disable,
    ).await;
    
    // Add file to collection with the rule
    db.add_file_to_collection(collection_id, file_id, Some(rule_id), None).await.unwrap();
    
    // Evaluate collection
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator.evaluate_collection(collection_id).await.unwrap();
    
    // File should be disabled
    assert!(!result.enabled_files.contains(&file_id));
    assert!(result.disabled_files.contains(&file_id));
}

#[tokio::test]
async fn test_collection_check_condition() {
    let db = create_test_db().await;
    let collection1_id = create_test_collection(&db, "Collection 1").await;
    let collection2_id = create_test_collection(&db, "Collection 2").await;
    let file_id = create_test_file(&db, "test-mod", "1.0.0").await;
    
    // Add a file to collection2 to make it "active"
    let other_file_id = create_test_file(&db, "other-mod", "1.0.0").await;
    db.add_file_to_collection(collection2_id, other_file_id, None, None).await.unwrap();
    
    // Create a rule that enables file if collection2 has files
    let rule_id = create_test_rule(
        &db,
        collection1_id,
        "Enable if Collection 2 active",
        ConditionType::CollectionCheck,
        serde_json::json!({"collection_id": collection2_id}),
        Action::Enable,
    ).await;
    
    // Add file to collection1 with the rule
    db.add_file_to_collection(collection1_id, file_id, Some(rule_id), None).await.unwrap();
    
    // Evaluate collection1
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator.evaluate_collection(collection1_id).await.unwrap();
    
    // File should be enabled because collection2 has files
    assert!(result.enabled_files.contains(&file_id));
}

#[tokio::test]
async fn test_collection_check_condition_empty_collection() {
    let db = create_test_db().await;
    let collection1_id = create_test_collection(&db, "Collection 1").await;
    let collection2_id = create_test_collection(&db, "Collection 2").await;
    let file_id = create_test_file(&db, "test-mod", "1.0.0").await;
    
    // Don't add any files to collection2 (it's empty)
    
    // Create a rule that enables file if collection2 has files
    let rule_id = create_test_rule(
        &db,
        collection1_id,
        "Enable if Collection 2 active",
        ConditionType::CollectionCheck,
        serde_json::json!({"collection_id": collection2_id}),
        Action::Enable,
    ).await;
    
    // Add file to collection1 with the rule
    db.add_file_to_collection(collection1_id, file_id, Some(rule_id), None).await.unwrap();
    
    // Evaluate collection1
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator.evaluate_collection(collection1_id).await.unwrap();
    
    // File should be disabled because collection2 is empty
    assert!(!result.enabled_files.contains(&file_id));
    assert!(result.disabled_files.contains(&file_id));
}

#[tokio::test]
async fn test_file_check_condition() {
    let db = create_test_db().await;
    let collection_id = create_test_collection(&db, "Test Collection").await;
    let file_id = create_test_file(&db, "test-mod", "1.0.0").await;
    let dependency_file_id = create_test_file(&db, "base-mod", "2.0.0").await;
    
    // Create a rule that enables file if base-mod@2.0.0 exists
    let rule_id = create_test_rule(
        &db,
        collection_id,
        "Enable if base-mod exists",
        ConditionType::FileCheck,
        serde_json::json!({
            "file_name": "base-mod",
            "file_version": "2.0.0"
        }),
        Action::Enable,
    ).await;
    
    // Add file to collection with the rule
    db.add_file_to_collection(collection_id, file_id, Some(rule_id), None).await.unwrap();
    
    // Evaluate collection
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator.evaluate_collection(collection_id).await.unwrap();
    
    // File should be enabled because base-mod@2.0.0 exists
    assert!(result.enabled_files.contains(&file_id));
}

#[tokio::test]
async fn test_file_check_condition_missing_file() {
    let db = create_test_db().await;
    let collection_id = create_test_collection(&db, "Test Collection").await;
    let file_id = create_test_file(&db, "test-mod", "1.0.0").await;
    
    // Don't create the dependency file
    
    // Create a rule that enables file if missing-mod@1.0.0 exists
    let rule_id = create_test_rule(
        &db,
        collection_id,
        "Enable if missing-mod exists",
        ConditionType::FileCheck,
        serde_json::json!({
            "file_name": "missing-mod",
            "file_version": "1.0.0"
        }),
        Action::Enable,
    ).await;
    
    // Add file to collection with the rule
    db.add_file_to_collection(collection_id, file_id, Some(rule_id), None).await.unwrap();
    
    // Evaluate collection
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator.evaluate_collection(collection_id).await.unwrap();
    
    // File should be disabled because missing-mod doesn't exist
    assert!(!result.enabled_files.contains(&file_id));
    assert!(result.disabled_files.contains(&file_id));
}

#[tokio::test]
async fn test_and_condition() {
    let db = create_test_db().await;
    let collection_id = create_test_collection(&db, "Test Collection").await;
    let file_id = create_test_file(&db, "test-mod", "1.0.0").await;
    
    // Create two boolean rules
    let rule1_id = create_test_rule(
        &db,
        collection_id,
        "Rule 1 - true",
        ConditionType::Boolean,
        serde_json::json!({"value": true}),
        Action::Enable,
    ).await;
    
    let rule2_id = create_test_rule(
        &db,
        collection_id,
        "Rule 2 - true",
        ConditionType::Boolean,
        serde_json::json!({"value": true}),
        Action::Enable,
    ).await;
    
    // Create an AND rule that combines both
    let and_rule_id = create_test_rule(
        &db,
        collection_id,
        "AND rule",
        ConditionType::And,
        serde_json::json!({"rules": [rule1_id, rule2_id]}),
        Action::Enable,
    ).await;
    
    // Add file to collection with the AND rule
    db.add_file_to_collection(collection_id, file_id, Some(and_rule_id), None).await.unwrap();
    
    // Evaluate collection
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator.evaluate_collection(collection_id).await.unwrap();
    
    // File should be enabled because both rules are true (AND)
    assert!(result.enabled_files.contains(&file_id));
}

#[tokio::test]
async fn test_and_condition_one_false() {
    let db = create_test_db().await;
    let collection_id = create_test_collection(&db, "Test Collection").await;
    let file_id = create_test_file(&db, "test-mod", "1.0.0").await;
    
    // Create two boolean rules - one true, one false
    let rule1_id = create_test_rule(
        &db,
        collection_id,
        "Rule 1 - true",
        ConditionType::Boolean,
        serde_json::json!({"value": true}),
        Action::Enable,
    ).await;
    
    let rule2_id = create_test_rule(
        &db,
        collection_id,
        "Rule 2 - false",
        ConditionType::Boolean,
        serde_json::json!({"value": false}),
        Action::Enable,
    ).await;
    
    // Create an AND rule that combines both
    let and_rule_id = create_test_rule(
        &db,
        collection_id,
        "AND rule",
        ConditionType::And,
        serde_json::json!({"rules": [rule1_id, rule2_id]}),
        Action::Enable,
    ).await;
    
    // Add file to collection with the AND rule
    db.add_file_to_collection(collection_id, file_id, Some(and_rule_id), None).await.unwrap();
    
    // Evaluate collection
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator.evaluate_collection(collection_id).await.unwrap();
    
    // File should be disabled because one rule is false (AND requires both true)
    assert!(!result.enabled_files.contains(&file_id));
    assert!(result.disabled_files.contains(&file_id));
}

#[tokio::test]
async fn test_or_condition() {
    let db = create_test_db().await;
    let collection_id = create_test_collection(&db, "Test Collection").await;
    let file_id = create_test_file(&db, "test-mod", "1.0.0").await;
    
    // Create two boolean rules - one true, one false
    let rule1_id = create_test_rule(
        &db,
        collection_id,
        "Rule 1 - true",
        ConditionType::Boolean,
        serde_json::json!({"value": true}),
        Action::Enable,
    ).await;
    
    let rule2_id = create_test_rule(
        &db,
        collection_id,
        "Rule 2 - false",
        ConditionType::Boolean,
        serde_json::json!({"value": false}),
        Action::Enable,
    ).await;
    
    // Create an OR rule that combines both
    let or_rule_id = create_test_rule(
        &db,
        collection_id,
        "OR rule",
        ConditionType::Or,
        serde_json::json!({"rules": [rule1_id, rule2_id]}),
        Action::Enable,
    ).await;
    
    // Add file to collection with the OR rule
    db.add_file_to_collection(collection_id, file_id, Some(or_rule_id), None).await.unwrap();
    
    // Evaluate collection
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator.evaluate_collection(collection_id).await.unwrap();
    
    // File should be enabled because one rule is true (OR requires at least one true)
    assert!(result.enabled_files.contains(&file_id));
}

#[tokio::test]
async fn test_or_condition_both_false() {
    let db = create_test_db().await;
    let collection_id = create_test_collection(&db, "Test Collection").await;
    let file_id = create_test_file(&db, "test-mod", "1.0.0").await;
    
    // Create two boolean rules - both false
    let rule1_id = create_test_rule(
        &db,
        collection_id,
        "Rule 1 - false",
        ConditionType::Boolean,
        serde_json::json!({"value": false}),
        Action::Enable,
    ).await;
    
    let rule2_id = create_test_rule(
        &db,
        collection_id,
        "Rule 2 - false",
        ConditionType::Boolean,
        serde_json::json!({"value": false}),
        Action::Enable,
    ).await;
    
    // Create an OR rule that combines both
    let or_rule_id = create_test_rule(
        &db,
        collection_id,
        "OR rule",
        ConditionType::Or,
        serde_json::json!({"rules": [rule1_id, rule2_id]}),
        Action::Enable,
    ).await;
    
    // Add file to collection with the OR rule
    db.add_file_to_collection(collection_id, file_id, Some(or_rule_id), None).await.unwrap();
    
    // Evaluate collection
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator.evaluate_collection(collection_id).await.unwrap();
    
    // File should be disabled because both rules are false (OR requires at least one true)
    assert!(!result.enabled_files.contains(&file_id));
    assert!(result.disabled_files.contains(&file_id));
}

#[tokio::test]
async fn test_file_without_rule() {
    let db = create_test_db().await;
    let collection_id = create_test_collection(&db, "Test Collection").await;
    let file_id = create_test_file(&db, "test-mod", "1.0.0").await;
    
    // Add file to collection without a rule (should be enabled by default)
    db.add_file_to_collection(collection_id, file_id, None, None).await.unwrap();
    
    // Evaluate collection
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator.evaluate_collection(collection_id).await.unwrap();
    
    // File should be enabled by default (no rule means always enabled)
    assert!(result.enabled_files.contains(&file_id));
    assert!(!result.disabled_files.contains(&file_id));
}

#[tokio::test]
async fn test_multiple_files_with_different_rules() {
    let db = create_test_db().await;
    let collection_id = create_test_collection(&db, "Test Collection").await;
    let file1_id = create_test_file(&db, "mod1", "1.0.0").await;
    let file2_id = create_test_file(&db, "mod2", "1.0.0").await;
    let file3_id = create_test_file(&db, "mod3", "1.0.0").await;
    
    // File1: enabled by boolean true
    let rule1_id = create_test_rule(
        &db,
        collection_id,
        "Enable mod1",
        ConditionType::Boolean,
        serde_json::json!({"value": true}),
        Action::Enable,
    ).await;
    db.add_file_to_collection(collection_id, file1_id, Some(rule1_id), None).await.unwrap();
    
    // File2: disabled by boolean false
    let rule2_id = create_test_rule(
        &db,
        collection_id,
        "Disable mod2",
        ConditionType::Boolean,
        serde_json::json!({"value": false}),
        Action::Disable,
    ).await;
    db.add_file_to_collection(collection_id, file2_id, Some(rule2_id), None).await.unwrap();
    
    // File3: no rule (always enabled)
    db.add_file_to_collection(collection_id, file3_id, None, None).await.unwrap();
    
    // Evaluate collection
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator.evaluate_collection(collection_id).await.unwrap();
    
    // File1 should be enabled, File2 disabled, File3 enabled
    assert!(result.enabled_files.contains(&file1_id));
    assert!(!result.disabled_files.contains(&file1_id));
    
    assert!(!result.enabled_files.contains(&file2_id));
    assert!(result.disabled_files.contains(&file2_id));
    
    assert!(result.enabled_files.contains(&file3_id));
    assert!(!result.disabled_files.contains(&file3_id));
    
    // Check evaluation details
    assert_eq!(result.evaluation_details.len(), 3);
}

#[tokio::test]
async fn test_nested_and_or_conditions() {
    let db = create_test_db().await;
    let collection_id = create_test_collection(&db, "Test Collection").await;
    let file_id = create_test_file(&db, "test-mod", "1.0.0").await;
    
    // Create nested conditions: (true AND false) OR true
    let rule1_id = create_test_rule(
        &db,
        collection_id,
        "Rule 1 - true",
        ConditionType::Boolean,
        serde_json::json!({"value": true}),
        Action::Enable,
    ).await;
    
    let rule2_id = create_test_rule(
        &db,
        collection_id,
        "Rule 2 - false",
        ConditionType::Boolean,
        serde_json::json!({"value": false}),
        Action::Enable,
    ).await;
    
    let rule3_id = create_test_rule(
        &db,
        collection_id,
        "Rule 3 - true",
        ConditionType::Boolean,
        serde_json::json!({"value": true}),
        Action::Enable,
    ).await;
    
    // Create AND rule: rule1 AND rule2
    let and_rule_id = create_test_rule(
        &db,
        collection_id,
        "AND rule",
        ConditionType::And,
        serde_json::json!({"rules": [rule1_id, rule2_id]}),
        Action::Enable,
    ).await;
    
    // Create OR rule: (rule1 AND rule2) OR rule3
    let or_rule_id = create_test_rule(
        &db,
        collection_id,
        "OR rule",
        ConditionType::Or,
        serde_json::json!({"rules": [and_rule_id, rule3_id]}),
        Action::Enable,
    ).await;
    
    // Add file to collection with the nested rule
    db.add_file_to_collection(collection_id, file_id, Some(or_rule_id), None).await.unwrap();
    
    // Evaluate collection
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator.evaluate_collection(collection_id).await.unwrap();
    
    // File should be enabled because: (true AND false) OR true = false OR true = true
    assert!(result.enabled_files.contains(&file_id));
}

