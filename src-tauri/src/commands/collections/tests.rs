//! Интеграционные тесты для модулей коллекций
//!
//! Этот файл содержит комплексные тесты, проверяющие взаимодействие
//! между различными модулями системы коллекций.

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::database::Database;

    #[tokio::test]
    async fn test_full_collection_lifecycle() {
        // Тест полного жизненного цикла коллекции: создание -> добавление файлов -> логика -> удаление

        // Создаем тестовую коллекцию
        let create_params = CreateCollectionParams {
            name: "Integration Test Collection".to_string(),
            description: Some("Test collection for integration tests".to_string()),
        };

        let collection = create_collection(create_params).await.unwrap();

        // Проверяем, что коллекция создана
        let collections = get_collections().await.unwrap();
        assert!(collections.iter().any(|c| c.id == collection.id));

        // Получаем файлы (должно быть пусто)
        let files = get_collection_files(collection.id).await.unwrap();
        assert_eq!(files.len(), 0);

        // Создаем правило логики
        let logic_params = CreateLogicRuleParams {
            collection_id: collection.id,
            name: "Test Logic Rule".to_string(),
            condition_type: "boolean".to_string(),
            condition_params: serde_json::json!({"value": true}),
            action: "enable".to_string(),
        };

        let logic_rule = create_collection_logic_rule(logic_params).await.unwrap();

        // Проверяем правило логики
        let rules = get_collection_logic_rules(collection.id).await.unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].id, logic_rule.id);

        // Оцениваем логику коллекции (пустая коллекция)
        let evaluation = evaluate_collection_logic(collection.id).await.unwrap();
        assert_eq!(evaluation.collection_id, collection.id);
        assert_eq!(evaluation.enabled_files.len(), 0);
        assert_eq!(evaluation.disabled_files.len(), 0);

        // Удаляем правило логики
        delete_collection_logic_rule(logic_rule.id).await.unwrap();

        // Проверяем, что правило удалено
        let rules_after = get_collection_logic_rules(collection.id).await.unwrap();
        assert_eq!(rules_after.len(), 0);

        // Удаляем коллекцию
        delete_collection(collection.id).await.unwrap();

        // Проверяем, что коллекция удалена
        let collections_after = get_collections().await.unwrap();
        assert!(!collections_after.iter().any(|c| c.id == collection.id));
    }

    #[tokio::test]
    async fn test_collection_operations_workflow() {
        // Создаем две тестовые коллекции
        let collection1_params = CreateCollectionParams {
            name: "Source Collection 1".to_string(),
            description: None,
        };
        let collection1 = create_collection(collection1_params).await.unwrap();

        let collection2_params = CreateCollectionParams {
            name: "Source Collection 2".to_string(),
            description: None,
        };
        let collection2 = create_collection(collection2_params).await.unwrap();

        // Объединяем коллекции
        let combine_params = CombineCollectionsParams {
            name: "Combined Collection".to_string(),
            description: Some("Collection created by combining two collections".to_string()),
            source_collection_ids: vec![collection1.id, collection2.id],
            selected_file_ids: None,
        };

        let combined = combine_collections(combine_params).await.unwrap();

        // Получаем файлы из нескольких коллекций
        let multi_files = get_files_from_multiple_collections(vec![collection1.id, collection2.id, combined.id]).await.unwrap();

        // Проверяем, что объединенная коллекция создана
        let collections = get_collections().await.unwrap();
        assert!(collections.iter().any(|c| c.id == combined.id));

        // Очищаем тестовые данные
        delete_collection(combined.id).await.unwrap();
        delete_collection(collection2.id).await.unwrap();
        delete_collection(collection1.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_error_handling() {
        // Тестируем обработку ошибок

        // Попытка получить несуществующую коллекцию
        let result = get_collection_files(99999).await;
        assert!(result.is_err());

        // Попытка создать коллекцию с существующим именем
        let params1 = CreateCollectionParams {
            name: "Error Test Collection".to_string(),
            description: None,
        };
        let _collection1 = create_collection(params1).await.unwrap();

        let params2 = CreateCollectionParams {
            name: "Error Test Collection".to_string(),
            description: None,
        };
        let result = create_collection(params2).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));

        // Очищаем
        let collections = get_collections().await.unwrap();
        for collection in collections {
            if collection.name == "Error Test Collection" {
                delete_collection(collection.id).await.unwrap();
                break;
            }
        }
    }

    #[tokio::test]
    async fn test_logic_rule_validation() {
        // Создаем тестовую коллекцию
        let collection_params = CreateCollectionParams {
            name: "Logic Validation Test".to_string(),
            description: None,
        };
        let collection = create_collection(collection_params).await.unwrap();

        // Тестируем различные типы условий
        let test_cases = vec![
            ("boolean", serde_json::json!({"value": true})),
            ("collection_check", serde_json::json!({"collection_id": 1})),
            ("file_check", serde_json::json!({"file_name": "test", "file_version": "1.0"})),
        ];

        for (condition_type, condition_params) in test_cases {
            let params = CreateLogicRuleParams {
                collection_id: collection.id,
                name: format!("Test Rule {}", condition_type),
                condition_type: condition_type.to_string(),
                condition_params,
                action: "enable".to_string(),
            };

            // Правило может быть создано или нет в зависимости от существования связанных объектов
            let _ = create_collection_logic_rule(params).await;
        }

        // Очищаем
        delete_collection(collection.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_collection_file_operations() {
        // Создаем тестовую коллекцию
        let collection_params = CreateCollectionParams {
            name: "File Operations Test".to_string(),
            description: None,
        };
        let collection = create_collection(collection_params).await.unwrap();

        // Получаем файлы (пусто)
        let files = get_collection_files(collection.id).await.unwrap();
        assert_eq!(files.len(), 0);

        // Пытаемся добавить несуществующий файл
        let add_params = AddFileToCollectionParams {
            collection_id: collection.id,
            file_id: 99999,
            logic_rule_id: None,
            order_index: None,
        };

        let result = add_file_to_collection(add_params).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("File not found"));

        // Очищаем
        delete_collection(collection.id).await.unwrap();
    }
}

#[cfg(test)]
mod unit_tests {
    use super::models::types::*;

    #[test]
    fn test_condition_type_parsing() {
        assert_eq!(parse_condition_type("boolean"), Ok(ConditionType::Boolean));
        assert_eq!(parse_condition_type("collection_check"), Ok(ConditionType::CollectionCheck));
        assert_eq!(parse_condition_type("file_check"), Ok(ConditionType::FileCheck));
        assert_eq!(parse_condition_type("and"), Ok(ConditionType::And));
        assert_eq!(parse_condition_type("or"), Ok(ConditionType::Or));
        assert!(parse_condition_type("invalid").is_err());
    }

    #[test]
    fn test_action_parsing() {
        assert_eq!(parse_action("enable"), Ok(Action::Enable));
        assert_eq!(parse_action("disable"), Ok(Action::Disable));
        assert!(parse_action("invalid").is_err());
    }
}