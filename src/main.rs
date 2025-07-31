mod app;
use color_eyre::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
mod components;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = app.run(&mut terminal).await;
    ratatui::restore();
    disable_raw_mode()?;
    result
}
