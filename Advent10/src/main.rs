use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::i32;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
use text_io::{read, scan, try_read, try_scan};

struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}
impl Point {
    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}
fn draw(points: &[Point]) -> bool {
    let mut coordinates = HashSet::new();
    let (mut min_x, mut min_y) = (i32::max_value(), i32::max_value());
    points.iter().for_each(|p| {
        min_x = cmp::min(min_x, p.x);
        min_y = cmp::min(min_y, p.y);
    });

    coordinates.extend(points.iter().map(|p| (p.x, p.y)));
    let (width, height) = (100, 30);
    let mut to_draw = 0;
    let draw_limit = 150;
    for y in min_y..(min_y + height) {
        for x in min_x..(min_x + width) {
            if coordinates.contains(&(x, y)) {
                to_draw += 1;
            } else {
            }
        }
    }
    if to_draw > draw_limit {
        println!("drawn: {}", to_draw);
        for y in min_y..(min_y + height) {
            for x in min_x..(min_x + width) {
                if coordinates.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
    to_draw > draw_limit
}
fn main() {
    let f = File::open("input").expect("loading failed");
    let buf = BufReader::new(f);
    let mut points = Vec::new();
    for l in buf.lines() {
        let l = l.unwrap();
        let (x, y, vx, vy): (i32, i32, i32, i32);
        scan!(l.bytes().filter(|b|*b!=b' ')=>"position=<{},{}>velocity=<{},{}>",x,y,vx,vy);
        points.push(Point { x, y, vx, vy });
    }
    let mut time = 0;
    loop {
        println!("{}", time);
        if draw(&points) {
            let s: i32 = read!();
            if s == 0 {
                break;
            }
        }
        points.iter_mut().for_each(|p| p.step());
        time += 1;
    }
}
