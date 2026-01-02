//! Логика коллекций
//!
//! Этот модуль содержит функции для управления правилами логики коллекций
//! и оценки выполнения этих правил.

use crate::database::Database;
use crate::models::collection_logic::{CollectionLogicRule, ConditionType, Action};
use crate::services::collection_service::CollectionLogicEvaluator;
use super::models::{CollectionEvaluationResult, FileEvaluationDetail, CreateLogicRuleParams, UpdateLogicRuleParams};
use log::{error, info};

/// Получить правила логики коллекции
#[tauri::command]
pub async fn get_collection_logic_rules(
    collection_id: i64,
) -> Result<Vec<CollectionLogicRule>, String> {
    info!("Getting logic rules for collection id: {}", collection_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", collection_id);
            "Collection not found".to_string()
        })?;

    let rules = db
        .get_collection_logic_rules(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get logic rules: {}", e);
            format!("Database error: {}", e)
        })?;

    info!(
        "Found {} logic rule(s) for collection {}",
        rules.len(),
        collection_id
    );
    Ok(rules)
}

/// Создать правило логики для коллекции
#[tauri::command]
pub async fn create_collection_logic_rule(
    params: CreateLogicRuleParams,
) -> Result<CollectionLogicRule, String> {
    info!(
        "Creating logic rule for collection {}: {}",
        params.collection_id, params.name
    );

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(params.collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", params.collection_id);
            "Collection not found".to_string()
        })?;

    // Парсим тип условия
    let condition_type = super::models::types::parse_condition_type(&params.condition_type)?;

    // Парсим действие
    let action = super::models::types::parse_action(&params.action)?;

    // Валидация параметров условия
    validate_condition_params(&condition_type, &params.condition_params, &db).await?;

    // Создаем правило
    let rule_to_create = CollectionLogicRule {
        id: 0, // будет присвоено базой данных
        collection_id: params.collection_id,
        name: params.name.clone(),
        condition_type,
        condition_params: params.condition_params.clone(),
        action,
        created_at: chrono::Utc::now(),
    };

    let rule = db
        .create_collection_logic_rule(&rule_to_create)
        .await
        .map_err(|e| {
            error!("Failed to create logic rule: {}", e);
            format!("Database error: {}", e)
        })?;

    info!(
        "Logic rule created successfully: {} (id: {})",
        rule.name, rule.id
    );
    Ok(rule)
}

/// Валидация параметров условия
async fn validate_condition_params(
    condition_type: &ConditionType,
    params: &serde_json::Value,
    db: &Database,
) -> Result<(), String> {
    match condition_type {
        ConditionType::Boolean => {
            if !params.is_object() || !params.get("value").and_then(|v| v.as_bool()).is_some() {
                return Err(
                    "Invalid condition params: boolean requires {\"value\": true/false}"
                        .to_string(),
                );
            }
        }
        ConditionType::CollectionCheck => {
            let collection_id = params.get("collection_id")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| "Invalid condition params: collection_check requires {\"collection_id\": number}".to_string())?;

            let collection = db
                .get_collection(collection_id)
                .await
                .map_err(|e| {
                    error!("Failed to check collection: {}", e);
                    format!("Database error: {}", e)
                })?
                .ok_or_else(|| format!("Referenced collection not found: {}", collection_id))?;

            // Проверяем, что коллекция существует (уже проверили выше)
            let _ = collection;
        }
        ConditionType::FileCheck => {
            let file_name = params
                .get("file_name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    "Invalid condition params: file_check requires {\"file_name\": string}"
                        .to_string()
                })?;

            let file_version = params
                .get("file_version")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    "Invalid condition params: file_check requires {\"file_version\": string}"
                        .to_string()
                })?;

            let file = db
                .get_file_by_name_version(file_name, file_version)
                .await
                .map_err(|e| {
                    error!("Failed to check file: {}", e);
                    format!("Database error: {}", e)
                })?
                .ok_or_else(|| {
                    format!("Referenced file not found: {}@{}", file_name, file_version)
                })?;

            // Проверяем, что файл существует (уже проверили выше)
            let _ = file;
        }
        ConditionType::And | ConditionType::Or => {
            let rules = params.get("rules")
                .and_then(|v| v.as_array())
                .ok_or_else(|| "Invalid condition params: and/or requires {\"rules\": [rule_id1, rule_id2, ...]}".to_string())?;

            for rule_id_val in rules {
                let rule_id = rule_id_val
                    .as_i64()
                    .ok_or_else(|| "Invalid rule ID in rules array".to_string())?;

                let rule = db
                    .get_collection_logic_rule(rule_id)
                    .await
                    .map_err(|e| {
                        error!("Failed to check rule: {}", e);
                        format!("Database error: {}", e)
                    })?
                    .ok_or_else(|| format!("Referenced rule not found: {}", rule_id))?;

                // Проверяем, что правило существует (уже проверили выше)
                let _ = rule;
            }
        }
    }
    Ok(())
}

/// Обновить правило логики
#[tauri::command]
pub async fn update_collection_logic_rule(
    params: UpdateLogicRuleParams,
) -> Result<CollectionLogicRule, String> {
    info!("Updating logic rule id: {}", params.id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Получаем текущее правило
    let current_rule = db
        .get_collection_logic_rule(params.id)
        .await
        .map_err(|e| {
            error!("Failed to get logic rule: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Rule not found: {}", params.id);
            "Rule not found".to_string()
        })?;

    // Определяем новые значения
    let condition_type = if let Some(ref ct_str) = params.condition_type {
        Some(super::models::types::parse_condition_type(ct_str)?)
    } else {
        None
    };

    let action = if let Some(ref a_str) = params.action {
        Some(super::models::types::parse_action(a_str)?)
    } else {
        None
    };

    // Валидация параметров условия, если они изменяются
    if let Some(ref condition_params) = params.condition_params {
        let ct = condition_type
            .as_ref()
            .unwrap_or(&current_rule.condition_type);
        validate_condition_params(ct, condition_params, &db).await?;
    } else if condition_type.is_some() {
        // Если тип меняется, но параметры не указаны, используем текущие
        let ct = condition_type.as_ref().unwrap();
        validate_condition_params(ct, &current_rule.condition_params, &db).await?;
    }

    // Создаем обновленное правило
    let updated_rule = CollectionLogicRule {
        id: params.id,
        collection_id: current_rule.collection_id,
        name: params.name.clone().unwrap_or_else(|| current_rule.name.clone()),
        condition_type: condition_type.unwrap_or(current_rule.condition_type),
        condition_params: params.condition_params.clone().unwrap_or_else(|| current_rule.condition_params.clone()),
        action: action.unwrap_or(current_rule.action),
        created_at: current_rule.created_at,
    };

    // Обновляем правило
    db.update_collection_logic_rule(&updated_rule)
    .await
    .map_err(|e| {
        error!("Failed to update logic rule: {}", e);
        format!("Database error: {}", e)
    })?;

    // Получаем обновленное правило
    let updated = db
        .get_collection_logic_rule(params.id)
        .await
        .map_err(|e| {
            error!("Failed to get updated rule: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Rule not found after update: {}", params.id);
            "Rule not found".to_string()
        })?;

    info!(
        "Logic rule updated successfully: {} (id: {})",
        updated.name, updated.id
    );
    Ok(updated)
}

/// Удалить правило логики
#[tauri::command]
pub async fn delete_collection_logic_rule(rule_id: i64) -> Result<(), String> {
    info!("Deleting logic rule id: {}", rule_id);

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование правила
    let _rule = db
        .get_collection_logic_rule(rule_id)
        .await
        .map_err(|e| {
            error!("Failed to get logic rule: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Rule not found: {}", rule_id);
            "Rule not found".to_string()
        })?;

    // Удаляем правило
    db.delete_collection_logic_rule(rule_id)
        .await
        .map_err(|e| {
            error!("Failed to delete logic rule: {}", e);
            format!("Database error: {}", e)
        })?;

    info!("Logic rule deleted successfully: id {}", rule_id);
    Ok(())
}

/// Оценить логику коллекции
#[tauri::command]
pub async fn evaluate_collection_logic(
    collection_id: i64,
) -> Result<CollectionEvaluationResult, String> {
    info!(
        "Evaluating collection logic for collection id: {}",
        collection_id
    );

    let db = Database::new().await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        format!("Database error: {}", e)
    })?;

    // Проверяем существование коллекции
    let _collection = db
        .get_collection(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to get collection: {}", e);
            format!("Database error: {}", e)
        })?
        .ok_or_else(|| {
            error!("Collection not found: {}", collection_id);
            "Collection not found".to_string()
        })?;

    // Оцениваем логику
    let evaluator = CollectionLogicEvaluator::new(db);
    let result = evaluator
        .evaluate_collection(collection_id)
        .await
        .map_err(|e| {
            error!("Failed to evaluate collection logic: {}", e);
            e
        })?;

    // Преобразуем в формат для API
    let api_result = CollectionEvaluationResult {
        collection_id: result.collection_id,
        enabled_files: result.enabled_files,
        disabled_files: result.disabled_files,
        evaluation_details: result
            .evaluation_details
            .iter()
            .map(|d| FileEvaluationDetail {
                file_id: d.file_id,
                enabled: d.enabled,
                applied_rules: d.applied_rules.clone(),
            })
            .collect(),
    };

    info!(
        "Collection logic evaluated: {} enabled, {} disabled files",
        api_result.enabled_files.len(),
        api_result.disabled_files.len()
    );

    Ok(api_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    #[tokio::test]
    async fn test_get_collection_logic_rules_invalid_collection() {
        let result = get_collection_logic_rules(99999).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Collection not found");
    }

    #[tokio::test]
    async fn test_create_collection_logic_rule_invalid_collection() {
        let params = CreateLogicRuleParams {
            collection_id: 99999,
            name: "Test Rule".to_string(),
            condition_type: "boolean".to_string(),
            condition_params: serde_json::json!({"value": true}),
            action: "enable".to_string(),
        };

        let result = create_collection_logic_rule(params).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Collection not found");
    }

    #[tokio::test]
    async fn test_create_collection_logic_rule_invalid_condition_type() {
        let params = CreateLogicRuleParams {
            collection_id: 1,
            name: "Test Rule".to_string(),
            condition_type: "invalid_type".to_string(),
            condition_params: serde_json::json!({"value": true}),
            action: "enable".to_string(),
        };

        let result = create_collection_logic_rule(params).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown condition type"));
    }

    #[tokio::test]
    async fn test_create_collection_logic_rule_invalid_action() {
        let params = CreateLogicRuleParams {
            collection_id: 1,
            name: "Test Rule".to_string(),
            condition_type: "boolean".to_string(),
            condition_params: serde_json::json!({"value": true}),
            action: "invalid_action".to_string(),
        };

        let result = create_collection_logic_rule(params).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown action"));
    }

    #[tokio::test]
    async fn test_update_collection_logic_rule_invalid_id() {
        let params = UpdateLogicRuleParams {
            id: 99999,
            name: Some("Updated Rule".to_string()),
            condition_type: None,
            condition_params: None,
            action: None,
        };

        let result = update_collection_logic_rule(params).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Rule not found");
    }

    #[tokio::test]
    async fn test_delete_collection_logic_rule_invalid_id() {
        let result = delete_collection_logic_rule(99999).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Rule not found");
    }

    #[tokio::test]
    async fn test_evaluate_collection_logic_invalid_collection() {
        let result = evaluate_collection_logic(99999).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Collection not found");
    }

    #[tokio::test]
    async fn test_validate_condition_params_boolean_valid() {
        let db = Database::new().await.unwrap();
        let condition_type = ConditionType::Boolean;
        let params = serde_json::json!({"value": true});

        let result = validate_condition_params(&condition_type, &params, &db).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_condition_params_boolean_invalid() {
        let db = Database::new().await.unwrap();
        let condition_type = ConditionType::Boolean;
        let params = serde_json::json!({"invalid": "value"});

        let result = validate_condition_params(&condition_type, &params, &db).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("boolean requires"));
    }
}