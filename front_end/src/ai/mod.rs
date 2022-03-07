use crate::state::BoardType;

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
}

pub fn get_ai_choice(difficulty: AIDifficulty, board: &BoardType) -> Result<SlotChoice, String> {
    Err("Unimplemented".into())
}
