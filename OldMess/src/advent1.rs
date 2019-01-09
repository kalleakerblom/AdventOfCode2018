use std::collections::HashSet;
use std::fs::File;
use std::i32;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};

pub fn main() {
    let f = File::open("input1").expect("loading failed");
    let buf = BufReader::new(f);
    let mut frequency = 0;
    let mut frequency_set = HashSet::new();
    frequency_set.insert(frequency);
    let mut changes = Vec::new();
    for l in buf.lines() {
        let mut l = l.unwrap();
        let num = i32::from_str_radix(&l, 10).unwrap();
        changes.push(num);
    }
    'outer: loop {
        for n in &changes {
            frequency += n;
            if !frequency_set.insert(frequency) {
                break 'outer;
            }
        }
    }
    println!("{}", frequency);
}
