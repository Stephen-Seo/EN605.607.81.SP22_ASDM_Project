pub const ROWS: u8 = 8;
pub const COLS: u8 = 7;

pub const INFO_TEXT_MAX_ITEMS: u32 = 100;

pub const AI_EASY_MAX_CHOICES: usize = 5;
pub const AI_NORMAL_MAX_CHOICES: usize = 3;

pub const PLAYER_COUNT_LIMIT: usize = 1000;
pub const TURN_SECONDS: u64 = 25;
pub const GAME_CLEANUP_TIMEOUT: u64 = (TURN_SECONDS + 1) * ((ROWS * COLS) as u64 + 5u64);
pub const PLAYER_CLEANUP_TIMEOUT: u64 = 300;
