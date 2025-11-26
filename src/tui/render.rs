use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear},
};

use crate::{
    app::{Beat, Tab},
    tui::{theme::Theme, widgets},
};

pub fn render(frame: &mut Frame, app: &Beat) {
    let theme = Theme::default();

    // Create overlay at bottom of screen
    let overlay = if app.view.show_browser {
        // When browser is shown, take up more space (50% of screen)
        bottom_overlay(frame.area(), 50)
    } else {
        // Minimalist mode at the bottom (help + player + borders)
        bottom_overlay(frame.area(), 5)
    };

    // Clear the background for the overlay effect
    frame.render_widget(Clear, overlay);

    // Outer border
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.overlay_border_style())
        .title("♫ beat");
    frame.render_widget(&outer_block, overlay);

    let inner = outer_block.inner(overlay);

    if app.view.show_browser {
        // Full layout with tabs and content
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Tab bar
                Constraint::Min(0),    // Content area
                Constraint::Length(2), // Player (2 lines: title + progress)
            ])
            .split(inner);

        // Render tab bar
        widgets::render_tabs(frame, main_chunks[0], app.view.current_tab, &theme);

        // Render current tab content
        match app.view.current_tab {
            Tab::Browse => widgets::render_browser(
                frame,
                main_chunks[1],
                app.library.entries(),
                &app.view.browser_selection,
                true,
                &theme,
            ),
            Tab::Queue => widgets::render_queue(frame, main_chunks[1], &app.queue, &theme),
        }

        // Render player
        widgets::render_player(frame, main_chunks[2], &app.player, &theme);
    } else {
        let mini_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Help text
                Constraint::Length(2), // Player (2 lines: title + progress)
            ])
            .split(inner);

        widgets::render_mini_help(frame, mini_chunks[0], &theme);
        widgets::render_player(frame, mini_chunks[1], &app.player, &theme);
    }
}

fn bottom_overlay(area: Rect, height_lines: u16) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(height_lines)])
        .split(area);

    chunks[1]
}
