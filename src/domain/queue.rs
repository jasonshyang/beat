use std::collections::VecDeque;

use rand::seq::SliceRandom;

use crate::domain::Track;

/// FIFO queue of tracks to play
#[derive(Default)]
pub struct PlayQueue(VecDeque<Track>);

impl PlayQueue {
    pub fn push(&mut self, track: Track) { self.0.push_back(track); }

    pub fn dequeue(&mut self) -> Option<Track> { self.0.pop_front() }

    pub fn len(&self) -> usize { self.0.len() }

    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    pub fn peek(&self) -> Option<&Track> { self.0.front() }

    pub fn get(&self, index: usize) -> Option<&Track> { self.0.get(index) }

    pub fn iter(&self) -> impl Iterator<Item = &Track> { self.0.iter() }

    /// Skips to a specific index, removing all tracks before it
    pub fn skip_to(&mut self, index: usize) -> Option<Track> {
        if index >= self.0.len() {
            return None;
        }

        // Remove all tracks before the target index
        self.0.drain(..index);

        // Dequeue the target track
        self.dequeue()
    }

    /// Shuffles the queue randomly
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.0.make_contiguous().shuffle(&mut rng);
    }

    /// Clears all tracks from the queue
    pub fn clear(&mut self) { self.0.clear(); }
}
