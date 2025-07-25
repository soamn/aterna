use crate::components::{render_bot, render_input};
use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::Backend, Frame, Terminal};
use std::time::Duration;
pub struct Escape;
//pub struct Active;

pub struct App<State = Escape> {
    model: String,
    state: std::marker::PhantomData<State>,
}
impl App {
    pub fn new() -> Self {
        Self {
            state: std::marker::PhantomData,
            model: "gpt".to_string(),
        }
    }
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        self.model = String::from("gpt");
        let mut input = String::new();
        let mut response = String::from("Hello from Bot!");

        loop {
            terminal.draw(|f: &mut Frame| {
                let chunks = ratatui::layout::Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .margin(1)
                    .constraints([
                        ratatui::layout::Constraint::Length(5),
                        ratatui::layout::Constraint::Length(3),
                    ])
                    .split(f.area());

                render_bot(f, chunks[0], &response, &self.model);
                render_input(f, chunks[1], &input);
            })?;
            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key) => match key.code {
                        KeyCode::Char(c) => input.push(c),
                        KeyCode::Backspace => {
                            input.pop();
                        }
                        KeyCode::Enter => {
                            response = format!("Echo: {}", input.trim());
                            input.clear();
                        }
                        KeyCode::Esc => break,
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
