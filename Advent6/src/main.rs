use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::i32;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
use text_io::{scan, try_scan};
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Pos(i32, i32);
/// ```
/// assert_eq!(dist(Pos(-1,1),Pos(0,0)),2);
/// ```
fn dist(a: Pos, b: Pos) -> u32 {
    (i32::abs(b.0 - a.0) + i32::abs(b.1 - a.1)) as u32
}
fn closest(curr_pos: &Pos, other_pos: &Vec<Pos>) -> (Option<usize>, u32) {
    let mut min_dist = u32::max_value();
    let mut min_id = None;
    for (i, p) in other_pos.iter().enumerate() {
        let d = dist(*p, *curr_pos);
        if d < min_dist {
            min_dist = d;
            min_id = Some(i);
        } else if d == min_dist {
            min_id = None;
        }
    }
    (min_id, min_dist)
}
fn sum_dist(curr_pos: &Pos, other_pos: &Vec<Pos>) -> u32 {
    other_pos.iter().map(|p| dist(*curr_pos, *p)).sum()
}
pub fn main() {
    assert_eq!(dist(Pos(-1, 1), Pos(0, 0)), 2);
    assert_eq!(dist(Pos(-1, 1), Pos(-1, 0)), 1);
    assert_eq!(dist(Pos(-1, 1), Pos(-1, 1)), 0);

    let f = File::open("input6").expect("loading failed");
    let buf = BufReader::new(f);
    let mut positions = Vec::new();
    for l in buf.lines() {
        let l = l.unwrap();
        let (x, y): (i32, i32);
        scan!(l.bytes()=>"{}, {}",x,y);
        positions.push(Pos(x, y));
    }
    let (mut max_x, mut max_y, mut min_x, mut min_y) = (
        i32::min_value(),
        i32::min_value(),
        i32::max_value(),
        i32::max_value(),
    );
    for Pos(x, y) in &positions {
        max_x = cmp::max(max_x, *x);
        max_y = cmp::max(max_y, *y);
        min_x = cmp::min(min_x, *x);
        min_y = cmp::min(min_y, *y);
    }
    let unbounded: HashSet<Pos> = positions
        .iter()
        .filter(|Pos(x, y)| !(*x < max_x && *x > min_x && *y < max_y && *y > min_y))
        .cloned()
        .collect();
    let mut closest_positions = HashMap::new();
    for _x in min_x..=max_x {
        for _y in min_y..=max_y {
            let close = closest(&Pos(_x, _y), &positions);
            closest_positions.insert((_x, _y), close);
        }
    }
    let mut closest_positions_outside = HashSet::new();
    for _x in vec![min_x - 1, max_x + 1] {
        for _y in min_y - 1..=max_y + 1 {
            let close = closest(&Pos(_x, _y), &positions);
            closest_positions_outside.insert(close.0);
        }
    }
    for _y in vec![min_y - 1, max_y + 1] {
        for _x in min_x - 1..=max_x + 1 {
            let close = closest(&Pos(_x, _y), &positions);
            closest_positions_outside.insert(close.0);
        }
    }
    let mut count_closest: HashMap<usize, u32> = HashMap::new();
    for (_pos, (id, _)) in closest_positions.iter() {
        if let Some(i) = id {
            *count_closest.entry(*i).or_default() += 1;
        }
    }
    let max_area = count_closest
        .iter()
        .filter_map(|(i, c)| {
            if !closest_positions_outside.contains(&Some(*i)) {
                Some(*c)
            } else {
                None
            }
        })
        .max();
    //5820 too high
    println!("ans1: {:?}", max_area);
    let mut acceptable_positions = 0;
    for _x in min_x..=max_x {
        for _y in min_y..=max_y {
            if sum_dist(&Pos(_x, _y), &positions) < 10_000 {
                acceptable_positions += 1;
            }
        }
    }
    println!("ans2: {:?}", acceptable_positions);
}
