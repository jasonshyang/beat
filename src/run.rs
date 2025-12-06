use std::{io, path::PathBuf, time::Duration};

use ratatui::{
    Terminal,
    crossterm::{
        event, execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::CrosstermBackend,
};

use crate::{
    app::{Beat, MusicState},
    domain::{AudioPlayer, Library},
    tui::{render, theme::Theme},
};

const INTERVAL: Duration = Duration::from_millis(10);

pub fn run_player(start_dir: Option<PathBuf>) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let theme = Theme::default();
    let library = Library::new(start_dir)?;
    let player = AudioPlayer::run_in_thread()?;
    let (playlists, config_path) = MusicState::load_playlists().unwrap_or_else(|_| {
        eprintln!("Warning: Could not load playlists, starting with empty list");
        MusicState::load_playlists().unwrap()
    });
    let mut beat = Beat::new(player, library, playlists, config_path);

    loop {
        beat.tick();
        terminal.draw(|f| render::render(f, &beat, &theme))?;

        if event::poll(INTERVAL)?
            && let event::Event::Key(key) = event::read()?
        {
            beat.handle_key(key)?;

            if beat.should_quit() {
                break;
            }
        }
    }

    beat.shutdown()?;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
