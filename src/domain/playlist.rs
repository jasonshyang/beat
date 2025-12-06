use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::domain::Track;

/// Pure domain container for all playlists
///
/// Manages playlist collection without any I/O operations.
/// Persistence is handled at the app layer.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Playlists {
    pub playlists: Vec<Playlist>,
}

impl Playlists {
    /// Creates a new empty playlists collection
    pub fn new() -> Self { Self { playlists: Vec::new() } }

    /// Get all playlists
    pub fn all(&self) -> &[Playlist] { &self.playlists }

    /// Get a playlist by index
    pub fn get(&self, index: usize) -> Option<&Playlist> { self.playlists.get(index) }

    /// Get a mutable reference to a playlist by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Playlist> {
        self.playlists.get_mut(index)
    }

    /// Get the number of playlists
    pub fn len(&self) -> usize { self.playlists.len() }

    /// Check if there are no playlists
    pub fn is_empty(&self) -> bool { self.playlists.is_empty() }

    /// Add a new playlist
    pub fn add(&mut self, playlist: Playlist) { self.playlists.push(playlist); }

    /// Remove a playlist by index, returning it if it existed
    pub fn remove(&mut self, index: usize) -> Option<Playlist> {
        if index < self.playlists.len() { Some(self.playlists.remove(index)) } else { None }
    }

    /// Check if a playlist name already exists
    pub fn has_name(&self, name: &str) -> bool { self.playlists.iter().any(|p| p.name == name) }

    /// Check if a playlist name exists, excluding a specific index
    pub fn has_name_except(&self, name: &str, except_index: usize) -> bool {
        self.playlists
            .iter()
            .enumerate()
            .any(|(i, p)| i != except_index && p.name == name)
    }

    /// Load tracks for all playlists from their stored paths
    pub fn load_all_tracks(&mut self) {
        for playlist in &mut self.playlists {
            playlist.load_tracks();
        }
    }
}

/// A playlist containing a list of track paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    #[serde(skip)]
    pub tracks: Vec<Track>,
    /// Persisted track paths
    track_paths: Vec<PathBuf>,
}

impl Playlist {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), tracks: Vec::new(), track_paths: Vec::new() }
    }

    /// Load tracks from stored paths
    pub fn load_tracks(&mut self) {
        self.tracks = self
            .track_paths
            .iter()
            .filter_map(|path| {
                if path.exists() {
                    let name = path.file_stem()?.to_string_lossy().to_string();
                    Some(Track::new(name, path))
                } else {
                    None
                }
            })
            .collect();
    }

    pub fn add_track(&mut self, track: Track) {
        self.track_paths.push(track.path.clone());
        self.tracks.push(track);
    }

    pub fn remove_track(&mut self, index: usize) -> Option<Track> {
        if index < self.tracks.len() {
            self.track_paths.remove(index);
            Some(self.tracks.remove(index))
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool { self.tracks.is_empty() }

    pub fn len(&self) -> usize { self.tracks.len() }
}
