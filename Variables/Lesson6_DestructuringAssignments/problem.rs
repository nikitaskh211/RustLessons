/* -- original = unmodified
fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], __);

    println!("Success!");
} 
*/

fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
}

// conclusion 12 = double dots are for blank variables
// conclusion 13 = we do not need 'mut' keyword for tuple, slice and struct on the lhs
