use std::collections::HashSet;

use ratatui::widgets::ListState;

use super::Tab;
use crate::app::music::MusicState;

/// Represents which view is active in the Playlist tab
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlaylistView {
    /// Viewing the list of playlists
    #[default]
    List,
    /// Viewing the contents of a specific playlist
    Detail(usize),
}

/// Selection state management for all lists in the application
///
/// Centralizes selection tracking for browser, queue, playlists, and modals.
#[derive(Default)]
pub struct SelectionState {
    /// Selection state in the library browser
    pub browser: ListState,
    /// Selection state in the queue list
    pub queue: ListState,
    /// Selection state in the playlist list
    pub playlist: ListState,
    /// Selection state in the playlist selector modal
    pub playlist_selector: ListState,
    /// Selection state when viewing playlist contents
    pub playlist_track: ListState,
    /// Multi-selected indices in the browser
    pub browser_multi_select: HashSet<usize>,
    /// Current playlist view
    pub playlist_view: PlaylistView,
}

impl SelectionState {
    /// Returns the currently selected index for the given tab
    pub fn selected_index(&self, tab: Tab) -> Option<usize> {
        match tab {
            Tab::Browse => self.browser.selected(),
            Tab::Queue => self.queue.selected(),
            Tab::Playlist => match self.playlist_view {
                PlaylistView::Detail(_) => self.playlist_track.selected(),
                PlaylistView::List => self.playlist.selected(),
            },
        }
    }

    /// Gets the length of the list for the given tab
    pub fn list_len(&self, tab: Tab, music: &MusicState) -> usize {
        match tab {
            Tab::Browse => music.library_len(),
            Tab::Queue => music.queue_len(),
            Tab::Playlist => match self.playlist_view {
                PlaylistView::Detail(idx) => music.playlist_len(idx),
                PlaylistView::List => music.playlists_len(),
            },
        }
    }

    /// Moves selection up in the appropriate list
    pub fn move_up(
        &mut self,
        tab: Tab,
        toggle_multi_select: bool,
        music: &MusicState,
        in_selector: bool,
    ) {
        // Toggle multi-select if requested in Browse tab
        if toggle_multi_select && tab == Tab::Browse {
            self.toggle_browser_multi_select();
        }

        // Route to appropriate list
        if in_selector {
            let selector_len = music.playlists_len() + 1; // +1 for "Create new"
            Self::navigate_list(&mut self.playlist_selector, selector_len, true);
        } else {
            let list_len = self.list_len(tab, music);
            match tab {
                Tab::Browse => Self::navigate_list(&mut self.browser, list_len, true),
                Tab::Queue => Self::navigate_list(&mut self.queue, list_len, true),
                Tab::Playlist => match self.playlist_view {
                    PlaylistView::Detail(_) => {
                        Self::navigate_list(&mut self.playlist_track, list_len, true)
                    },
                    PlaylistView::List => Self::navigate_list(&mut self.playlist, list_len, true),
                },
            }
        }
    }

    /// Moves selection down in the appropriate list
    pub fn move_down(
        &mut self,
        tab: Tab,
        toggle_multi_select: bool,
        music: &MusicState,
        in_selector: bool,
    ) {
        // Toggle multi-select if requested in Browse tab
        if toggle_multi_select && tab == Tab::Browse {
            self.toggle_browser_multi_select();
        }

        // Route to appropriate list
        if in_selector {
            let selector_len = music.playlists_len() + 1; // +1 for "Create new"
            Self::navigate_list(&mut self.playlist_selector, selector_len, false);
        } else {
            let list_len = self.list_len(tab, music);
            match tab {
                Tab::Browse => Self::navigate_list(&mut self.browser, list_len, false),
                Tab::Queue => Self::navigate_list(&mut self.queue, list_len, false),
                Tab::Playlist => match self.playlist_view {
                    PlaylistView::Detail(_) => {
                        Self::navigate_list(&mut self.playlist_track, list_len, false)
                    },
                    PlaylistView::List => Self::navigate_list(&mut self.playlist, list_len, false),
                },
            }
        }
    }

    /// Toggles an index in the browser multi-selection
    pub fn toggle_browser_multi_select(&mut self) {
        if let Some(current) = self.browser.selected() {
            if self.browser_multi_select.contains(&current) {
                self.browser_multi_select.remove(&current);
            } else {
                self.browser_multi_select.insert(current);
            }
        }
    }

    /// Clears the browser multi-selection
    pub fn clear_browser_multi_select(&mut self) { self.browser_multi_select.clear(); }

    /// Selects all items in the browser
    pub fn select_all_browser(&mut self, total_items: usize) {
        self.browser_multi_select.clear();
        for i in 0..total_items {
            self.browser_multi_select.insert(i);
        }
    }

    /// Returns whether there are any multi-selected items
    pub fn has_multi_selection(&self) -> bool { !self.browser_multi_select.is_empty() }

    /// Returns all selected indices in browser (only multi-select)
    pub fn browser_selected_indices(&self) -> Vec<usize> {
        if self.browser_multi_select.is_empty() {
            Vec::new()
        } else {
            let mut indices: Vec<usize> = self.browser_multi_select.iter().copied().collect();
            indices.sort_unstable();
            indices
        }
    }

    /// Sets the browser selection to a specific index
    pub fn select_browser_item(&mut self, index: Option<usize>) { self.browser.select(index); }

    /// Opens playlist selector modal
    pub fn open_playlist_selector(&mut self) { self.playlist_selector.select(Some(0)); }

    /// Closes playlist selector modal
    pub fn close_playlist_selector(&mut self) { self.playlist_selector.select(None); }

    /// Opens a playlist for viewing its tracks
    pub fn open_playlist(&mut self, index: usize) {
        self.playlist_view = PlaylistView::Detail(index);
        self.playlist_track.select(Some(0));
    }

    /// Closes the playlist view and returns to playlist list
    pub fn close_playlist_view(&mut self) {
        self.playlist_view = PlaylistView::List;
        self.playlist_track.select(None);
    }

    /// Adjusts playlist selection after deletion
    pub fn adjust_playlist_after_delete(&mut self, new_len: usize, deleted_index: usize) {
        if new_len > 0 && deleted_index >= new_len {
            self.playlist.select(Some(new_len - 1));
        }
    }

    /// Adjusts playlist track selection after removal
    pub fn adjust_track_after_remove(&mut self, new_len: usize, removed_index: usize) {
        if new_len == 0 {
            self.playlist_track.select(None);
        } else if removed_index >= new_len {
            self.playlist_track.select(Some(new_len - 1));
        }
    }

    /// Navigate a list state up or down with wraparound
    fn navigate_list(state: &mut ListState, len: usize, is_up: bool) {
        if len == 0 {
            state.select(None);
            return;
        }

        let i = state.selected().unwrap_or(0);
        let new_index = if is_up {
            if i == 0 { len - 1 } else { i - 1 }
        } else if i >= len - 1 {
            0
        } else {
            i + 1
        };
        state.select(Some(new_index));
    }
}
