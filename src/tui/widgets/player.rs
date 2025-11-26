use std::time::Duration;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{app::PlayerState, tui::theme::Theme};

pub fn render_player(frame: &mut Frame, area: Rect, state: &PlayerState, theme: &Theme) {
    // Split into two rows: title and progress
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(area);

    let title_area = chunks[0];
    let progress_area = chunks[1];

    let status_icon = if state.is_paused() {
        " ⏸  "
    } else if state.is_playing() {
        " ▶  "
    } else {
        " ⏹  "
    };

    // Build title line with track info
    let title_text = if let Some(track) = &state.current_track {
        let mut parts = vec![format!("[♫] {}", track.name)];

        // Add artist if available
        if let Some(artist) = &track.artist {
            parts.push(format!(" • {}", artist));
        }

        // Add album if available
        if let Some(album) = &track.album {
            parts.push(format!(" • {}", album));
        }

        parts.join("")
    } else {
        format!("[♫] No Track")
    };

    let title_line = Line::from(Span::styled(
        title_text,
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ));
    let title_widget = Paragraph::new(title_line);
    frame.render_widget(title_widget, title_area);

    // Build progress bar line with status icon and time
    let mut progress_spans = vec![Span::raw(status_icon)];

    let available_width = progress_area.width as usize;
    let status_len = status_icon.len();

    let (time_text, ratio) = if let Some((elapsed, total)) = state.current_track_progress() {
        let text = format!(
            " {} / {} ",
            format_duration(elapsed),
            format_duration(total)
        );
        let r = (elapsed.as_secs_f64() / total.as_secs_f64()).clamp(0.0, 1.0);
        (text, r)
    } else {
        (" -:- / -:- ".to_string(), 0.0)
    };

    let time_len = time_text.len();
    let progress_width = available_width.saturating_sub(status_len + time_len);

    // Progress bar
    let filled = (ratio * progress_width as f64) as usize;
    let empty = progress_width.saturating_sub(filled);

    progress_spans.push(Span::styled("█".repeat(filled), theme.player_progress_filled_style()));
    progress_spans.push(Span::styled("░".repeat(empty), theme.player_progress_empty_style()));
    progress_spans.push(Span::styled(time_text, theme.player_time_style()));

    let progress_line = Line::from(progress_spans);
    let progress_widget = Paragraph::new(progress_line);
    frame.render_widget(progress_widget, progress_area);
}

fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let minutes = total_secs / 60;
    let seconds = total_secs % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
