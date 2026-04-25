-- D1 migration template
-- Copy this file and replace __items__ with the real table name.
-- File name example: 0002_add___items__.sql, using the next migration number.

CREATE TABLE IF NOT EXISTS __items__ (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    title      TEXT    NOT NULL,
    status     TEXT    NOT NULL DEFAULT 'active',
    created_at TEXT    NOT NULL,
    updated_at TEXT    NOT NULL
);
