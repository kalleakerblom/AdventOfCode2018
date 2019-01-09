#![allow(dead_code)]
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
#[derive(PartialEq, Eq, Hash)]
struct Pos(usize, usize);
#[derive(PartialEq, Eq, Clone, Copy)]
enum Acre {
    Tree,
    Lumberyard,
    Ground,
}
fn get_surrounding(pos: Pos, map: &HashMap<Pos, Acre>) -> Vec<Acre> {
    let mut surrounding = Vec::new();
    let Pos(x, y) = pos;
    let xmin = if x == 0 { 0 } else { x - 1 };
    let ymin = if y == 0 { 0 } else { y - 1 };
    let xmax = if x == 49 { 49 } else { x + 1 };
    let ymax = if y == 49 { 49 } else { y + 1 };
    for _x in xmin..=xmax {
        for _y in ymin..=ymax {
            if Pos(_x, _y) == pos {
                continue;
            }
            surrounding.push(map[&Pos(_x, _y)]);
        }
    }
    surrounding
}
fn calc_next_state(current: Acre, near: &Vec<Acre>) -> Acre {
    let lumber_count = near.iter().filter(|n| **n == Acre::Lumberyard).count();
    let tree_count = near.iter().filter(|n| **n == Acre::Tree).count();
    match current {
        Acre::Tree => {
            if lumber_count >= 3 {
                Acre::Lumberyard
            } else {
                Acre::Tree
            }
        }
        Acre::Lumberyard => {
            if lumber_count >= 1 && tree_count >= 1 {
                Acre::Lumberyard
            } else {
                Acre::Ground
            }
        }
        Acre::Ground => {
            if tree_count >= 3 {
                Acre::Tree
            } else {
                Acre::Ground
            }
        }
    }
}
fn main() {
    let f = File::open("input").expect("loading failed");
    let buf = BufReader::new(f);
    let mut map: HashMap<Pos, Acre> = HashMap::new();
    for (y, l) in buf.lines().enumerate() {
        let l = l.unwrap();
        for (x, c) in l.chars().enumerate() {
            let acre = match c {
                '|' => Acre::Tree,
                '.' => Acre::Ground,
                '#' => Acre::Lumberyard,
                symbol @ _ => panic!("bad symbol: ({})", symbol),
            };
            map.insert(Pos(x, y), acre);
        }
    }
    // 1_000_000_000
    for i in 1..=1_000_000_000 {
        let mut new_map = HashMap::new();
        for y in 0..50 {
            for x in 0..50 {
                let current = map[&Pos(x, y)];
                let near = get_surrounding(Pos(x, y), &map);
                new_map.insert(Pos(x, y), calc_next_state(current, &near));
            }
        }
        map = new_map;
        let lumber_count = map.values().filter(|n| **n == Acre::Lumberyard).count();
        let tree_count = map.values().filter(|n| **n == Acre::Tree).count();
        // 214292 too low
        // 208962 too low
        // 213239 too high
        println!("{}: {}", i, lumber_count * tree_count);
    }
}
