use std::ascii;

fn main() {
    let escaped = ascii::escape_default(b'0').next().unwrap();
    println!("{}", escaped);
}
