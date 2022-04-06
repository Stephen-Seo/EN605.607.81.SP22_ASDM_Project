//Four Line Dropper Frontend/Backend - A webapp that allows one to play a game of Four Line Dropper
//Copyright (C) 2022 Stephen Seo
//
//This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
//This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
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
#[cfg(debug_assertions)]
pub const BACKEND_URL: &str = "http://testlocalhost/api";

#[cfg(not(debug_assertions))]
pub const BACKEND_URL: &str = "https://asdm.seodisparate.com/api";
