//Four Line Dropper Frontend/Backend - A webapp that allows one to play a game of Four Line Dropper
//Copyright (C) 2022 Stephen Seo
//
//This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
//This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
use crate::constants::{COLS, ROWS};
use crate::state::{BoardState, BoardType};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WinType {
    Horizontal(usize),
    Vertical(usize),
    DiagonalUp(usize),
    DiagonalDown(usize),
    None,
}

/// Returns a BoardState if win/draw, None if game is still going
pub fn check_win_draw(board: &BoardType) -> Option<(BoardState, WinType)> {
    let mut has_empty_slot = false;
    for slot in board {
        match slot.get() {
            BoardState::Empty => {
                has_empty_slot = true;
                break;
            }
            BoardState::Cyan
            | BoardState::CyanWin
            | BoardState::Magenta
            | BoardState::MagentaWin => (),
        }
    }

    let check_result = |state| -> Option<BoardState> {
        match state {
            BoardState::Empty => None,
            BoardState::Cyan | BoardState::CyanWin => Some(BoardState::Cyan),
            BoardState::Magenta | BoardState::MagentaWin => Some(BoardState::Magenta),
        }
    };

    // check horizontals
    for y in 0..(ROWS as usize) {
        for x in 0..((COLS - 3) as usize) {
            let idx = x + y * (COLS as usize);
            let result = check_result(has_right_horizontal_at_idx(idx, board));
            if let Some(result) = result {
                return Some((result, WinType::Horizontal(idx)));
            }
        }
    }

    // check verticals
    for y in 0..((ROWS - 3) as usize) {
        for x in 0..(COLS as usize) {
            let idx = x + y * (COLS as usize);
            let result = check_result(has_down_vertical_at_idx(idx, board));
            if let Some(result) = result {
                return Some((result, WinType::Vertical(idx)));
            }
        }
    }

    // check up diagonals
    for y in 3..(ROWS as usize) {
        for x in 0..((COLS - 3) as usize) {
            let idx = x + y * (COLS as usize);
            let result = check_result(has_right_up_diagonal_at_idx(idx, board));
            if let Some(result) = result {
                return Some((result, WinType::DiagonalUp(idx)));
            }
        }
    }

    // check down diagonals
    for y in 0..((ROWS - 3) as usize) {
        for x in 0..((COLS - 3) as usize) {
            let idx = x + y * (COLS as usize);
            let result = check_result(has_right_down_diagonal_at_idx(idx, board));
            if let Some(result) = result {
                return Some((result, WinType::DiagonalDown(idx)));
            }
        }
    }

    if !has_empty_slot {
        return Some((BoardState::Empty, WinType::None));
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

#[cfg(test)]
mod tests {
    use crate::state::{new_empty_board, BoardState};

    use super::*;

    #[test]
    fn test_horizontal_check() {
        let board = new_empty_board();

        for y in 0..(ROWS as usize) {
            for x in 0..(COLS as usize) {
                assert_eq!(
                    has_right_horizontal_at_idx(x + y * (COLS as usize), &board),
                    BoardState::Empty
                );
            }
        }

        board[50].replace(BoardState::Cyan);
        board[51].replace(BoardState::Cyan);
        board[52].replace(BoardState::Cyan);
        board[53].replace(BoardState::Cyan);

        for y in 0..(ROWS as usize) {
            for x in 0..(COLS as usize) {
                let idx = x + y * (COLS as usize);
                if idx == 50 {
                    assert_eq!(has_right_horizontal_at_idx(idx, &board), BoardState::Cyan);
                } else {
                    assert_eq!(has_right_horizontal_at_idx(idx, &board), BoardState::Empty);
                }
            }
        }

        board[51].replace(BoardState::Magenta);

        board[43].replace(BoardState::Magenta);
        board[44].replace(BoardState::Magenta);
        board[45].replace(BoardState::Magenta);
        board[46].replace(BoardState::Magenta);

        for y in 0..(ROWS as usize) {
            for x in 0..(COLS as usize) {
                let idx = x + y * (COLS as usize);
                if idx == 43 {
                    assert_eq!(
                        has_right_horizontal_at_idx(idx, &board),
                        BoardState::Magenta
                    );
                } else {
                    assert_eq!(has_right_horizontal_at_idx(idx, &board), BoardState::Empty);
                }
            }
        }
    }

    #[test]
    fn test_vertical_check() {
        let board = new_empty_board();

        for y in 0..(ROWS as usize) {
            for x in 0..(COLS as usize) {
                assert_eq!(
                    has_down_vertical_at_idx(x + y * (COLS as usize), &board),
                    BoardState::Empty
                );
            }
        }

        board[30].replace(BoardState::Cyan);
        board[37].replace(BoardState::Cyan);
        board[44].replace(BoardState::Cyan);
        board[51].replace(BoardState::Cyan);

        for y in 0..(ROWS as usize) {
            for x in 0..(COLS as usize) {
                let idx = x + y * (COLS as usize);
                if idx == 30 {
                    assert_eq!(has_down_vertical_at_idx(idx, &board), BoardState::Cyan);
                } else {
                    assert_eq!(has_down_vertical_at_idx(idx, &board), BoardState::Empty);
                }
            }
        }

        board[16].replace(BoardState::Magenta);
        board[23].replace(BoardState::Magenta);
        board[30].replace(BoardState::Magenta);
        board[37].replace(BoardState::Magenta);

        for y in 0..(ROWS as usize) {
            for x in 0..(COLS as usize) {
                let idx = x + y * (COLS as usize);
                if idx == 16 {
                    assert_eq!(has_down_vertical_at_idx(idx, &board), BoardState::Magenta);
                } else {
                    assert_eq!(has_down_vertical_at_idx(idx, &board), BoardState::Empty);
                }
            }
        }
    }

    #[test]
    fn test_upper_diagonal_check() {
        let board = new_empty_board();

        board[44].replace(BoardState::Cyan);
        board[38].replace(BoardState::Cyan);
        board[32].replace(BoardState::Cyan);
        board[26].replace(BoardState::Cyan);

        for y in 0..(ROWS as usize) {
            for x in 0..(COLS as usize) {
                let idx = x + y * (COLS as usize);
                if idx == 44 {
                    assert_eq!(has_right_up_diagonal_at_idx(idx, &board), BoardState::Cyan);
                } else {
                    assert_eq!(has_right_up_diagonal_at_idx(idx, &board), BoardState::Empty);
                }
            }
        }

        board[38].replace(BoardState::Magenta);

        board[28].replace(BoardState::Magenta);
        board[22].replace(BoardState::Magenta);
        board[16].replace(BoardState::Magenta);
        board[10].replace(BoardState::Magenta);

        for y in 0..(ROWS as usize) {
            for x in 0..(COLS as usize) {
                let idx = x + y * (COLS as usize);
                if idx == 28 {
                    assert_eq!(
                        has_right_up_diagonal_at_idx(idx, &board),
                        BoardState::Magenta
                    );
                } else {
                    assert_eq!(has_right_up_diagonal_at_idx(idx, &board), BoardState::Empty);
                }
            }
        }
    }

    #[test]
    fn test_lower_diagonal_check() {
        let board = new_empty_board();

        board[17].replace(BoardState::Cyan);
        board[25].replace(BoardState::Cyan);
        board[33].replace(BoardState::Cyan);
        board[41].replace(BoardState::Cyan);

        for y in 0..(ROWS as usize) {
            for x in 0..(COLS as usize) {
                let idx = x + y * (COLS as usize);
                if idx == 17 {
                    assert_eq!(
                        has_right_down_diagonal_at_idx(idx, &board),
                        BoardState::Cyan
                    );
                } else {
                    assert_eq!(
                        has_right_down_diagonal_at_idx(idx, &board),
                        BoardState::Empty
                    );
                }
            }
        }

        board[25].replace(BoardState::Magenta);

        board[28].replace(BoardState::Magenta);
        board[36].replace(BoardState::Magenta);
        board[44].replace(BoardState::Magenta);
        board[52].replace(BoardState::Magenta);

        for y in 0..(ROWS as usize) {
            for x in 0..(COLS as usize) {
                let idx = x + y * (COLS as usize);
                if idx == 28 {
                    assert_eq!(
                        has_right_down_diagonal_at_idx(idx, &board),
                        BoardState::Magenta
                    );
                } else {
                    assert_eq!(
                        has_right_down_diagonal_at_idx(idx, &board),
                        BoardState::Empty
                    );
                }
            }
        }
    }
}
