#![allow(dead_code)]
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
#[derive(PartialEq, Eq)]
enum Tile {
    Floor,
    Wall,
}
#[derive(PartialEq, Eq, Clone, Copy)]
enum Team {
    Goblin,
    Elf,
}
#[derive(Clone, Copy)]
struct Entity {
    hp: usize,
    team: Team,
}
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Pos(usize, usize);

fn main() {
    let f = File::open("input").expect("loading failed");
    let buf = BufReader::new(f);
    let mut map = HashMap::new();
    let mut start_entities = HashMap::new();
    for (row, line) in buf.lines().enumerate() {
        let line = line.unwrap();
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    map.insert(Pos(col, row), Tile::Wall);
                }
                '.' => {
                    map.insert(Pos(col, row), Tile::Floor);
                }
                'G' => {
                    map.insert(Pos(col, row), Tile::Floor);
                    start_entities.insert(
                        Pos(col, row),
                        Entity {
                            team: Team::Goblin,
                            hp: 200,
                        },
                    );
                }
                'E' => {
                    map.insert(Pos(col, row), Tile::Floor);
                    start_entities.insert(
                        Pos(col, row),
                        Entity {
                            team: Team::Elf,
                            hp: 200,
                        },
                    );
                }
                symbol @ _ => panic!("unrecognize symbol ({})", symbol),
            }
        }
    }
    // Finished parsing
    'boost: for elf_boost in 1.. {
        println!("boost: {}", elf_boost);
        let mut entities = start_entities.clone();
        let mut turn_count = 0;
        'turn: loop {
            let mut action_order: Vec<Pos> = entities.keys().cloned().collect();
            action_order.sort_by_key(|Pos(x, y)| (*y, *x));
            for ent_pos in action_order {
                if !entities.contains_key(&ent_pos) {
                    // entity was killed before it's turn
                    continue;
                }
                let current_team = entities[&ent_pos].team;
                let targets: Vec<Pos> = entities
                    .iter()
                    .filter(|(_, ent)| ent.team != current_team)
                    .map(|(pos, _)| pos)
                    .cloned()
                    .collect();
                if targets.is_empty() {
                    break 'turn;
                }
                let mut searching: HashSet<Pos> = HashSet::new();
                // Assemble set of tiles next to enemy and empty
                searching.extend(
                    targets
                        .iter()
                        .flat_map(|target| get_neighbors(target))
                        .filter(|near_target| {
                            pos_empty(*near_target, &map, &entities) || *near_target == ent_pos
                        }),
                );
                let mut current_pos = ent_pos;
                if let Some(next_pos) = search(ent_pos, &map, &entities, &searching) {
                    let moved_ent = entities.remove(&ent_pos).expect("missing entity");
                    entities.insert(next_pos, moved_ent);
                    current_pos = next_pos;
                }
                // move complete, start attack
                let neighbors = get_neighbors(&current_pos);
                let mut fight_positions: Vec<Pos> = entities
                    .iter()
                    .filter(|(pos, ent)| ent.team != current_team && neighbors.contains(pos))
                    .map(|(pos, _)| *pos)
                    .collect();
                if fight_positions.is_empty() {
                    continue;
                }
                fight_positions.sort_unstable_by_key(|pos| (entities[&pos].hp, pos.1, pos.0));
                let attacked = entities.get_mut(fight_positions.first().unwrap()).unwrap();
                let dmg = if current_team == Team::Elf {
                    3 + elf_boost
                } else {
                    3
                };
                if attacked.hp <= dmg {
                    let dead = entities.remove(fight_positions.first().unwrap()).unwrap();
                    // no elves are allowed to die, try higher boost
                    if dead.team == Team::Elf {
                        continue 'boost;
                    }
                } else {
                    attacked.hp -= dmg;
                }
            }
            turn_count += 1;
        } //game loop
        let hp_sum: usize = entities.iter().map(|(_, e)| e.hp).sum();

        println!("boost: {} outcome: {}", elf_boost, turn_count * hp_sum);
        break;
    } //next boost
}
fn pos_empty(pos: Pos, map: &HashMap<Pos, Tile>, ents: &HashMap<Pos, Entity>) -> bool {
    map.get(&pos) == Some(&Tile::Floor) && !ents.contains_key(&pos)
}

fn get_neighbors(pos: &Pos) -> Vec<Pos> {
    let Pos(x, y) = pos;
    let neighbors = vec![
        Pos(*x, *y - 1),
        Pos(*x - 1, *y),
        Pos(*x + 1, *y),
        Pos(*x, *y + 1),
    ];
    neighbors
}

fn search(
    pos: Pos,
    map: &HashMap<Pos, Tile>,
    ents: &HashMap<Pos, Entity>,
    searching: &HashSet<Pos>,
) -> Option<Pos> {
    if searching.contains(&pos) {
        return Some(pos);
    }
    let mut frontier = VecDeque::new();
    frontier.push_back((pos, 0));
    let mut came_from: HashMap<Pos, Option<Pos>> = HashMap::new();
    came_from.insert(pos, None);
    let mut nearest_distance = usize::max_value();
    let mut nearest_searches = Vec::new();
    while let Some((current, current_distance)) = frontier.pop_front() {
        if searching.contains(&current) && current_distance <= nearest_distance {
            nearest_searches.push(current);
            nearest_distance = current_distance;
        }
        for &next in get_neighbors(&current)
            .iter()
            .filter(|pos| pos_empty(**pos, map, ents))
        {
            if !came_from.contains_key(&next) {
                frontier.push_back((next, current_distance + 1));
                came_from.insert(next, Some(current));
            }
        }
    }
    if nearest_searches.is_empty() {
        // no enemy reachable
        None
    } else {
        nearest_searches.sort_by_key(|Pos(x, y)| (*y, *x));
        let selected_destination = *nearest_searches.first().unwrap();
        Some(retrace_to_start(&pos, &selected_destination, &came_from))
    }
}

fn retrace_to_start(start: &Pos, end: &Pos, came_from: &HashMap<Pos, Option<Pos>>) -> Pos {
    let mut current = *end;
    while let Some(next) = came_from[&current] {
        if next == *start {
            break;
        }
        current = next;
    }
    current
}
