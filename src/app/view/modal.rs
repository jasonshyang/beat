const RENAME_PLAYLIST_TITLE: &str = "Rename Playlist";
const RENAME_PLAYLIST_PROMPT: &str = "Enter new playlist name...";
const GO_TO_DIR_TITLE: &str = "Go to Directory";
const GO_TO_DIR_PROMPT: &str = "Enter directory path...";

/// Modal state - represents different types of overlays that can be shown
#[derive(Default)]
pub enum ModalState {
    /// No modal is shown
    #[default]
    None,
    /// Text input modal for various purposes
    Input(InputModal),
    /// Playlist selector for adding tracks
    PlaylistSelector,
    /// Full help screen
    Help,
    /// Error notification
    Error(String),
}

/// Mode for what the input modal is being used for
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputModalMode {
    /// Renaming a playlist
    RenamePlaylist,
    /// Navigating to a directory
    GoToDirectory,
}

/// Input modal state for text input
#[derive(Debug, Clone)]
pub struct InputModal {
    /// Title of the modal
    pub title: String,
    /// Prompt text to show when input is empty
    pub prompt: String,
    /// Current input text
    pub input: String,
    /// Mode indicating what action this input is for
    pub mode: InputModalMode,
}

impl ModalState {
    /// Opens an input modal with the given mode
    pub fn open_input(&mut self, mode: InputModalMode) {
        let (title, prompt) = match mode {
            InputModalMode::RenamePlaylist => (RENAME_PLAYLIST_TITLE, RENAME_PLAYLIST_PROMPT),
            InputModalMode::GoToDirectory => (GO_TO_DIR_TITLE, GO_TO_DIR_PROMPT),
        };

        *self = ModalState::Input(InputModal {
            title: title.to_string(),
            prompt: prompt.to_string(),
            input: String::new(),
            mode,
        });
    }

    /// Closes any modal and returns input text if it was an input modal
    pub fn close_input(&mut self) -> Option<(String, InputModalMode)> {
        if let ModalState::Input(modal) = self {
            let input = std::mem::take(&mut modal.input);
            let mode = modal.mode;
            *self = ModalState::None;
            Some((input, mode))
        } else {
            None
        }
    }

    /// Adds a character to the input modal (if currently showing)
    pub fn push_char(&mut self, c: char) {
        if let ModalState::Input(modal) = self {
            modal.input.push(c);
        }
    }

    /// Removes the last character from the input modal (if currently showing)
    pub fn pop_char(&mut self) {
        if let ModalState::Input(modal) = self {
            modal.input.pop();
        }
    }

    /// Opens the playlist selector modal
    pub fn open_playlist_selector(&mut self) { *self = ModalState::PlaylistSelector; }

    /// Closes the playlist selector modal
    pub fn close_playlist_selector(&mut self) {
        if matches!(self, ModalState::PlaylistSelector) {
            *self = ModalState::None;
        }
    }

    /// Toggles the help modal
    pub fn toggle_help(&mut self) {
        if matches!(self, ModalState::Help) {
            *self = ModalState::None;
        } else {
            *self = ModalState::Help;
        }
    }

    /// Sets an error message
    pub fn set_error(&mut self, message: String) { *self = ModalState::Error(message); }

    /// Clears any modal
    pub fn clear(&mut self) { *self = ModalState::None; }

    /// Returns true if any modal is currently shown
    pub fn is_showing(&self) -> bool { !matches!(self, ModalState::None) }

    /// Returns true if showing the playlist selector
    pub fn is_playlist_selector(&self) -> bool { matches!(self, ModalState::PlaylistSelector) }

    /// Returns true if showing an error
    pub fn is_error(&self) -> bool { matches!(self, ModalState::Error(_)) }

    /// Returns the input modal if currently showing
    pub fn input_modal(&self) -> Option<&InputModal> {
        if let ModalState::Input(modal) = self { Some(modal) } else { None }
    }

    /// Returns the error message if currently showing
    pub fn error_message(&self) -> Option<&str> {
        if let ModalState::Error(msg) = self { Some(msg) } else { None }
    }

    /// Returns true if showing help
    pub fn is_help(&self) -> bool { matches!(self, ModalState::Help) }
}
