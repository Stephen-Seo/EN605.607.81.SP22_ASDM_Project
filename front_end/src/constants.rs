pub const ROWS: u8 = 8;
pub const COLS: u8 = 7;

pub const INFO_TEXT_MAX_ITEMS: u32 = 100;

pub const AI_EASY_MAX_CHOICES: usize = 5;
pub const AI_NORMAL_MAX_CHOICES: usize = 3;
pub const AI_CHOICE_DURATION_MILLIS: i32 = 1000;

pub const PLAYER_COUNT_LIMIT: usize = 1000;
pub const TURN_SECONDS: u64 = 25;
pub const GAME_CLEANUP_TIMEOUT: u64 = (TURN_SECONDS + 1) * ((ROWS * COLS) as u64 + 5u64);
pub const PLAYER_CLEANUP_TIMEOUT: u64 = 300;

pub const BACKEND_TICK_DURATION_MILLIS: i32 = 500;

// TODO: Change this to "https://asdm.seodisparate.com/api" when backend is installed
pub const BACKEND_URL: &str = "http://localhost:1237/";
