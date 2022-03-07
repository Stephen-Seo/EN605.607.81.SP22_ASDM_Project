use std::collections::BTreeMap;

use crate::constants::{AI_EASY_MAX_CHOICES, AI_NORMAL_MAX_CHOICES, COLS, ROWS};
use crate::state::{BoardState, BoardType, Turn};

use rand::{thread_rng, Rng};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AIDifficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SlotChoice {
    Slot0,
    Slot1,
    Slot2,
    Slot3,
    Slot4,
    Slot5,
    Slot6,
    Invalid,
}

impl From<SlotChoice> for usize {
    fn from(slot_choice: SlotChoice) -> Self {
        match slot_choice {
            SlotChoice::Slot0 => 0,
            SlotChoice::Slot1 => 1,
            SlotChoice::Slot2 => 2,
            SlotChoice::Slot3 => 3,
            SlotChoice::Slot4 => 4,
            SlotChoice::Slot5 => 5,
            SlotChoice::Slot6 => 6,
            SlotChoice::Invalid => 10,
        }
    }
}

impl From<usize> for SlotChoice {
    fn from(idx: usize) -> Self {
        if idx >= (ROWS * COLS) as usize {
            return SlotChoice::Invalid;
        }

        match idx % (COLS as usize) {
            0 => SlotChoice::Slot0,
            1 => SlotChoice::Slot1,
            2 => SlotChoice::Slot2,
            3 => SlotChoice::Slot3,
            4 => SlotChoice::Slot4,
            5 => SlotChoice::Slot5,
            6 => SlotChoice::Slot6,
            _ => SlotChoice::Invalid,
        }
    }
}

pub fn get_ai_choice(
    difficulty: AIDifficulty,
    player: Turn,
    board: &BoardType,
) -> Result<SlotChoice, String> {
    let mut utilities = Vec::with_capacity(COLS as usize);
    for i in 0..(COLS as usize) {
        let slot = i.into();
        if slot == SlotChoice::Invalid {
            return Err("Internal error: get_ai_choice() iterated to SlotChoice::Invalid".into());
        }
        if let Some(utility) = get_utility_for_slot(player, slot, board) {
            utilities.push(utility);
        }
    }

    let pick_some_of_choices = |amount: usize| -> Result<SlotChoice, String> {
        let mut maximums: BTreeMap<i64, usize> = BTreeMap::new();
        for (idx, utility) in utilities.iter().enumerate() {
            if *utility <= 0.0 {
                continue;
            }
            maximums.insert((utility * 10000.0) as i64, idx);
        }
        let mod_amount = if maximums.len() < amount {
            maximums.len()
        } else {
            amount
        };
        let random_number: usize = thread_rng().gen::<usize>() % mod_amount;
        let rand_idx = maximums.len() - 1 - random_number;
        // turns the map into a vector of (key, value), then pick out of the
        // last few values by index the "value" which is the SlotChoice.
        Ok((*maximums.iter().collect::<Vec<(&i64, &usize)>>()[rand_idx].1).into())
    };

    match difficulty {
        AIDifficulty::Easy => pick_some_of_choices(AI_EASY_MAX_CHOICES),
        AIDifficulty::Normal => pick_some_of_choices(AI_NORMAL_MAX_CHOICES),
        AIDifficulty::Hard => {
            // only pick the best option all the time
            let mut max = 0.0f64;
            let mut max_idx: usize = 0;
            for (idx, utility) in utilities.iter().enumerate() {
                if *utility > max {
                    max = *utility;
                    max_idx = idx;
                }
            }
            Ok(max_idx.into())
        }
    }
}

/// Returns a value between 0.0 and 1.0 where 1.0 is highest utility
/// "None" indicates it is impossible to place at the given slot
fn get_utility_for_slot(player: Turn, slot: SlotChoice, board: &BoardType) -> Option<f64> {
    // get idx of location where dropped token will reside in
    let mut idx: usize = slot.into();
    if board[idx].get() != BoardState::Empty {
        // slot is full, cannot place in slot
        return None;
    }
    while idx < (ROWS * COLS) as usize && board[idx + COLS as usize].get() == BoardState::Empty {
        idx += COLS as usize;
    }

    // check if placing a token here blocks a win
    if get_block_win(player, idx, board) {
        return Some(1.0);
    }

    // TODO more impl here

    Some(0.0)
}

/// Returns true if placing a token at idx will block the opposite player from winning
fn get_block_win(player: Turn, idx: usize, board: &BoardType) -> bool {
    let opposite = player.get_opposite();

    // setup for checks
    let mut count = 0;
    let mut temp_idx = idx;

    // check left
    while temp_idx % (COLS as usize) > 0 {
        temp_idx -= 1;
        if board[temp_idx].get() == opposite.into() {
            count += 1;
            if count >= 3 {
                return true;
            }
        } else {
            break;
        }
    }

    // check right
    count = 0;
    temp_idx = idx;
    while temp_idx % (COLS as usize) < (COLS - 1) as usize {
        temp_idx += 1;
        if board[temp_idx].get() == opposite.into() {
            count += 1;
            if count >= 3 {
                return true;
            }
        } else {
            break;
        }
    }

    // check down
    count = 0;
    temp_idx = idx;
    while temp_idx / (COLS as usize) < (ROWS - 1) as usize {
        temp_idx += COLS as usize;
        if board[temp_idx].get() == opposite.into() {
            count += 1;
            if count >= 3 {
                return true;
            }
        } else {
            break;
        }
    }

    // check diagonal left down
    count = 0;
    temp_idx = idx;
    while temp_idx % (COLS as usize) > 0 && temp_idx / (COLS as usize) < (ROWS - 1) as usize {
        temp_idx = temp_idx - 1 + COLS as usize;
        if board[temp_idx].get() == opposite.into() {
            count += 1;
            if count >= 3 {
                return true;
            }
        } else {
            break;
        }
    }

    // check diagonal right down
    count = 0;
    temp_idx = idx;
    while temp_idx % (COLS as usize) < (COLS - 1) as usize
        && temp_idx / (COLS as usize) < (ROWS - 1) as usize
    {
        temp_idx = temp_idx + 1 + COLS as usize;
        if board[temp_idx].get() == opposite.into() {
            count += 1;
            if count >= 3 {
                return true;
            }
        } else {
            break;
        }
    }

    // check diagonal left up
    count = 0;
    temp_idx = idx;
    while temp_idx % (COLS as usize) > 0 && temp_idx / (COLS as usize) > 0 {
        temp_idx = temp_idx - 1 - COLS as usize;
        if board[temp_idx].get() == opposite.into() {
            count += 1;
            if count >= 3 {
                return true;
            }
        } else {
            break;
        }
    }

    // check diagonal right up
    count = 0;
    temp_idx = idx;
    while temp_idx % (COLS as usize) < (COLS - 1) as usize && temp_idx / (COLS as usize) > 0 {
        temp_idx = temp_idx + 1 - COLS as usize;
        if board[temp_idx].get() == opposite.into() {
            count += 1;
            if count >= 3 {
                return true;
            }
        } else {
            break;
        }
    }

    // exhausted all possible potential wins, therefore does not block a win
    false
}
