use rusqlite::{Connection, Result};

pub fn open() -> Result<Connection> {
    let conn = Connection::open("tasks.db")?;
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT 0
        );
    ")?;
    Ok(conn)
}

pub fn add_task(conn: &Connection, title: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (title) VALUES (?1)",
        [title],
    )?;
    Ok(())
}
