use ratatui::widgets::ListState;

/// Available tabs in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    /// Browse music library
    #[default]
    Browse,
    /// View play queue
    Queue,
}

/// View state management - tracks current tab and navigation selections
///
/// Manages which tab is active, browser visibility, and selection state
/// for both the library browser and queue list.
pub struct ViewState {
    /// Currently active tab
    pub current_tab: Tab,
    /// Whether the full browser is shown (vs minimalist player-only mode)
    pub show_browser: bool,
    /// Selection state in the library browser
    pub browser_selection: ListState,
    /// Selection state in the queue list
    pub queue_selection: ListState,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            current_tab: Tab::Browse,
            show_browser: false,
            browser_selection: ListState::default(),
            queue_selection: ListState::default(),
        }
    }
}

impl ViewState {
    /// Toggles browser visibility between full view and minimalist player mode
    pub fn toggle_browser(&mut self) { self.show_browser = !self.show_browser; }

    /// Switches to the specified tab
    pub fn switch_tab(&mut self, tab: Tab) { self.current_tab = tab; }

    /// Moves selection up in the current tab's list
    ///
    /// Wraps around to the bottom when at the top.
    pub fn move_up(&mut self, list_len: usize) {
        match self.current_tab {
            Tab::Browse => Self::navigate_list(&mut self.browser_selection, list_len, true),
            Tab::Queue => Self::navigate_list(&mut self.queue_selection, list_len, true),
        }
    }

    /// Moves selection down in the current tab's list
    ///
    /// Wraps around to the top when at the bottom.
    pub fn move_down(&mut self, list_len: usize) {
        match self.current_tab {
            Tab::Browse => Self::navigate_list(&mut self.browser_selection, list_len, false),
            Tab::Queue => Self::navigate_list(&mut self.queue_selection, list_len, false),
        }
    }

    /// Returns the currently selected index in the active tab's list
    pub fn selected_index(&self) -> Option<usize> {
        match self.current_tab {
            Tab::Browse => self.browser_selection.selected(),
            Tab::Queue => self.queue_selection.selected(),
        }
    }

    /// Sets the selection in the browser list
    pub fn select_browser_item(&mut self, index: Option<usize>) {
        self.browser_selection.select(index);
    }

    /// Navigate a list state up or down with wraparound
    ///
    /// # Arguments
    /// * `state` - The list state to modify
    /// * `len` - Total number of items in the list
    /// * `is_up` - True to move up, false to move down
    pub fn navigate_list(state: &mut ListState, len: usize, is_up: bool) {
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
