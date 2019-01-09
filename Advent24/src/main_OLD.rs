#![allow(dead_code)]
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Type {
    Rad,
    Fire,
    Slashing,
    Blud,
    Cold,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Id(usize);

#[derive(PartialEq, Eq, Debug, Clone)]
struct Group {
    hp: usize,
    atk: usize,
    atk_type: Type,
    units: usize,
    init: usize,
    weakness: Vec<Type>,
    immune: Vec<Type>,
    target: Option<Id>,
}
impl Group {
    fn power(&self) -> usize {
        self.atk * self.units
    }

    fn set_target(
        &mut self,
        targets: &HashMap<Id, Group>,
        already_targetted: &HashSet<Id>,
    ) -> Option<Id> {
        let available_targets = targets.iter().filter(|(id, group)| {
            !already_targetted.contains(&id) && !group.immune.contains(&self.atk_type)
        });
        let mut weak_target_order: BinaryHeap<TargetOrder> = BinaryHeap::new();
        let mut normal_target_order: BinaryHeap<TargetOrder> = BinaryHeap::new();
        for (target_id, target) in available_targets {
            if target.weakness.contains(&self.atk_type) {
                weak_target_order.push(TargetOrder(*target_id, target.power(), target.init));
            } else {
                normal_target_order.push(TargetOrder(*target_id, target.power(), target.init));
            }
        }
        if let Some(weak_target) = weak_target_order.pop() {
            self.target = Some(weak_target.0);
            return self.target;
        }
        if let Some(target) = normal_target_order.pop() {
            self.target = Some(target.0);
            return self.target;
        }
        self.target = None;
        self.target
    }
    /// Returns number of killed units
    fn attack(&self, enemies: &mut HashMap<Id, Group>) -> usize {
        if self.target == None {
            return 0;
        }
        if self.units == 0 {
            return 0;
        }
        let target = enemies.get_mut(&self.target.unwrap()).unwrap();
        let multiplier = if target.weakness.contains(&self.atk_type) {
            2
        } else {
            1
        };
        let kills = multiplier * self.power() / target.hp;
        target.units = target.units.saturating_sub(kills);
        kills
    }
}

#[derive(PartialEq, Eq)]
struct TargetOrder(Id, usize, usize);

impl PartialOrd for TargetOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for TargetOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 > other.1 || (self.1 == other.1 && self.2 > other.2) {
            return Ordering::Greater;
        }
        Ordering::Less
    }
}

#[derive(PartialEq, Eq)]
struct AttackOrder(Id, usize);

impl PartialOrd for AttackOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for AttackOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

fn main() {
    let f = File::open("input").expect("loading failed");
    let buf = BufReader::new(f);
    let mut input_lines = buf.lines().filter_map(|r| r.ok());
    let mut start_immune_groups = HashMap::with_capacity(10);
    start_immune_groups.extend(
        input_lines
            .by_ref()
            .skip(1)
            .take(10)
            .enumerate()
            .map(|(i, txt)| (Id(i), parse_group(&txt))),
    );
    let mut start_infect_groups = HashMap::with_capacity(10);
    start_infect_groups.extend(
        input_lines
            .skip(2)
            .take(10)
            .enumerate()
            .map(|(i, txt)| (Id(i + 10), parse_group(&txt))),
    );
    for boost in 0.. {
        let mut immune_groups = start_immune_groups.clone();
        let mut infect_groups = start_infect_groups.clone();
        immune_groups
            .iter_mut()
            .for_each(|(_, group)| group.atk += boost);
        while !immune_groups.is_empty() && !infect_groups.is_empty() {
            //target phase
            let mut target_order = BinaryHeap::with_capacity(20);
            target_order.extend(
                immune_groups
                    .iter()
                    .map(|(id, g)| TargetOrder(*id, g.power(), g.init)),
            );
            target_order.extend(
                infect_groups
                    .iter()
                    .map(|(id, g)| TargetOrder(*id, g.power(), g.init)),
            );

            let mut targetted = HashSet::new();
            while let Some(TargetOrder(targetter_id, _, _)) = target_order.pop() {
                if immune_groups.contains_key(&targetter_id) {
                    if let Some(target_id) = immune_groups
                        .get_mut(&targetter_id)
                        .unwrap()
                        .set_target(&infect_groups, &targetted)
                    {
                        targetted.insert(target_id);
                    }
                } else {
                    if let Some(target_id) = infect_groups
                        .get_mut(&targetter_id)
                        .unwrap()
                        .set_target(&immune_groups, &targetted)
                    {
                        targetted.insert(target_id);
                    }
                }
            }
            //attack phase
            let mut attack_order = BinaryHeap::with_capacity(20);
            attack_order.extend(
                immune_groups
                    .iter()
                    .map(|(id, group)| AttackOrder(*id, group.init)),
            );
            attack_order.extend(
                infect_groups
                    .iter()
                    .map(|(id, group)| AttackOrder(*id, group.init)),
            );
            let mut kill_count = 0;
            while let Some(AttackOrder(atk_id, _)) = attack_order.pop() {
                if immune_groups.contains_key(&atk_id) {
                    let attacker = immune_groups.get(&atk_id).unwrap();
                    kill_count += attacker.attack(&mut infect_groups);
                } else {
                    let attacker = infect_groups.get(&atk_id).unwrap();
                    kill_count += attacker.attack(&mut immune_groups);
                }
            }
            if kill_count == 0 {
                break;
            }
            immune_groups.retain(|_, g| g.units > 0);
            infect_groups.retain(|_, g| g.units > 0);
        } //next round of fight
        if !immune_groups.is_empty() && !infect_groups.is_empty() {
            println!("tie!, boost = {}", boost);
        } else if !infect_groups.is_empty() {
            println!(
                "Infection wins!: {} , boost = {}",
                infect_groups.values().map(|g| g.units).sum::<usize>(),
                boost
            );
        } else {
            println!(
                "Immunity wins!: {} , boost = {}",
                immune_groups.values().map(|g| g.units).sum::<usize>(),
                boost
            );
            break;
        }
    }
}
///////////////////////// Parsing /////////////////////
use text_io::{scan, try_scan};
fn parse_group(line: &str) -> Group {
    let (units, hp, atk, init): (usize, usize, usize, usize);
    let (mut immunity_weakness_string, mut atk_type_string) = (String::new(), String::new());
    if line.find('(').is_some() {
        scan!(line.bytes()=>"{} units each with {} hit points ({}) with an attack that does {} {} damage at initiative {}"
    , units, hp, immunity_weakness_string, atk, atk_type_string, init);
    } else {
        scan!(line.bytes()=>"{} units each with {} hit points with an attack that does {} {} damage at initiative {}"
    , units, hp, atk, atk_type_string, init);
    }
    let (immune, weakness) = parse_immune_weak(&immunity_weakness_string);
    let atk_type = parse_type(&atk_type_string);
    Group {
        units,
        hp,
        weakness,
        immune,
        atk: atk,
        atk_type,
        init,
        target: None,
    }
}

fn parse_immune_weak(input: &str) -> (Vec<Type>, Vec<Type>) {
    let input = input.trim_matches(|c| c == '(' || c == ')');
    let (mut immune, mut weak) = (Vec::new(), Vec::new());
    if input.is_empty() {
        return (immune, weak);
    }
    if let Some(i) = input.find(';') {
        let split_immune_weak: Vec<&str> = input.split(';').collect();
        let (i1, w1) = parse_immune_weak(split_immune_weak[0]);
        let (i2, w2) = parse_immune_weak(split_immune_weak[1]);
        immune.extend(i1);
        immune.extend(i2);
        weak.extend(w1);
        weak.extend(w2);
    } else {
        let split: Vec<&str> = input.split_whitespace().collect();
        let types: Vec<Type> = split[2..]
            .iter()
            .map(|s| s.trim_matches(','))
            .map(|s| parse_type(s))
            .collect();
        match split[0] {
            "immune" => immune.extend(types),
            "weak" => weak.extend(types),
            _ => panic!(),
        }
    }
    (immune, weak)
}
fn parse_type(type_str: &str) -> Type {
    match type_str {
        "radiation" => Type::Rad,
        "cold" => Type::Cold,
        "fire" => Type::Fire,
        "bludgeoning" => Type::Blud,
        "slashing" => Type::Slashing,
        _ => panic!("unrecognized type string"),
    }
}

#[test]
fn parse_test() {
    let expected_group = Group {
        units: 2153,
        hp: 14838,
        weakness: vec![Type::Slashing],
        immune: vec![Type::Fire, Type::Blud, Type::Rad],
        atk: 11,
        atk_type: Type::Rad,
        init: 3,
        target: None,
    };
    let input = "2153 units each with 14838 hit points (immune to fire, bludgeoning, radiation; weak to slashing) with an attack that does 11 radiation damage at initiative 3";
    let actual_group = parse_group(input);
    assert_eq!(actual_group, expected_group);
}

#[test]
fn parse_test_no_weak_or_immune() {
    let expected_group = Group {
        units: 5372,
        hp: 5729,
        weakness: vec![],
        immune: vec![],
        atk: 9,
        atk_type: Type::Fire,
        init: 14,
        target: None,
    };
    let input = "5372 units each with 5729 hit points with an attack that does 9 fire damage at initiative 14";
    let actual_group = parse_group(input);
    assert_eq!(actual_group, expected_group);
}
