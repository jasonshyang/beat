use std::{path::PathBuf, time::Duration};

use encoding_rs::{BIG5, EUC_KR, GBK, SHIFT_JIS};
use lofty::{
    config::ParseOptions,
    file::{AudioFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};
use rodio::Source;

pub struct DirEntry {
    pub name: String,
    pub path: PathBuf,
    pub ty: EntryType,
}

impl DirEntry {
    pub fn dir(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self { name: name.into(), path: path.into(), ty: EntryType::Directory }
    }

    pub fn audio(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self { name: name.into(), path: path.into(), ty: EntryType::Audio }
    }

    pub fn is_dir(&self) -> bool { matches!(self.ty, EntryType::Directory) }

    pub fn is_audio(&self) -> bool { matches!(self.ty, EntryType::Audio) }
}

pub enum EntryType {
    Directory,
    Audio,
}

#[derive(Clone)]
pub struct Track {
    pub name: String,
    pub path: PathBuf,
    pub duration: Option<Duration>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

impl Track {
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let (duration, artist, album) = Self::extract_metadata(&path);
        Self { name: name.into(), path, duration, artist, album }
    }

    fn extract_metadata(path: &PathBuf) -> (Option<Duration>, Option<String>, Option<String>) {
        let parse_options = ParseOptions::new().parsing_mode(lofty::config::ParsingMode::Relaxed);

        // Try to get metadata using lofty
        let tagged_file = Probe::open(path)
            .ok()
            .and_then(|probe| probe.options(parse_options).read().ok());

        let duration = if let Some(ref file) = tagged_file {
            Some(Duration::from_millis(file.properties().duration().as_millis() as u64))
        } else {
            // Fallback to rodio for duration if lofty fails
            Self::get_duration_rodio(path)
        };

        let (artist, album) = if let Some(file) = tagged_file {
            let tag = file.primary_tag().or_else(|| file.first_tag());
            let artist = tag.and_then(|t| t.artist()).map(|s| Self::decode_text(&s));
            let album = tag.and_then(|t| t.album()).map(|s| Self::decode_text(&s));
            (artist, album)
        } else {
            (None, None)
        };

        (duration, artist, album)
    }

    fn decode_text(text: &str) -> String {
        // If text already looks valid (has unicode above Latin-1 range), keep it
        if text.chars().any(|c| (c as u32) > 0xFF) {
            return text.to_string();
        }

        let bytes: Vec<u8> = text.chars().map(|c| c as u8).collect();

        // Try common encodings and pick the first one that decodes without errors
        // and produces reasonable text
        for encoding in [GBK, BIG5, SHIFT_JIS, EUC_KR] {
            if let (decoded, _, false) = encoding.decode(&bytes) {
                // Accept if it has alphanumeric or common CJK characters
                if decoded.chars().any(|c| c.is_alphanumeric()) {
                    return decoded.to_string();
                }
            }
        }

        text.to_string()
    }

    fn get_duration_rodio(path: &PathBuf) -> Option<Duration> {
        let file = std::fs::File::open(path).ok()?;
        let source = rodio::Decoder::new(std::io::BufReader::new(file)).ok()?;
        source.total_duration()
    }
}
