#![allow(dead_code)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader};
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point(i32, i32, i32, i32);
fn distance(p1: &Point, p2: &Point) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs() + (p1.3 - p2.3).abs()
}

fn main() {
    let f = File::open("input").expect("loading failed");
    let buf = BufReader::new(f);
    let mut points = Vec::new();
    for l in buf.lines() {
        let l = l.unwrap();
        // 5,-7,-5,-4
        let mut split = l.split(',');
        let (x, y, z, t) = (
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        );
        points.push(Point(x, y, z, t));
    }
    let mut neighbors: HashMap<Point, Vec<Point>> = HashMap::new();
    for p in &points {
        let point_neighbors: Vec<Point> = points
            .iter()
            .filter(|other| **other != *p && distance(p, other) <= 3)
            .cloned()
            .collect();
        neighbors.insert(*p, point_neighbors);
    }
    let mut unmapped_points: HashSet<Point> = points.iter().cloned().collect();
    let mut count = 0;
    while !unmapped_points.is_empty() {
        count += 1;
        let first = unmapped_points.iter().cloned().next().unwrap();
        let mut frontier = VecDeque::new();
        frontier.push_back(first);
        while !frontier.is_empty() {
            let current = frontier.pop_front().unwrap();
            unmapped_points.remove(&current);
            let current_neighbors: Vec<Point> = neighbors[&current]
                .iter()
                .filter(|n| unmapped_points.contains(n))
                .cloned()
                .collect();
            for neighbor in current_neighbors {
                frontier.push_back(neighbor);
            }
        }
    }
    println!("ans1: {}", count);
}
