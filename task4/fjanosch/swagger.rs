use std::fmt::{Display, Formatter, Result};

fn main() {
    // let a: i32 = 42;
    let s = Swagger {42};
    println!("{}", s);
}

#[derive(Debug)]
struct Swagger<T> {
    x: T,
}

impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Wenn man jetzt mit x noch etwas machen wollen würde,
        // müsste man Copy implementieren und dann je bspw. noch Add
        write!(f, "#swag {} #yolo", self.x)
    }
}
