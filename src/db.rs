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

use crate::models::Task;

pub fn list_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, done FROM tasks ORDER BY id DESC"
    )?;
    let tasks = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            done: row.get(2)?,
        })
    })?.filter_map(Result::ok).collect();
    Ok(tasks)
}

pub fn toggle_done(conn: &Connection, id: i64) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET done = CASE WHEN done = 1 THEN 0 ELSE 1 END WHERE id = ?1",
        [id],
    )?;
    Ok(())
}

pub fn delete_task(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
    Ok(())
}

pub fn reset(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        DELETE FROM tasks;
        DELETE FROM sqlite_sequence WHERE name='tasks';
    ")?;
    Ok(())
}
