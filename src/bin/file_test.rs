use std::io::{Read, Write};
use tempfile::tempfile;

fn main() {
    let mut temp_file = tempfile().unwrap();
    temp_file.write_all("Rust\n💖\nFun".as_ref());

    let mut buffer = String::new();
    let res = temp_file.read_to_string(&mut buffer).unwrap();
    println!("{}", buffer)
    
    
}