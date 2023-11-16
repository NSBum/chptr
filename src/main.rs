use std::env;
use std::io::{self};

fn ucode(character: char) -> String {
    format!("U+{:04X}", character as u32)
}

fn utf8_bytes(character: char) -> String {
    let mut utf8_bytes_hex = String::new();
    for byte in character.to_string().as_bytes() {
        utf8_bytes_hex.push_str(&format!("{:02X} ", byte));
    }
    utf8_bytes_hex.trim_end().to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let text = if args.len() > 1 {
        args[1].clone() // Take the argument directly
    } else {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string() // Trim and convert to a String
    };

    
    for character in text.chars() {
        let unicode_value = ucode(character);
        let utf8_bytes_hex = utf8_bytes(character);
        println!("{} | {} | {}", character, unicode_value, utf8_bytes_hex);
    }    
}
