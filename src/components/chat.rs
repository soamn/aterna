use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
pub fn render_bot(f: &mut Frame, area: Rect, text: &str, model: &str) {
    // Escape hint in top-right
    let escape_hint = Span::styled(
        "[Esc] Escape mode",
        Style::default()
            .fg(Color::Gray)
            .add_modifier(Modifier::ITALIC),
    );

    // Title with model name and escape hint
    let title_line = Line::from(vec![
        Span::styled(model, Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("    "),
        escape_hint,
    ]);

    let para = Paragraph::new(text)
        .wrap(Wrap { trim: true }) // 👈 enables text wrapping
        .block(Block::default().borders(Borders::ALL).title(title_line));

    f.render_widget(para, area);
}
