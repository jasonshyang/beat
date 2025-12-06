use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::{
    domain::{Playlist, Playlists},
    tui::{text::TuiText, theme::Theme},
};

pub fn render_playlists(
    frame: &mut Frame,
    area: Rect,
    playlists: &Playlists,
    selection: &ListState,
    theme: &Theme,
) {
    let items: Vec<ListItem> = playlists
        .all()
        .iter()
        .map(|playlist| {
            let track_count = if playlist.is_empty() {
                "empty".to_string()
            } else {
                format!("{} tracks", playlist.len())
            };
            ListItem::new(format!("♫ {} ({})", playlist.name, track_count))
        })
        .collect();

    let title = if playlists.is_empty() {
        TuiText::PLAYLIST_TITLE_EMPTY
    } else {
        TuiText::PLAYLIST_TITLE_WITH_ITEMS
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(theme.browser_border_style(false)),
        )
        .highlight_style(theme.browser_highlight_style());

    frame.render_stateful_widget(list, area, &mut selection.clone());
}

pub fn render_playlist_detail(
    frame: &mut Frame,
    area: Rect,
    playlist: &Playlist,
    selection: &ListState,
    theme: &Theme,
) {
    let items: Vec<ListItem> = playlist
        .tracks
        .iter()
        .map(|track| {
            let mut parts = vec![track.name.clone()];

            if let Some(artist) = &track.artist {
                parts.push(format!(" • {}", artist));
            }

            ListItem::new(parts.join(""))
        })
        .collect();

    let title = if playlist.is_empty() {
        format!("♫ {} • {}", playlist.name, TuiText::PLAYLIST_DETAIL_EMPTY_HINT)
    } else {
        format!(
            "♫ {} ({} tracks) • {} • {}",
            playlist.name,
            playlist.tracks.len(),
            TuiText::PLAYLIST_DETAIL_PLAY_HINT,
            TuiText::PLAYLIST_DETAIL_HINT
        )
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(theme.browser_border_style(false)),
        )
        .highlight_style(theme.browser_highlight_style());

    frame.render_stateful_widget(list, area, &mut selection.clone());
}

/// Renders an interactive playlist selector for adding tracks
///
/// Shows a list of existing playlists with an option to create a new one
pub fn render_playlist_selector(
    frame: &mut Frame,
    area: Rect,
    playlists: &Playlists,
    selection: &ListState,
    theme: &Theme,
) {
    let mut items: Vec<ListItem> = playlists
        .all()
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let track_count = p.tracks.len();
            let label = format!("  {}. {} ({} tracks)", i + 1, p.name, track_count);
            ListItem::new(Line::from(label))
        })
        .collect();

    // Add "Create new playlist" option at the end
    let create_option = format!("  {}. {}", items.len() + 1, TuiText::PLAYLIST_SELECTOR_NEW);
    items.push(ListItem::new(Line::from(create_option)));

    let title =
        format!("{} • {}", TuiText::PLAYLIST_SELECTOR_TITLE, TuiText::PLAYLIST_SELECTOR_HINT);

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.overlay_border)),
        )
        .highlight_style(
            Style::default()
                .bg(theme.browser_highlight)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut selection.clone());
}
