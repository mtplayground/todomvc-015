use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub type DbPool = Arc<Mutex<Connection>>;

/// Open (or create) the SQLite database and run migrations.
pub fn init_db() -> Result<DbPool, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "todos.db".to_string());

    let conn = Connection::open(&database_url)?;

    // Enable WAL mode for better concurrent read performance
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;

    run_migrations(&conn)?;

    tracing::info!("database initialized at {}", database_url);

    Ok(Arc::new(Mutex::new(conn)))
}

fn run_migrations(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY NOT NULL,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0,
            display_order INTEGER NOT NULL DEFAULT 0
        );",
    )?;

    tracing::info!("migrations applied");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_in_memory() {
        let conn = Connection::open_in_memory().expect("open in-memory db");
        run_migrations(&conn).expect("migrations should succeed");

        // Verify table exists by inserting a row
        conn.execute(
            "INSERT INTO todos (id, title, completed, display_order) VALUES (?1, ?2, ?3, ?4)",
            ("test-id", "Test todo", false, 0),
        )
        .expect("insert should succeed");

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM todos", [], |row| row.get(0))
            .expect("count query");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_migrations_idempotent() {
        let conn = Connection::open_in_memory().expect("open in-memory db");
        run_migrations(&conn).expect("first migration");
        run_migrations(&conn).expect("second migration should also succeed");
    }
}
