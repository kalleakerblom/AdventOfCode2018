#![allow(dead_code)]
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader};
use text_io::{scan, try_scan};
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(i64, i64, i64);
impl Pos {
    fn dist(&self, other: &Pos) -> i64 {
        use std::i64;
        i64::abs(self.0 - other.0) + i64::abs(self.1 - other.1) + i64::abs(self.2 - other.2)
    }
}
#[derive(Debug)]
struct Bot {
    p: Pos,
    r: i64,
}
fn main() {
    let f = File::open("input").expect("loading failed");
    let buf = BufReader::new(f);
    let mut bots = Vec::new();
    for l in buf.lines() {
        //pos=<25673166,11893376,-2752142>, r=74017690
        let (x, y, z, r): (i64, i64, i64, i64);
        let l = l.unwrap();
        scan!(l.bytes()=>"pos=<{},{},{}>, r={}",x,y,z,r);
        bots.push(Bot { p: Pos(x, y, z), r });
    }
    // find bot with largest r
    let &Bot {
        p: ref large_pos,
        r,
    } = bots.iter().max_by_key(|bot| bot.r).unwrap();
    let count = bots.iter().filter(|bot| bot.p.dist(large_pos) < r).count();
    println!("ans1: {}", count);
    let mut current_pos = Pos(0, 0, 0);

    let steps = vec![10000, 1000, 100, 10];
    // minimize distance to spheres, hopefully close to answer
    let mut old_pos = current_pos;
    let mut current_sum = sum_of_sphere_dist(&current_pos, &bots);
    for step in steps {
        loop {
            let Pos(cx, cy, cz) = current_pos;
            let xp = Pos(cx + step, cy, cz);
            let xn = Pos(cx - step, cy, cz);
            let yp = Pos(cx, cy + step, cz);
            let yn = Pos(cx, cy - step, cz);
            let zp = Pos(cx, cy, cz + step);
            let zn = Pos(cx, cy, cz - step);
            // move in x?
            let xp_sum = sum_of_sphere_dist(&xp, &bots);
            let xn_sum = sum_of_sphere_dist(&xn, &bots);
            if xp_sum < xn_sum {
                if xp_sum < current_sum {
                    current_pos.0 += step;
                    current_sum = xp_sum;
                }
            } else if xn_sum < current_sum {
                current_pos.0 -= step;
                current_sum = xn_sum;
            }
            // move in y?
            let yp_sum = sum_of_sphere_dist(&yp, &bots);
            let yn_sum = sum_of_sphere_dist(&yn, &bots);
            if yp_sum < yn_sum || (yp_sum == yn_sum && yp.1.abs() < yn.1.abs()) {
                if yp_sum < current_sum {
                    current_pos.1 += step;
                    current_sum = yp_sum;
                }
            } else if yn_sum < current_sum {
                current_pos.1 -= step;
                current_sum = yn_sum;
            }
            // move in z?
            let zp_sum = sum_of_sphere_dist(&zp, &bots);
            let zn_sum = sum_of_sphere_dist(&zn, &bots);
            if zp_sum < zn_sum {
                if zp_sum < current_sum {
                    current_pos.2 += step;
                    current_sum = zp_sum;
                }
            } else if zn_sum < current_sum {
                current_pos.2 -= step;
                current_sum = zn_sum;
            }
            if old_pos == current_pos {
                break;
            }
            old_pos = current_pos;
        }
    } // next step length
    let window = 10;
    let mut max = (current_pos, 0);
    for x in 11382548 - window..11382548 + window {
        for y in 29059452 - window..29059452 + window {
            for z in 39808797 - window..39808797 + window {
                let count = bots
                    .iter()
                    .filter(|b| distance_to_sphere(&Pos(x, y, z), b) == 0)
                    .count();
                if count > max.1 {
                    max = (Pos(x, y, z), count);
                }
            }
        }
    }
    println!(
        "distance between optimized pos and true pos = {}",
        max.0.dist(&current_pos)
    );
    println!("Ans2: {}", max.0.dist(&Pos(0, 0, 0)));
}
fn sum_of_sphere_dist(pos: &Pos, bots: &Vec<Bot>) -> i64 {
    bots.iter().map(|b| distance_to_sphere(pos, b)).sum::<i64>()
}
fn distance_to_sphere(pos: &Pos, bot: &Bot) -> i64 {
    if pos.dist(&bot.p) < bot.r {
        return 0;
    }
    i64::abs(pos.dist(&bot.p) - bot.r)
}
