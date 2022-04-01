use crate::ai::AIDifficulty;
use crate::constants::{COLS, ROWS};
use crate::game_logic::{check_win_draw, WinType};

use std::cell::Cell;
use std::collections::hash_set::HashSet;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    SinglePlayer(Turn, AIDifficulty),
    LocalMultiplayer,
    NetworkedMultiplayer(Turn),
    PostGameResults(BoardState),
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
            MainMenuMessage::NetworkedMultiplayer(t) => GameState::NetworkedMultiplayer(t),
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
    pub game_state: Rc<Cell<GameState>>,
    pub turn: Rc<Cell<Turn>>,
    pub placed: PlacedType,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            // cannot use [<type>; 56] because Rc does not impl Copy
            board: new_empty_board(),
            game_state: Rc::new(Cell::new(GameState::default())),
            turn: Rc::new(Cell::new(Turn::CyanPlayer)),
            placed: new_placed(),
        }
    }
}

// This enum moved from yew_components module so that this module would have no
// dependencies on the yew_components module
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MainMenuMessage {
    SinglePlayer(Turn, AIDifficulty),
    LocalMultiplayer,
    NetworkedMultiplayer(Turn),
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
            'b' | 'd' | 'f' => board[idx].replace(BoardState::Cyan),
            'c' | 'e' | 'g' => board[idx].replace(BoardState::Magenta),
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
                    'd'
                } else if idx == placed {
                    'f'
                } else {
                    'b'
                }
            }
            BoardState::Magenta | BoardState::MagentaWin => {
                if win_set.contains(&idx) {
                    'e'
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
