-- Migration 003: Add dependencies support
-- Adds version field to files table and creates file_dependencies table

-- Добавление поля version в таблицу files (если еще не добавлено)
-- Используем проверку через PRAGMA table_info для безопасного добавления
-- Если колонка уже существует, команда не выполнится (SQLite не поддерживает IF NOT EXISTS для ALTER TABLE)
-- В этом случае нужно обработать ошибку в коде миграции

-- Проверка существования колонки version (выполняется в коде миграции)
-- ALTER TABLE files ADD COLUMN version TEXT NOT NULL DEFAULT '1.0.0';

-- Создание уникального индекса для name@version
CREATE UNIQUE INDEX IF NOT EXISTS idx_files_name_version ON files(name, version);

-- Таблица зависимостей файлов
CREATE TABLE IF NOT EXISTS file_dependencies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_file_id INTEGER NOT NULL,
    target_file_name TEXT NOT NULL,
    target_file_version TEXT,
    dependency_type TEXT NOT NULL CHECK(dependency_type IN ('required', 'optional', 'peer')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_file_id) REFERENCES files(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_file_dependencies_source ON file_dependencies(source_file_id);
CREATE INDEX IF NOT EXISTS idx_file_dependencies_target ON file_dependencies(target_file_name, target_file_version);
























