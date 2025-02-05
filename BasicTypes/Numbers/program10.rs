/* -- original = unmodified
// Fill the blanks
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..__), Range{ start: 1, end: 5 });
    assert_eq!((1..__), RangeInclusive::new(1, 5));

    println!("Success!");
}
*/

// Fill the blanks
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

// conclusion 29 = if we were to iterate to the highest number we would have to use = operator
