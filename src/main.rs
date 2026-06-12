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

enum Screen {
    Search,
    Detail,
}

struct App {
    input: String,
    results: Vec<String>,
    result_ids: Vec<i64>,
    error: Option<String>,
    selected: usize,
    detail: Option<String>,
    screen: Screen,
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
            results: Vec::new(),
            result_ids: Vec::new(),
            error: None,
            selected: 0,
            detail: None,
            screen: Screen::Search,
        }
    }

    fn search(&mut self) {
        if self.input.is_empty() {
            return;
        }
        self.selected = 0;
        self.result_ids = Vec::new();
        self.detail = None;

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

                let mut ids = Vec::new();
                self.results = media_list
                    .iter()
                    .map(|item| {
                        let item = item.downcast::<PyDict>().unwrap();

                        let id: i64 = item.get_item("id").unwrap()
                            .map(|v| v.extract::<i64>().unwrap_or(0))
                            .unwrap_or(0);
                        ids.push(id);

                        let binding = item.get_item("title").unwrap().unwrap();
                        let title_obj = binding.downcast::<PyDict>().unwrap();

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
                            title, year, score, eps, status
                        )
                    })
                    .collect();
                self.result_ids = ids;

                self.error = None;
                Ok(())
            })();

            if let Err(e) = result {
                self.error = Some(format!("[error] {}", e));
            }
        });
    }

    fn fetch_detail(&mut self) {
        if self.result_ids.is_empty() {
            return;
        }
        let id = self.result_ids[self.selected];

        Python::with_gil(|py| {
            let result: PyResult<()> = (|| {
                let module = PyModule::import(py, "yaru_core.api")?;
                let detail_fn = module.getattr("get_anime_details")?;
                let response = detail_fn.call1((id,))?;
                let media = response.downcast::<PyDict>()?;

                let binding = media.get_item("title").unwrap().unwrap();
                let title_obj = binding.downcast::<PyDict>().unwrap();
                let english = title_obj.get_item("english").unwrap();
                let romaji = title_obj.get_item("romaji").unwrap();
                let title = match english {
                    Some(e) if !e.is_none() => e.to_string(),
                    _ => romaji.map(|r| r.to_string()).unwrap_or_default(),
                };

                let year = media.get_item("seasonYear").unwrap()
                    .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                let score = media.get_item("averageScore").unwrap()
                    .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                let eps = media.get_item("episodes").unwrap()
                    .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                let status = media.get_item("status").unwrap()
                    .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                let genres = media.get_item("genres").unwrap()
                    .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                let desc = media.get_item("description").unwrap()
                    .map(|v| v.to_string()).unwrap_or("N/A".to_string());

                self.detail = Some(format!(
                    "{} [{}]\n\nscore: {} | eps: {} | {}\ngenres: {}\n\n{}",
                    title, year, score, eps, status, genres, desc
                ));
                Ok(())
            })();

            if let Err(e) = result {
                self.detail = Some(format!("[error] {}", e));
            }
        });

        self.screen = Screen::Detail;
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

            match app.screen {
                Screen::Search => {
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

                    let footer = Paragraph::new("[esc] quit  [enter] search  [up/down] navigate  [i] details")
                        .style(Style::default().fg(Color::DarkGray));
                    f.render_widget(footer, chunks[2]);
                }
                Screen::Detail => {
                    let placeholder = Paragraph::new("")
                        .block(Block::default().borders(Borders::ALL).title(" yaru "));
                    f.render_widget(placeholder, chunks[0]);

                    let detail_text = app.detail.as_deref().unwrap_or("");
                    let detail = Paragraph::new(detail_text)
                        .block(Block::default().borders(Borders::ALL).title(" details "))
                        .wrap(ratatui::widgets::Wrap { trim: true });
                    f.render_widget(detail, chunks[1]);

                    let footer = Paragraph::new("[esc] back  [l] add to list")
                        .style(Style::default().fg(Color::DarkGray));
                    f.render_widget(footer, chunks[2]);
                }
            }
        })?;

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
                    KeyCode::Char('i') => app.fetch_detail(),
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                },
                Screen::Detail => match key.code {
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