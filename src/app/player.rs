use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use crate::domain::{AudioHandle, Track};

/// Playback state management
///
/// Manages current track, playback timing, pause/resume state, and audio
/// control.
pub struct PlaybackState {
    /// Currently playing track, if any
    pub current_track: Option<Track>,
    /// Timestamp when the current track started playing
    pub current_track_started_at: Option<Instant>,
    /// Timestamp when playback was paused, if currently paused
    pub paused_at: Option<Instant>,
    /// History of previously played tracks
    pub play_history: VecDeque<Track>,

    /// Audio handle for controlling playback
    audio: AudioHandle,
}

impl PlaybackState {
    /// Creates a new PlayerState with the given audio handle
    pub fn new(audio: AudioHandle) -> Self {
        Self {
            current_track: None,
            current_track_started_at: None,
            paused_at: None,
            play_history: VecDeque::new(),
            audio,
        }
    }

    /// Calculate current playback progress
    ///
    /// Returns (elapsed, total) duration. When paused, elapsed time is frozen
    /// at the pause point. Returns None if no track is playing.
    pub fn current_track_progress(&self) -> Option<(Duration, Duration)> {
        let started_at = self.current_track_started_at?;
        let track = self.current_track.as_ref()?;
        let duration = track.duration?;

        let elapsed = if let Some(paused_at) = self.paused_at {
            // If paused, use the time when we paused
            paused_at.duration_since(started_at)
        } else {
            // If playing, use current time
            started_at.elapsed()
        };
        Some((elapsed.min(duration), duration))
    }

    /// Starts playing a new track
    ///
    /// Resets timing state and begins audio playback.
    pub fn start_track(&mut self, track: Track) -> anyhow::Result<()> {
        self.audio.play_track(track.clone())?;
        self.current_track = Some(track);
        self.current_track_started_at = Some(Instant::now());
        self.paused_at = None;
        Ok(())
    }

    /// Toggles between play and pause
    ///
    /// If currently playing, pauses. If paused, resumes.
    pub fn toggle_play_pause(&mut self) -> anyhow::Result<()> {
        if self.is_playing() {
            self.pause()?;
        } else {
            self.resume()?;
        }
        Ok(())
    }

    /// Returns true if playback is currently paused
    pub fn is_paused(&self) -> bool { self.audio.state().is_paused() }

    /// Returns true if playback is currently playing
    pub fn is_playing(&self) -> bool { self.audio.state().is_playing() }

    /// Returns true if the current track has ended
    pub fn is_ended(&self) -> bool { self.audio.state().is_ended() }

    /// Shuts down the audio player and consumes the player state
    pub fn shutdown(self) -> anyhow::Result<()> {
        self.audio.shutdown()?;
        Ok(())
    }

    /// Pauses playback and records the pause time
    ///
    /// The elapsed time will be frozen at this point until resumed.
    fn pause(&mut self) -> anyhow::Result<()> {
        self.audio.pause()?;
        if self.paused_at.is_none() {
            self.paused_at = Some(Instant::now());
        }
        Ok(())
    }

    /// Resumes playback after pause
    ///
    /// Adjusts the start time to account for the paused duration,
    /// so the progress continues from where it was paused.
    fn resume(&mut self) -> anyhow::Result<()> {
        self.audio.play()?;
        if let (Some(started_at), Some(paused_at)) = (self.current_track_started_at, self.paused_at)
        {
            let paused_duration = paused_at.elapsed();
            self.current_track_started_at = Some(started_at + paused_duration);
            self.paused_at = None;
        }
        Ok(())
    }
}
