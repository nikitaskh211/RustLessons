/* -- original = unchanged
// Make it work
fn main() {
    let c1 = "中";
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}
*/

// Make it work
fn main() {
    let c1: char = '中';
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}

// conclusion 32 = compiler makes a default an assumption that we initialize with string using ""
// conclusion 33 = to make it a character we use '' (single-quotes)
