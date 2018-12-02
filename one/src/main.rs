use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::FromIterator;
use std::collections::HashSet;

fn read_changelist() -> Vec<i64> {
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);

    reader.lines().map(|line| {
        let l = line.unwrap();
        let mut letters = l.chars();
        let sign = letters.next().unwrap().to_string();
        let value = String::from_iter(letters).parse::<i64>().unwrap();

        if sign == "-" {
            -value
        } else {
            value
        }
    }).collect()
}

fn main() {
    let changelist = read_changelist();
    let mut total: i64 = 0;
    let mut totals: HashSet<i64> = HashSet::new();

    for value in changelist.into_iter().cycle() {
        total += value;
        if totals.contains(&total) {
            println!("{}", total);
            break;
        } else {
            totals.insert(total);
        }
    }
}
