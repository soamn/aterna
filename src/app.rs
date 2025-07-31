use crate::components::{render_bot, render_input, select_model};
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{Frame, Terminal, prelude::Backend};
use std::{env, time::Duration};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    Active,
    Escape,
    ModelMenu,
}

pub struct App {
    model: String,
    state: AppState,
    input: String,
    response: String,
    rx: mpsc::UnboundedReceiver<String>, // Receiver
    tx: mpsc::UnboundedSender<String>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            model: String::from("deepseek-r1-distill-llama-70b"),
            state: AppState::Active,
            input: String::new(),
            response: String::from("Hello from Bot!"),
            rx,
            tx,
        }
    }
    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            if let Ok(msg) = self.rx.try_recv() {
                self.response = msg;
            }
            if self.state == AppState::ModelMenu {
                if let Ok(model) = select_model(terminal).await {
                    if model.is_empty() {
                        self.response = "Model selection cancelled.".to_string();
                    } else {
                        self.model = model;
                        self.response = format!("Model changed to {}", self.model);
                    }
                } else {
                    self.response = "Model selection failed.".to_string();
                }
                self.state = AppState::Escape;
                continue;
            }

            // Draw the UI
            terminal.draw(|f: &mut Frame| {
                let chunks = ratatui::layout::Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .margin(1)
                    .constraints([
                        ratatui::layout::Constraint::Percentage(90),
                        ratatui::layout::Constraint::Percentage(10),
                    ])
                    .split(f.area());

                let display_response = match self.state {
                    AppState::Active => format!("{}", self.response),
                    AppState::Escape => format!("{} [ESCAPE MODE]", self.response),
                    AppState::ModelMenu => format!("{}", self.response),
                };

                render_bot(f, chunks[0], &display_response, &self.model);
                render_input(f, chunks[1], &self.input);
            })?;

            // Handle events with timeout
            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key) => {
                        if self.handle_key_event(key).await? {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    async fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<bool> {
        match (key.code, key.modifiers, self.state) {
            // Global quit command - return true to break the loop
            (KeyCode::Char('q'), m, _) if m.contains(KeyModifiers::CONTROL) => {
                return Ok(true); // Signal to break the loop
            }
            (KeyCode::Esc, _, AppState::Active) => {
                self.state = AppState::Escape;
            }
            (KeyCode::Esc, _, AppState::Escape) => {
                self.state = AppState::Active;
            }
            // Active state key handling
            (KeyCode::Char(c), m, AppState::Active) if !m.contains(KeyModifiers::CONTROL) => {
                self.input.push(c);
            }
            (KeyCode::Backspace, _, AppState::Active) => {
                self.input.pop();
            }
            (KeyCode::Enter, _, AppState::Active) => {
                self.process_input();
            }

            // Escape state key handling
            (KeyCode::Char('q'), _, AppState::Escape) => {
                return Ok(true); // Signal to break the loop
            }
            (KeyCode::Char('c'), _, AppState::Escape) => {
                self.input.clear();
                self.response = "Input cleared!".to_string();
            }
            (KeyCode::Char('r'), _, AppState::Escape) => {
                self.response = "Response reset!".to_string();
            }
            (KeyCode::Char('m'), _, AppState::Escape) => {
                self.state = AppState::ModelMenu;
            }

            // Ignore other keys or show help
            _ => {
                if self.state == AppState::Escape {
                    self.show_escape_help();
                }
            }
        }

        Ok(false) // Continue the loop
    }

    fn process_input(&mut self) {
        if !self.input.trim().is_empty() {
            let input = self.input.clone();
            self.input.clear();
            self.response = "Thinking...".to_string();
            let key = match env::var("API_KEY") {
                Ok(val) => val,
                Err(_) => "nokey".to_string(),
            };
            let model = self.model.clone();
            let tx = self.tx.clone(); // clone sender

            tokio::spawn(async move {
                match send_to_groq(&input, &model, &key).await {
                    Ok(reply) => {
                        let _ = tx.send(reply); // send back response
                    }
                    Err(e) => {
                        let _ = tx.send(format!("Error: {}", e));
                    }
                }
            });
        } else {
            self.response = "Please enter some text!".to_string();
        }

        self.input.clear();
    }
    fn show_escape_help(&mut self) {
        self.response = [
            "Escape mode:",
            "  [q] Quit",
            "  [c] Clear input",
            "  [r] Reset response",
            "  [m] Toggle model menu",
            "  [Esc] Back",
        ]
        .join("\n");
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn send_to_groq(prompt: &str, model: &str, api_key: &str) -> Result<String> {
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "model": model,
        "messages": [
            { "role": "user", "content": prompt }
        ]
    });

    let res = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let response_text = res["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No response")
        .to_string();

    Ok(response_text)
}
