use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::{
    domain::PlayQueue,
    tui::{text::TuiText, theme::Theme},
};

pub fn render_queue(
    frame: &mut Frame,
    area: Rect,
    queue: &PlayQueue,
    selection: &ListState,
    theme: &Theme,
) {
    let items: Vec<ListItem> = queue
        .iter()
        .enumerate()
        .map(|(i, track)| ListItem::new(format!("{}. {}", i + 1, track.name)))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.browser_border_style(false))
                .title(TuiText::QUEUE_TITLE),
        )
        .highlight_style(theme.browser_highlight_style());

    frame.render_stateful_widget(list, area, &mut selection.clone());
}
