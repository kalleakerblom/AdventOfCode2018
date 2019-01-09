#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
use text_io::{read, scan, try_read, try_scan};

enum Op {
    addi,
    addr,
    muli,
    mulr,
    bani,
    banr,
    bori,
    borr,
    seti,
    setr,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}
fn parse_op(s: &str) -> Op {
    match s {
        "addi" => Op::addi,
        "addr" => Op::addr,
        "muli" => Op::muli,
        "mulr" => Op::mulr,
        "bani" => Op::bani,
        "banr" => Op::banr,
        "bori" => Op::bori,
        "borr" => Op::borr,
        "seti" => Op::seti,
        "setr" => Op::setr,
        "gtir" => Op::gtir,
        "gtri" => Op::gtri,
        "gtrr" => Op::gtrr,
        "eqir" => Op::eqir,
        "eqri" => Op::eqri,
        "eqrr" => Op::eqrr,
        _ => panic!("bad op"),
    }
}
fn main() {
    let f = File::open("input").expect("loading failed");
    let buf = BufReader::new(f);
    let mut lines = Vec::new();
    for l in buf.lines() {
        let l = l.expect("bad read");
        lines.push(l);
    }
    let ip: usize;
    scan!(lines[0].bytes()=>"#ip {}",ip);
    let mut reg = [1, 0, 0, 0, 0, 0];
    let mut instructions = Vec::with_capacity(lines.len() - 1);
    for l in &lines[1..] {
        let (op, A, B, C): (String, u8, u8, u8);
        scan!(l.bytes()=>"{} {} {} {}", op, A, B, C);
        instructions.push((parse_op(&op), [A, B, C]));
    }
    let mut ins_ptr = 0;
    while ins_ptr < instructions.len() {
        reg[ip] = ins_ptr;
        println!("{:?}", reg);
        operate(&mut reg, &instructions[ins_ptr]);
        ins_ptr = reg[ip];
        ins_ptr += 1;

        //new strategy
        break;
    }
    let mut sum = 0;
    for i in 1..=10551339 {
        for j in 1..=10551339 / i {
            if 10551339 == i * j {
                sum += i;
            }
        }
    }
    // 5586237
    // 5586237 too low
    println!("Ans: {}", sum);
}
fn operate(input: &mut [usize; 6], op: &(Op, [u8; 3])) {
    let A = op.1[0] as usize;
    let B = op.1[1] as usize;
    let C = op.1[2] as usize;
    match op.0 {
        Op::addi => input[C as usize] = input[A as usize] + B,
        Op::addr => input[C as usize] = input[A as usize] + input[B as usize],
        Op::muli => input[C as usize] = input[A as usize] * B,
        Op::mulr => input[C as usize] = input[A as usize] * input[B as usize],
        Op::bani => input[C as usize] = input[A as usize] & B,
        Op::banr => input[C as usize] = input[A as usize] & input[B as usize],
        Op::bori => input[C as usize] = input[A as usize] | B,
        Op::borr => input[C as usize] = input[A as usize] | input[B as usize],
        Op::seti => input[C as usize] = A,
        Op::setr => input[C as usize] = input[A as usize],
        Op::gtir => input[C as usize] = if A > input[B as usize] { 1 } else { 0 },
        Op::gtri => input[C as usize] = if input[A as usize] > B { 1 } else { 0 },
        Op::gtrr => {
            input[C as usize] = if input[A as usize] > input[B as usize] {
                1
            } else {
                0
            }
        }
        Op::eqir => input[C as usize] = if A == input[B as usize] { 1 } else { 0 },
        Op::eqri => input[C as usize] = if input[A as usize] == B { 1 } else { 0 },
        Op::eqrr => {
            input[C as usize] = if input[A as usize] == input[B as usize] {
                1
            } else {
                0
            }
        }
    }
}
