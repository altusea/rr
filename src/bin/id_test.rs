use ulid::Ulid;

fn main() {
    let ulid = Ulid::new();
    let s = ulid.to_string(); // generate a string for a ulid
    println!("{}", s);
}
