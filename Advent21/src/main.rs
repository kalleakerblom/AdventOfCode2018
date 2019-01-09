#![allow(dead_code)]
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader};
use text_io::{scan, try_scan};

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
    let mut instructions = Vec::with_capacity(lines.len() - 1);
    for l in &lines[1..] {
        let (op, A, B, C): (String, usize, usize, usize);
        scan!(l.bytes()=>"{} {} {} {}", op, A, B, C);
        instructions.push((parse_op(&op), [A, B, C]));
    }
    let i = 0;
    let mut reg = [i, 0, 0, 0, 0, 0];
    let mut ins_ptr = 0;
    let mut op_count = 0;
    let mut reg_repeat = HashSet::new();
    let mut prev_five = 0;
    while ins_ptr < instructions.len() {
        reg[ip] = ins_ptr;
        operate(&mut reg, &instructions[ins_ptr]);
        op_count += 1;
        ins_ptr = reg[ip];
        ins_ptr += 1;
        if ins_ptr == 13 && reg[1] < 256 {
            if !reg_repeat.insert(reg[5]) {
                //  6228483 too low
                // 12390302
                println!("ans2: {}", prev_five);
                break;
            }
            prev_five = reg[5];
        }
    }
    println!("ans1: {}", op_count);
}
fn operate(input: &mut [usize; 6], op: &(Op, [usize; 3])) {
    let A = op.1[0];
    let B = op.1[1];
    let C = op.1[2];
    match op.0 {
        Op::addi => input[C] = input[A] + B,
        Op::addr => input[C] = input[A] + input[B],
        Op::muli => input[C] = input[A] * B,
        Op::mulr => input[C] = input[A] * input[B],
        Op::bani => input[C] = input[A] & B,
        Op::banr => input[C] = input[A] & input[B],
        Op::bori => input[C] = input[A] | B,
        Op::borr => input[C] = input[A] | input[B],
        Op::seti => input[C] = A,
        Op::setr => input[C] = input[A],
        Op::gtir => input[C] = if A > input[B] { 1 } else { 0 },
        Op::gtri => input[C] = if input[A] > B { 1 } else { 0 },
        Op::gtrr => input[C] = if input[A] > input[B] { 1 } else { 0 },
        Op::eqir => input[C] = if A == input[B] { 1 } else { 0 },
        Op::eqri => input[C] = if input[A] == B { 1 } else { 0 },
        Op::eqrr => input[C] = if input[A] == input[B] { 1 } else { 0 },
    }
}
