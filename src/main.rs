mod app;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pyo3::prelude::*;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use app::{App, Screen};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    Python::with_gil(|py| {
        let module = PyModule::import(py, "yaru_core.db").unwrap();
        let init_fn = module.getattr("init_db").unwrap();
        init_fn.call0().unwrap();
    });

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.screen {
                Screen::Search => match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => app.search(),
                    KeyCode::Down => {
                        if !app.results.is_empty() && app.selected < app.results.len() - 1 {
                            app.selected += 1;
                        }
                    }
                    KeyCode::Up => {
                        if app.selected > 0 {
                            app.selected -= 1;
                        }
                    }
                    KeyCode::Tab => app.fetch_detail(),
                    KeyCode::Char('w') => app.fetch_list(),
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                },
                Screen::Detail => match key.code {
                    KeyCode::Esc => app.screen = Screen::Search,
                    KeyCode::Char('l') => app.add_to_list(),
                    _ => {}
                },
                Screen::List => match key.code {
                    KeyCode::Esc => app.screen = Screen::Search,
                    _ => {}
                },
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}