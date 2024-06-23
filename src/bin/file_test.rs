use std::fs::File;
use std::io::{Read, Write};
use tempfile::NamedTempFile;

fn main() {
    let mut temp_file = NamedTempFile::new().unwrap();
    // write
    temp_file.write_all("Rust\n💖\nFun".as_ref()).unwrap();
    let temp_file_path = temp_file.path();

    // read
    let mut file = File::open(temp_file_path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    println!("{}", buffer)
}
