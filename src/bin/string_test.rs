fn main() {
    // write exanples of strings in rust
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = "hello";
    let s2 = s1;
    println!("{}", s2);
}
