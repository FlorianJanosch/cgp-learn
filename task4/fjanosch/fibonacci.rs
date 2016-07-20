use std::iter::Iterator;

fn main() {
    let x = fibonacci::new();
    for i in fibonacci::new().take(20) {
        println!("{}", i);
    }
}

#[derive(Debug)]
struct fibonacci {
    current: u32,
    prev: u32,
}

impl Iterator for fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.prev;
        self.prev = self.current;
        self.current = a + self.prev;
        Some(a)
    }
}

impl fibonacci {
    pub fn new() -> fibonacci {
        fibonacci {
            current: 1,
            prev: 0,
        }
    }
}
