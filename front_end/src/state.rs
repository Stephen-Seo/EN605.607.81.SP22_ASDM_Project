//Four Line Dropper Frontend/Backend - A webapp that allows one to play a game of Four Line Dropper
//Copyright (C) 2022 Stephen Seo
//
//This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
//This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
use crate::ai::AIDifficulty;
use crate::constants::{COLS, ROWS};
use crate::game_logic::{check_win_draw, WinType};

use serde::{Deserialize, Serialize};

use std::cell::{Cell, RefCell};
use std::collections::hash_set::HashSet;
use std::fmt::Display;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    SinglePlayer(Turn, AIDifficulty),
    LocalMultiplayer,
    NetworkedMultiplayer {
        paired: bool,
        current_side: Option<Turn>,
        current_turn: Turn,
        phrase: Option<String>,
    },
    PostGameResults(BoardState),
}

impl GameState {
    #[allow(dead_code)]
    pub fn is_networked_multiplayer(&self) -> bool {
        matches!(
            *self,
            GameState::NetworkedMultiplayer {
                paired: _,
                current_side: _,
                current_turn: _,
                phrase: _,
            }
        )
    }

    #[allow(dead_code)]
    pub fn set_networked_paired(&mut self) {
        if let GameState::NetworkedMultiplayer {
            ref mut paired,
            current_side: _,
            current_turn: _,
            phrase: _,
        } = self
        {
            *paired = true;
        }
    }

    #[allow(dead_code)]
    pub fn get_networked_current_side(&self) -> Option<Turn> {
        if let GameState::NetworkedMultiplayer {
            paired: _,
            current_side,
            current_turn: _,
            phrase: _,
        } = *self
        {
            current_side
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn set_networked_current_side(&mut self, side: Option<Turn>) {
        if let GameState::NetworkedMultiplayer {
            paired: _,
            ref mut current_side,
            current_turn: _,
            phrase: _,
        } = self
        {
            *current_side = side;
        }
    }

    #[allow(dead_code)]
    pub fn get_current_turn(&self) -> Turn {
        if let GameState::SinglePlayer(turn, _) = *self {
            turn
        } else if let GameState::NetworkedMultiplayer {
            paired: _,
            current_side: _,
            current_turn,
            phrase: _,
        } = *self
        {
            current_turn
        } else {
            Turn::CyanPlayer
        }
    }

    #[allow(dead_code)]
    pub fn set_networked_current_turn(&mut self, turn: Turn) {
        if let GameState::NetworkedMultiplayer {
            paired: _,
            current_side: _,
            ref mut current_turn,
            phrase: _,
        } = self
        {
            *current_turn = turn;
        }
    }

    #[allow(dead_code)]
    pub fn get_phrase(&self) -> Option<String> {
        if let GameState::NetworkedMultiplayer {
            paired: _,
            current_side: _,
            current_turn: _,
            phrase,
        } = self
        {
            phrase.clone()
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn get_singleplayer_current_side(&self) -> Option<Turn> {
        if let GameState::SinglePlayer(turn, _) = *self {
            Some(turn)
        } else {
            None
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::MainMenu
    }
}

impl From<MainMenuMessage> for GameState {
    fn from(msg: MainMenuMessage) -> Self {
        match msg {
            MainMenuMessage::SinglePlayer(t, ai) => GameState::SinglePlayer(t, ai),
            MainMenuMessage::LocalMultiplayer => GameState::LocalMultiplayer,
            MainMenuMessage::NetworkedMultiplayer(phrase) => GameState::NetworkedMultiplayer {
                paired: false,
                current_side: None,
                current_turn: Turn::CyanPlayer,
                phrase,
            },
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BoardState {
    Empty,
    Cyan,
    Magenta,
    CyanWin,
    MagentaWin,
}

impl Default for BoardState {
    fn default() -> Self {
        Self::Empty
    }
}

impl Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            BoardState::Empty => f.write_str("open"),
            BoardState::Cyan => f.write_str("cyan"),
            BoardState::CyanWin => f.write_str("cyan win"),
            BoardState::Magenta => f.write_str("magenta"),
            BoardState::MagentaWin => f.write_str("magenta win"),
        }
    }
}

impl From<Turn> for BoardState {
    fn from(t: Turn) -> Self {
        match t {
            Turn::CyanPlayer => BoardState::Cyan,
            Turn::MagentaPlayer => BoardState::Magenta,
        }
    }
}

impl BoardState {
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        *self == BoardState::Empty
    }

    #[allow(dead_code)]
    pub fn is_win(self) -> bool {
        match self {
            BoardState::Empty | BoardState::Cyan | BoardState::Magenta => false,
            BoardState::CyanWin | BoardState::MagentaWin => true,
        }
    }

    #[allow(dead_code)]
    pub fn into_win(self) -> Self {
        match self {
            BoardState::Empty => BoardState::Empty,
            BoardState::Cyan | BoardState::CyanWin => BoardState::CyanWin,
            BoardState::Magenta | BoardState::MagentaWin => BoardState::MagentaWin,
        }
    }

    #[allow(dead_code, clippy::wrong_self_convention)]
    pub fn from_win(self) -> Self {
        match self {
            BoardState::Empty => BoardState::Empty,
            BoardState::Cyan | BoardState::CyanWin => BoardState::Cyan,
            BoardState::Magenta | BoardState::MagentaWin => BoardState::Magenta,
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Turn {
    CyanPlayer,
    MagentaPlayer,
}

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Turn::CyanPlayer => f.write_str("CyanPlayer"),
            Turn::MagentaPlayer => f.write_str("MagentaPlayer"),
        }
    }
}

impl From<BoardState> for Turn {
    fn from(board_state: BoardState) -> Self {
        match board_state {
            BoardState::Empty | BoardState::Cyan | BoardState::CyanWin => Turn::CyanPlayer,
            BoardState::Magenta | BoardState::MagentaWin => Turn::MagentaPlayer,
        }
    }
}

impl Turn {
    #[allow(dead_code)]
    pub fn get_color(&self) -> &str {
        match *self {
            Turn::CyanPlayer => "cyan",
            Turn::MagentaPlayer => "magenta",
        }
    }

    pub fn get_opposite(&self) -> Self {
        match *self {
            Turn::CyanPlayer => Turn::MagentaPlayer,
            Turn::MagentaPlayer => Turn::CyanPlayer,
        }
    }
}

pub type BoardType = [Rc<Cell<BoardState>>; 56];

#[allow(dead_code)]
pub fn new_empty_board() -> BoardType {
    [
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
        Rc::new(Cell::new(BoardState::default())),
    ]
}

#[allow(dead_code)]
pub fn board_deep_clone(board: &BoardType) -> BoardType {
    let cloned_board = new_empty_board();
    for i in 0..board.len() {
        cloned_board[i].replace(board[i].get());
    }

    cloned_board
}

pub type PlacedType = [Rc<Cell<bool>>; 56];

#[allow(dead_code)]
pub fn new_placed() -> PlacedType {
    [
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
    ]
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct SharedState {
    pub board: BoardType,
    pub game_state: Rc<RefCell<GameState>>,
    pub turn: Rc<Cell<Turn>>,
    pub placed: PlacedType,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            // cannot use [<type>; 56] because Rc does not impl Copy
            board: new_empty_board(),
            game_state: Rc::new(RefCell::new(GameState::default())),
            turn: Rc::new(Cell::new(Turn::CyanPlayer)),
            placed: new_placed(),
        }
    }
}

// This enum moved from yew_components module so that this module would have no
// dependencies on the yew_components module
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MainMenuMessage {
    SinglePlayer(Turn, AIDifficulty),
    LocalMultiplayer,
    NetworkedMultiplayer(Option<String>),
}

#[allow(dead_code)]
pub fn new_string_board() -> String {
    let mut board = String::with_capacity(56);
    for _i in 0..56 {
        board.push('a');
    }
    board
}

#[allow(dead_code)]
pub fn board_from_string(board_string: &str) -> BoardType {
    let board = new_empty_board();

    for (idx, c) in board_string.chars().enumerate() {
        match c {
            'a' => board[idx].replace(BoardState::Empty),
            'b' | 'f' => board[idx].replace(BoardState::Cyan),
            'd' | 'h' => board[idx].replace(BoardState::CyanWin),
            'c' | 'g' => board[idx].replace(BoardState::Magenta),
            'e' | 'i' => board[idx].replace(BoardState::MagentaWin),
            _ => BoardState::Empty,
        };
    }

    board
}

/// Returns the board as a String, and None if game has not ended, Empty if game
/// ended in a draw, or a player if that player has won
#[allow(dead_code)]
pub fn string_from_board(board: &BoardType, placed: usize) -> (String, Option<BoardState>) {
    let mut board_string = String::with_capacity(56);

    // check for winning pieces
    let mut win_set: HashSet<usize> = HashSet::new();
    let win_opt = check_win_draw(board);
    if let Some((_board_state, win_type)) = win_opt {
        match win_type {
            WinType::Horizontal(pos) => {
                for i in pos..(pos + 4) {
                    win_set.insert(i);
                }
            }
            WinType::Vertical(pos) => {
                for i in 0..4 {
                    win_set.insert(pos + i * COLS as usize);
                }
            }
            WinType::DiagonalUp(pos) => {
                for i in 0..4 {
                    win_set.insert(pos + i - i * COLS as usize);
                }
            }
            WinType::DiagonalDown(pos) => {
                for i in 0..4 {
                    win_set.insert(pos + i + i * COLS as usize);
                }
            }
            WinType::None => (),
        }
    }

    // set values to String
    let mut is_full = true;
    for (idx, board_state) in board.iter().enumerate().take((COLS * ROWS) as usize) {
        board_string.push(match board_state.get() {
            BoardState::Empty => {
                is_full = false;
                'a'
            }
            BoardState::Cyan | BoardState::CyanWin => {
                if win_set.contains(&idx) {
                    if idx == placed {
                        'h'
                    } else {
                        'd'
                    }
                } else if idx == placed {
                    'f'
                } else {
                    'b'
                }
            }
            BoardState::Magenta | BoardState::MagentaWin => {
                if win_set.contains(&idx) {
                    if idx == placed {
                        'i'
                    } else {
                        'e'
                    }
                } else if idx == placed {
                    'g'
                } else {
                    'c'
                }
            }
        });
    }

    if is_full && win_set.is_empty() {
        (board_string, Some(BoardState::Empty))
    } else if !win_set.is_empty() {
        let winning_char: char =
            board_string.chars().collect::<Vec<char>>()[*win_set.iter().next().unwrap()];
        (
            board_string.clone(),
            if winning_char == 'd' || winning_char == 'h' {
                Some(BoardState::CyanWin)
            } else {
                Some(BoardState::MagentaWin)
            },
        )
    } else {
        (board_string, None)
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PairingRequestResponse {
    pub r#type: String,
    pub id: u32,
    pub status: String,
    pub color: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PairingStatusResponse {
    pub r#type: String,
    pub status: String,
    pub color: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct GameStateResponse {
    pub r#type: String,
    pub status: String,
    pub board: Option<String>,
    pub peer_emote: Option<String>,
    pub updated_time: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceTokenResponse {
    pub r#type: String,
    pub status: String,
    pub board: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SendEmoteRequestResponse {
    pub r#type: String,
    pub status: String,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NetworkedGameState {
    CyanTurn,
    MagentaTurn,
    CyanWon,
    MagentaWon,
    Draw,
    Disconnected,
    InternalError,
    NotPaired,
    UnknownID,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlacedEnum {
    Accepted,
    Illegal,
    NotYourTurn,
    Other(NetworkedGameState),
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EmoteEnum {
    Smile,
    Neutral,
    Frown,
    Think,
}

impl Display for EmoteEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EmoteEnum::Smile => f.write_str("smile"),
            EmoteEnum::Neutral => f.write_str("neutral"),
            EmoteEnum::Frown => f.write_str("frown"),
            EmoteEnum::Think => f.write_str("think"),
        }
    }
}

impl TryFrom<&str> for EmoteEnum {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "smile" => Ok(Self::Smile),
            "neutral" => Ok(Self::Neutral),
            "frown" => Ok(Self::Frown),
            "think" => Ok(Self::Think),
            _ => Err(()),
        }
    }
}

impl From<EmoteEnum> for String {
    fn from(e: EmoteEnum) -> Self {
        match e {
            EmoteEnum::Smile => "smile".into(),
            EmoteEnum::Neutral => "neutral".into(),
            EmoteEnum::Frown => "frown".into(),
            EmoteEnum::Think => "think".into(),
        }
    }
}

impl EmoteEnum {
    pub fn get_unicode(&self) -> char {
        match *self {
            EmoteEnum::Smile => '🙂',
            EmoteEnum::Neutral => '😐',
            EmoteEnum::Frown => '🙁',
            EmoteEnum::Think => '🤔',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_networked_multiplayer_enum() {
        let state = GameState::MainMenu;
        assert!(!state.is_networked_multiplayer());
        let state = GameState::LocalMultiplayer;
        assert!(!state.is_networked_multiplayer());
        let state = GameState::NetworkedMultiplayer {
            paired: false,
            current_side: None,
            current_turn: Turn::CyanPlayer,
            phrase: None,
        };
        assert!(state.is_networked_multiplayer());
        let state = GameState::NetworkedMultiplayer {
            paired: true,
            current_side: Some(Turn::CyanPlayer),
            current_turn: Turn::MagentaPlayer,
            phrase: None,
        };
        assert!(state.is_networked_multiplayer());
    }

    #[test]
    fn test_board_string() {
        let board = new_empty_board();
        board[49].set(BoardState::Cyan);
        board[51].set(BoardState::Cyan);
        board[52].set(BoardState::Cyan);
        board[53].set(BoardState::Cyan);
        board[54].set(BoardState::Cyan);
        board[55].set(BoardState::Magenta);

        let (board_string, state_opt) = string_from_board(&board, 51);

        let board_chars: Vec<char> = board_string.chars().collect();
        assert_eq!(board_chars[49], 'b');
        assert_eq!(board_chars[50], 'a');
        assert_eq!(board_chars[51], 'h');
        assert_eq!(board_chars[52], 'd');
        assert_eq!(board_chars[53], 'd');
        assert_eq!(board_chars[54], 'd');
        assert_eq!(board_chars[55], 'c');

        assert_eq!(state_opt, Some(BoardState::CyanWin));

        board[49].set(BoardState::Magenta);
        board[51].set(BoardState::Magenta);
        board[52].set(BoardState::Magenta);
        board[53].set(BoardState::Magenta);
        board[54].set(BoardState::Magenta);
        board[55].set(BoardState::Cyan);

        let (board_string, state_opt) = string_from_board(&board, 51);

        let board_chars: Vec<char> = board_string.chars().collect();
        assert_eq!(board_chars[49], 'c');
        assert_eq!(board_chars[50], 'a');
        assert_eq!(board_chars[51], 'i');
        assert_eq!(board_chars[52], 'e');
        assert_eq!(board_chars[53], 'e');
        assert_eq!(board_chars[54], 'e');
        assert_eq!(board_chars[55], 'b');

        assert_eq!(state_opt, Some(BoardState::MagentaWin));
    }
}
