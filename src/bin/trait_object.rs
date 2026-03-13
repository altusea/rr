#![allow(dead_code)]

trait Draw {
    fn draw(&self);
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing a button: {}x{} with label '{}'",
            self.width, self.height, self.label
        );
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing a select box: {}x{} with {} options",
            self.width,
            self.height,
            self.options.len()
        );
    }
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn new() -> Self {
        Screen {
            components: Vec::new(),
        }
    }

    fn add_component(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

trait Animal {
    fn make_sound(&self);
    fn name(&self) -> &str;
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("{} says: Woof!", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("{} says: Meow!", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }
}

fn let_them_speak(animals: &[Box<dyn Animal>]) {
    for animal in animals {
        animal.make_sound();
    }
}

fn main() {
    let mut screen = Screen::new();
    screen.add_component(Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
    }));
    screen.add_component(Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
        ],
    }));
    screen.run();

    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: String::from("Buddy"),
        }),
        Box::new(Cat {
            name: String::from("Whiskers"),
        }),
        Box::new(Dog {
            name: String::from("Max"),
        }),
    ];
    let_them_speak(&animals);
}
