use tauri_plugin_sql::{Migration, MigrationKind};

pub fn m() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "create_initial_tables",
        sql: "
        CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY,
            router_ip TEXT NOT NULL,
            user TEXT NOT NULL,
            pass TEXT NOT NULL,
            dry_run BOOLEAN NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY,
            message TEXT NOT NULL,
            level TEXT NOT NULL DEFAULT 'info',
            status TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        ",
        kind: MigrationKind::Up,
    }]
}
