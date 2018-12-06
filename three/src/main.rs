#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

struct Claim {
    id: i32,
    left: usize,
    top: usize,
    right: usize,
    bottom: usize
}

// #1 @ 1,3: 4x4
fn parse_claim(line: &str) -> Claim {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }
    let caps = RE.captures(line).unwrap();

    let id = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let left = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let top = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
    let right = left + caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
    let bottom = top + caps.get(5).unwrap().as_str().parse::<usize>().unwrap();

    Claim {
        id, left, top, right, bottom
    }
}

fn parse_claims() -> Vec<Claim> {
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);

    let mut claims = Vec::with_capacity(1300);
    for line in reader.lines() {
        claims.push(parse_claim(&line.unwrap()));
    }

    claims
}

const SIDE: usize = 1200;

fn part1(fabric: &[[i32; SIDE]; SIDE]) {
    let mut contested = 0;
    for i in 0..SIDE {
        for j in 0..SIDE {
            if fabric[i][j] > 1 { contested += 1; }
        }
    }

    println!("{}", contested);
}

fn is_uncontested(claim: &Claim, fabric: &[[i32; SIDE]; SIDE]) -> bool {
    for x in claim.left..claim.right {
        for y in claim.top..claim.bottom {
            if fabric[x][y] > 1 {
                return false;
            }
        }
    }

    true
}

fn part2(claims: &Vec<Claim>, fabric: &[[i32; SIDE]; SIDE]) {
    for claim in claims {
        if is_uncontested(&claim, fabric) {
            println!("{}", claim.id);
        }
    }
}

fn main() {
    let claims = parse_claims();
    let mut fabric = [[0; SIDE]; SIDE];

    for claim in &claims {
        for x in claim.left..claim.right {
            for y in claim.top..claim.bottom {
                fabric[x][y] += 1;
            }
        }
    }

    part1(&fabric);
    part2(&claims, &fabric);
}