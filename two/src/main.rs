use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn letter_counts(line: String) -> HashMap<char, i32> {
    let mut counts = HashMap::new();
    for c in line.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }
    counts
}

fn has_value(counts: &HashMap<char, i32>, n: i32) -> bool {
    counts.values().any(|&v| v == n)
}

/**
 * Returns a tuple (has_twos, has_threes) for an input line.
 * has_twos
 *   is true if the line contains atleast one letter that appears
 *   exactly twice
 * has_threes
 *   is true if the line contains atleast one letter that appears
 *   exactly thrice
 */
fn has_twos_and_threes(line: String) -> (bool, bool) {
    let counts = letter_counts(line);
    (has_value(&counts, 2), has_value(&counts, 3))
}

fn checksum() -> i32 {
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);

    let mut twos = 0;
    let mut threes = 0;

    for line in reader.lines() {
        let (has_twos, has_threes) = has_twos_and_threes(line.unwrap());
        if has_twos { twos += 1; }
        if has_threes { threes += 1; }
    }

    twos * threes
}

// true if s1 and s2 differ by one char
fn is_similar(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() { return false }

    let mut diffs = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            diffs += 1;
            if diffs > 1 { return false }
        }
    }

    diffs == 1
}

fn print_diff(s1: &str, s2: &str) {
    let mut result = String::new();
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 == c2 {
            result.push(c1);
        }
    }
    println!("{}", result);
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);

    println!("{}", checksum());

    let ids: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    for id1 in &ids {
        for id2 in &ids {
            if is_similar(id1, id2) {
                print_diff(id1, id2);
                return
            }
        }
    }
}
