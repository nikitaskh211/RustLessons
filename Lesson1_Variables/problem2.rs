/* -- original code = unsolved
// Fill the blanks in the code to make it compile
fn main() {
    let __ __ = 1;
    __ += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}
*/

// Fill the blanks in the code to make it compile
fn main() {
    let mut x = 1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}

// conclusion 3 = variables cannot mutate by default
// conclusion 4 = if a variable must change use mut keyword to make it mutuable
