#![allow(dead_code)]
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Type {
    Rad,
    Fire,
    Slashing,
    Blud,
    Cold,
}
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Team {
    Immune,
    Infect,
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
    team: Team,
}
impl Group {
    fn power(&self) -> usize {
        self.atk * self.units
    }
}

fn find_target(
    targetter: &Group,
    targets: &HashMap<Id, Group>,
    already_targetted: &HashSet<Id>,
) -> Option<Id> {
    targets
        .iter()
        .filter(|(id, group)| {
            targetter.team != group.team
                && !already_targetted.contains(&id)
                && !group.immune.contains(&targetter.atk_type)
        })
        .max_by_key(|(_, target)| (calc_dmg(targetter, target), target.power(), target.init))
        .map(|(id, _)| *id)
}

fn calc_dmg(attacker: &Group, target: &Group) -> usize {
    if target.weakness.contains(&attacker.atk_type) {
        2 * attacker.power()
    } else {
        attacker.power()
    }
}

fn main() {
    let f = File::open("input").expect("loading failed");
    let buf = BufReader::new(f);
    let mut input_lines = buf.lines().filter_map(|r| r.ok());
    let mut start_groups = HashMap::with_capacity(20);
    start_groups.extend(
        input_lines
            .by_ref()
            .skip(1)
            .take(10)
            .enumerate()
            .map(|(i, txt)| (Id(i), parse_group(&txt, Team::Immune))),
    );
    start_groups.extend(
        input_lines
            .skip(2)
            .take(10)
            .enumerate()
            .map(|(i, txt)| (Id(i + 10), parse_group(&txt, Team::Infect))),
    );
    // try boosts to immune team until they win
    'boost: for boost in 0.. {
        let mut groups = start_groups.clone();
        // add atk boost to immune system team
        groups
            .iter_mut()
            .filter(|(_, g)| g.team == Team::Immune)
            .for_each(|(_, group)| group.atk += boost);
        // fight loop until one team wins or stuck in tie
        while groups.iter().any(|(_, g)| g.team == Team::Immune)
            && groups.iter().any(|(_, g)| g.team == Team::Infect)
        {
            //target phase
            let mut order = Vec::with_capacity(groups.len());
            order.extend(
                groups
                    .iter()
                    .map(|(id, group)| (*id, group.power(), group.init)),
            );
            order.sort_unstable_by_key(|(_, pow, init)| Reverse((*pow, *init)));
            let mut target_map = HashMap::new();
            let mut already_targetted = HashSet::new();
            for (targetter_id, _, _) in &order {
                let targetter = &groups[&targetter_id];
                if let Some(target_id) = find_target(targetter, &groups, &already_targetted) {
                    target_map.insert(*targetter_id, target_id);
                    already_targetted.insert(target_id);
                }
            }
            //attack phase
            order.sort_unstable_by_key(|(_id, _pow, init)| Reverse(*init));
            let mut kill_count = 0;
            for (atk_id, _, _) in order {
                let attacker = &groups[&atk_id];
                if let Some(target_id) = target_map.get(&atk_id) {
                    let target = &groups[&target_id];
                    let dmg = calc_dmg(&attacker, &target);
                    let kills = dmg / groups[&target_id].hp;
                    kill_count += kills;
                    let target = groups.get_mut(&target_id).expect("target missing");
                    target.units = target.units.saturating_sub(kills);
                }
            }
            // cleanup after attacks
            if kill_count == 0 {
                println!("tie!, boost = {}", boost);
                continue 'boost;
            }
            groups.retain(|_, g| g.units > 0);
        } //next round of fight
        if groups.iter().any(|(_, g)| g.team == Team::Infect) {
            println!(
                "Infection wins!: {} , boost = {}",
                groups.values().map(|g| g.units).sum::<usize>(),
                boost
            );
        } else {
            println!(
                "Immunity wins!: {} , boost = {}",
                groups.values().map(|g| g.units).sum::<usize>(),
                boost
            );
            break;
        }
    }
}
///////////////////////// Parsing /////////////////////
use text_io::{scan, try_scan};
fn parse_group(line: &str, team: Team) -> Group {
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
        team,
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
