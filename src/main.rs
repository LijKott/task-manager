mod db;
mod models;

fn main() {
    let conn = db::open().expect("Failed to open database");
    db::add_task(&conn, "my first task").expect("Failed to add task");
    println!("Database opened {:?} and task added", conn);
}
