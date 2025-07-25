use ratatui::{
    prelude::*,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render_input(f: &mut Frame, area: Rect, input: &str) {
    let para = Paragraph::new(Line::from(Span::raw(input)))
        .block(Block::default().borders(Borders::ALL).title("You"));
    f.render_widget(para, area);
}
