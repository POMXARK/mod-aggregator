-- Migration 006: Add navigation state to session_state
-- Adds current_page and selected_site_id fields for preserving navigation state

-- Добавляем новые столбцы в таблицу session_state
ALTER TABLE session_state ADD COLUMN current_page TEXT;
ALTER TABLE session_state ADD COLUMN selected_site_id INTEGER;