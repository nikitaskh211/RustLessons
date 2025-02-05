/* -- original = unmodified
// Fill the blank to make it work
fn main() {
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "__".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
*/

// Fill the blank to make it work
fn main() {
    let x: f32 = 1_000.000_1; // for f16 it is slightly bigger so it must be f32
    let _y: f32 = 0.12; // f32
    let _z: f64 = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f32".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// conclusion 23 = f16 has exactly up to a thousand
// conclusion 24 = asserts are mostly c-like static asserts
