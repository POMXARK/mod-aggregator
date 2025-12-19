// Простой тест подключения к MongoDB через Node.js
const { MongoClient } = require('mongodb');

async function testConnection() {
    const uri = "mongodb://localhost:9999";
    const client = new MongoClient(uri);

    try {
        console.log('🔍 Тестируем подключение к MongoDB...');

        // Попытка подключения с таймаутом
        await client.connect();

        console.log('✅ Подключение к MongoDB успешно!');

        // Получаем список баз данных
        const databases = await client.db().admin().listDatabases();
        console.log('📊 Доступные базы данных:');
        databases.databases.forEach(db => {
            console.log(`  - ${db.name}`);
        });

    } catch (error) {
        console.error('❌ Ошибка подключения к MongoDB:');
        console.error(error.message);

        if (error.message.includes('ECONNREFUSED')) {
            console.log('💡 Возможные причины:');
            console.log('  - SSH туннель не запущен');
            console.log('  - MongoDB на сервере не работает');
            console.log('  - Неправильный порт в туннеле');
        }
    } finally {
        await client.close();
    }
}

testConnection();
