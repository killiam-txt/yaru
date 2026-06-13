use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};
use crate::app::{App, Screen};

pub fn draw(f: &mut Frame, app: &App) {
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

            let footer_text = if app.insert_mode {
                "[esc] normal mode  [enter] search"
            } else {
                "[esc] quit  [i] insert  [up/down] navigate  [tab] details  [l] add to list  [w] watchlist"
            };
            let footer = Paragraph::new(footer_text)
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
                .wrap(Wrap { trim: true });
            f.render_widget(detail, chunks[1]);

            let footer = Paragraph::new("[esc] back  [l] add to list")
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(footer, chunks[2]);
        }
        Screen::List => {
            let placeholder = Paragraph::new("")
                .block(Block::default().borders(Borders::ALL).title(" yaru "));
            f.render_widget(placeholder, chunks[0]);

            let items: Vec<ListItem> = if app.list_items.is_empty() {
                vec![ListItem::new("-- watchlist is empty --")]
            } else {
                app.list_items
                    .iter()
                    .map(|r| ListItem::new(r.as_str()))
                    .collect()
            };

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" watchlist "))
                .highlight_symbol("> ");

            let mut state = ListState::default();
            state.select(Some(app.list_selected));
            f.render_stateful_widget(list, chunks[1], &mut state);

            let footer = Paragraph::new("[esc] back  [up/down] navigate  [d] remove")
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(footer, chunks[2]);
        }
    }
}