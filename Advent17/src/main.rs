#![allow(dead_code)]
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
use text_io::{read, scan, try_read, try_scan};
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Pos(u32, u32);
fn main() {
    let f = File::open("input").expect("loading failed");
    let buf = BufReader::new(f);
    let mut clay = HashSet::new();
    for l in buf.lines() {
        let l = l.unwrap();
        let (first, second, a, b, c): (char, char, u32, u32, u32);
        scan!(l.bytes()=>"{}={}, {}={}..{}",first,a,second,b,c);
        match first {
            'x' => {
                let x = a;
                for y in b..=c {
                    clay.insert(Pos(x, y));
                }
            }
            'y' => {
                let y = a;
                for x in b..=c {
                    clay.insert(Pos(x, y));
                }
            }
            _ => panic!(),
        }
    }
    let y_max = clay.iter().map(|Pos(_, y)| y).max().expect("no max");
    let y_min = clay.iter().map(|Pos(_, y)| y).min().expect("no min");
    let mut water: HashSet<Pos> = HashSet::new();
    let mut stationary: HashSet<Pos> = HashSet::new();
    stationary.extend(clay.iter());
    let mut frontier = Vec::new();
    frontier.push(Pos(500, 0));
    water.insert(Pos(500, 0));
    'next_frontier: while !frontier.is_empty() {
        //let water travel, spawn new frontiers
        let mut current = frontier.pop().expect("No frontier");
        while open_below(current, &stationary) {
            current = fall(current);
            water.insert(current);
            if current.1 > *y_max {
                continue 'next_frontier;
            }
        }
        use self::SidewayEnd::*;
        let (ref left_end, ref right_end) = sideways_flow(current, &stationary, &mut water);
        if let (Closed(left), Closed(right)) = (left_end, right_end) {
            for x in left.0..=right.0 {
                stationary.insert(Pos(x, current.1));
            }
            frontier.push(Pos(current.0, current.1 - 1));
        } else {
            if let Open(left) = left_end {
                if !water.contains(&fall(*left)) {
                    frontier.push(*left);
                }
            }
            if let Open(right) = right_end {
                if !water.contains(&fall(*right)) {
                    frontier.push(*right);
                }
            }
        }
    }
    let count = water
        .iter()
        .filter(|Pos(_, y)| y >= y_min && y <= y_max)
        .count();
    // 34791 too low
    println!("ans1 {}", count);
    let count2 = water.iter().filter(|pos| stationary.contains(&pos)).count();
    // 34791 too low
    println!("ans2 {}", count2);
}
enum SidewayEnd {
    Open(Pos),
    Closed(Pos),
}
fn fall(pos: Pos) -> Pos {
    let Pos(x, y) = pos;
    Pos(x, y + 1)
}
fn left(pos: Pos) -> Pos {
    let Pos(x, y) = pos;
    Pos(x - 1, y)
}
fn right(pos: Pos) -> Pos {
    let Pos(x, y) = pos;
    Pos(x + 1, y)
}

fn sideways_flow(
    pos: Pos,
    stationaries: &HashSet<Pos>,
    water: &mut HashSet<Pos>,
) -> (SidewayEnd, SidewayEnd) {
    use self::SidewayEnd::*;
    let (mut left_end, mut right_end) = (Closed(pos), Closed(pos));
    //check left side
    let mut current = pos;
    loop {
        water.insert(current);
        let next = left(current);
        if open_below(next, stationaries) {
            left_end = Open(next);
            water.insert(next);
            break;
        } else if stationaries.contains(&next) {
            left_end = Closed(current);
            break;
        } else {
            current = next;
        }
    }
    //check right side
    current = pos;
    loop {
        water.insert(current);
        let next = right(current);
        if open_below(next, stationaries) {
            right_end = Open(next);
            water.insert(next);
            break;
        } else if stationaries.contains(&next) {
            right_end = Closed(current);
            break;
        } else {
            current = next;
        }
    }
    (left_end, right_end)
}
fn open_below(Pos(x, y): Pos, stationaries: &HashSet<Pos>) -> bool {
    !stationaries.contains(&Pos(x, y + 1))
}
