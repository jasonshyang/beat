use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{domain::PlayQueue, tui::theme::Theme};

pub fn render_queue(frame: &mut Frame, area: Rect, queue: &PlayQueue, theme: &Theme) {
    let items: Vec<ListItem> = queue
        .iter()
        .enumerate()
        .map(|(i, music)| ListItem::new(format!("{}. {}", i + 1, music.name)))
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme.browser_border_style(false))
            .title("Playing Next"),
    );

    frame.render_widget(list, area);
}
