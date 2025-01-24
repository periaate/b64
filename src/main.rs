use b64::*;

fn main() {
    let val: [u8; 11] = [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    println!("{}", to_str(&val));
    println!("{}", b64_to_string_safe(&val));
}
