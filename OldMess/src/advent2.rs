use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use std::io::{BufRead, BufReader, Write};
pub fn main() {
    let f = File::open("input2").expect("loading failed");
    let buf = BufReader::new(f);
    let words: Vec<String> = buf.lines().filter_map(|r| r.ok()).collect();
    let mut threes = 0;
    let mut twos = 0;
    for w in &words {
        let mut char_count: HashMap<char, u32> = HashMap::new();
        for c in w.chars() {
            let mut c_count = char_count.entry(c).or_insert(0);
            *c_count += 1;
        }

        if char_count.iter().any(|(_, v)| *v == 2) {
            twos += 1;
        }
        if char_count.iter().any(|(_, v)| *v == 3) {
            threes += 1;
        }
    }
    println!("Day2Ans1: {}", twos * threes);
    for i in 0..words.len() {
        let word = &words[i];
        let word_len = word.len();
        for other in &words[i + 1..] {
            let shared: String = word
                .chars()
                .zip(other.chars())
                .filter_map(|e| if e.0 == e.1 { Some(e.0) } else { None })
                .collect();
            if shared.len() == word_len - 1 {
                println!("Ans2: {}", shared);
                break;
            }
        }
    }
}
