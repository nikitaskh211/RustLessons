/* -- original = unmodified
fn main() {
    let x = 1; 
}
*/

/* -- solution # 1
fn main() {
    let _x = 1; 
}
*/

fn main() {
    let _x: i32;
}

// conclusion 10 = if a variable remains unused the compiler will throw a warning, not an error
