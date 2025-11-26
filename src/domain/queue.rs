use std::collections::VecDeque;

use crate::domain::Track;

#[derive(Default)]
pub struct PlayQueue(VecDeque<Track>);

impl PlayQueue {
    pub fn push(&mut self, music: Track) { self.0.push_back(music); }

    pub fn dequeue(&mut self) -> Option<Track> { self.0.pop_front() }

    pub fn len(&self) -> usize { self.0.len() }

    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    pub fn peek(&self) -> Option<&Track> { self.0.front() }

    pub fn get(&self, index: usize) -> Option<&Track> { self.0.get(index) }

    pub fn iter(&self) -> impl Iterator<Item = &Track> { self.0.iter() }
}
