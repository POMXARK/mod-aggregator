// Быстрый тест команд коллекций
// Скопируйте этот код в консоль DevTools (F12) после запуска приложения

(async function() {
    console.log("🚀 Запуск быстрого теста коллекций...\n");
    
    try {
        // Получаем invoke функцию
        const { invoke } = window.__TAURI__.core;
        
        // Тест 1: Создание коллекции
        console.log("📝 Тест 1: Создание коллекции");
        const collection = await invoke('create_collection', {
            params: {
                name: "TestCollection_" + Date.now(),
                description: "Тестовая коллекция"
            }
        });
        console.log("✅ Коллекция создана:", collection);
        const collectionId = collection.id;
        
        // Тест 2: Получение всех коллекций
        console.log("\n📋 Тест 2: Получение всех коллекций");
        const collections = await invoke('get_collections');
        console.log(`✅ Найдено коллекций: ${collections.length}`);
        
        // Тест 3: Создание файла (если нужно)
        console.log("\n📁 Тест 3: Создание тестового файла");
        let file;
        try {
            file = await invoke('create_file', {
                params: {
                    name: "TestMod_" + Date.now(),
                    version: "1.0.0",
                    path: "/mods/test",
                    metadata: { author: "Test" },
                    dependencies: []
                }
            });
            console.log("✅ Файл создан:", file);
        } catch (e) {
            // Используем существующий файл
            const files = await invoke('get_file_versions', { name: '' });
            if (files.length > 0) {
                file = files[0];
                console.log("✅ Используем существующий файл:", file);
            } else {
                throw new Error("Нет файлов для теста");
            }
        }
        
        // Тест 4: Добавление файла в коллекцию
        console.log("\n➕ Тест 4: Добавление файла в коллекцию");
        const collectionFile = await invoke('add_file_to_collection', {
            params: {
                collection_id: collectionId,
                file_id: file.id,
                logic_rule_id: null,
                order_index: 0
            }
        });
        console.log("✅ Файл добавлен в коллекцию:", collectionFile);
        
        // Тест 5: Получение файлов коллекции
        console.log("\n📦 Тест 5: Получение файлов коллекции");
        const collectionFiles = await invoke('get_collection_files', {
            collection_id: collectionId
        });
        console.log(`✅ Файлов в коллекции: ${collectionFiles.length}`);
        
        // Тест 6: Создание правила логики
        console.log("\n⚙️ Тест 6: Создание правила логики");
        const logicRule = await invoke('create_collection_logic_rule', {
            params: {
                collection_id: collectionId,
                name: "Test Rule",
                condition_type: "boolean",
                condition_params: { value: true },
                action: "enable"
            }
        });
        console.log("✅ Правило логики создано:", logicRule);
        
        // Тест 7: Получение правил логики
        console.log("\n📜 Тест 7: Получение правил логики");
        const rules = await invoke('get_collection_logic_rules', {
            collection_id: collectionId
        });
        console.log(`✅ Правил логики: ${rules.length}`);
        
        // Тест 8: Оценка логики коллекции
        console.log("\n🔍 Тест 8: Оценка логики коллекции");
        const evaluation = await invoke('evaluate_collection_logic', {
            collection_id: collectionId
        });
        console.log("✅ Оценка завершена:");
        console.log(`   Включено файлов: ${evaluation.enabled_files.length}`);
        console.log(`   Выключено файлов: ${evaluation.disabled_files.length}`);
        
        console.log("\n" + "=".repeat(50));
        console.log("✅ ВСЕ ТЕСТЫ ПРОЙДЕНЫ УСПЕШНО!");
        console.log("=".repeat(50));
        
    } catch (error) {
        console.error("\n" + "=".repeat(50));
        console.error("❌ ОШИБКА ПРИ ВЫПОЛНЕНИИ ТЕСТОВ:");
        console.error("=".repeat(50));
        console.error("Сообщение:", error.message);
        console.error("Стек:", error.stack);
        console.error("\nПроверьте:");
        console.error("1. Приложение запущено (npm run tauri dev)");
        console.error("2. DevTools открыт (F12)");
        console.error("3. Команды зарегистрированы в main.rs");
    }
})();
























