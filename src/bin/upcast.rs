#![allow(dead_code)]

trait Obj {
    fn id(&self) -> String;
}

trait Aninal: Obj {
    fn spiece(&self) -> String;
}

struct Cat {
    id: String,
    spiece: String,
}

impl Obj for Cat {
    fn id(&self) -> String {
        "1".to_string()
    }
}

impl Aninal for Cat {
    fn spiece(&self) -> String {
        "cat".to_string()
    }
}

fn print_id(obj: &impl Obj) {
    println!("{}", obj.id());
}

fn main() {
    let a = Cat {
        id: "1".to_string(),
        spiece: "cat".to_string(),
    };

    print_id(&a);
}
