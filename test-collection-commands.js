// test-collection-commands.js
// Тестовый скрипт для проверки команд коллекций
// Использование: скопировать в консоль браузера или Tauri DevTools

// Импорт функции invoke (если используется ES модули)
// import { invoke } from './src/lib/tauri-wrapper.js';

// Для использования в консоли браузера:
// const { invoke } = window.__TAURI__;

async function testCreateCollection() {
    console.log("--- Testing create_collection ---");
    try {
        const collection1 = await invoke('create_collection', {
            params: {
                name: "TestCollectionA",
                description: "Test collection A"
            }
        });
        console.log("Created collection1:", collection1);

        const collection2 = await invoke('create_collection', {
            params: {
                name: "TestCollectionB",
                description: "Test collection B"
            }
        });
        console.log("Created collection2:", collection2);

        return { collection1, collection2 };
    } catch (e) {
        console.error("Error during create_collection test:", e);
        throw e;
    }
}

async function testGetCollections() {
    console.log("--- Testing get_collections ---");
    try {
        const collections = await invoke('get_collections');
        console.log("All collections:", collections);
        return collections;
    } catch (e) {
        console.error("Error during get_collections test:", e);
        throw e;
    }
}

async function testAddFileToCollection(collectionId, fileId) {
    console.log(`--- Testing add_file_to_collection (collection: ${collectionId}, file: ${fileId}) ---`);
    try {
        const result = await invoke('add_file_to_collection', {
            params: {
                collection_id: collectionId,
                file_id: fileId,
                logic_rule_id: null,
                order_index: 0
            }
        });
        console.log("File added to collection:", result);
        return result;
    } catch (e) {
        console.error("Error during add_file_to_collection test:", e);
        throw e;
    }
}

async function testGetCollectionFiles(collectionId) {
    console.log(`--- Testing get_collection_files (collection: ${collectionId}) ---`);
    try {
        const files = await invoke('get_collection_files', { collection_id: collectionId });
        console.log("Collection files:", files);
        return files;
    } catch (e) {
        console.error("Error during get_collection_files test:", e);
        throw e;
    }
}

async function testCreateLogicRule(collectionId, ruleName, conditionType, conditionParams, action) {
    console.log(`--- Testing create_collection_logic_rule (collection: ${collectionId}, rule: ${ruleName}) ---`);
    try {
        const rule = await invoke('create_collection_logic_rule', {
            params: {
                collection_id: collectionId,
                name: ruleName,
                condition_type: conditionType,
                condition_params: conditionParams,
                action: action
            }
        });
        console.log("Created logic rule:", rule);
        return rule;
    } catch (e) {
        console.error("Error during create_collection_logic_rule test:", e);
        throw e;
    }
}

async function testGetLogicRules(collectionId) {
    console.log(`--- Testing get_collection_logic_rules (collection: ${collectionId}) ---`);
    try {
        const rules = await invoke('get_collection_logic_rules', { collection_id: collectionId });
        console.log("Collection logic rules:", rules);
        return rules;
    } catch (e) {
        console.error("Error during get_collection_logic_rules test:", e);
        throw e;
    }
}

async function testEvaluateCollectionLogic(collectionId) {
    console.log(`--- Testing evaluate_collection_logic (collection: ${collectionId}) ---`);
    try {
        const result = await invoke('evaluate_collection_logic', { collection_id: collectionId });
        console.log("Evaluation result:", result);
        console.log(`Enabled files: ${result.enabled_files.length}, Disabled files: ${result.disabled_files.length}`);
        return result;
    } catch (e) {
        console.error("Error during evaluate_collection_logic test:", e);
        throw e;
    }
}

async function testCombineCollections(collectionIds, newName) {
    console.log(`--- Testing combine_collections (collections: ${collectionIds.join(', ')}) ---`);
    try {
        const result = await invoke('combine_collections', {
            params: {
                name: newName,
                description: `Combined from collections: ${collectionIds.join(', ')}`,
                source_collection_ids: collectionIds,
                selected_file_ids: null // Все файлы
            }
        });
        console.log("Combined collection:", result);
        return result;
    } catch (e) {
        console.error("Error during combine_collections test:", e);
        throw e;
    }
}

async function testGetFilesFromMultipleCollections(collectionIds) {
    console.log(`--- Testing get_files_from_multiple_collections (collections: ${collectionIds.join(', ')}) ---`);
    try {
        const result = await invoke('get_files_from_multiple_collections', { collection_ids: collectionIds });
        console.log("Files from multiple collections:", result);
        console.log(`Total unique files: ${result.length}`);
        return result;
    } catch (e) {
        console.error("Error during get_files_from_multiple_collections test:", e);
        throw e;
    }
}

async function testUpdateCollection(collectionId, newName, newDescription) {
    console.log(`--- Testing update_collection (collection: ${collectionId}) ---`);
    try {
        const result = await invoke('update_collection', {
            params: {
                id: collectionId,
                name: newName,
                description: newDescription
            }
        });
        console.log("Updated collection:", result);
        return result;
    } catch (e) {
        console.error("Error during update_collection test:", e);
        throw e;
    }
}

async function testReorderCollectionFiles(collectionId, fileOrders) {
    console.log(`--- Testing reorder_collection_files (collection: ${collectionId}) ---`);
    try {
        const result = await invoke('reorder_collection_files', {
            collection_id: collectionId,
            file_orders: fileOrders
        });
        console.log("Files reordered successfully");
        return result;
    } catch (e) {
        console.error("Error during reorder_collection_files test:", e);
        throw e;
    }
}

async function testRemoveFileFromCollection(collectionId, fileId) {
    console.log(`--- Testing remove_file_from_collection (collection: ${collectionId}, file: ${fileId}) ---`);
    try {
        const result = await invoke('remove_file_from_collection', {
            collection_id: collectionId,
            file_id: fileId
        });
        console.log("File removed from collection successfully");
        return result;
    } catch (e) {
        console.error("Error during remove_file_from_collection test:", e);
        throw e;
    }
}

async function testDeleteCollection(collectionId) {
    console.log(`--- Testing delete_collection (collection: ${collectionId}) ---`);
    try {
        const result = await invoke('delete_collection', { collection_id: collectionId });
        console.log("Collection deleted successfully");
        return result;
    } catch (e) {
        console.error("Error during delete_collection test:", e);
        throw e;
    }
}

// Полный тестовый сценарий
async function runFullTest() {
    console.log("=========================================");
    console.log("Starting Full Collection Commands Test");
    console.log("=========================================\n");

    try {
        // 1. Создаем коллекции
        console.log("Step 1: Creating collections...");
        const { collection1, collection2 } = await testCreateCollection();
        const collection1Id = collection1.id;
        const collection2Id = collection2.id;

        // 2. Получаем все коллекции
        console.log("\nStep 2: Getting all collections...");
        await testGetCollections();

        // 3. Создаем тестовые файлы (если их еще нет)
        console.log("\nStep 3: Creating test files...");
        let file1, file2, file3;
        try {
            file1 = await invoke('create_file', {
                params: {
                    name: "TestModA",
                    version: "1.0.0",
                    path: "/mods/TestModA",
                    metadata: { author: "Dev" },
                    dependencies: []
                }
            });
            console.log("Created file1:", file1);
        } catch (e) {
            // Файл может уже существовать, попробуем получить его
            const files = await invoke('get_file_versions', { name: 'TestModA' });
            file1 = files.find(f => f.version === '1.0.0');
            console.log("Using existing file1:", file1);
        }

        try {
            file2 = await invoke('create_file', {
                params: {
                    name: "TestModB",
                    version: "1.0.0",
                    path: "/mods/TestModB",
                    metadata: { author: "Dev" },
                    dependencies: []
                }
            });
            console.log("Created file2:", file2);
        } catch (e) {
            const files = await invoke('get_file_versions', { name: 'TestModB' });
            file2 = files.find(f => f.version === '1.0.0');
            console.log("Using existing file2:", file2);
        }

        try {
            file3 = await invoke('create_file', {
                params: {
                    name: "TestModC",
                    version: "1.0.0",
                    path: "/mods/TestModC",
                    metadata: { author: "Dev" },
                    dependencies: []
                }
            });
            console.log("Created file3:", file3);
        } catch (e) {
            const files = await invoke('get_file_versions', { name: 'TestModC' });
            file3 = files.find(f => f.version === '1.0.0');
            console.log("Using existing file3:", file3);
        }

        // 4. Добавляем файлы в коллекции
        console.log("\nStep 4: Adding files to collections...");
        await testAddFileToCollection(collection1Id, file1.id);
        await testAddFileToCollection(collection1Id, file2.id);
        await testAddFileToCollection(collection2Id, file2.id);
        await testAddFileToCollection(collection2Id, file3.id);

        // 5. Получаем файлы коллекций
        console.log("\nStep 5: Getting collection files...");
        await testGetCollectionFiles(collection1Id);
        await testGetCollectionFiles(collection2Id);

        // 6. Создаем правила логики
        console.log("\nStep 6: Creating logic rules...");
        
        // Boolean правило
        const booleanRule = await testCreateLogicRule(
            collection1Id,
            "Enable if true",
            "boolean",
            { value: true },
            "enable"
        );

        // File check правило
        const fileCheckRule = await testCreateLogicRule(
            collection1Id,
            "Enable if TestModB exists",
            "file_check",
            { file_name: "TestModB", file_version: "1.0.0" },
            "enable"
        );

        // Collection check правило
        const collectionCheckRule = await testCreateLogicRule(
            collection2Id,
            "Enable if CollectionA has files",
            "collection_check",
            { collection_id: collection1Id },
            "enable"
        );

        // 7. Получаем правила логики
        console.log("\nStep 7: Getting logic rules...");
        await testGetLogicRules(collection1Id);
        await testGetLogicRules(collection2Id);

        // 8. Оцениваем логику коллекций
        console.log("\nStep 8: Evaluating collection logic...");
        await testEvaluateCollectionLogic(collection1Id);
        await testEvaluateCollectionLogic(collection2Id);

        // 9. Объединяем коллекции
        console.log("\nStep 9: Combining collections...");
        const combinedCollection = await testCombineCollections(
            [collection1Id, collection2Id],
            "CombinedCollection"
        );

        // 10. Получаем файлы из нескольких коллекций
        console.log("\nStep 10: Getting files from multiple collections...");
        await testGetFilesFromMultipleCollections([collection1Id, collection2Id]);

        // 11. Изменяем порядок файлов
        console.log("\nStep 11: Reordering collection files...");
        const files1 = await testGetCollectionFiles(collection1Id);
        if (files1.length >= 2) {
            await testReorderCollectionFiles(collection1Id, [
                { file_id: files1[1].file_id, order_index: 0 },
                { file_id: files1[0].file_id, order_index: 1 }
            ]);
        }

        // 12. Обновляем коллекцию
        console.log("\nStep 12: Updating collection...");
        await testUpdateCollection(collection1Id, "UpdatedCollectionA", "Updated description");

        // 13. Удаляем файл из коллекции
        console.log("\nStep 13: Removing file from collection...");
        if (files1.length > 0) {
            await testRemoveFileFromCollection(collection1Id, files1[0].file_id);
        }

        // 14. Удаляем коллекции (опционально, закомментировано для сохранения данных)
        // console.log("\nStep 14: Deleting collections...");
        // await testDeleteCollection(combinedCollection.id);
        // await testDeleteCollection(collection2Id);
        // await testDeleteCollection(collection1Id);

        console.log("\n=========================================");
        console.log("✓ All tests completed successfully!");
        console.log("=========================================");

    } catch (e) {
        console.error("\n=========================================");
        console.error("✗ Test failed with error:", e);
        console.error("=========================================");
    }
}

// Экспорт функций для использования
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        testCreateCollection,
        testGetCollections,
        testAddFileToCollection,
        testGetCollectionFiles,
        testCreateLogicRule,
        testGetLogicRules,
        testEvaluateCollectionLogic,
        testCombineCollections,
        testGetFilesFromMultipleCollections,
        testUpdateCollection,
        testReorderCollectionFiles,
        testRemoveFileFromCollection,
        testDeleteCollection,
        runFullTest
    };
}

// Для использования в консоли браузера:
// runFullTest();



































