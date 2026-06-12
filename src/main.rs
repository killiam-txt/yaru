use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::io;

struct App {
    input: String,
    results: Vec<String>,
    error: Option<String>,
    selected: usize,
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
            results: Vec::new(),
            error: None,
            selected: 0,
        }
    }

    fn search(&mut self) {
        if self.input.is_empty() {
            return;
        }
        self.selected = 0;

        Python::with_gil(|py| {
            let result: PyResult<()> = (|| {
                let module = PyModule::import(py, "yaru_core.api")?;
                let search_fn = module.getattr("search_anime")?;
                let response = search_fn.call1((&self.input,))?;

                let media = response
                    .downcast::<PyDict>()?
                    .get_item("media")?
                    .unwrap();

                let media_list = media.downcast::<pyo3::types::PyList>()?;

                self.results = media_list
                    .iter()
                    .map(|item| {
                        let item = item.downcast::<PyDict>().unwrap();

                        let binding = item
                            .get_item("title").unwrap().unwrap();
                        let title_obj = binding
                            .downcast::<PyDict>().unwrap();

                        let english = title_obj.get_item("english").unwrap();
                        let romaji = title_obj.get_item("romaji").unwrap();

                        let title = match english {
                            Some(e) if !e.is_none() => e.to_string(),
                            _ => romaji.map(|r| r.to_string()).unwrap_or_default(),
                        };

                        let year = item.get_item("seasonYear").unwrap()
                            .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                        let score = item.get_item("averageScore").unwrap()
                            .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                        let eps = item.get_item("episodes").unwrap()
                            .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                        let status = item.get_item("status").unwrap()
                            .map(|v| v.to_string()).unwrap_or("N/A".to_string());

                        format!(
                            "{:<35} [{:>4}] | score: {:>4} | eps: {:>4} | {}",
                            title,
                            year,
                            score,
                            eps,
                            status
                        )
                    })
                    .collect();

                self.error = None;
                Ok(())
            })();

            if let Err(e) = result {
                self.error = Some(format!("[error] {}", e));
            }
        });
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

            let items: Vec<ListItem> = if let Some(ref err) = app.error {
                vec![ListItem::new(err.as_str())]
            } else {
                app.results
                    .iter()
                    .map(|r| ListItem::new(r.as_str()))
                    .collect()
            };

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" results "))
                .highlight_symbol("> ");

            let mut state = ListState::default();
            state.select(Some(app.selected));
            f.render_stateful_widget(list, chunks[1], &mut state);

            let footer = Paragraph::new("[esc] quit  [enter] search  [up/down] navigate")
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(footer, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
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
                KeyCode::Char(c) => app.input.push(c),
                KeyCode::Backspace => {
                    app.input.pop();
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}