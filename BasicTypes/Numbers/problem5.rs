/* -- original = unmodified
// Fix errors and panics to make it work
fn main() {
   let v1 = 251_u8 + 8;
   let v2 = i8::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}
*/

// Fix errors and panics to make it work
fn main() {
   let v1: u16 = 251_u16 + 8; // in theory 251_u8 is close to the u8::MAX so we need at least u16
   let v2: i16 = i16::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}

// conclusion 19 = overflow is possible but safe
// conclusion 20 = overflow can be avoided using bigger datatypes
