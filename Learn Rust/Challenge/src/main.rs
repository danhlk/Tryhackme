#![allow(non_snake_case)]

extern crate base64;
extern crate std;

use std::fs::File;
use std::io::Read;

// decrypt rot13 string
pub fn rot13 (cipher: String) -> String {
    let mut result = vec![];

    for c in cipher.chars() {
        let c_code = c as u8;

        if c.is_uppercase() {
            result.push(((c_code + 13 - 65) % 26 + 65) as char);
        }
        else if c.is_lowercase() {
            result.push(((c_code + 13 - 97) % 26 + 97) as char);
        }
        else if c == '\n' {
            continue;
        }
        else {
            result.push(c);
        }
    }
    return result.iter().collect::<String>();
}

// use external package base64 to decode base64 string
pub fn decode_base64 (cipher: String) -> String {
    let bytes = base64::decode(cipher.clone()).unwrap();
    let result = bytes.iter().map(|&e| e as char).collect::<Vec<char>>();
    return result.iter().collect::<String>();
}

fn main () {
    // read content of attached file
    let mut file = File::open("/media/ubuntu/PRACTICE/Github/Tryhackme/Learn Rust/chal.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file.");
    
    println!("{:?}", rot13(decode_base64(rot13(contents))).to_ascii_uppercase());
}
