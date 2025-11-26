use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::theme::Theme;

/// Renders a compact help text row for minimalist mode
pub fn render_mini_help(frame: &mut Frame, area: Rect, theme: &Theme) {
    let help_spans = vec![
        Span::styled("[b]", theme.tab_style(true)),
        Span::styled(" browser  ", theme.player_time_style()),
        Span::styled("[p]", theme.tab_style(true)),
        Span::styled(" play/pause  ", theme.player_time_style()),
        Span::styled("[n]", theme.tab_style(true)),
        Span::styled(" next  ", theme.player_time_style()),
        Span::styled("[a]", theme.tab_style(true)),
        Span::styled(" add all  ", theme.player_time_style()),
        Span::styled("[↑↓]", theme.tab_style(true)),
        Span::styled(" navigate  ", theme.player_time_style()),
        Span::styled("[Enter]", theme.tab_style(true)),
        Span::styled(" select  ", theme.player_time_style()),
        Span::styled("[Shift+Q]", theme.tab_style(true)),
        Span::styled(" quit", theme.player_time_style()),
    ];

    let help_line = Line::from(help_spans);
    let help = Paragraph::new(help_line).alignment(Alignment::Left);

    frame.render_widget(help, area);
}

pub fn render_help_text(frame: &mut Frame, area: Rect, theme: &Theme) {
    let help_lines = vec![
        Line::from(""),
        Line::from(""),
        Line::from(vec![Span::styled("♫ beat", theme.player_title_style())]),
        Line::from(""),
        Line::from(vec![
            Span::styled("[b]", theme.tab_style(true)),
            Span::styled(" toggle browser  ", theme.player_time_style()),
            Span::styled("[space]", theme.tab_style(true)),
            Span::styled(" play/pause  ", theme.player_time_style()),
            Span::styled("[q]", theme.tab_style(true)),
            Span::styled(" quit", theme.player_time_style()),
        ]),
    ];

    let help = Paragraph::new(help_lines).alignment(Alignment::Center);

    frame.render_widget(help, area);
}
