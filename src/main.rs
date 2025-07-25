mod app;
use color_eyre::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
mod components;

fn main() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;
    let mut terminal = ratatui::init();
    let mut app = app::App::new();
    let result = app.run(&mut terminal);
    ratatui::restore();
    disable_raw_mode()?;
    result
}
