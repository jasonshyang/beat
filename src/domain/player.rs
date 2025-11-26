use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::JoinHandle,
    time::Duration,
};

use crossbeam_channel::{Receiver, Sender};
use rodio::{OutputStream, Sink};

use crate::domain::Track;

const INTERVAL: Duration = Duration::from_millis(100);

enum Command {
    Play,
    Pause,
    PlayTrack(Track),
    Shutdown,
}

#[derive(Default)]
pub struct AudioState {
    is_playing: AtomicBool,
    is_paused: AtomicBool,
    is_ended: AtomicBool,
}

impl AudioState {
    pub fn is_playing(&self) -> bool { self.is_playing.load(Ordering::Relaxed) }

    pub fn is_paused(&self) -> bool { self.is_paused.load(Ordering::Relaxed) }

    pub fn is_ended(&self) -> bool { self.is_ended.load(Ordering::Relaxed) }
}

pub struct AudioPlayer {
    sink: Sink,
    state: Arc<AudioState>,
}

impl AudioPlayer {
    pub fn run_in_thread() -> anyhow::Result<AudioHandle> {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
        let sink = rodio::Sink::connect_new(stream_handle.mixer());
        let state = Arc::new(AudioState::default());

        let (tx, rx) = crossbeam_channel::unbounded();

        let player = AudioPlayer { sink, state: state.clone() };
        let player_handle = std::thread::spawn(move || player.run(rx));

        Ok(AudioHandle { cmd_tx: tx, player_handle, stream_handle, state })
    }

    fn run(mut self, rx: Receiver<Command>) -> anyhow::Result<Self> {
        let ticker = crossbeam_channel::tick(INTERVAL);

        loop {
            crossbeam_channel::select! {
                recv(rx) -> msg => {
                    match msg {
                        Ok(Command::Play) => self.play(),
                        Ok(Command::Pause) => self.pause(),
                        Ok(Command::PlayTrack(track)) => self.play_track(track)?,
                        Ok(Command::Shutdown) | Err(_) => break,
                    }
                }
                recv(ticker) -> _ => {
                    self.update_state();
                }
            }
        }

        Ok(self)
    }

    fn play_track(&mut self, music: Track) -> anyhow::Result<()> {
        let file = std::fs::File::open(&music.path)?;
        self.sink.clear();
        self.sink.append(rodio::Decoder::try_from(file)?);
        self.sink.play();

        self.update_state();
        Ok(())
    }

    fn play(&mut self) {
        self.sink.play();
        self.update_state();
    }

    fn pause(&mut self) {
        self.sink.pause();
        self.update_state();
    }

    fn update_state(&mut self) {
        self.state
            .is_playing
            .store(!self.sink.empty() && !self.sink.is_paused(), Ordering::Relaxed);
        self.state
            .is_paused
            .store(self.sink.is_paused(), Ordering::Relaxed);
        self.state
            .is_ended
            .store(self.sink.empty(), Ordering::Relaxed);
    }
}

pub struct AudioHandle {
    cmd_tx: Sender<Command>,
    player_handle: JoinHandle<anyhow::Result<AudioPlayer>>,
    stream_handle: OutputStream,
    state: Arc<AudioState>,
}

impl AudioHandle {
    pub fn play(&self) -> anyhow::Result<()> {
        self.cmd_tx.send(Command::Play)?;
        Ok(())
    }

    pub fn pause(&self) -> anyhow::Result<()> {
        self.cmd_tx.send(Command::Pause)?;
        Ok(())
    }

    pub fn play_track(&self, music: Track) -> anyhow::Result<()> {
        self.cmd_tx.send(Command::PlayTrack(music))?;
        Ok(())
    }

    pub fn shutdown(self) -> anyhow::Result<AudioPlayer> {
        self.cmd_tx.send(Command::Shutdown)?;
        let player = self.player_handle.join().expect("Player thread panicked")?;
        Ok(player)
    }

    pub fn stream_handle(&self) -> &OutputStream { &self.stream_handle }

    pub fn state(&self) -> &AudioState { &self.state }
}
