use oorandom::Rand32;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_seeded_random() -> Result<Rand32, String> {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("{}", e))?;

    Ok(Rand32::new(duration.as_secs()))
}
