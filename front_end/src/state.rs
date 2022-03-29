use crate::ai::AIDifficulty;
use std::cell::Cell;
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
