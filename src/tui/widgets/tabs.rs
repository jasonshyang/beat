use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::Tab, tui::theme::Theme};

pub fn render_tabs(frame: &mut Frame, area: Rect, current_tab: Tab, theme: &Theme) {
    let tabs = [("1", "Browse", Tab::Browse), ("2", "Queue", Tab::Queue)];

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
