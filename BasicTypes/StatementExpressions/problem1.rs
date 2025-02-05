/* -- original = unchanged
// Make it work with two ways
fn main() {
   let v = {
       let mut x = 1;
       x += 2
   };

   assert_eq!(v, 3);

   println!("Success!");
}
*/

// Make it work with two ways
fn main() {
    let v = {
        let mut x = 1; // x = 1
        x += 2; // x = 3
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}

// conclusion 37 = you can assign expressions within variables
