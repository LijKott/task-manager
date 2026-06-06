mod db;
mod models;
mod commands;
mod tui;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name= "task-manager", about = "A simple CLI task manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String },
    List,
    Done { id: i64 },
    Delete { id: i64 },
    Tui,
    Reset,
}

fn main() {
    let cli = Cli::parse();
    let conn = db::open().expect("Failed to open database");

    match cli.command {
        Some(Commands::Add { title }) => commands::add(&conn, &title),
        Some(Commands::List) => commands::list(&conn),
        Some(Commands::Done { id }) => commands::done(&conn, id),
        Some(Commands::Delete { id }) => commands::delete(&conn, id),
        Some(Commands::Tui) => tui::run(&conn).expect("TUI failed"),
        Some(Commands::Reset) => commands::reset(&conn),
        None => tui::run(&conn).expect("TUI failed"),
    }
}
