pub struct TuiText;

impl TuiText {
    // === Mini Help (shown in minimalist mode) ===
    pub const MINI_HELP_KEYS: &'static [(&'static str, &'static str)] = &[
        ("[b]", "browser"),
        ("[space]", "play/pause"),
        ("[n]", "next"),
        ("[Enter]", "select"),
        ("[Shift+Q]", "quit"),
        ("[?]", "help"),
    ];

    // === Full Help (shown when user presses ?) ===
    pub const HELP_TITLE: &'static str = "♫ beat - Help";

    pub const HELP_NAVIGATION: &'static [(&'static str, &'static str)] = &[
        ("[↑↓/jk]", "move"),
        ("[1/2/3]", "switch tab"),
        ("[Enter]", "select"),
        ("[Shift+A]", "select all (Browse tab)"),
        ("[g]", "go to directory (Browse tab)"),
        ("[Esc]", "back"),
    ];

    pub const HELP_PLAYBACK: &'static [(&'static str, &'static str)] =
        &[("[space]", "play/pause"), ("[n]", "next"), ("[a]", "add all to queue")];

    pub const HELP_QUEUE: &'static [(&'static str, &'static str)] =
        &[("[Enter]", "skip to track"), ("[s]", "shuffle"), ("[c]", "clear")];

    pub const HELP_PLAYLISTS: &'static [(&'static str, &'static str)] = &[
        ("[p]", "play playlist / add to playlist"),
        ("[c]", "create playlist"),
        ("[r]", "rename playlist"),
        ("[d/x]", "delete/remove"),
    ];

    pub const HELP_GENERAL: &'static [(&'static str, &'static str)] =
        &[("[b]", "toggle browser"), ("[?]", "toggle help"), ("[Shift+Q]", "quit")];

    // === Widget titles and hints ===

    // Browser
    pub const BROWSER_HINT_NORMAL: &'static str =
        "Files • [Enter] queue/open • [a] add all • [Shift+A] select all • [p] add to playlist...";
    pub const BROWSER_HINT_MULTI_SELECT: &'static str = "Files • [Shift+↑↓] multi-select • [Shift+A] select all • [Enter] queue selected • [p] add selected to playlist...";

    // Queue
    pub const QUEUE_TITLE: &'static str = "Queue • [Enter] skip to • [s] shuffle • [c] clear";

    // Playlists
    pub const PLAYLIST_TITLE_EMPTY: &'static str = "Playlists • [c] create";
    pub const PLAYLIST_TITLE_WITH_ITEMS: &'static str =
        "Playlists • [p] play • [Enter] view • [r] rename • [c] create • [d] delete";

    // Playlist detail (viewing tracks in a playlist)
    pub const PLAYLIST_DETAIL_HINT: &'static str = "[x] remove • [Esc] back";
    pub const PLAYLIST_DETAIL_EMPTY_HINT: &'static str =
        "Empty • Add tracks from Browse tab with [p]";
    pub const PLAYLIST_DETAIL_PLAY_HINT: &'static str = "[Enter] play";

    // Playlist selector (modal for adding tracks)
    pub const PLAYLIST_SELECTOR_TITLE: &'static str = "Add to Playlist";
    pub const PLAYLIST_SELECTOR_HINT: &'static str = "[Enter] select • [Esc] cancel";
    pub const PLAYLIST_SELECTOR_NEW: &'static str = "➕ Create New Playlist";

    // Input modal
    pub const INPUT_MODAL_HINT: &'static str = "[Enter] confirm • [Esc] cancel";

    // Error notification
    pub const ERROR_NOTIFICATION_HINT: &'static str = "Press any key to dismiss";

    // Tabs
    pub const TAB_BROWSE: &'static str = "Browse";
    pub const TAB_QUEUE: &'static str = "Queue";
    pub const TAB_PLAYLIST: &'static str = "Playlists";
}
