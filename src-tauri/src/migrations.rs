use crate::errors::AppResult;
use rusqlite::Connection;

pub const CURRENT_SCHEMA_VERSION: i64 = 2;

pub fn migrate(conn: &Connection) -> AppResult<()> {
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;

        CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS app_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS model_packages (
            id TEXT PRIMARY KEY,
            kind TEXT NOT NULL,
            display_name TEXT NOT NULL,
            internal_name TEXT NOT NULL,
            relative_path TEXT NOT NULL,
            required_files_json TEXT NOT NULL,
            manifest_json TEXT NOT NULL,
            installed INTEGER NOT NULL,
            verified INTEGER NOT NULL,
            last_verified_at TEXT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS engine_profiles (
            id TEXT PRIMARY KEY,
            kind TEXT NOT NULL,
            name TEXT NOT NULL,
            enabled INTEGER NOT NULL,
            binary_mode TEXT NOT NULL,
            binary_name TEXT NOT NULL,
            binary_path TEXT NULL,
            model_package_id TEXT NULL,
            model_path TEXT NULL,
            model_dir TEXT NULL,
            tokens_path TEXT NULL,
            host TEXT NOT NULL,
            port INTEGER NOT NULL,
            health_url TEXT NULL,
            runtime_json TEXT NOT NULL,
            extra_args_json TEXT NOT NULL,
            auto_start INTEGER NOT NULL,
            status TEXT NOT NULL,
            pid INTEGER NULL,
            last_error TEXT NULL,
            last_exit_code INTEGER NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS engine_runtime_state (
            engine_id TEXT PRIMARY KEY,
            status TEXT NOT NULL,
            pid INTEGER NULL,
            health_url TEXT NULL,
            started_at TEXT NULL,
            stopped_at TEXT NULL,
            last_error TEXT NULL,
            last_exit_code INTEGER NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS log_files (
            id TEXT PRIMARY KEY,
            engine_id TEXT NULL,
            kind TEXT NOT NULL,
            path TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        "#,
    )?;

    let current_version: i64 = conn
        .query_row("SELECT COALESCE(MAX(version), 0) FROM schema_version", [], |row| row.get(0))
        .unwrap_or(0);

    if current_version < CURRENT_SCHEMA_VERSION {
        conn.execute("DELETE FROM schema_version", [])?;
        conn.execute(
            "INSERT INTO schema_version (version) VALUES (?)",
            [CURRENT_SCHEMA_VERSION],
        )?;
    }

    Ok(())
}
