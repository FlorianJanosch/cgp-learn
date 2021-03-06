use std::ops::{Add, Mul};

fn main() {
    let z = sum_pro(5, 5);
    println!("Summe: {} Produkt: {}", z.0, z.1);
}

// Output ist dann der assozierte Typ der Addition
// (der in der Implementation vorgegeben ist)
// Bspw. können 2 u32 bei einer Div einen f32 ergeben
// Output ist dann dieser "Ausgabetyp"
fn sum_pro<T: Add + Mul + Copy>(x: T, y: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    (x + y, x * y)
}
