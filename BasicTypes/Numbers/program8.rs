/* -- original = unmodified
fn main() {
    assert!(0.1+0.2==0.3);

    println!("Success!");
}
*/

fn main() {
    assert!(0.1_f32+0.2_f32==0.3_f32); // tricky... floating point precision is not so great

    println!("Success!");
}

// conclusion 25 = floating point precision is like everywhere
// conclusion 26 = to get around this problem less-'precise number types' can be utilized
