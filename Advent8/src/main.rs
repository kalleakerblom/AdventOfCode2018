use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::u32;

struct Node {
    meta_data: Vec<u32>,
    children: Vec<Node>,
}

fn sum_meta(node: &Node) -> u32 {
    let meta_sum: u32 = node.meta_data.iter().sum();
    let child_sum: u32 = node.children.iter().map(|ch| sum_meta(ch)).sum();
    child_sum + meta_sum
}

fn node_value(node: &Node) -> u32 {
    if node.children.is_empty() {
        return node.meta_data.iter().sum();
    }
    node.meta_data
        .iter()
        .filter_map(|&m| node.children.get(m as usize - 1))
        .map(|ch| node_value(ch))
        .sum()
}

fn parse_nodes(input: &mut VecDeque<u32>) -> Result<Node, Box<Error>> {
    let num_children = input.pop_front().ok_or("missing first number")?;
    let num_meta = input.pop_front().ok_or("missing second number")?;
    let mut children = Vec::new();
    for _ in 0..num_children {
        children.push(parse_nodes(input)?);
    }
    let mut meta_data = Vec::new();
    for _ in 0..num_meta {
        meta_data.push(input.pop_front().ok_or("missing meta-data")?);
    }
    Ok(Node {
        meta_data,
        children,
    })
}
fn main() -> Result<(), Box<Error>> {
    let f = File::open("input")?;
    let mut buf = BufReader::new(f);
    let mut input = String::new();
    buf.read_to_string(&mut input)?;
    let mut input: VecDeque<u32> = input
        .split_whitespace()
        .map(|s| u32::from_str_radix(s, 10).unwrap())
        .collect();
    let root = parse_nodes(&mut input)?;
    println!("ans1: {}", sum_meta(&root));
    println!("ans2: {}", node_value(&root));
    Ok(())
}
