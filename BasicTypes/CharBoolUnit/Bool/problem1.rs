/* -- original = unchanged
// Make println! work
fn main() {
    let _f: bool = false;

    let t = true;
    if !t {
        println!("Success!");
    }
} 
*/

// Make println! work
fn main() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!");
    }
}

// conclusion 34 = !bool operator does inverse of bool
