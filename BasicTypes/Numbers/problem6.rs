/* -- original = unmodified
// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1579);

    println!("Success!");
}
*/

// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // in order dec, hex, opt, bin
    assert!(v == 1_024 + 0xff + 0o77 + 0b1111_1111);

    println!("Success!");
}

// conclusion 21 = delimiter of _ for decimal is simillar to c/cpp, it's just for readability
// conclusion 22 = numbers can be represented using any variant of notation
