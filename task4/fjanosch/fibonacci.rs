use std::iter::Iterator;

fn main() {
    let x = Fibonacci::new();
    for i in Fibonacci::new().take(20) {
        println!("{}", i);
    }
}

#[derive(Debug)]
struct Fibonacci {
    current: u32,
    prev: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.prev;
        self.prev = self.current;
        self.current = a + self.prev;
        Some(a)
    }
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci {
            current: 1,
            prev: 0,
        }
    }
}
