#![allow(dead_code)]
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
#[derive(Clone, Copy, Debug)]
enum Dir {
    N,
    W,
    E,
    S,
}
fn parse_dir(c: char) -> Dir {
    match c {
        'N' => Dir::N,
        'W' => Dir::W,
        'E' => Dir::E,
        'S' => Dir::S,
        _ => panic!("bad dir"),
    }
}
#[derive(Debug)]
enum Route {
    Door(Dir),
    OrRoutes(Vec<Vec<Route>>),
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Pos(i32, i32);
fn main() {
    let f = File::open("input").expect("loading failed");
    let mut buf = BufReader::new(f);
    let mut input = String::new();
    buf.read_to_string(&mut input);
    let chars: Vec<char> = input.chars().collect();
    let mut door_map = HashMap::<Pos, Vec<Dir>>::new();
    // skip first char '^'
    let (full_route, _) = parse_route(&chars[1..]);
    let mut pos = Pos(0, 0);
    traverse(&full_route[0], &mut pos, &mut door_map);
    // door map filled, find room furthest away
    let mut frontier: VecDeque<Pos> = vec![Pos(0, 0)].into_iter().collect();
    let mut came_from: HashMap<Pos, Option<Pos>> = HashMap::new();
    came_from.insert(Pos(0, 0), None);
    let mut current = Pos(0, 0);
    while !frontier.is_empty() {
        current = frontier.pop_front().unwrap();
        for door in &door_map[&current] {
            let next = move_dir(current, *door);
            if !came_from.contains_key(&next) {
                frontier.push_back(next);
                came_from.insert(next, Some(current));
            }
        }
    }
    println!("last pos: {:?}", current);

    println!("ans1: {}", steps_to_origin(current, &came_from));
    let count2 = door_map
        .keys()
        .filter(|pos| steps_to_origin(**pos, &came_from) >= 1000)
        .count();
    // 1831 too low
    println!("ans2: {}", count2);
}
fn steps_to_origin(pos: Pos, came_from: &HashMap<Pos, Option<Pos>>) -> u32 {
    let mut count = 0;
    let mut current = pos;
    while let Some(from) = came_from[&current] {
        count += 1;
        current = from;
    }
    count
}
fn traverse(routes: &[Route], pos: &mut Pos, door_map: &mut HashMap<Pos, Vec<Dir>>) {
    use self::Route::*;
    for r in routes {
        match r {
            Door(dir) => {
                let new_pos = move_dir(*pos, *dir);
                door_map.entry(*pos).or_default().push(*dir);
                door_map
                    .entry(new_pos)
                    .or_default()
                    .push(opposite_dir(*dir));
                *pos = new_pos;
            }
            OrRoutes(or_routes) => {
                for or_route in or_routes {
                    let mut curr_pos = *pos;
                    //TODO: only work if all or_routes end same place
                    traverse(or_route, &mut curr_pos, door_map);
                }
            }
        }
    }
}
fn move_dir(pos: Pos, dir: Dir) -> Pos {
    let Pos(x, y) = pos;
    use self::Dir::*;
    match dir {
        N => Pos(x, y + 1),
        S => Pos(x, y - 1),
        W => Pos(x - 1, y),
        E => Pos(x + 1, y),
    }
}
fn opposite_dir(dir: Dir) -> Dir {
    use self::Dir::*;
    match dir {
        N => S,
        S => N,
        W => E,
        E => W,
    }
}
fn parse_route(input: &[char]) -> (Vec<Vec<Route>>, usize) {
    let mut head = 0;
    use self::Route::*;
    let mut result = Vec::new();
    let mut current_route = Vec::new();
    loop {
        // ( ) $ |
        match input[head] {
            '(' => {
                let (sub_res, sub_head) = parse_route(&input[head + 1..]);
                current_route.push(OrRoutes(sub_res));
                head += sub_head + 2;
            }
            '$' | ')' => {
                result.push(current_route);
                return (result, head);
            }
            '|' => {
                result.push(current_route);
                current_route = Vec::new();
                head += 1;
            }
            c => {
                current_route.push(Door(parse_dir(c)));
                head += 1;
            }
        }
    }
}
