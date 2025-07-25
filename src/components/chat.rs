use ratatui::{
    prelude::*,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render_bot(f: &mut Frame, area: Rect, text: &str, model: &str) {
    let para = Paragraph::new(Line::from(Span::raw(text)))
        .block(Block::default().borders(Borders::ALL).title(model));
    f.render_widget(para, area);
}
