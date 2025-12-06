use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::view::{Tab, ViewState};

/// Actions that can be performed in the application
#[derive(Debug, Clone)]
pub enum Action {
    // Application control
    Quit,

    // View actions
    ToggleBrowser,
    SwitchTab(Tab),
    ToggleHelp,

    // Navigation
    MoveUp { toggle_multi_select: bool },
    MoveDown { toggle_multi_select: bool },
    NavigateBack,

    // Playback control
    PlayNext,
    PlayPause,

    // Content actions
    AddAllToQueue,
    Enter,
    GoToDirectory,

    // Playlist management
    Playlist(PlaylistAction),

    // Queue management
    Queue(QueueAction),

    // Playlist selector modal
    PlaylistSelector(PlaylistSelectorAction),

    // Text input modal
    TextInput(TextInputAction),

    // No-op
    Noop,
}

/// Playlist management actions
#[derive(Debug, Clone)]
pub enum PlaylistAction {
    Create,
    Rename,
    Delete,
    RemoveTrack,
    Play,
}

/// Queue management actions
#[derive(Debug, Clone)]
pub enum QueueAction {
    Shuffle,
    Clear,
}

/// Playlist selector modal actions
#[derive(Debug, Clone)]
pub enum PlaylistSelectorAction {
    Open,
    Confirm,
    Close,
}

/// Text input modal actions
#[derive(Debug, Clone)]
pub enum TextInputAction {
    Char(char),
    Backspace,
    Confirm,
    Cancel,
}

/// Input modes for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Normal browsing and navigation mode
    Normal,
    /// Playlist selector modal is open
    PlaylistSelector,
    /// Text input modal is open
    TextInput,
}

/// Translates keyboard input to an action based on current state
pub fn handle_key(view: &ViewState, key: KeyEvent) -> Action {
    let mode = current_mode(view);

    match mode {
        InputMode::Normal => handle_normal_mode(view, key),
        InputMode::PlaylistSelector => handle_playlist_selector_mode(key),
        InputMode::TextInput => handle_text_input_mode(key),
    }
}

/// Determines the current input mode based on view state
fn current_mode(view: &ViewState) -> InputMode {
    if view.modal.input_modal().is_some() {
        InputMode::TextInput
    } else if view.modal.is_playlist_selector() {
        InputMode::PlaylistSelector
    } else {
        InputMode::Normal
    }
}

/// Handles input in normal mode
fn handle_normal_mode(view: &ViewState, key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('Q') => Action::Quit,
        KeyCode::Char('b') => Action::ToggleBrowser,
        KeyCode::Char('1') => Action::SwitchTab(Tab::Browse),
        KeyCode::Char('2') => Action::SwitchTab(Tab::Queue),
        KeyCode::Char('3') => Action::SwitchTab(Tab::Playlist),
        KeyCode::Char('a') => Action::AddAllToQueue,
        KeyCode::Char('n') => Action::PlayNext,
        KeyCode::Char('c') => handle_c_key(view),
        KeyCode::Char('r') => Action::Playlist(PlaylistAction::Rename),
        KeyCode::Char('d') => Action::Playlist(PlaylistAction::Delete),
        KeyCode::Char('x') => Action::Playlist(PlaylistAction::RemoveTrack),
        KeyCode::Char('s') => handle_s_key(view),
        KeyCode::Char('g') => handle_g_key(view),
        KeyCode::Backspace | KeyCode::Esc => handle_escape(view),
        KeyCode::Char('p') => handle_p_key(view),
        KeyCode::Char(' ') => Action::PlayPause,
        KeyCode::Char('?') => Action::ToggleHelp,
        KeyCode::Up | KeyCode::Char('k') => {
            Action::MoveUp { toggle_multi_select: key.modifiers.contains(KeyModifiers::SHIFT) }
        },
        KeyCode::Down | KeyCode::Char('j') => {
            Action::MoveDown { toggle_multi_select: key.modifiers.contains(KeyModifiers::SHIFT) }
        },
        KeyCode::Enter => Action::Enter,
        _ => Action::Noop,
    }
}

/// Handles input in playlist selector mode
fn handle_playlist_selector_mode(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc => Action::PlaylistSelector(PlaylistSelectorAction::Close),
        KeyCode::Up | KeyCode::Char('k') => Action::MoveUp { toggle_multi_select: false },
        KeyCode::Down | KeyCode::Char('j') => Action::MoveDown { toggle_multi_select: false },
        KeyCode::Enter => Action::PlaylistSelector(PlaylistSelectorAction::Confirm),
        _ => Action::Noop,
    }
}

/// Handles input in text input mode
fn handle_text_input_mode(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Enter => Action::TextInput(TextInputAction::Confirm),
        KeyCode::Esc => Action::TextInput(TextInputAction::Cancel),
        KeyCode::Backspace => Action::TextInput(TextInputAction::Backspace),
        KeyCode::Char(c) => Action::TextInput(TextInputAction::Char(c)),
        _ => Action::Noop,
    }
}

/// Handles Escape key - returns appropriate action
fn handle_escape(_view: &ViewState) -> Action {
    // NavigateBack handles both clearing multi-select and going back from playlist
    Action::NavigateBack
}

/// Handles 'p' key - returns appropriate action
fn handle_p_key(view: &ViewState) -> Action {
    match view.current_tab {
        Tab::Browse => Action::PlaylistSelector(PlaylistSelectorAction::Open),
        Tab::Playlist => Action::Playlist(PlaylistAction::Play),
        _ => Action::PlayPause,
    }
}

/// Handles 'g' key - returns appropriate action
fn handle_g_key(view: &ViewState) -> Action {
    if view.current_tab == Tab::Browse { Action::GoToDirectory } else { Action::Noop }
}

/// Handles 's' key - returns appropriate action
fn handle_s_key(view: &ViewState) -> Action {
    if view.current_tab == Tab::Queue { Action::Queue(QueueAction::Shuffle) } else { Action::Noop }
}

/// Handles 'c' key - returns appropriate action
fn handle_c_key(view: &ViewState) -> Action {
    match view.current_tab {
        Tab::Queue => Action::Queue(QueueAction::Clear),
        Tab::Playlist => Action::Playlist(PlaylistAction::Create),
        _ => Action::Noop,
    }
}
