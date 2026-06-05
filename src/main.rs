mod db;
mod models;
mod commands;

fn main() {
    let conn = db::open().expect("Failed to open database");

    db::mark_done(&conn, 1).expect("Failed to mark done");
    db::delete_task(&conn, 2).expect("Failed to delete task");

    let tasks = db::list_tasks(&conn).expect("Failed to list tasks");
    for task in tasks {
        println!("{}: {} [{}]", task.id, task.title, if task.done { "x" } else { "" });
    }
}
