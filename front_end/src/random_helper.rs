use js_sys::Math::random;
use oorandom::Rand32;

pub fn get_seeded_random() -> Result<Rand32, String> {
    Ok(Rand32::new((random() * u64::MAX as f64) as u64))
}
