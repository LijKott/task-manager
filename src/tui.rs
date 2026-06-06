use rusqlite::Connection;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use std::io;
use crate::models::Task;
use crate::db;
use crossterm::event::{ self, Event, KeyCode, KeyEventKind };

//App state
enum Mode {
    Normal,
    Adding,
}

struct App {
    tasks: Vec<Task>,
    selected: usize,
    mode: Mode,
    input: String,
}

impl App {
    fn new(conn: &Connection) -> Self {
        let tasks = db::list_tasks(conn).unwrap_or_default();
        App { tasks, selected: 0, mode: Mode::Normal, input: String::new() }
    }

    fn next(&mut self) {
        if !self.tasks.is_empty() {
            self.selected = (self.selected + 1) % self.tasks.len();
        }
    }
    
    fn previous(&mut self) {
        if !self.tasks.is_empty() {
            if self.selected == 0 {
                self.selected = self.tasks.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }
}

use ratatui::{
    layout::{ Constraint, Direction, Layout },
    style::{ Color, Modifier, Style },
    widgets::{ Block, Borders, List, ListItem, ListState },
    Frame,
};

fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.size());
    
    //Build task list
    let items: Vec<ListItem> = app.tasks.iter().map(|t| {
        let status = if t.done { "[x]" } else { "[ ]" };
        ListItem::new(format!("{} {} {}", t.id, status, t.title))
    }).collect();

    let mut state = ListState::default();
    state.select(Some(app.selected));

    let list = List::new(items)
        .block(Block::default().title(" Tasks ").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, chunks[0], &mut state);

    //Bottom bar
    match app.mode {
        Mode::Adding => {
            let input = ratatui::widgets::Paragraph::new(format!("Add task: {}", app.input))
                .block(Block::default().borders(Borders::ALL).title(" New Task "))
                .style(Style::default().fg(Color::Yellow));
            frame.render_widget(input, chunks[1]);
        }
        Mode::Normal => {
            let help = ratatui::widgets::Paragraph::new("a: add d: done x: delete r: reset q: quit")
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(help, chunks[1]);
        }
    }
}

pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    //Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //App
    let mut app = App::new(conn);
    loop {
        terminal.draw(|f| draw(f, &app))?;
        
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press { continue; }
            match app.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char('a') => app.mode = Mode::Adding,
                    KeyCode::Char('d') => {
                        if let Some(task) = app.tasks.get(app.selected) {
                            db::toggle_done(&conn, task.id).ok();
                            app.tasks = db::list_tasks(&conn).unwrap_or_default();
                        }
                    }
                    KeyCode::Char('x') => {
                        if let Some(task) = app.tasks.get(app.selected) {
                            db::delete_task(&conn, task.id).ok();
                            app.tasks = db::list_tasks(&conn).unwrap_or_default();
                            if app.selected > 0 { app.selected -= 1; }
                        }
                    }
                    KeyCode::Char('r') => {
                        if let Some(_task) = app.tasks.get(app.selected) {
                            db::reset(&conn).ok();
                            app.tasks = db::list_tasks(&conn).unwrap_or_default();
                            app.selected = 0;
                        }
                    }
                    _ => {}
                },
                Mode::Adding => match key.code {
                    KeyCode::Enter => {
                        if !app.input.is_empty() {
                            db::add_task(&conn, &app.input).ok();
                            app.tasks = db::list_tasks(&conn).unwrap_or_default();
                            app.input.clear();
                        }
                        app.mode = Mode::Normal;
                    }
                    KeyCode::Esc => {
                        app.input.clear();
                        app.mode = Mode::Normal;
                    }
                    KeyCode::Backspace => { app.input.pop(); }
                    KeyCode::Char(c) => app.input.push(c),
                    _ => {}
                },
            }
        }
    }

    //Exit
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
