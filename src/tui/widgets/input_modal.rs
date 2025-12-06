use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::{text::TuiText, theme::Theme};

/// Renders a generic text input modal
///
/// Used for user text input like renaming playlists
pub fn render_input_modal(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    prompt: &str,
    input: &str,
    theme: &Theme,
) {
    let input_text = if input.is_empty() {
        Line::from(vec![ratatui::text::Span::styled(
            prompt,
            Style::default().fg(theme.player_time),
        )])
    } else {
        Line::from(input.to_string())
    };

    let paragraph = Paragraph::new(vec![
        Line::from(""),
        input_text,
        Line::from(""),
        Line::from(TuiText::INPUT_MODAL_HINT),
    ])
    .block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.overlay_border)),
    );

    frame.render_widget(paragraph, area);
}
