use ratatui::crossterm::event::KeyEvent;

use super::{
    input::{self, Action, PlaylistAction, PlaylistSelectorAction, QueueAction, TextInputAction},
    music::MusicState,
    player::PlaybackState,
    view::{InputModalMode, PlaylistView, Tab, ViewState},
};
use crate::domain::{AudioHandle, Library, Playlists, Track};

/// Main application state
///
/// Orchestrates between music state, playback state, and view state
pub struct Beat {
    /// Music state (library, queue, playlists)
    pub music: MusicState,
    /// Playback state managing audio playback
    pub playback: PlaybackState,
    /// View state managing UI navigation and tab selection
    pub view: ViewState,

    /// Flag indicating if the application should quit
    should_quit: bool,
}

impl Beat {
    /// Creates a new Beat app
    pub fn new(
        audio: AudioHandle,
        library: Library,
        playlists: Playlists,
        playlists_config_path: std::path::PathBuf,
    ) -> Self {
        Self {
            music: MusicState::new(library, playlists, playlists_config_path),
            playback: PlaybackState::new(audio),
            view: ViewState::default(),
            should_quit: false,
        }
    }

    /// Updates application state on each frame
    pub fn tick(&mut self) {
        // Auto advance queue when track ends
        if self.playback.is_ended()
            && !self.playback.is_paused()
            && let Some(next) = self.music.dequeue()
        {
            let _ = self.playback.start_track(next);
        }
    }

    /// Handles keyboard input and routes to appropriate actions
    pub fn handle_key(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        // Clear error message on any keypress if one is showing
        if self.view.modal.is_error() {
            self.view.clear_error();
            return Ok(());
        }

        let action = input::handle_key(&self.view, key);
        self.execute_action(action)
    }

    /// Checks if the application should quit
    pub fn should_quit(&self) -> bool { self.should_quit }

    /// Shuts down the application, cleaning up resources
    pub fn shutdown(self) -> anyhow::Result<()> { self.playback.shutdown() }

    // ===== Action Dispatchers =====

    /// Executes a given action
    ///
    /// This is the main dispatcher for handling user actions
    /// captured by the input handler.
    fn execute_action(&mut self, action: Action) -> anyhow::Result<()> {
        match action {
            Action::Quit => self.should_quit = true,
            Action::ToggleBrowser => self.view.toggle_browser(),
            Action::SwitchTab(tab) => self.view.switch_tab(tab),
            Action::ToggleHelp => self.view.toggle_help(),
            Action::MoveUp { toggle_multi_select } => {
                self.view.move_up(toggle_multi_select, &self.music)
            },
            Action::MoveDown { toggle_multi_select } => {
                self.view.move_down(toggle_multi_select, &self.music)
            },
            Action::NavigateBack => self.handle_navigate_back(),
            Action::PlayNext => self.play_next()?,
            Action::PlayPause => self.playback.toggle_play_pause()?,
            Action::AddAllToQueue => self.add_all_to_queue()?,
            Action::Enter => self.execute_enter()?,
            Action::GoToDirectory => self.go_to_directory(),
            Action::Playlist(action) => self.handle_playlist_action(action)?,
            Action::PlaylistSelector(action) => self.handle_playlist_selector_action(action)?,
            Action::TextInput(action) => self.handle_text_input_action(action)?,
            Action::Queue(queue_action) => self.handle_queue_action(queue_action)?,
            Action::Noop => {},
        }
        Ok(())
    }

    /// Handles navigation back (clears multi-select or backs from playlist)
    fn handle_navigate_back(&mut self) {
        if self.view.current_tab == Tab::Browse && self.view.has_multi_selection() {
            self.view.clear_browser_multi_select();
        } else {
            self.view.back_from_playlist();
        }
    }

    /// Handles playlist management actions
    fn handle_playlist_action(&mut self, action: PlaylistAction) -> anyhow::Result<()> {
        match action {
            PlaylistAction::Create => self.create_playlist(),
            PlaylistAction::Rename => self.rename_playlist(),
            PlaylistAction::Delete => self.execute_delete(),
            PlaylistAction::RemoveTrack => self.delete_track_from_playlist(),
            PlaylistAction::Play => self.play_selected_playlist(),
        }
    }

    /// Handles playlist selector modal actions
    fn handle_playlist_selector_action(
        &mut self,
        action: PlaylistSelectorAction,
    ) -> anyhow::Result<()> {
        match action {
            PlaylistSelectorAction::Open => {
                self.open_playlist_selector();
                Ok(())
            },
            PlaylistSelectorAction::Confirm => self.confirm_playlist_selection(),
            PlaylistSelectorAction::Close => {
                self.view.close_playlist_selector();
                Ok(())
            },
        }
    }

    /// Handles text input modal actions
    fn handle_text_input_action(&mut self, action: TextInputAction) -> anyhow::Result<()> {
        match action {
            TextInputAction::Char(c) => {
                self.view.input_modal_push_char(c);
                Ok(())
            },
            TextInputAction::Backspace => {
                self.view.input_modal_pop_char();
                Ok(())
            },
            TextInputAction::Confirm => self.confirm_input(),
            TextInputAction::Cancel => {
                self.cancel_input();
                Ok(())
            },
        }
    }

    /// Handles queue management actions
    fn handle_queue_action(&mut self, action: QueueAction) -> anyhow::Result<()> {
        match action {
            QueueAction::Shuffle => {
                self.music.shuffle_queue();
                Ok(())
            },
            QueueAction::Clear => {
                self.music.clear_queue();
                Ok(())
            },
        }
    }

    // ===== Playback Actions =====
    fn play_next(&mut self) -> anyhow::Result<()> {
        if let Some(next) = self.music.dequeue()
            && let Err(e) = self.playback.start_track(next)
        {
            self.view.set_error(format!("Cannot play track: {}", e));
        }
        Ok(())
    }

    /// Adds all audio files from current directory to queue and starts playing
    fn add_all_to_queue(&mut self) -> anyhow::Result<()> {
        if self.music.add_all_to_queue() {
            self.play_next()?;
        }
        Ok(())
    }

    // ===== Enter Key Handlers =====

    /// Handles Enter key - routes to appropriate action based on context
    fn execute_enter(&mut self) -> anyhow::Result<()> {
        match self.view.current_tab {
            Tab::Browse => self.enter_browse()?,
            Tab::Queue => self.enter_queue()?,
            Tab::Playlist => self.enter_playlist()?,
        }
        Ok(())
    }

    /// Handles Enter in Browse tab
    fn enter_browse(&mut self) -> anyhow::Result<()> {
        let selected_indices = self.view.browser_selected_indices();

        // Handle multi-select
        if !selected_indices.is_empty() {
            self.music.add_tracks_to_queue(&selected_indices);
            self.view.clear_browser_multi_select();
            return Ok(());
        }

        // Handle single selection
        let Some(i) = self.view.selected_index() else {
            return Ok(());
        };
        let Some(entry) = self.music.library.get_entry(i) else {
            return Ok(());
        };

        match entry.is_dir() {
            true => {
                let path = entry.path.clone();
                match self.music.change_directory(path) {
                    Ok(()) => {
                        self.view.select_browser_item(Some(0));
                        self.view.clear_browser_multi_select();
                    },
                    Err(e) => {
                        self.view.set_error(format!("Cannot open directory: {}", e));
                    },
                }
            },
            false if entry.is_audio() => {
                let track = Track::new(&entry.name, &entry.path);
                self.music.add_track_to_queue(track);
            },
            _ => {},
        }
        Ok(())
    }

    /// Handles Enter in Queue tab
    fn enter_queue(&mut self) -> anyhow::Result<()> {
        let Some(i) = self.view.selected_index() else {
            return Ok(());
        };

        // Skip to this track (removes tracks before it)
        if let Some(track) = self.music.skip_to_track(i) {
            self.playback.start_track(track)?;
        }

        Ok(())
    }

    /// Handles Enter in Playlist tab
    fn enter_playlist(&mut self) -> anyhow::Result<()> {
        match self.view.selection.playlist_view {
            PlaylistView::Detail(playlist_idx) => {
                // Inside playlist: play track
                let Some(track_idx) = self.view.selected_index() else {
                    return Ok(());
                };
                let Some(track) = self.music.get_playlist_track(playlist_idx, track_idx) else {
                    return Ok(());
                };

                if let Err(e) = self.playback.start_track(track.clone()) {
                    self.view.set_error(format!("Cannot play track: {}", e));
                }
            },
            PlaylistView::List => {
                // Playlist list: open playlist
                if let Some(i) = self.view.selected_index() {
                    self.view.open_playlist(i);
                }
            },
        }
        Ok(())
    }

    // ===== Playlist Management =====

    /// Creates a new playlist
    fn create_playlist(&mut self) -> anyhow::Result<()> {
        // Only allow creating playlist when on Playlist tab and not viewing a playlist
        if self.view.current_tab != Tab::Playlist
            || matches!(self.view.selection.playlist_view, PlaylistView::Detail(_))
        {
            return Ok(());
        }

        let count = self.music.playlists_len() + 1;
        let name = format!("Playlist {}", count);

        if let Err(e) = self.music.create_playlist(name) {
            self.view
                .set_error(format!("Failed to create playlist: {}", e));
        }

        Ok(())
    }

    /// Renames the selected playlist
    fn rename_playlist(&mut self) -> anyhow::Result<()> {
        // Only allow renaming when on Playlist tab and not viewing a playlist
        if self.view.current_tab != Tab::Playlist
            || matches!(self.view.selection.playlist_view, PlaylistView::Detail(_))
        {
            return Ok(());
        }

        if self.view.selection.playlist.selected().is_some() {
            self.view.open_input_modal(InputModalMode::RenamePlaylist);
        }

        Ok(())
    }

    /// Deletes the selected item based on context (playlist or track)
    fn execute_delete(&mut self) -> anyhow::Result<()> {
        // Only allow deletion in Playlist tab
        if self.view.current_tab != Tab::Playlist {
            return Ok(());
        }

        match self.view.selection.playlist_view {
            PlaylistView::Detail(_) => self.delete_track_from_playlist(),
            PlaylistView::List => self.delete_playlist(),
        }
    }

    /// Deletes the selected playlist
    fn delete_playlist(&mut self) -> anyhow::Result<()> {
        if let Some(index) = self.view.selected_index() {
            self.music.delete_playlist(index)?;
            let new_len = self.music.playlists_len();
            self.view
                .adjust_playlist_selection_after_delete(new_len, index);
        }
        Ok(())
    }

    /// Deletes a track from the currently viewed playlist
    fn delete_track_from_playlist(&mut self) -> anyhow::Result<()> {
        let PlaylistView::Detail(playlist_idx) = self.view.selection.playlist_view else {
            return Ok(());
        };
        let Some(track_idx) = self.view.selected_index() else {
            return Ok(());
        };

        match self
            .music
            .remove_track_from_playlist(playlist_idx, track_idx)
        {
            Ok(()) => {
                let new_len = self.music.playlist_len(playlist_idx);
                self.view
                    .adjust_track_selection_after_remove(new_len, track_idx);
            },
            Err(e) => {
                self.view
                    .set_error(format!("Failed to remove track: {}", e));
            },
        }

        Ok(())
    }

    // ===== Playlist Selector Modal =====

    /// Opens the playlist selector modal
    fn open_playlist_selector(&mut self) {
        if self.view.current_tab != Tab::Browse {
            return;
        }

        let selected_indices = self.view.browser_selected_indices();
        let has_audio = if !selected_indices.is_empty() {
            !self.music.collect_tracks(&selected_indices).is_empty()
        } else if let Some(i) = self.view.selected_index() {
            self.music.is_audio_at(i)
        } else {
            false
        };

        if has_audio {
            self.view.open_playlist_selector();
        }
    }

    /// Confirms playlist selection and adds tracks
    fn confirm_playlist_selection(&mut self) -> anyhow::Result<()> {
        let Some(selected_idx) = self.view.selection.playlist_selector.selected() else {
            return Ok(());
        };

        let playlist_count = self.music.playlists_len();

        // Collect tracks to add
        let tracks_to_add = {
            let selected_indices = self.view.browser_selected_indices();

            if !selected_indices.is_empty() {
                self.music.collect_tracks(&selected_indices)
            } else if let Some(browser_idx) = self.view.selection.browser.selected() {
                self.music.collect_tracks(&[browser_idx])
            } else {
                Vec::new()
            }
        };

        if tracks_to_add.is_empty() {
            self.view.close_playlist_selector();
            return Ok(());
        }

        // Add tracks to playlist
        let result = match selected_idx {
            idx if idx == playlist_count => {
                // Create new playlist
                self.music
                    .create_playlist("My Playlist".to_string())
                    .and_then(|_| {
                        self.music
                            .add_tracks_to_playlist(playlist_count, tracks_to_add)
                    })
            },
            idx if idx < playlist_count => {
                // Add to existing playlist
                self.music.add_tracks_to_playlist(idx, tracks_to_add)
            },
            _ => Ok(()),
        };

        if let Err(e) = result {
            self.view
                .set_error(format!("Failed to add tracks to playlist: {}", e));
        }

        self.view.clear_browser_multi_select();
        self.view.close_playlist_selector();
        Ok(())
    }

    // ===== Text Input Modal =====

    /// Confirms input modal and processes the input
    fn confirm_input(&mut self) -> anyhow::Result<()> {
        let Some((input, mode)) = self.view.close_input_modal() else {
            return Ok(());
        };

        if input.trim().is_empty() {
            return Ok(());
        }

        let result = match mode {
            InputModalMode::RenamePlaylist => {
                let Some(index) = self.view.selection.playlist.selected() else {
                    return Ok(());
                };
                self.music.rename_playlist(index, input.trim().to_string())
            },
            InputModalMode::GoToDirectory => {
                // Expand tilde
                let expanded = shellexpand::tilde(input.trim());
                let path = std::path::PathBuf::from(expanded.as_ref());

                match self.music.change_directory(path) {
                    Ok(()) => {
                        self.view.select_browser_item(Some(0));
                        Ok(())
                    },
                    Err(e) => Err(e),
                }
            },
        };

        if let Err(e) = result {
            self.view.set_error(format!("Error: {}", e));
        }

        Ok(())
    }

    /// Cancels input modal without processing
    fn cancel_input(&mut self) {
        // Silently drop input
        let _ = self.view.close_input_modal();
    }

    /// Plays the currently selected playlist
    fn play_selected_playlist(&mut self) -> anyhow::Result<()> {
        // Only allow playing from the playlist list view
        if self.view.current_tab != Tab::Playlist {
            return Ok(());
        }

        let Some(playlist_idx) = self.view.selected_index() else {
            return Ok(());
        };

        if self.music.play_playlist(playlist_idx) {
            self.play_next()?;
        }

        Ok(())
    }

    /// Opens input modal for directory navigation
    fn go_to_directory(&mut self) {
        if self.view.current_tab != Tab::Browse {
            return;
        }

        self.view.open_input_modal(InputModalMode::GoToDirectory);
    }
}
