use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::{text::TuiText, theme::Theme};

/// Renders a compact help text row for minimalist mode
pub fn render_mini_help(frame: &mut Frame, area: Rect, theme: &Theme) {
    let mut help_spans = Vec::new();

    for (key, description) in TuiText::MINI_HELP_KEYS {
        help_spans.push(Span::styled(*key, theme.tab_style(true)));
        help_spans.push(Span::styled(format!(" {}  ", description), theme.player_time_style()));
    }

    let help_line = Line::from(help_spans);
    let help = Paragraph::new(help_line).alignment(Alignment::Left);

    frame.render_widget(help, area);
}

/// Renders full help modal overlay
pub fn render_full_help(frame: &mut Frame, area: Rect, theme: &Theme) {
    fn make_section<'a>(
        title: &'a str,
        items: &'a [(&str, &str)],
        theme: &'a Theme,
    ) -> Vec<Line<'a>> {
        let mut lines = vec![
            Line::from(""),
            Line::from(Span::styled(title, Style::default().add_modifier(Modifier::BOLD))),
        ];

        for (key, description) in items {
            let spans = vec![
                Span::raw("  "),
                Span::styled(*key, theme.tab_style(true)),
                Span::raw(" "),
                Span::styled(*description, theme.player_time_style()),
            ];
            lines.push(Line::from(spans));
        }

        lines
    }

    let mut help_lines = vec![
        Line::from(""),
        Line::from(Span::styled("♫ beat - Keyboard Shortcuts", theme.player_title_style())),
    ];

    help_lines.extend(make_section("Navigation:", TuiText::HELP_NAVIGATION, theme));
    help_lines.extend(make_section("Playback:", TuiText::HELP_PLAYBACK, theme));
    help_lines.extend(make_section("Queue:", TuiText::HELP_QUEUE, theme));
    help_lines.extend(make_section("Playlists:", TuiText::HELP_PLAYLISTS, theme));
    help_lines.extend(make_section("General:", TuiText::HELP_GENERAL, theme));

    help_lines.push(Line::from(""));
    help_lines.push(Line::from(Span::styled(
        "Press [?] or [Esc] to close",
        Style::default().add_modifier(Modifier::DIM),
    )));

    let help = Paragraph::new(help_lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(TuiText::HELP_TITLE)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.overlay_border)),
        );

    frame.render_widget(help, area);
}
