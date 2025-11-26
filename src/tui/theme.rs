use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    // Player colors
    pub player_icon_playing: Color,
    pub player_icon_stopped: Color,
    pub player_title: Color,
    pub player_progress_filled: Color,
    pub player_progress_empty: Color,
    pub player_time: Color,

    // Browser colors
    pub browser_border_focused: Color,
    pub browser_border_unfocused: Color,
    pub browser_highlight: Color,
    pub browser_folder_icon: Color,
    pub browser_file_icon: Color,
    pub browser_text: Color,

    // Tab colors
    pub tab_active: Color,
    pub tab_inactive: Color,
    pub tab_border: Color,

    // General UI
    pub overlay_border: Color,
    pub background: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            // Player
            player_icon_playing: Color::Rgb(136, 171, 152),
            player_icon_stopped: Color::Rgb(103, 128, 121),
            player_title: Color::Rgb(242, 195, 92),
            player_progress_filled: Color::Rgb(136, 171, 152),
            player_progress_empty: Color::Rgb(60, 60, 60),
            player_time: Color::Rgb(154, 155, 158),

            // Browser
            browser_border_focused: Color::Rgb(136, 171, 152),
            browser_border_unfocused: Color::Rgb(103, 128, 121),
            browser_highlight: Color::Rgb(50, 50, 50),
            browser_folder_icon: Color::Rgb(242, 195, 92),
            browser_file_icon: Color::Rgb(136, 171, 152),
            browser_text: Color::Rgb(242, 195, 92),

            // Tabs
            tab_active: Color::Rgb(242, 195, 92),
            tab_inactive: Color::Rgb(103, 128, 121),
            tab_border: Color::Rgb(103, 128, 121),

            // General
            overlay_border: Color::Rgb(136, 171, 152),
            background: Color::Reset,
        }
    }
}

impl Theme {
    pub fn player_icon_style(&self, is_playing: bool) -> Style {
        Style::default().fg(if is_playing {
            self.player_icon_playing
        } else {
            self.player_icon_stopped
        })
    }

    pub fn player_title_style(&self) -> Style {
        Style::default()
            .fg(self.player_title)
            .add_modifier(Modifier::BOLD)
    }

    pub fn player_progress_filled_style(&self) -> Style {
        Style::default().fg(self.player_progress_filled)
    }

    pub fn player_progress_empty_style(&self) -> Style {
        Style::default().fg(self.player_progress_empty)
    }

    pub fn player_time_style(&self) -> Style { Style::default().fg(self.player_time) }

    pub fn browser_border_style(&self, is_focused: bool) -> Style {
        Style::default().fg(if is_focused {
            self.browser_border_focused
        } else {
            self.browser_border_unfocused
        })
    }

    pub fn browser_highlight_style(&self) -> Style { Style::default().bg(self.browser_highlight) }

    pub fn tab_style(&self, is_active: bool) -> Style {
        Style::default()
            .fg(if is_active { self.tab_active } else { self.tab_inactive })
            .add_modifier(if is_active { Modifier::BOLD } else { Modifier::empty() })
    }

    pub fn tab_border_style(&self) -> Style { Style::default().fg(self.tab_border) }

    pub fn overlay_border_style(&self) -> Style { Style::default().fg(self.overlay_border) }
}
