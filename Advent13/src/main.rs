#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
#[derive(PartialEq, Eq, Debug)]
enum Tile {
    UpDown,
    LeftRight,
    DownLeft,
    DownRight,
    Cross,
}
#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Copy, Debug)]

enum CarState {
    Left,
    Straight,
    Right,
}
#[derive(Clone, Copy, Debug)]

struct Car {
    dir: Dir,
    state: CarState,
}
impl Car {
    fn new(dir: Dir) -> Car {
        Car {
            dir,
            state: CarState::Left,
        }
    }
    fn dir_after_cross(&mut self) -> Dir {
        let next_dir = match self.state {
            CarState::Left => turn_left(self.dir),
            CarState::Right => turn_right(self.dir),
            CarState::Straight => self.dir,
        };
        self.next_state();
        next_dir
    }
    fn next_state(&mut self) {
        match self.state {
            CarState::Left => self.state = CarState::Straight,
            CarState::Straight => self.state = CarState::Right,
            CarState::Right => self.state = CarState::Left,
        }
    }
}
fn parse_row(
    row: usize,
    s: &str,
    map: &mut HashMap<(usize, usize), Tile>,
    cars: &mut HashMap<(usize, usize), Car>,
) {
    // FIXME: parse all the symbols!
    for (col, c) in s.chars().enumerate() {
        match c {
            '/' => {
                map.insert((col, row), Tile::DownLeft);
            }
            '\\' => {
                map.insert((col, row), Tile::DownRight);
            }
            '+' => {
                map.insert((col, row), Tile::Cross);
            }
            '|' => {
                map.insert((col, row), Tile::UpDown);
            }
            '-' => {
                map.insert((col, row), Tile::LeftRight);
            }
            'v' => {
                map.insert((col, row), Tile::UpDown);
                cars.insert((col, row), Car::new(Dir::Down));
            }
            '^' => {
                map.insert((col, row), Tile::UpDown);
                cars.insert((col, row), Car::new(Dir::Up));
            }
            '>' => {
                map.insert((col, row), Tile::LeftRight);
                cars.insert((col, row), Car::new(Dir::Right));
            }
            '<' => {
                map.insert((col, row), Tile::LeftRight);
                cars.insert((col, row), Car::new(Dir::Left));
            }
            ' ' => (),
            c => panic!("unrecognized symbol: {}", c),
        }
    }
}
fn dir_down_left(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Down,
        Dir::Right => Dir::Up,
    }
}
fn dir_down_right(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Left,
        Dir::Down => Dir::Right,
        Dir::Left => Dir::Up,
        Dir::Right => Dir::Down,
    }
}
fn turn_left(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Left,
        Dir::Down => Dir::Right,
        Dir::Left => Dir::Down,
        Dir::Right => Dir::Up,
    }
}
fn turn_right(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
        Dir::Right => Dir::Down,
    }
}
fn tick(
    cars: &mut HashMap<(usize, usize), Car>,
    map: &HashMap<(usize, usize), Tile>,
) -> Option<(usize, usize)> {
    if cars.len() == 1 {
        return cars.keys().next().cloned();
    }
    let mut car_keys: Vec<(usize, usize)> = cars.keys().cloned().collect();
    car_keys.sort_unstable_by_key(|(x, _)| *x);
    car_keys.sort_by_key(|(_, y)| *y);
    let mut new_cars = HashMap::new();
    for key in car_keys {
        if !cars.contains_key(&key) {
            continue;
        }
        let mut car = cars.remove(&key).unwrap();
        let (x, y) = key;
        let new_pos: (usize, usize) = match car.dir {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
        };
        if cars.contains_key(&new_pos) {
            cars.remove(&new_pos);
            continue;
        }
        if new_cars.contains_key(&new_pos) {
            new_cars.remove(&new_pos);
            continue;
        }

        let new_dir = match &map[&new_pos] {
            Tile::DownLeft => dir_down_left(car.dir),
            Tile::DownRight => dir_down_right(car.dir),
            Tile::Cross => car.dir_after_cross(),
            _ => car.dir,
        };
        new_cars.insert(
            new_pos,
            Car {
                dir: new_dir,
                state: car.state,
            },
        );
    }
    *cars = new_cars;
    // no collisions
    None
}
fn main() {
    let f = File::open("input").expect("loading failed");
    let buf = BufReader::new(f);
    let mut map = HashMap::new();
    let mut cars = HashMap::new();
    for (row, l) in buf.lines().enumerate() {
        let l = l.unwrap();
        parse_row(row, &l, &mut map, &mut cars);
    }
    tick(&mut cars, &map);
    println!();
    loop {
        let mut buf = String::new();
        println!("{}", cars.len());
        if let Some(pos) = tick(&mut cars, &map) {
            println!("ANS1:{:?}", pos);
            break;
        }
    }
}
