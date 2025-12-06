use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear},
};

use crate::{
    app::{Beat, PlaylistView, Tab},
    tui::{theme::Theme, widgets},
};

/// Renders the entire TUI based on the current application state
pub fn render(frame: &mut Frame, app: &Beat, theme: &Theme) {
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
        widgets::render_tabs(frame, main_chunks[0], app.view.current_tab, theme);

        // Render current tab content
        match app.view.current_tab {
            Tab::Browse => widgets::render_browser(
                frame,
                main_chunks[1],
                app.music.library.entries(),
                &app.view.selection.browser,
                &app.view.selection.browser_multi_select,
                true,
                theme,
            ),
            Tab::Queue => widgets::render_queue(
                frame,
                main_chunks[1],
                &app.music.queue,
                &app.view.selection.queue,
                theme,
            ),
            Tab::Playlist => match app.view.selection.playlist_view {
                PlaylistView::Detail(playlist_index) => {
                    if let Some(playlist) = app.music.playlists.get(playlist_index) {
                        widgets::render_playlist_detail(
                            frame,
                            main_chunks[1],
                            playlist,
                            &app.view.selection.playlist_track,
                            theme,
                        );
                    }
                },
                PlaylistView::List => {
                    widgets::render_playlists(
                        frame,
                        main_chunks[1],
                        &app.music.playlists,
                        &app.view.selection.playlist,
                        theme,
                    );
                },
            },
        }

        // Render player
        widgets::render_player(frame, main_chunks[2], &app.playback, theme);
    } else {
        let mini_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Help text
                Constraint::Length(2), // Player (2 lines: title + progress)
            ])
            .split(inner);

        widgets::render_mini_help(frame, mini_chunks[0], theme);
        widgets::render_player(frame, mini_chunks[1], &app.playback, theme);
    }

    // Render playlist selector overlay if shown
    if app.view.modal.is_playlist_selector() {
        let selector_area = centered_rect(60, 50, frame.area());
        frame.render_widget(Clear, selector_area);
        widgets::playlist::render_playlist_selector(
            frame,
            selector_area,
            &app.music.playlists,
            &app.view.selection.playlist_selector,
            theme,
        );
    }

    // Render input modal overlay if shown
    if let Some(modal) = app.view.modal.input_modal() {
        let input_area = centered_rect(50, 20, frame.area());
        frame.render_widget(Clear, input_area);
        widgets::input_modal::render_input_modal(
            frame,
            input_area,
            &modal.title,
            &modal.prompt,
            &modal.input,
            theme,
        );
    }

    // Render error notification if there's an error
    if let Some(error) = app.view.modal.error_message() {
        let error_area = centered_rect(60, 25, frame.area());
        frame.render_widget(Clear, error_area);
        widgets::error_notification::render_error(frame, error_area, error, theme);
    }

    // Render full help modal if shown
    if app.view.modal.is_help() {
        let help_area = centered_rect(70, 60, frame.area());
        frame.render_widget(Clear, help_area);
        widgets::help::render_full_help(frame, help_area, theme);
    }
}

fn bottom_overlay(area: Rect, height_lines: u16) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(height_lines)])
        .split(area);

    chunks[1]
}

/// Creates a centered rectangle for modal overlays
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
