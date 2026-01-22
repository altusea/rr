fn get_random_u128() -> Result<u128, getrandom::Error> {
    let mut buf = [0u8; 16];
    getrandom::fill(&mut buf)?;
    Ok(u128::from_ne_bytes(buf))
}

fn main() {
    for _ in 0..10 {
        println!("{}", get_random_u128().unwrap());
    }
}
