use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::{text::TuiText, theme::Theme};

/// Renders an error notification overlay
pub fn render_error(frame: &mut Frame, area: Rect, message: &str, _theme: &Theme) {
    let paragraph = Paragraph::new(vec![
        Line::from(""),
        Line::from(message),
        Line::from(""),
        Line::from(vec![ratatui::text::Span::styled(
            TuiText::ERROR_NOTIFICATION_HINT,
            Style::default().add_modifier(Modifier::DIM),
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .title("Error")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red)),
    );

    frame.render_widget(paragraph, area);
}
