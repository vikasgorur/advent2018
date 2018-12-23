use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

struct Unit {
    letter: char,
    active: bool
}

fn parse_polymer() -> Vec<Unit> {
    let mut input = File::open("input.txt").unwrap();
    let mut contents = String::new();

    let mut units = Vec::with_capacity(50100);
    input.read_to_string(&mut contents).unwrap();

    // remove newline
    let len = contents.len();
    contents.truncate(len - 1);

    for letter in contents.chars() {
        units.push(Unit { 
            letter,
            active: true
        });
    }

    units
}

fn uppercase_val(c: char) -> u8 {
    if c.is_uppercase() {
        return 1;
    } else {
        return 0;
    }
}

fn reacts(u1: &Unit, u2: &Unit) -> bool {
    u1.letter.to_ascii_lowercase() == u2.letter.to_ascii_lowercase() && 
        uppercase_val(u1.letter) + uppercase_val(u2.letter) == 1
}

// return the index of the previous active unit in the list,
// starting from i
fn prev_active(units: &Vec<Unit>, start: usize) -> usize {
    if start < 2 {
        return 0;
    }

    let mut i: usize = start - 1;
    while i > 0 && !units[i].active {
        i -= 1;
    }
    i
}

// return the index of the next active unit in the list, starting
// from i
fn next_active(units: &Vec<Unit>, start: usize) -> usize {
    if start >= units.len() - 1 {
        return units.len() - 1;
    }

    let mut i = start + 1;
    while i < units.len() && !units[i].active{
        i += 1;
    }
    if i == units.len() {
        // there is no active unit after start
        return start
    }
    i
}

fn part1(units: &mut Vec<Unit>) -> usize {
    let (mut i, mut j) = (0, 1);

    while i != j && i < units.len() - 1 {
        if reacts(&units[i], &units[j]) {
            units[i].active = false;
            units[j].active = false;
            i = prev_active(&units, i);
            j = next_active(&units, i);
        } else {
            i = j;
            j = next_active(&units, i);
        }
    }

    let mut total = 0;
    for unit in units.iter() {
        if unit.active {
            total += 1;
        }
    }
    total
}

fn mark_all_active(units: &mut Vec<Unit>) {
    units.iter_mut().for_each(|u| { u.active = true; });
}

fn mark_type_inactive(units: &mut Vec<Unit>, type_letter: char) {
    units.iter_mut().for_each(|u| {
        if u.letter.to_ascii_lowercase() == type_letter.to_ascii_lowercase() {
            u.active = false;
        }
    });
}

fn part2(units: &mut Vec<Unit>) -> usize {
    let mut types: HashSet<char> = HashSet::new();
    units.iter().for_each(|u| { types.insert(u.letter.to_ascii_lowercase()); });

    // length of the smallest polymer
    let mut smallest = std::usize::MAX;

    for t in types.iter() {
        mark_all_active(units);
        mark_type_inactive(units, *t);

        let length = part1(units);
        if length < smallest { smallest = length; }
    }
    smallest
}

fn main() {
    let mut units = parse_polymer();
    println!("{}", part1(&mut units));
    println!("{}", part2(&mut units));
}
