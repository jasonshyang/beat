use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::{domain::DirEntry, tui::theme::Theme};

pub fn render_browser(
    frame: &mut Frame,
    area: Rect,
    entries: &[DirEntry],
    selection: &ListState,
    is_focused: bool,
    theme: &Theme,
) {
    let items: Vec<ListItem> = entries
        .iter()
        .map(|entry| {
            let prefix = if entry.is_dir() { "📁 " } else { "🎵 " };
            ListItem::new(format!("{}{}", prefix, entry.name))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Files")
                .borders(Borders::ALL)
                .border_style(theme.browser_border_style(is_focused)),
        )
        .highlight_style(theme.browser_highlight_style());

    frame.render_stateful_widget(list, area, &mut selection.clone());
}
