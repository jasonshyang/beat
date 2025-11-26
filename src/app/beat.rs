use ratatui::crossterm::event::{KeyCode, KeyEvent};

use super::{
    player::PlayerState,
    view::{Tab, ViewState},
};
use crate::domain::{AudioHandle, Library, PlayQueue, Track};

/// Main application state
///
/// Coordinates between domain and app state
pub struct Beat {
    /// Music library for browsing tracks
    pub library: Library,
    /// Queue of tracks to be played
    pub queue: PlayQueue,
    /// Player state managing playback and audio
    pub player: PlayerState,
    /// View state managing UI navigation and tab selection
    pub view: ViewState,

    /// Flag indicating if the application should quit
    should_quit: bool,
}

impl Beat {
    /// Creates a new Beat application state
    pub fn new(audio: AudioHandle, library: Library) -> Self {
        Self {
            library,
            queue: PlayQueue::default(),
            player: PlayerState::new(audio),
            view: ViewState::default(),
            should_quit: false,
        }
    }

    /// Updates application state on each frame
    ///
    /// Handles automatic track advancement when current track ends
    pub fn tick(&mut self) {
        // Auto-advance queue when track ends
        if self.player.is_ended()
            && !self.player.is_paused()
            && let Some(next) = self.queue.dequeue()
        {
            let _ = self.player.start_track(next);
        }
    }

    /// Handles keyboard input and routes to appropriate actions
    pub fn handle_key(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        match key.code {
            KeyCode::Char('Q') => self.should_quit = true,
            KeyCode::Char('b') => self.view.toggle_browser(),
            KeyCode::Char('1') => self.view.switch_tab(Tab::Browse),
            KeyCode::Char('2') => self.view.switch_tab(Tab::Queue),
            KeyCode::Char('a') => self.add_all_to_queue()?,
            KeyCode::Char('n') => self.play_next()?,
            KeyCode::Char('p') => {
                if self.player.is_playing() {
                    self.player.pause()?
                } else {
                    self.player.resume()?
                }
            },
            KeyCode::Up | KeyCode::Char('k') => self.move_up(),
            KeyCode::Down | KeyCode::Char('j') => self.move_down(),
            KeyCode::Enter => self.handle_enter()?,
            _ => {},
        }
        Ok(())
    }

    pub fn should_quit(&self) -> bool { self.should_quit }

    pub fn shutdown(self) -> anyhow::Result<()> { self.player.shutdown() }

    /// Plays the next track from the queue
    fn play_next(&mut self) -> anyhow::Result<()> {
        if let Some(next) = self.queue.dequeue() {
            self.player.start_track(next)?;
        }
        Ok(())
    }

    /// Moves the selection up in the current list (library or queue)
    fn move_up(&mut self) {
        let len = match self.view.current_tab {
            Tab::Browse => self.library.len(),
            Tab::Queue => self.queue.len(),
        };
        self.view.move_up(len);
    }

    /// Moves the selection down in the current list (library or queue)
    fn move_down(&mut self) {
        let len = match self.view.current_tab {
            Tab::Browse => self.library.len(),
            Tab::Queue => self.queue.len(),
        };
        self.view.move_down(len);
    }

    /// Handles Enter key action based on current context
    ///
    /// - In Browse tab: Navigate into directory or add track to queue
    /// - In Queue tab: Start playing selected track
    fn handle_enter(&mut self) -> anyhow::Result<()> {
        match self.view.current_tab {
            Tab::Browse => {
                if let Some(i) = self.view.selected_index()
                    && let Some(entry) = self.library.entries().get(i)
                {
                    if entry.is_dir() {
                        self.library.change_dir(entry.path.clone())?;
                        self.view.select_browser_item(Some(0));
                    } else {
                        let music = Track::new(&entry.name, &entry.path);
                        self.queue.push(music);
                    }
                }
            },
            Tab::Queue => {
                if let Some(i) = self.view.selected_index()
                    && let Some(music) = self.queue.get(i)
                {
                    self.player.start_track(music.clone())?;
                }
            },
        }
        Ok(())
    }

    /// Adds all audio files from current directory to queue and starts playing
    fn add_all_to_queue(&mut self) -> anyhow::Result<()> {
        let audios: Vec<_> = self.library.audios().collect();

        if audios.is_empty() {
            return Ok(());
        }

        for entry in audios {
            let music = Track::new(&entry.name, &entry.path);
            self.queue.push(music);
        }

        // Start playing first track in queue
        if let Some(first) = self.queue.dequeue() {
            self.player.start_track(first)?;
        }

        Ok(())
    }
}
