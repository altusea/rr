use inline_option::{IOption, Nullable};
use std::fmt;

// Define a struct that can be null.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Value(i32);

// Implement Display for Value
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Implement the Nullable trait for the struct.
impl Nullable for Value {
    const NULL: Self = Value(i32::MAX);

    fn is_null(&self) -> bool {
        self.0 == i32::MAX
    }
}

fn main() {
    // Create an inline option that can hold a Value.
    let mut opt = IOption::<Value>::none();
    println!("{}", opt.is_none()); // should be true

    // Replace the value in the inline option.
    opt.replace(Value(42));
    println!("{}", opt.is_some()); // should be true

    // Get the value from the inline option.
    let value = opt.unwrap();
    println!("{}", value.0);

    // Or, convert it to a standard option.
    let std_opt = opt.as_ref();
    println!("{:?}", std_opt);
}
