use chrono::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
use text_io::{scan, try_scan};

enum Event {
    Wake,
    Sleep,
    Guard(String),
}
pub fn main() {
    let f = File::open("input4").expect("loading failed");
    let buf = BufReader::new(f);
    let mut events = Vec::new();
    for l in buf.lines() {
        let l = l.unwrap();
        let mut split = l.split(']');

        let (month, day, hour, minute): (u32, u32, u32, u32);
        scan!(split.next().unwrap().bytes()=> "[1518-{}-{} {}:{}",month,day,hour,minute);
        let dt = Utc.ymd(1518, month, day).and_hms(hour, minute, 0);
        let event_string: String = split.next().unwrap().to_string();
        let mut event_split = event_string.split_whitespace();
        match event_split.next().unwrap() {
            "wakes" => {
                events.push((dt, Event::Wake));
            }
            "falls" => {
                events.push((dt, Event::Sleep));
            }
            "Guard" => {
                let id = event_split.next().unwrap();
                events.push((dt, Event::Guard(id.to_string())));
            }
            _ => panic!(),
        }
    }
    events.sort_by_key(|p| p.0);
    let mut guard_sleep_minutes = HashMap::new();
    let mut guard = String::new();
    let mut start_min = 0;
    let mut end_min = 0;
    for (dt, event) in events {
        match event {
            Event::Guard(id) => guard = id,
            Event::Sleep => start_min = dt.minute(),
            Event::Wake => {
                end_min = dt.minute();
                let sleep = guard_sleep_minutes
                    .entry(guard.clone())
                    .or_insert(HashMap::new());
                for m in start_min..end_min {
                    *sleep.entry(m).or_insert(0) += 1;
                }
            }
        }
    }
    let mut guards_minutes: Vec<_> = guard_sleep_minutes
        .iter()
        .map(|(k, v)| (k, v.values().sum::<u32>()))
        .collect();
    guards_minutes.sort_by_key(|p| p.1);
    guards_minutes
        .iter()
        .rev()
        .take(5)
        .for_each(|p| println!("{:?}", p));
    let minute = guard_sleep_minutes
        .get("#3023")
        .unwrap()
        .iter()
        .max_by_key(|(_, v)| **v)
        .unwrap();
    println!("ans1:{:?}", minute);
    let frequent_minute = guard_sleep_minutes
        .iter()
        .map(|(k, v)| (k, v.iter().max_by_key(|(m, count)| *count).unwrap()))
        .max_by_key(|(k, m)| m.1);

    //152204 too high
    // 97884
    println!("ans2:{:?}", frequent_minute);
}
