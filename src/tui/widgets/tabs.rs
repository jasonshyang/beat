use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    app::Tab,
    tui::{text::TuiText, theme::Theme},
};

pub fn render_tabs(frame: &mut Frame, area: Rect, current_tab: Tab, theme: &Theme) {
    let tabs = [
        ("1", TuiText::TAB_BROWSE, Tab::Browse),
        ("2", TuiText::TAB_QUEUE, Tab::Queue),
        ("3", TuiText::TAB_PLAYLIST, Tab::Playlist),
    ];

    let spans: Vec<Span> = tabs
        .iter()
        .flat_map(|(key, label, tab)| {
            let is_active = *tab == current_tab;
            let style = theme.tab_style(is_active);

            vec![
                Span::raw("  "),
                Span::styled(format!("[{}]", key), style),
                Span::raw(" "),
                Span::styled(*label, style),
            ]
        })
        .collect();

    let tabs_paragraph = Paragraph::new(Line::from(spans)).block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(theme.tab_border_style()),
    );

    frame.render_widget(tabs_paragraph, area);
}
