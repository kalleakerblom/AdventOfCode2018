#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
fn get_digits(mut n: usize) -> Vec<usize> {
    let mut digits = Vec::new();
    if n == 0 {
        return vec![0];
    }
    while n > 0 {
        let rem = n % 10;
        digits.push(rem);
        n /= 10;
    }
    digits.reverse();
    digits
}
fn main() {
    let input = 360781;
    let mut head1 = 0;
    let mut head2 = 1;
    let mut scores: Vec<usize> = vec![3, 7];
    let search = get_digits(input);
    loop {
        //sum heads
        let score1 = scores[head1];
        let score2 = scores[head2];
        let sum = score1 + score2;
        // get digits and add
        let digits = get_digits(sum);
        scores.extend(digits.iter());
        if let Some(n) = search_score_tail(&scores, &search) {
            println!("ANS2:{}", scores.len() - n);
            break;
        }
        // move heads, loop if needed
        head1 = (head1 + score1 + 1) % scores.len();
        head2 = (head2 + score2 + 1) % scores.len();
    }
}
fn search_score_tail(scores: &Vec<usize>, search: &Vec<usize>) -> Option<usize> {
    if scores.len() < search.len() {
        return None;
    }
    if scores[scores.len() - search.len()..].eq(&search[..]) {
        return Some(search.len());
    }
    if scores.len() < search.len() + 1 {
        return None;
    }
    if scores[scores.len() - search.len() - 1..scores.len() - 1].eq(&search[..]) {
        return Some(search.len() + 1);
    }
    None
}

// 2018 : 5941429882
//5941429882
//79634351415
