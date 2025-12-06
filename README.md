# Beat

A minimalist terminal-based music player built with Rust.

![Demo](docs/demo.gif)

## Key Features

- Terminal-based user interface
- File browser for music library navigation
- Playlist management (create, rename, delete)
- Queue control (shuffle, clear, skip to track)
- Multi-select support for batch operations
- Keyboard-driven controls
- Persistent playlist storage (saved under `~/.config/beat/playlists.toml` )

## Installation

### Prerequisites
- Rust

### Install from GitHub
```bash
cargo install --git https://github.com/jasonshyang/beat.git
```

This will install `beat` to `~/.cargo/bin/`.

### Build without installing
```bash
git clone https://github.com/jasonshyang/beat.git
cd beat
cargo build --release
```

## Usage

### Running Beat
```bash
# If installed via cargo install
beat

# Optionally, start in a specific directory
beat /path/to/music

# Or run from project directory
cargo run
```

## Keyboard Controls

### Navigation
- `↑`/`↓` or `j`/`k` - Move up/down
- `1`/`2`/`3` - Switch to Browse/Queue/Playlists tab
- `Enter` - Select item (play track, open directory, view playlist)
- `Esc` - Go back / Clear multi-selection
- `g` - Go to directory (in Browse tab)

### Playback
- `Space` - Play/Pause
- `n` - Play next track
- `a` - Add all tracks from current directory to queue

### Browse Tab
- `Enter` - Queue/open selected file or directory
- `Shift`+`↑`/`↓` - Multi-select tracks
- `a` - Add all tracks to queue
- `p` - Add selected track(s) to playlist

### Queue Tab
- `Enter` - Skip to selected track (removes all tracks before it)
- `s` - Shuffle queue
- `c` - Clear queue

### Playlists Tab
- `p` - Play entire playlist (clears queue and starts playing)
- `Enter` - View playlist contents
- `c` - Create new playlist
- `r` - Rename selected playlist
- `d` - Delete selected playlist
- `x` - Remove track from playlist (when viewing playlist)

### General
- `b` - Toggle browser visibility (minimalist mode)
- `?` - Toggle help screen
- `Shift`+`Q` - Quit
