import Database from 'better-sqlite3';
const db = Database('mod_aggregator.db');
const result = db.prepare('PRAGMA table_info(session_state)').all();
console.log('Session state columns:');
result.forEach(col => console.log(- \: \));
db.close();
