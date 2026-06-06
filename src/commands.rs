use rusqlite::Connection;
use crate::db;

pub fn add(conn: &Connection, title: &str) {
    match db::add_task(conn, title) {
        Ok(_) => println!("Task added: {}", title),
        Err(e) => println!("Error adding task: {}", e),
    }
}

pub fn list(conn: &Connection) {
    match db::list_tasks(conn) {
        Ok(tasks) if tasks.is_empty() => println!("No tasks yet"),
        Ok(tasks) => {
            for task in tasks {
                println!("{}, {} [{}]", task.id, task.title, if task.done { "x" } else { "" });
            }
        }
        Err(e) => println!("Error listing tasks: {}", e),
    }
}

pub fn done(conn: &Connection, id: i64) {
    match db::toggle_done(conn, id) {
        Ok(_) => println!("Task {} toggled", id),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn delete(conn: &Connection, id: i64) {
    match db::delete_task(conn, id) {
        Ok(_) => println!("Task {} deleted", id),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn reset(conn: &Connection) {
    match db::reset(conn) {
        Ok(_) => println!("Database reset"),
        Err(e) => println!("Error: {}", e),
    }
}
