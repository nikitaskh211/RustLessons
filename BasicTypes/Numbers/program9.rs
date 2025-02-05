/* -- original = unmodified
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}",c);
    }
}
*/

fn main() {
    let mut sum = 0;
    for i in -3..2 { // this goes from -3 to 1 in reality
        sum += i // - 3 - 2 - 1 + 0 + 1 = - 5
    }

    assert!(sum == -5);

    for c in 'a'..='z' { // we could look ascii table to lookup the numbers
        println!("{}",c as u8);
    }
}

// conclusion 27 = for-cycles are like in fortran
// conclusion 28 = indexing within cycles is like c/cpp since it is off by 1
