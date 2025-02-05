/* -- original = unchanged
// Solve it in two ways
// DON'T let `println!` work
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    
}
*/

// Solve it in two ways
// DON'T let `println!` work
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!();
}

// conclusion 45 = ! is for 'divergent' functions which have no end
// conclusion 46 = panic!() macro makes the program panic and stop execution
// conclusion 47 = in this example the println!("") statement is unreachable
