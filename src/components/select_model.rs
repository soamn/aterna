use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, ListState},
};
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use std::{env, time::Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
    pub active: bool,
    pub context_window: usize,
    pub public_apps: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub object: String,
    pub data: Vec<Model>,
}
pub async fn select_model<B: Backend>(terminal: &mut Terminal<B>) -> Result<String> {
    dotenv::dotenv().ok();
    let key = env::var("API_KEY")?;

    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.groq.com/openai/v1/models")
        .header(AUTHORIZATION, format!("Bearer {}", key))
        .send()
        .await?
        .text()
        .await?;

    let parsed: ApiResponse = serde_json::from_str(&resp)?;
    let models: Vec<String> = parsed.data.into_iter().map(|m| m.id).collect();

    let mut state = ListState::default();
    state.select(Some(0));

    let selected_model = loop {
        terminal.draw(|f| {
            let area = centered_rect(60, 50, f.area());
            let items: Vec<ListItem> = models.iter().map(|id| ListItem::new(id.clone())).collect();
            let list = List::new(items)
                .block(Block::default().title("Select Model").borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_widget(Clear, area);
            f.render_stateful_widget(list, area, &mut state);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Down => {
                        let i = match state.selected() {
                            Some(i) => (i + 1) % models.len(),
                            None => 0,
                        };
                        state.select(Some(i));
                    }
                    KeyCode::Up => {
                        let i = match state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    models.len() - 1
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        state.select(Some(i));
                    }
                    KeyCode::Enter => {
                        if let Some(i) = state.selected() {
                            break models[i].clone();
                        }
                    }
                    KeyCode::Esc => {
                        break String::new(); // treat as cancel
                    }
                    _ => {}
                }
            }
        }
    };

    Ok(selected_model)
}

/// Helper to center a rectangle in the terminal
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
