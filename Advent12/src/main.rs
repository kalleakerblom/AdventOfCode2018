use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let f = File::open("input").expect("loading failed");
    let buf = BufReader::new(f);
    let mut input: Vec<String> = buf.lines().filter_map(|l| l.ok()).collect();
    let state = input[0].split_whitespace().skip(2).next().unwrap();
    let mut flowers = HashMap::new();
    for (i, c) in state.chars().enumerate() {
        flowers.insert(i as i32, c == '#');
    }

    let mut instructions = HashMap::new();
    for instruction in &input[2..] {
        let mut split = instruction.split(" => ");
        let key: Vec<bool> = split.next().unwrap().chars().map(|c| c == '#').collect();
        let val: bool = split.next().unwrap().trim() == "#";
        if val {
            instructions.insert(key, val);
        }
    }
    let count = flowers.values().filter(|b| **b).count();
    println!("{}", count);

    for _ in 0..200 {
        step(&mut flowers, &mut instructions);
        let ans = flowers
            .iter()
            .filter(|(_, v)| **v)
            .map(|(k, v)| k)
            .sum::<i32>();
        println!("{}", ans);
    }
    println!("{}", (50_000_000_000u64 - 200) * 78 + 17812);
}
fn step(flowers: &mut HashMap<i32, bool>, instructions: &mut HashMap<Vec<bool>, bool>) {
    let mut new_flowers = HashMap::new();
    for x in -3000..3000 {
        let surr = get_surrounding(x, flowers);
        let next_state = instructions.contains_key(&surr);
        new_flowers.insert(x, next_state);
    }
    *flowers = new_flowers;
}
fn get_surrounding(i: i32, flowers: &HashMap<i32, bool>) -> Vec<bool> {
    let mut res: Vec<bool> = Vec::new();
    res.push(*flowers.get(&(i - 2)).unwrap_or(&false));
    res.push(*flowers.get(&(i - 1)).unwrap_or(&false));
    res.push(*flowers.get(&(i)).unwrap_or(&false));
    res.push(*flowers.get(&(i + 1)).unwrap_or(&false));
    res.push(*flowers.get(&(i + 2)).unwrap_or(&false));
    res
}
