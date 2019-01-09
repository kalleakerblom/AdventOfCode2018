use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
pub fn main() {
    let mut f = File::open("input5").expect("loading failed");
    let mut input = String::new();
    f.read_to_string(&mut input);
    let test = "dabAcCaCBAcCcaDA".to_string();
    let mut min_len = 500000;
    for letter in (b'a'..=b'z').map(char::from) {
        let mut chars: Vec<_> = input
            .chars()
            .filter(|c| c.is_alphabetic() && !c.eq_ignore_ascii_case(&letter))
            .collect();
        'outer: loop {
            let len = chars.len();
            for i in 0..(len - 1) {
                let c1 = chars[i] as char;
                let c2 = chars[i + 1] as char;
                let opposite_case = c1.is_uppercase() == c2.is_lowercase();
                if opposite_case && c1.eq_ignore_ascii_case(&c2) {
                    chars.drain(i..=i + 1);
                    continue 'outer;
                }
            }
            break;
        }
        println!("{:?}", chars.len());
        min_len = cmp::min(min_len, chars.len());
    }
    println!("ans2: {}", min_len);
}
