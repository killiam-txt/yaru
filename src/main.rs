use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;

struct App {
    input: String,
    results: Vec<String>,
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
            results: vec![
                "Cowboy Bebop (1998) | score: 8.6 | eps: 26 | FINISHED".to_string(),
                "Evangelion (1995) | score: 8.5 | eps: 26 | FINISHED".to_string(),
            ],
        }
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(f.area());

            let input = Paragraph::new(app.input.as_str())
                .block(Block::default().borders(Borders::ALL).title(" yaru "));
            f.render_widget(input, chunks[0]);

            let items: Vec<ListItem> = app
                .results
                .iter()
                .map(|r| ListItem::new(r.as_str()))
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" results "));
            f.render_widget(list, chunks[1]);

            let footer = Paragraph::new("[q] quit  [enter] search  [j/k] navigate")
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(footer, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char(c) => app.input.push(c),
                KeyCode::Backspace => { app.input.pop(); }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}