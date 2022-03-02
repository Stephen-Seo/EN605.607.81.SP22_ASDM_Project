use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct MessageBus {
    queued: VecDeque<String>,
}

impl MessageBus {
    pub fn get_next_msg(&mut self) -> Option<String> {
        self.queued.pop_front()
    }

    pub fn push_msg(&mut self, msg: String) -> Result<(), String> {
        self.queued.push_back(msg);
        Ok(())
    }
}

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Turn {
    CyanPlayer,
    MagentaPlayer,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SharedState {
    pub board: [Rc<Cell<BoardState>>; 56],
    pub bus: Rc<RefCell<MessageBus>>,
    pub turn: Turn,
    pub info_text_ref: NodeRef,
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
            bus: Rc::new(RefCell::new(MessageBus::default())),
            turn: Turn::CyanPlayer,
            info_text_ref: NodeRef::default(),
        }
    }
}
