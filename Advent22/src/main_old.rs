#![allow(dead_code)]
use std::cmp;
use std::collections::{BinaryHeap, HashMap};
#[derive(Clone, Copy, PartialEq, Eq)]
enum RegionType {
    Wet,
    Narrow,
    Rocky,
}

fn erosion(
    pos: (usize, usize),
    target: (usize, usize),
    depth: usize,
    erosion_map: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if erosion_map.contains_key(&pos) {
        return erosion_map[&pos];
    } else {
        let er = (geo_index(pos, target, depth, erosion_map) + depth) % 20183;
        erosion_map.insert(pos, er);
        er
    }
}
fn get_type(
    pos: (usize, usize),
    target: (usize, usize),
    depth: usize,
    erosion_map: &mut HashMap<(usize, usize), usize>,
) -> RegionType {
    let erosion = erosion(pos, target, depth, erosion_map);
    match erosion % 3 {
        0 => RegionType::Rocky,
        1 => RegionType::Wet,
        2 => RegionType::Narrow,
        _ => unreachable!(),
    }
}
fn geo_index(
    pos: (usize, usize),
    target: (usize, usize),
    depth: usize,
    erosion_map: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if pos == (0, 0) || pos == target {
        return 0;
    }
    if pos.0 == 0 {
        return pos.1 * 48271;
    }
    if pos.1 == 0 {
        return pos.0 * 16807;
    }
    erosion((pos.0, pos.1 - 1), target, depth, erosion_map)
        * erosion((pos.0 - 1, pos.1), target, depth, erosion_map)
}
fn main() {
    let depth = 9171;
    let target = (7, 721);
    let mut erosion_map = HashMap::new();
    let mut type_map = HashMap::new();
    for x in 0..=target.0 + 300 {
        for y in 0..=target.1 + 300 {
            let r_type = get_type((x, y), target, depth, &mut erosion_map);
            type_map.insert((x, y), r_type);
        }
    }
    // Dijkstra’s Algorithm
    // (x,y,State) is a node
    let mut frontier = BinaryHeap::new();
    frontier.push(NodeOrder(Node(0, 0, State::Torch), 0));
    let mut cost_so_far = HashMap::<Node, usize>::new();
    cost_so_far.insert(Node(0, 0, State::Torch), 0);
    while !frontier.is_empty() {
        let NodeOrder(current_node, cost) = frontier.pop().unwrap();
        if current_node == Node(target.0, target.1, State::Torch) {
            // 976 too low
            println!("ans2:{}", cost);
            break;
        }
        for (next, step_cost) in neighbors(&current_node, &type_map) {
            let new_cost = cost + step_cost;
            if !cost_so_far.contains_key(&next) {
                cost_so_far.insert(next.clone(), new_cost);
                frontier.push(NodeOrder(next, new_cost));
            } else if new_cost < cost_so_far[&next] {
                *cost_so_far.get_mut(&next).expect("no cost") = new_cost;
                frontier.push(NodeOrder(next, new_cost));
            }
        }
    }
}
fn neighbors(node: &Node, map: &HashMap<(usize, usize), RegionType>) -> Vec<(Node, usize)> {
    let Node(x, y, state) = node;
    let mut coords = Vec::new();
    if *x > 0 {
        coords.push((x - 1, *y));
    }
    if *y > 0 {
        coords.push((*x, y - 1));
    }
    coords.push((*x, y + 1));
    coords.push((x + 1, *y));
    let current_region = map[&(*x, *y)];
    let mut result = Vec::new();
    match state {
        State::Torch => {
            if current_region == RegionType::Rocky {
                result.push((Node(*x, *y, State::Climbing), 7));
            } else {
                result.push((Node(*x, *y, State::Neither), 7));
            }
            for coord in &coords {
                if map.contains_key(&coord) {
                    if map[&coord] != RegionType::Wet {
                        result.push((Node(coord.0, coord.1, State::Torch), 1));
                    }
                }
            }
        }
        State::Climbing => {
            if current_region == RegionType::Rocky {
                result.push((Node(*x, *y, State::Torch), 7));
            } else {
                result.push((Node(*x, *y, State::Neither), 7));
            }
            for coord in &coords {
                if map.contains_key(&coord) {
                    if map[&coord] != RegionType::Narrow {
                        result.push((Node(coord.0, coord.1, State::Climbing), 1));
                    }
                }
            }
        }
        State::Neither => {
            if current_region == RegionType::Wet {
                result.push((Node(*x, *y, State::Climbing), 7));
            } else {
                result.push((Node(*x, *y, State::Torch), 7));
            }
            for coord in &coords {
                if map.contains_key(&coord) {
                    if map[&coord] != RegionType::Rocky {
                        result.push((Node(coord.0, coord.1, State::Neither), 1));
                    }
                }
            }
        }
    }
    result
}
#[derive(PartialEq, Eq, Hash, Clone)]
enum State {
    Climbing,
    Torch,
    Neither,
}
#[derive(PartialEq, Eq, Hash, Clone)]
struct Node(usize, usize, State);
#[derive(PartialEq, Eq)]
struct NodeOrder(Node, usize);
use std::cmp::Ordering;
impl PartialOrd for NodeOrder {
    fn partial_cmp(&self, other: &NodeOrder) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for NodeOrder {
    fn cmp(&self, other: &NodeOrder) -> Ordering {
        if self.1 < other.1 {
            Ordering::Greater
        } else if self.1 == other.1 {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}
