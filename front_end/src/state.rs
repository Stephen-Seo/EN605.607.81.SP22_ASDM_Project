use crate::yew_components::MainMenuMessage;
use std::cell::Cell;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    SinglePlayer,
    LocalMultiplayer,
    NetworkedMultiplayer,
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
            MainMenuMessage::SinglePlayer => GameState::SinglePlayer,
            MainMenuMessage::LocalMultiplayer => GameState::LocalMultiplayer,
            MainMenuMessage::NetworkedMultiplayer => GameState::NetworkedMultiplayer,
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

    pub fn into_win(self) -> Self {
        match self {
            BoardState::Empty => BoardState::Empty,
            BoardState::Cyan | BoardState::CyanWin => BoardState::CyanWin,
            BoardState::Magenta | BoardState::MagentaWin => BoardState::MagentaWin,
        }
    }

    pub fn from_win(self) -> Self {
        match self {
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

#[derive(Clone, Debug, PartialEq)]
pub struct SharedState {
    pub board: BoardType,
    pub game_state: Rc<Cell<GameState>>,
    pub turn: Rc<Cell<Turn>>,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            // cannot use [<type>; 56] because Rc does not impl Copy
            board: new_empty_board(),
            game_state: Rc::new(Cell::new(GameState::default())),
            turn: Rc::new(Cell::new(Turn::CyanPlayer)),
        }
    }
}
