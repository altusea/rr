#[macro_use]
extern crate maplit;

fn main() {
    let map = hashmap! {
        "a" => 1,
        "b" => 2,
    };
    println!("{}", map.capacity());
    let set = hashset! {12, 24, 36};
    println!("{}", set.capacity());
}
