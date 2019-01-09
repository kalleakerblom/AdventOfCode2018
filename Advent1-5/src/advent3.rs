use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
use text_io::{scan, try_scan};
pub fn main() {
    let f = File::open("input3").expect("loading failed");
    let buf = BufReader::new(f);
    let mut hm = HashMap::new();
    for l in buf.lines() {
        let l = l.unwrap();
        let (id, x, y, w, h): (u32, u32, u32, u32, u32);
        scan!(l.bytes() => "#{} @ {},{}: {}x{}",id,x,y,w,h);
        for _x in x..x + w {
            for _y in y..y + h {
                let mut ids = hm.entry((_x, _y)).or_insert(Vec::new());
                ids.push(id);
            }
        }
    }
    let ans = hm.values().filter(|v| v.len() > 1).count();
    println!("Day3:Ans1: {}", ans);
    let mut overlapping: HashSet<u32> = HashSet::new();
    hm.values()
        .filter(|v| v.len() > 1)
        .for_each(|v| overlapping.extend(v));
    for i in 1..=overlapping.len() as u32 {
        if !overlapping.contains(&i) {
            println!("Day3:Ans2: {}", i);
            break;
        }
    }
}
