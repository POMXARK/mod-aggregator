// Collection logic evaluator service
// Evaluates collection logic rules to determine which files should be enabled/disabled

use crate::database::Database;
use crate::models::collection_logic::{CollectionLogicRule, ConditionType};
use log::{error, info};

/// Результат оценки логики коллекции
#[derive(Debug, Clone)]
pub struct CollectionEvaluationResult {
    pub collection_id: i64,
    pub enabled_files: Vec<i64>,
    pub disabled_files: Vec<i64>,
    pub evaluation_details: Vec<FileEvaluationDetail>,
}

/// Детали оценки файла
#[derive(Debug, Clone)]
pub struct FileEvaluationDetail {
    pub file_id: i64,
    pub enabled: bool,
    pub applied_rules: Vec<i64>,
}

/// Контекст для оценки правил (кэшированные данные)
struct EvaluationContext {
    collection_files_map: std::collections::HashMap<i64, bool>, // collection_id -> has_files
    file_exists_map: std::collections::HashMap<(String, String), bool>, // (name, version) -> exists
}

/// Оценщик логики коллекций
pub struct CollectionLogicEvaluator {
    db: Database,
}

impl CollectionLogicEvaluator {
    /// Создать новый оценщик
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Оценить логику коллекции
    pub async fn evaluate_collection(
        &self,
        collection_id: i64,
    ) -> Result<CollectionEvaluationResult, String> {
        info!("Evaluating logic for collection id: {}", collection_id);

        // Получаем файлы коллекции
        let collection_files = self
            .db
            .get_collection_files_detailed(collection_id)
            .await
            .map_err(|e| {
                error!("Failed to get collection files: {}", e);
                format!("Database error: {}", e)
            })?;

        // Получаем все правила логики коллекции
        let rules = self
            .db
            .get_collection_logic_rules(collection_id)
            .await
            .map_err(|e| {
                error!("Failed to get logic rules: {}", e);
                format!("Database error: {}", e)
            })?;

        // Создаем карту правил по ID для быстрого доступа
        let rules_map: std::collections::HashMap<i64, &CollectionLogicRule> =
            rules.iter().map(|r| (r.id, r)).collect();

        // Собираем контекст для оценки (кэшируем результаты проверок)
        let mut context = EvaluationContext {
            collection_files_map: std::collections::HashMap::new(),
            file_exists_map: std::collections::HashMap::new(),
        };

        // Предварительно проверяем все коллекции и файлы, на которые ссылаются правила
        for rule in &rules {
            match rule.condition_type {
                ConditionType::CollectionCheck => {
                    if let Some(collection_id_val) = rule
                        .condition_params
                        .get("collection_id")
                        .and_then(|v| v.as_i64())
                    {
                        if !context
                            .collection_files_map
                            .contains_key(&collection_id_val)
                        {
                            let collection_files = self
                                .db
                                .get_collection_files_detailed(collection_id_val)
                                .await
                                .map_err(|e| {
                                    error!("Failed to check collection: {}", e);
                                    format!("Database error: {}", e)
                                })?;
                            context
                                .collection_files_map
                                .insert(collection_id_val, !collection_files.is_empty());
                        }
                    }
                }
                ConditionType::FileCheck => {
                    if let (Some(file_name), Some(file_version)) = (
                        rule.condition_params
                            .get("file_name")
                            .and_then(|v| v.as_str()),
                        rule.condition_params
                            .get("file_version")
                            .and_then(|v| v.as_str()),
                    ) {
                        let key = (file_name.to_string(), file_version.to_string());
                        if !context.file_exists_map.contains_key(&key) {
                            let file = self
                                .db
                                .get_file_by_name_version(file_name, file_version)
                                .await
                                .map_err(|e| {
                                    error!("Failed to check file: {}", e);
                                    format!("Database error: {}", e)
                                })?;
                            context.file_exists_map.insert(key, file.is_some());
                        }
                    }
                }
                _ => {}
            }
        }

        let mut enabled_files = Vec::new();
        let mut disabled_files = Vec::new();
        let mut evaluation_details = Vec::new();

        // Оцениваем каждый файл
        for collection_file in &collection_files {
            let file_id = collection_file.file_id;
            let mut applied_rules = Vec::new();
            let mut enabled = true; // По умолчанию файл включен

            // Если у файла есть правило логики, оцениваем его
            if let Some(rule_id) = collection_file.logic_rule_id {
                if let Some(rule) = rules_map.get(&rule_id) {
                    let rule_result = self.evaluate_rule(rule, &rules_map, &context)?;
                    applied_rules.push(rule_id);

                    match rule.action {
                        crate::models::collection_logic::Action::Enable => {
                            enabled = rule_result;
                        }
                        crate::models::collection_logic::Action::Disable => {
                            enabled = !rule_result;
                        }
                    }
                }
            }

            // Классифицируем файл
            if enabled {
                enabled_files.push(file_id);
            } else {
                disabled_files.push(file_id);
            }

            evaluation_details.push(FileEvaluationDetail {
                file_id,
                enabled,
                applied_rules,
            });
        }

        info!(
            "Collection evaluation completed: {} enabled, {} disabled",
            enabled_files.len(),
            disabled_files.len()
        );

        Ok(CollectionEvaluationResult {
            collection_id,
            enabled_files,
            disabled_files,
            evaluation_details,
        })
    }

    /// Оценить правило логики (синхронная функция, использует предварительно собранный контекст)
    fn evaluate_rule(
        &self,
        rule: &CollectionLogicRule,
        rules_map: &std::collections::HashMap<i64, &CollectionLogicRule>,
        context: &EvaluationContext,
    ) -> Result<bool, String> {
        match rule.condition_type {
            ConditionType::Boolean => {
                let value = rule
                    .condition_params
                    .get("value")
                    .and_then(|v| v.as_bool())
                    .ok_or_else(|| "Invalid boolean condition params".to_string())?;
                Ok(value)
            }
            ConditionType::CollectionCheck => {
                let collection_id = rule
                    .condition_params
                    .get("collection_id")
                    .and_then(|v| v.as_i64())
                    .ok_or_else(|| "Invalid collection_check condition params".to_string())?;

                Ok(context
                    .collection_files_map
                    .get(&collection_id)
                    .copied()
                    .unwrap_or(false))
            }
            ConditionType::FileCheck => {
                let file_name = rule
                    .condition_params
                    .get("file_name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Invalid file_check condition params".to_string())?;

                let file_version = rule
                    .condition_params
                    .get("file_version")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Invalid file_check condition params".to_string())?;

                let key = (file_name.to_string(), file_version.to_string());
                Ok(context.file_exists_map.get(&key).copied().unwrap_or(false))
            }
            ConditionType::And => {
                let rules = rule
                    .condition_params
                    .get("rules")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| "Invalid and condition params".to_string())?;

                let mut result = true;
                for rule_id_val in rules {
                    let rule_id = rule_id_val
                        .as_i64()
                        .ok_or_else(|| "Invalid rule ID".to_string())?;

                    let sub_rule = rules_map
                        .get(&rule_id)
                        .ok_or_else(|| format!("Referenced rule not found: {}", rule_id))?;

                    let sub_result = self.evaluate_rule(sub_rule, rules_map, context)?;
                    result = result && sub_result;

                    if !result {
                        break; // Короткое замыкание
                    }
                }
                Ok(result)
            }
            ConditionType::Or => {
                let rules = rule
                    .condition_params
                    .get("rules")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| "Invalid or condition params".to_string())?;

                let mut result = false;
                for rule_id_val in rules {
                    let rule_id = rule_id_val
                        .as_i64()
                        .ok_or_else(|| "Invalid rule ID".to_string())?;

                    let sub_rule = rules_map
                        .get(&rule_id)
                        .ok_or_else(|| format!("Referenced rule not found: {}", rule_id))?;

                    let sub_result = self.evaluate_rule(sub_rule, rules_map, context)?;
                    result = result || sub_result;

                    if result {
                        break; // Короткое замыкание
                    }
                }
                Ok(result)
            }
        }
    }
}
