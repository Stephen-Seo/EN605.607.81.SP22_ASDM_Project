use crate::constants::{COLS, ROWS};
use crate::state::{BoardState, BoardType};

/// Returns a BoardState if win/draw, None if game is still going
pub fn check_win_draw(board: &BoardType) -> Option<BoardState> {
    let mut has_empty_slot = false;
    for slot in board {
        match slot.get() {
            BoardState::Empty => {
                has_empty_slot = true;
                break;
            }
            BoardState::Cyan | BoardState::Magenta => (),
        }
    }

    if has_empty_slot {
        return None;
    }

    let check_result = |state| -> Option<BoardState> {
        match state {
            BoardState::Empty => None,
            BoardState::Cyan => Some(BoardState::Cyan),
            BoardState::Magenta => Some(BoardState::Magenta),
        }
    };

    // check horizontals
    for y in 0..(ROWS as usize) {
        for x in 0..((COLS - 3) as usize) {
            let result = check_result(has_right_horizontal_at_idx(x + y * (COLS as usize), board));
            if result.is_some() {
                return result;
            }
        }
    }

    // check verticals
    for y in 0..((ROWS - 3) as usize) {
        for x in 0..(COLS as usize) {
            let result = check_result(has_down_vertical_at_idx(x + y * (COLS as usize), board));
            if result.is_some() {
                return result;
            }
        }
    }

    // check up diagonals
    for y in 3..(ROWS as usize) {
        for x in 0..((COLS - 3) as usize) {
            let result = check_result(has_right_up_diagonal_at_idx(x + y * (COLS as usize), board));
            if result.is_some() {
                return result;
            }
        }
    }

    // check down diagonals
    for y in 0..((ROWS - 3) as usize) {
        for x in 0..((COLS - 3) as usize) {
            let result = check_result(has_right_down_diagonal_at_idx(
                x + y * (COLS as usize),
                board,
            ));
            if result.is_some() {
                return result;
            }
        }
    }

    None
}

fn has_right_horizontal_at_idx(idx: usize, board: &BoardType) -> BoardState {
    let state_at_idx = board[idx].get();
    if idx % (COLS as usize) < (COLS as usize) - 3 {
        for x in 0..=3 {
            if board[idx + x].get() != state_at_idx {
                break;
            } else if x == 3 {
                return state_at_idx;
            }
        }
    }

    BoardState::Empty
}

fn has_down_vertical_at_idx(idx: usize, board: &BoardType) -> BoardState {
    let state_at_idx = board[idx].get();
    if idx / (COLS as usize) < (ROWS as usize) - 3 {
        for y in 0..=3 {
            if board[idx + y * (COLS as usize)].get() != state_at_idx {
                break;
            } else if y == 3 {
                return state_at_idx;
            }
        }
    }

    BoardState::Empty
}

fn has_right_up_diagonal_at_idx(idx: usize, board: &BoardType) -> BoardState {
    let state_at_idx = board[idx].get();
    if idx % (COLS as usize) < (COLS as usize) - 3 && idx / (COLS as usize) > 2 {
        for i in 0..=3 {
            if board[idx + i - i * (COLS as usize)].get() != state_at_idx {
                break;
            } else if i == 3 {
                return state_at_idx;
            }
        }
    }

    BoardState::Empty
}

fn has_right_down_diagonal_at_idx(idx: usize, board: &BoardType) -> BoardState {
    let state_at_idx = board[idx].get();
    if idx % (COLS as usize) < (COLS as usize) - 3 && idx / (COLS as usize) < (ROWS as usize) - 3 {
        for i in 0..=3 {
            if board[idx + i + i * (COLS as usize)].get() != state_at_idx {
                break;
            } else if i == 3 {
                return state_at_idx;
            }
        }
    }
    BoardState::Empty
}
