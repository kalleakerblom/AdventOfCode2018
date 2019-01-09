use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::i32;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
use text_io::{scan, try_scan};
fn cost(state: char) -> u32 {
    let diff = state as u8 - b'A';
    61 + diff as u32
}
fn main() {
    let f = File::open("input").expect("loading failed");
    let buf = BufReader::new(f);
    let mut prereqs = HashMap::new();
    let mut states = HashSet::new();
    for l in buf.lines() {
        //Step D must be finished before step L can begin.
        let l = l.unwrap();
        let (c1, c2): (char, char);
        scan!(l.bytes()=>"Step {} must be finished before step {} can begin.",c1,c2);
        prereqs.entry(c2).or_insert(Vec::new()).push(c1);
        states.insert(c1);
        states.insert(c2);
    }
    let mut state_list: Vec<char> = states.iter().cloned().collect();
    state_list.sort();
    let mut states_on = HashSet::new();
    let mut states_worked = HashSet::new();
    let mut order = Vec::new();
    let mut available_workers = 5;
    let mut time = 0;
    let mut state_change: HashMap<u32, Vec<char>> = HashMap::new();
    while states_on.len() < state_list.len() {
        if state_change.contains_key(&time) {
            //switch on stuff, update order and free workers
            let mut changed_states: Vec<char> = state_change[&time].clone();
            if changed_states.len() > 1 {
                println!("{:?}", changed_states);
            }
            //changed_states.sort();
            for changed_state in changed_states {
                states_on.insert(changed_state);
                order.push(changed_state);
                available_workers += 1;
            }
        }
        for state in &state_list {
            if available_workers == 0 {
                break;
            }
            if states_worked.contains(state) {
                continue;
            }
            if !prereqs.contains_key(state)
                || prereqs[state].iter().all(|pre| states_on.contains(pre))
            {
                let time_cost = cost(*state);
                state_change
                    .entry(time + time_cost)
                    .or_insert(Vec::new())
                    .push(*state);
                available_workers -= 1;
                states_worked.insert(state);
            }
        }
        // if nothing else
        time += 1;
    }

    println!("ans2: {}", time);
}
