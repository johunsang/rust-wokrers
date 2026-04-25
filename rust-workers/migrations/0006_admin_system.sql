-- Admin users
CREATE TABLE IF NOT EXISTS admin_users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'viewer',
    avatar_url TEXT,
    github_login TEXT,
    last_login_at TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Access logs
CREATE TABLE IF NOT EXISTS access_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_email TEXT NOT NULL,
    action TEXT NOT NULL,
    path TEXT NOT NULL,
    method TEXT NOT NULL DEFAULT 'GET',
    status_code INTEGER,
    ip_address TEXT,
    user_agent TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_access_logs_email ON access_logs(user_email);
CREATE INDEX IF NOT EXISTS idx_access_logs_created ON access_logs(created_at DESC);

-- API request logs
CREATE TABLE IF NOT EXISTS api_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    method TEXT NOT NULL,
    path TEXT NOT NULL,
    status_code INTEGER NOT NULL,
    duration_ms INTEGER,
    request_body TEXT,
    response_size INTEGER,
    ip_address TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_api_logs_created ON api_logs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_api_logs_path ON api_logs(path);

-- Seed: default administrator placeholder
INSERT OR IGNORE INTO admin_users (email, name, role, github_login, is_active, created_at, updated_at)
VALUES ('REPLACE_WITH_ADMIN_EMAIL', 'Admin User', 'super_admin', 'example', 1, datetime('now'), datetime('now'));
