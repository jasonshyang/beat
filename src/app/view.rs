mod modal;
mod selection;

pub use modal::{InputModalMode, ModalState};
pub use selection::{PlaylistView, SelectionState};

use super::music::MusicState;

/// Available tabs in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    /// Browse music library
    #[default]
    Browse,
    /// View play queue
    Queue,
    /// Playlist
    Playlist,
}

/// View state management - tracks current tab and navigation selections
///
/// Manages which tab is active, browser visibility, selection state,
/// and modal overlays.
pub struct ViewState {
    /// Currently active tab
    pub current_tab: Tab,
    /// Whether the full browser is shown (vs minimalist player-only mode)
    pub show_browser: bool,
    /// Selection state for all lists
    pub selection: SelectionState,
    /// Modal state (input, playlist selector, help, error)
    pub modal: ModalState,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            current_tab: Tab::Browse,
            show_browser: false,
            selection: SelectionState::default(),
            modal: ModalState::default(),
        }
    }
}

impl ViewState {
    /// Toggles browser visibility between full view and minimalist player mode
    pub fn toggle_browser(&mut self) { self.show_browser = !self.show_browser; }

    /// Switches to the specified tab
    pub fn switch_tab(&mut self, tab: Tab) {
        // Clear multi-selection when switching tabs
        if self.current_tab != tab {
            self.selection.clear_browser_multi_select();
        }
        self.current_tab = tab;
    }

    /// Moves selection up
    pub fn move_up(&mut self, toggle_multi_select: bool, music: &MusicState) {
        self.selection.move_up(
            self.current_tab,
            toggle_multi_select,
            music,
            self.modal.is_playlist_selector(),
        );
    }

    /// Moves selection down
    pub fn move_down(&mut self, toggle_multi_select: bool, music: &MusicState) {
        self.selection.move_down(
            self.current_tab,
            toggle_multi_select,
            music,
            self.modal.is_playlist_selector(),
        );
    }

    /// Returns the currently selected index in the active tab's list
    pub fn selected_index(&self) -> Option<usize> {
        self.selection.selected_index(self.current_tab)
    }

    /// Sets the selection in the browser list
    pub fn select_browser_item(&mut self, index: Option<usize>) {
        self.selection.select_browser_item(index);
    }

    /// Clears the browser multi-selection
    pub fn clear_browser_multi_select(&mut self) { self.selection.clear_browser_multi_select(); }

    /// Selects all items in the browser
    pub fn select_all_browser(&mut self, music: &MusicState) {
        let total_items = music.library_len();
        self.selection.select_all_browser(total_items);
    }

    /// Returns whether there are any multi-selected items
    pub fn has_multi_selection(&self) -> bool { self.selection.has_multi_selection() }

    /// Returns all selected indices in browser (multi-select)
    pub fn browser_selected_indices(&self) -> Vec<usize> {
        self.selection.browser_selected_indices()
    }

    /// Opens playlist selector modal
    pub fn open_playlist_selector(&mut self) {
        self.selection.open_playlist_selector();
        self.modal.open_playlist_selector();
    }

    /// Closes playlist selector modal
    pub fn close_playlist_selector(&mut self) {
        self.selection.close_playlist_selector();
        self.modal.close_playlist_selector();
    }

    /// Opens input modal with the given mode
    pub fn open_input_modal(&mut self, mode: InputModalMode) { self.modal.open_input(mode); }

    /// Closes input modal and returns the input text if confirmed
    pub fn close_input_modal(&mut self) -> Option<(String, InputModalMode)> {
        self.modal.close_input()
    }

    /// Adds a character to the input modal
    pub fn input_modal_push_char(&mut self, c: char) { self.modal.push_char(c); }

    /// Removes the last character from the input modal
    pub fn input_modal_pop_char(&mut self) { self.modal.pop_char(); }

    /// Sets an error message to display
    pub fn set_error(&mut self, message: String) { self.modal.set_error(message); }

    /// Clears the error message
    pub fn clear_error(&mut self) { self.modal.clear(); }

    /// Toggles the help modal
    pub fn toggle_help(&mut self) { self.modal.toggle_help(); }

    /// Opens a playlist for viewing its tracks
    pub fn open_playlist(&mut self, index: usize) { self.selection.open_playlist(index); }

    /// Closes the playlist view and returns to playlist list
    pub fn close_playlist_view(&mut self) { self.selection.close_playlist_view(); }

    /// Goes back from playlist detail view if currently viewing one
    pub fn back_from_playlist(&mut self) {
        if self.current_tab == Tab::Playlist
            && matches!(self.selection.playlist_view, PlaylistView::Detail(_))
        {
            self.close_playlist_view();
        }
    }

    /// Adjusts playlist selection after deletion
    pub fn adjust_playlist_selection_after_delete(&mut self, new_len: usize, deleted_index: usize) {
        self.selection
            .adjust_playlist_after_delete(new_len, deleted_index);
    }

    /// Adjusts playlist track selection after removal
    pub fn adjust_track_selection_after_remove(&mut self, new_len: usize, removed_index: usize) {
        self.selection
            .adjust_track_after_remove(new_len, removed_index);
    }
}
