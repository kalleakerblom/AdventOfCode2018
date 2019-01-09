#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
use text_io::{read, scan, try_read, try_scan};
fn main() {
    let f = File::open("input").expect("loading failed");
    let buf = BufReader::new(f);
    let mut lines = Vec::new();
    for l in buf.lines() {
        let l = l.unwrap();
        lines.push(l);
    }
    assert_eq!(op_addi([3, 2, 1, 1], [9, 2, 1, 2], [3, 2, 2, 1]), true);
    let mut split = lines.split(|s| s.is_empty());
    let mut op_code_map: HashMap<u8, HashSet<Op>> = HashMap::new();
    for cluster in split {
        if let [l1, l2, l3] = &cluster {
            // Before: [1, 2, 2, 1]
            // 9 3 1 3
            // After:  [1, 2, 2, 1]
            let (ba, bb, bc, bd): (u8, u8, u8, u8);
            scan!(l1.bytes()=>"Before: [{}, {}, {}, {}]",ba,bb,bc,bd);
            let (op, a, b, c): (u8, u8, u8, u8);
            scan!(l2.bytes()=>"{} {} {} {}",op,a,b,c);
            let (aa, ab, ac, ad): (u8, u8, u8, u8);
            scan!(l3.bytes()=>"After:  [{}, {}, {}, {}]",aa,ab,ac,ad);
            println!("{},{},{},{}", aa, ab, ac, ad);
            let possible = get_possible_ops([ba, bb, bc, bd], [op, a, b, c], [aa, ab, ac, ad]);
            if !op_code_map.contains_key(&op) {
                op_code_map.insert(op, possible);
            } else {
                let old_possible = op_code_map.get_mut(&op).expect("did find op set");
                let intersection: HashSet<Op> =
                    old_possible.intersection(&possible).cloned().collect();
                *old_possible = intersection;
            }
        } else {
            break;
        }
    }
    // op_code_map
    //     .iter()
    //     .for_each(|(k, v)| println!("{}:{}", k, v.len()));

    // figure out final op map :
    // Find all codes with one OP, add that pair to final
    // remove that op from all codes
    // repeat until all ops mapped
    let mut final_code_map = HashMap::new();
    loop {
        let mut ops_to_remove = HashSet::<Op>::new();
        for code_op in op_code_map.iter().filter(|(_, v)| v.len() == 1) {
            final_code_map.insert(*code_op.0, code_op.1.clone());
            ops_to_remove.extend(code_op.1.clone());
        }
        if ops_to_remove.len() == 0 {
            break;
        }
        for op_set in op_code_map.values_mut() {
            let difference: HashSet<Op> = op_set.difference(&ops_to_remove).cloned().collect();
            *op_set = difference;
        }
    }
    // println!();
    // final_code_map
    //     .iter()
    //     .for_each(|(k, v)| println!("{}:{}", k, v.len()));
    let mut register = [0, 0, 0, 0];
    for l in lines.iter().skip(3246) {
        if l.is_empty() {
            continue;
        }
        let (op, a, b, c): (u8, u8, u8, u8);
        scan!(l.bytes() => "{} {} {} {}",op,a,b,c);
        operate(&mut register, &[op, a, b, c], &final_code_map);
    }
    println!("{:?}", register);
}
fn operate(input: &mut [u32; 4], op: &[u8; 4], code_map: &HashMap<u8, HashSet<Op>>) {
    let operation = code_map[&op[0]].iter().next().unwrap();
    let A = op[1] as u32;
    let B = op[2] as u32;
    let C = op[3] as u32;
    let refA = input[A as usize];
    let refB = input[B as usize];
    match operation {
        Op::addi => input[op[3] as usize] = refA + B,
        Op::addr => input[op[3] as usize] = refA + refB,
        Op::muli => input[op[3] as usize] = refA * B,
        Op::mulr => input[op[3] as usize] = refA * refB,
        Op::bani => input[op[3] as usize] = refA & B,
        Op::banr => input[op[3] as usize] = refA & refB,
        Op::bori => input[op[3] as usize] = refA | B,
        Op::borr => input[op[3] as usize] = refA | refB,
        Op::seti => input[op[3] as usize] = A,
        Op::setr => input[op[3] as usize] = refA,
        Op::gtir => input[op[3] as usize] = if A > refB { 1 } else { 0 },
        Op::gtri => input[op[3] as usize] = if refA > B { 1 } else { 0 },
        Op::gtrr => input[op[3] as usize] = if refA > refB { 1 } else { 0 },
        Op::eqir => input[op[3] as usize] = if A == refB { 1 } else { 0 },
        Op::eqri => input[op[3] as usize] = if refA == B { 1 } else { 0 },
        Op::eqrr => input[op[3] as usize] = if refA == refB { 1 } else { 0 },
    }
}
fn get_possible_ops(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> HashSet<Op> {
    let mut possible = HashSet::new();
    if op_addi(before, op, after) {
        possible.insert(Op::addi);
    }
    if op_addr(before, op, after) {
        possible.insert(Op::addr);
    }
    if op_muli(before, op, after) {
        possible.insert(Op::muli);
    }
    if op_mulr(before, op, after) {
        possible.insert(Op::mulr);
    }
    if op_bani(before, op, after) {
        possible.insert(Op::bani);
    }
    if op_banr(before, op, after) {
        possible.insert(Op::banr);
    }
    if op_bori(before, op, after) {
        possible.insert(Op::bori);
    }
    if op_borr(before, op, after) {
        possible.insert(Op::borr);
    }
    if op_seti(before, op, after) {
        possible.insert(Op::seti);
    }
    if op_setr(before, op, after) {
        possible.insert(Op::setr);
    }
    if op_gtir(before, op, after) {
        possible.insert(Op::gtir);
    }
    if op_gtri(before, op, after) {
        possible.insert(Op::gtri);
    }
    if op_gtrr(before, op, after) {
        possible.insert(Op::gtrr);
    }
    if op_eqir(before, op, after) {
        possible.insert(Op::eqir);
    }
    if op_eqri(before, op, after) {
        possible.insert(Op::eqri);
    }
    if op_eqrr(before, op, after) {
        possible.insert(Op::eqrr);
    }
    possible
}
#[derive(PartialEq, Eq, Hash, Clone)]
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

fn op_addi(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    before[op[1] as usize] + op[2] == after[op[3] as usize]
}
fn op_addr(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    before[op[1] as usize] + before[op[2] as usize] == after[op[3] as usize]
}
fn op_muli(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    before[op[1] as usize] * op[2] == after[op[3] as usize]
}
fn op_mulr(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    before[op[1] as usize] * before[op[2] as usize] == after[op[3] as usize]
}
fn op_bani(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    before[op[1] as usize] & op[2] == after[op[3] as usize]
}
fn op_banr(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    before[op[1] as usize] & before[op[2] as usize] == after[op[3] as usize]
}
fn op_bori(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    before[op[1] as usize] | op[2] == after[op[3] as usize]
}
fn op_borr(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    before[op[1] as usize] | before[op[2] as usize] == after[op[3] as usize]
}
fn op_seti(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    op[1] == after[op[3] as usize]
}
fn op_setr(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    before[op[1] as usize] == after[op[3] as usize]
}
fn op_gtir(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    let is_greater = if op[1] > before[op[2] as usize] { 1 } else { 0 };
    is_greater == after[op[3] as usize]
}
fn op_gtri(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    let is_greater = if before[op[1] as usize] > op[2] { 1 } else { 0 };
    is_greater == after[op[3] as usize]
}
fn op_gtrr(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    let is_greater = if before[op[1] as usize] > before[op[2] as usize] {
        1
    } else {
        0
    };
    is_greater == after[op[3] as usize]
}
fn op_eqir(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    let is_greater = if op[1] == before[op[2] as usize] {
        1
    } else {
        0
    };
    is_greater == after[op[3] as usize]
}
fn op_eqri(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    let is_greater = if before[op[1] as usize] == op[2] {
        1
    } else {
        0
    };
    is_greater == after[op[3] as usize]
}
fn op_eqrr(before: [u8; 4], op: [u8; 4], after: [u8; 4]) -> bool {
    let is_greater = if before[op[1] as usize] == before[op[2] as usize] {
        1
    } else {
        0
    };
    is_greater == after[op[3] as usize]
}
