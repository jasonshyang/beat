use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

use crate::domain::{Library, PlayQueue, Playlist, Playlists, Track};

/// Music state - manages library, queue, and playlists
///
/// Encapsulates all domain logic for music data operations
/// and handles playlist persistence.
pub struct MusicState {
    /// Music library for browsing tracks
    pub library: Library,
    /// Queue of tracks to be played
    pub queue: PlayQueue,
    /// Playlists collection
    pub playlists: Playlists,

    /// Path to the playlists config file
    playlists_config_path: PathBuf,
}

impl MusicState {
    pub fn new(library: Library, playlists: Playlists, config_path: PathBuf) -> Self {
        Self { library, queue: PlayQueue::default(), playlists, playlists_config_path: config_path }
    }

    /// Adds tracks to the queue by their indices in the library
    pub fn add_tracks_to_queue(&mut self, indices: &[usize]) {
        let tracks = self.library.collect_audio_tracks(indices);
        for (name, path) in tracks {
            self.queue.push(Track::new(&name, &path));
        }
    }

    /// Adds all audio tracks from the current directory to the queue
    pub fn add_all_to_queue(&mut self) -> bool {
        let tracks = self.library.collect_all_audio_tracks();
        if tracks.is_empty() {
            return false;
        }

        for (name, path) in tracks {
            self.queue.push(Track::new(&name, &path));
        }
        true
    }

    /// Adds a single track to the queue
    pub fn add_track_to_queue(&mut self, track: Track) { self.queue.push(track); }

    /// Collects tracks at given indices for playlist operations
    pub fn collect_tracks(&self, indices: &[usize]) -> Vec<(String, std::path::PathBuf)> {
        self.library.collect_audio_tracks(indices)
    }

    /// Checks if a library entry at the given index is audio
    pub fn is_audio_at(&self, index: usize) -> bool {
        self.library
            .get_entry(index)
            .map(|e| e.is_audio())
            .unwrap_or(false)
    }

    /// Changes the library directory
    pub fn change_directory(&mut self, path: std::path::PathBuf) -> anyhow::Result<()> {
        self.library.change_dir(path)
    }

    /// Gets the length of the library
    pub fn library_len(&self) -> usize { self.library.len() }

    /// Gets the length of the queue
    pub fn queue_len(&self) -> usize { self.queue.len() }

    /// Gets the number of playlists
    pub fn playlists_len(&self) -> usize { self.playlists.len() }

    /// Gets the length of a specific playlist
    pub fn playlist_len(&self, index: usize) -> usize {
        self.playlists.get(index).map(|p| p.len()).unwrap_or(0)
    }

    /// Creates a new playlist
    pub fn create_playlist(&mut self, name: String) -> anyhow::Result<()> {
        // Check for duplicate names
        if self.playlists.has_name(&name) {
            anyhow::bail!("Playlist '{}' already exists", name);
        }

        self.playlists.add(Playlist::new(name));
        self.save_playlists()?;
        Ok(())
    }

    /// Deletes a playlist by index
    pub fn delete_playlist(&mut self, index: usize) -> anyhow::Result<()> {
        if self.playlists.remove(index).is_some() {
            self.save_playlists()?;
            Ok(())
        } else {
            anyhow::bail!("Invalid playlist index: {}", index);
        }
    }

    /// Renames a playlist by index
    pub fn rename_playlist(&mut self, index: usize, new_name: String) -> anyhow::Result<()> {
        // Check for duplicate names (excluding current)
        if self.playlists.has_name_except(&new_name, index) {
            anyhow::bail!("Playlist '{}' already exists", new_name);
        }

        if let Some(playlist) = self.playlists.get_mut(index) {
            playlist.name = new_name;
            self.save_playlists()?;
            Ok(())
        } else {
            anyhow::bail!("Invalid playlist index: {}", index);
        }
    }

    /// Adds tracks to a playlist
    pub fn add_tracks_to_playlist(
        &mut self,
        playlist_idx: usize,
        tracks: Vec<(String, std::path::PathBuf)>,
    ) -> anyhow::Result<()> {
        let playlist = self
            .playlists
            .get_mut(playlist_idx)
            .ok_or_else(|| anyhow::anyhow!("Invalid playlist index: {}", playlist_idx))?;

        for (name, path) in tracks {
            playlist.add_track(Track::new(&name, &path));
        }
        self.save_playlists()?;
        Ok(())
    }

    /// Removes a track from a playlist
    pub fn remove_track_from_playlist(
        &mut self,
        playlist_idx: usize,
        track_idx: usize,
    ) -> anyhow::Result<()> {
        let playlist = self
            .playlists
            .get_mut(playlist_idx)
            .ok_or_else(|| anyhow::anyhow!("Invalid playlist index: {}", playlist_idx))?;

        if playlist.remove_track(track_idx).is_some() {
            self.save_playlists()?;
            Ok(())
        } else {
            anyhow::bail!("Invalid track index: {}", track_idx);
        }
    }

    /// Gets a track from the queue
    pub fn get_queue_track(&self, index: usize) -> Option<&Track> { self.queue.get(index) }

    /// Gets a track from a playlist
    pub fn get_playlist_track(&self, playlist_idx: usize, track_idx: usize) -> Option<&Track> {
        self.playlists
            .get(playlist_idx)
            .and_then(|p| p.tracks.get(track_idx))
    }

    /// Dequeues the next track from the queue
    pub fn dequeue(&mut self) -> Option<Track> { self.queue.dequeue() }

    /// Shuffles the queue randomly
    pub fn shuffle_queue(&mut self) { self.queue.shuffle(); }

    /// Clears all tracks from the queue
    pub fn clear_queue(&mut self) { self.queue.clear(); }

    /// Skips to a track at the given index in the queue
    /// Removes all tracks before it and returns the track to play
    pub fn skip_to_track(&mut self, index: usize) -> Option<Track> { self.queue.skip_to(index) }

    /// Clears queue and loads all tracks from a playlist
    /// Returns true if any tracks were added
    pub fn play_playlist(&mut self, playlist_idx: usize) -> bool {
        let Some(playlist) = self.playlists.get(playlist_idx) else {
            return false;
        };

        if playlist.is_empty() {
            return false;
        }

        // Clear existing queue and load playlist tracks
        self.queue.clear();
        for track in &playlist.tracks {
            self.queue.push(track.clone());
        }

        true
    }

    /// Save playlists to config file
    fn save_playlists(&self) -> Result<()> {
        // Ensure config directory exists
        if let Some(parent) = self.playlists_config_path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let content =
            toml::to_string_pretty(&self.playlists).context("Failed to serialize playlists")?;

        fs::write(&self.playlists_config_path, content)
            .context("Failed to write playlists file")?;

        Ok(())
    }

    /// Load playlists from config file
    /// Load playlists from config file
    pub fn load_playlists() -> Result<(Playlists, PathBuf)> {
        let config_path = Self::playlists_config_path()?;

        let mut playlists = if config_path.exists() {
            let content =
                fs::read_to_string(&config_path).context("Failed to read playlists file")?;
            toml::from_str::<Playlists>(&content).context("Failed to parse playlists file")?
        } else {
            Playlists::default()
        };

        // Load actual tracks from paths
        playlists.load_all_tracks();

        Ok((playlists, config_path))
    }
    /// Get config file path: ~/Documents/beat/playlists.toml
    fn playlists_config_path() -> Result<PathBuf> {
        let doc_dir = dirs::document_dir()
            .context("Could not determine documents directory for playlists")?;
        Ok(doc_dir.join("beat-config").join("playlists.toml"))
    }
}
