use std::ops::{Add, Mul};

fn main() {
    let x = 5;
    let y = 5;
    let z = sum_pro(x, y);
    println!("Summe: {} Produkt: {}", z.0, z.1);
}

// Output ist dann der assozierte Typ der Addition
// (der in der Implementation vorgegeben ist)
// Bspw. können 2 u32 bei einer Div einen f32 ergeben
// Output ist dann dieser "Ausgabetyp"
fn sum_pro<T: Add + Mul + Copy>(x: T, y: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    let a = x + y;
    let b = x * y;
    (a, b)
}
