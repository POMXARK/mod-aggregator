-- Migration 005: Add session state support
-- Creates session_state table for persisting application state

-- Таблица состояния сессии
CREATE TABLE IF NOT EXISTS session_state (
    id INTEGER PRIMARY KEY CHECK(id = 1),
    file_order TEXT NOT NULL DEFAULT '[]', -- JSON stored as TEXT in SQLite
    ui_preferences TEXT NOT NULL DEFAULT '{}', -- JSON stored as TEXT in SQLite
    open_collections TEXT DEFAULT '[]', -- JSON stored as TEXT in SQLite
    selected_files TEXT DEFAULT '[]', -- JSON stored as TEXT in SQLite
    recent_actions TEXT DEFAULT '[]', -- JSON stored as TEXT in SQLite
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Вставка начальной записи (если не существует)
INSERT OR IGNORE INTO session_state (id) VALUES (1);
























