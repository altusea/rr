use uuid::Uuid;

fn main() {
    let id: Uuid = Uuid::now_v7();
    println!("{}", id);
    println!(
        "{}",
        match id.get_timestamp() {
            Some(ts) => format!("{:?}", ts),
            None => "".to_string(),
        }
    );
}
