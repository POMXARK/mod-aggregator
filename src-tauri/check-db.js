import sqlite3 from 'sqlite3';
const db = new sqlite3.Database('mod_aggregator.db');
db.all('PRAGMA table_info(session_state)', (err, rows) => {
  if (err) console.error('Error:', err);
  else {
    console.log('Session state table columns:');
    rows.forEach(row => console.log('- ' + row.name + ' (' + row.type + ')'));
  }
  db.close();
});
