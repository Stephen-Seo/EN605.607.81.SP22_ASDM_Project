use oorandom::Rand32;
use rand::prelude::*;

pub fn get_seeded_random() -> Result<Rand32, String> {
    Ok(Rand32::new(thread_rng().gen()))
}
