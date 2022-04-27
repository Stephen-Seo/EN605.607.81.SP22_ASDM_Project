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

    pub fn get_network_current_side(&self) -> Option<Turn> {
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
    pub fn is_empty(&self) -> bool {
        *self == BoardState::Empty
    }

    pub fn is_win(self) -> bool {
        match self {
            BoardState::Empty | BoardState::Cyan | BoardState::Magenta => false,
            BoardState::CyanWin | BoardState::MagentaWin => true,
        }
    }

    pub fn into_win(self) -> Self {
        match self {
            BoardState::Empty => BoardState::Empty,
            BoardState::Cyan | BoardState::CyanWin => BoardState::CyanWin,
            BoardState::Magenta | BoardState::MagentaWin => BoardState::MagentaWin,
        }
    }

    pub fn from_win(&self) -> Self {
        match *self {
            BoardState::Empty => BoardState::Empty,
            BoardState::Cyan | BoardState::CyanWin => BoardState::Cyan,
            BoardState::Magenta | BoardState::MagentaWin => BoardState::Magenta,
        }
    }
}

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

pub fn board_deep_clone(board: &BoardType) -> BoardType {
    let cloned_board = new_empty_board();
    for i in 0..board.len() {
        cloned_board[i].replace(board[i].get());
    }

    cloned_board
}

pub type PlacedType = [Rc<Cell<bool>>; 56];

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
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MainMenuMessage {
    SinglePlayer(Turn, AIDifficulty),
    LocalMultiplayer,
    NetworkedMultiplayer(Option<String>),
}

pub fn new_string_board() -> String {
    let mut board = String::with_capacity(56);
    for _i in 0..56 {
        board.push('a');
    }
    board
}

pub fn board_from_string(board_string: String) -> BoardType {
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
pub fn string_from_board(board: BoardType, placed: usize) -> (String, Option<BoardState>) {
    let mut board_string = String::with_capacity(56);

    // check for winning pieces
    let mut win_set: HashSet<usize> = HashSet::new();
    let win_opt = check_win_draw(&board);
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
        (
            board_string.clone(),
            if board_string.chars().collect::<Vec<char>>()[*win_set.iter().next().unwrap()] == 'd' {
                Some(BoardState::CyanWin)
            } else {
                Some(BoardState::MagentaWin)
            },
        )
    } else {
        (board_string, None)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PairingRequestResponse {
    pub r#type: String,
    pub id: u32,
    pub status: String,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PairingStatusResponse {
    pub r#type: String,
    pub status: String,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStateResponse {
    pub r#type: String,
    pub status: String,
    pub board: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceTokenResponse {
    pub r#type: String,
    pub status: String,
    pub board: String,
}

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlacedEnum {
    Accepted,
    Illegal,
    NotYourTurn,
    Other(NetworkedGameState),
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
}
