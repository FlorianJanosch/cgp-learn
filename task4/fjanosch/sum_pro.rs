use std::ops::Add;
use std::ops::Mul;

fn main() {
    let x = 5;
    let y = 5;
    let z = sum_pro(x, y);
    println!("Summe: {} Produkt: {}", z.0, z.1);
}

fn sum_pro<T: Add + Mul + Copy>(x: T, y: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    let a = x + y;
    let b = x * y;
    (a, b)
}
