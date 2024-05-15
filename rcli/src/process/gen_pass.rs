use anyhow::Result;
use rand::seq::SliceRandom; // Import the SliceRandom trait

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const SYMBOL: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?/";
const NUMBER: &[u8] = b"123456789";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut charset = Vec::new();
    if upper {
        charset.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("chars won't be empty"));
    }
    if lower {
        charset.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("chars won't be empty"));
    }
    if symbol {
        charset.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("chars won't be empty"));
    }
    if number {
        charset.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("chars won't be empty"));
    }
    if charset.is_empty() {
        charset.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("chars won't be empty"));
    }
    for _ in 0..(length - password.len() as u8) {
        let c = charset.choose(&mut rng).expect("chars won't be empty");
        password.push(*c);
    }
    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;

    Ok(password)
}
