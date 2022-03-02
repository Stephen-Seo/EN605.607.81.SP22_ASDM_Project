use std::cell::Cell;
use std::fmt::Display;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BoardState {
    Empty,
    Cyan,
    Magenta,
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
            BoardState::Magenta => f.write_str("magenta"),
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

#[derive(Clone, Debug, PartialEq)]
pub struct SharedState {
    pub board: [Rc<Cell<BoardState>>; 56],
    pub turn: Rc<Cell<Turn>>,
    pub info_text_ref: [NodeRef; 2],
    pub slot_refs: [NodeRef; 56],
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            // cannot use [<type>; 56] because Rc does not impl Copy
            board: [
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
            ],
            turn: Rc::new(Cell::new(Turn::CyanPlayer)),
            // NodeRef array needs to have unique values
            info_text_ref: [NodeRef::default(), NodeRef::default()],
            // slot_refs array needs to have unique values
            slot_refs: [
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
                NodeRef::default(),
            ],
        }
    }
}
