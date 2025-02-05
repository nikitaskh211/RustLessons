/* -- original = unmodified
// Remove something to make it work
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x;
    
    let z = 10; // Type of z ? 

    println!("Success!");
}
*/

// Remove something to make it work
fn main() {
    let x: i32 = 5;
    let mut y: i32 = 5;

    y = x;
    
    let z: i32 = 10; // Type of z ? 

    println!("Success!");
}

// conclusion 14 = integers come signed and unsigned (difference is it is either 'i' or 'u') and
// their lenght matches their bit-length 8, 16, 32, 64, 128, arch, where arch is determined by
// architecture
// conclusion 15 = default types for integers and floats are i32 and f64
