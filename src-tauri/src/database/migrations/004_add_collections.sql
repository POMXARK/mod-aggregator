-- Migration 004: Add collections and collection logic support
-- Creates collections, collection_files, and collection_logic_rules tables

-- Таблица коллекций
CREATE TABLE IF NOT EXISTS collections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Промежуточная таблица коллекций и файлов
CREATE TABLE IF NOT EXISTS collection_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    collection_id INTEGER NOT NULL,
    file_id INTEGER NOT NULL,
    logic_rule_id INTEGER,
    order_index INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (logic_rule_id) REFERENCES collection_logic_rules(id) ON DELETE SET NULL,
    UNIQUE(collection_id, file_id)
);

-- Таблица правил логики коллекций
CREATE TABLE IF NOT EXISTS collection_logic_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    collection_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    condition_type TEXT NOT NULL CHECK(condition_type IN ('boolean', 'collection_check', 'file_check', 'and', 'or')),
    condition_params TEXT NOT NULL, -- JSON stored as TEXT in SQLite
    action TEXT NOT NULL CHECK(action IN ('enable', 'disable')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_collection_files_collection ON collection_files(collection_id);
CREATE INDEX IF NOT EXISTS idx_collection_files_file ON collection_files(file_id);
CREATE INDEX IF NOT EXISTS idx_collection_logic_rules_collection ON collection_logic_rules(collection_id);



































