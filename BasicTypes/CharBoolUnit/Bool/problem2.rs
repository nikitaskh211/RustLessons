/* -- original = unchanged
// Make it work
fn main() {
    let f = true;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}
*/

// Make it work
fn main() {
    let f = true;
    let t = true && f;
    assert_eq!(t, f);

    println!("Success!");
}

// conclusion 35 = 
