use std::collections::HashSet;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::{
    domain::DirEntry,
    tui::{text::TuiText, theme::Theme},
};

pub fn render_browser(
    frame: &mut Frame,
    area: Rect,
    entries: &[DirEntry],
    selection: &ListState,
    multi_select: &HashSet<usize>,
    is_focused: bool,
    theme: &Theme,
) {
    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(idx, entry)| {
            let prefix = if entry.is_dir() { "📁 " } else { "🎵 " };
            let is_multi_selected = multi_select.contains(&idx);
            let checkbox = if is_multi_selected { "[✓] " } else { "" };

            let mut item = ListItem::new(format!("{}{}{}", checkbox, prefix, entry.name));

            // Style multi-selected items
            if is_multi_selected {
                item = item.style(Style::default().add_modifier(Modifier::BOLD));
            }

            item
        })
        .collect();

    let title = if multi_select.is_empty() {
        TuiText::BROWSER_HINT_NORMAL
    } else {
        TuiText::BROWSER_HINT_MULTI_SELECT
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(theme.browser_border_style(is_focused)),
        )
        .highlight_style(theme.browser_highlight_style());

    frame.render_stateful_widget(list, area, &mut selection.clone());
}
