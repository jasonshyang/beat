use std::path::PathBuf;

use crate::domain::DirEntry;

pub const AUDIO_FILE_EXTENSIONS: &[&str] = &["mp3", "wav", "flac", "ogg"];

/// File browser for navigating and listing audio files
pub struct Library {
    current_dir: PathBuf,
    entries: Vec<DirEntry>,
}

impl Library {
    pub fn new(start_dir: Option<PathBuf>) -> anyhow::Result<Self> {
        let current_dir = start_dir.unwrap_or(std::env::current_dir()?);
        let mut lib = Self { current_dir, entries: Vec::new() };
        lib.refresh()?;
        Ok(lib)
    }

    pub fn len(&self) -> usize { self.entries.len() }

    pub fn is_empty(&self) -> bool { self.entries.is_empty() }

    pub fn entries(&self) -> &[DirEntry] { &self.entries }

    pub fn audios(&self) -> impl Iterator<Item = &DirEntry> {
        self.entries.iter().filter(|e| e.is_audio())
    }

    pub fn current_dir(&self) -> &PathBuf { &self.current_dir }

    pub fn change_dir(&mut self, path: PathBuf) -> anyhow::Result<()> {
        let absolute_path = if path.is_absolute() { path } else { self.current_dir.join(path) };

        self.current_dir = absolute_path.canonicalize()?;
        self.refresh()?;
        Ok(())
    }

    pub fn get_entry(&self, index: usize) -> Option<&DirEntry> { self.entries.get(index) }

    /// Collects audio tracks at the given indices
    pub fn collect_audio_tracks(&self, indices: &[usize]) -> Vec<(String, PathBuf)> {
        indices
            .iter()
            .filter_map(|&i| self.entries.get(i))
            .filter(|e| e.is_audio())
            .map(|e| (e.name.clone(), e.path.clone()))
            .collect()
    }

    /// Collects all audio tracks in current directory
    pub fn collect_all_audio_tracks(&self) -> Vec<(String, PathBuf)> {
        self.audios()
            .map(|e| (e.name.clone(), e.path.clone()))
            .collect()
    }

    fn refresh(&mut self) -> anyhow::Result<()> {
        self.entries.clear();

        // Add parent directory
        if let Some(parent) = self.current_dir.parent()
            && parent != self.current_dir
        {
            self.entries.push(DirEntry::dir("..", parent));
        }

        // Read directory
        for entry in std::fs::read_dir(&self.current_dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if name.starts_with('.') {
                continue;
            }

            if path.is_dir() {
                self.entries.push(DirEntry::dir(name, path));
                continue;
            } else if path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| AUDIO_FILE_EXTENSIONS.contains(&e))
                .unwrap_or(false)
            {
                self.entries.push(DirEntry::audio(name, path));
            }
        }

        self.entries.sort_by(|a, b| match (a.is_dir(), b.is_dir()) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        });

        Ok(())
    }
}
