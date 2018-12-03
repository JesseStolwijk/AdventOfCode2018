#![feature(test)]
extern crate test;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
#[cfg(test)]
mod day2;

fn main() {
    let filename = "input/input-day-3.txt";

    println!("In file {}", filename);
    let file = File::open(filename).expect("file not found");

    let reader = BufReader::new(file);
    let claims: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    println!("{:?}", get_claim_no_collisions(claims));
}

//PART 1
fn count_collisions(claims: Vec<Claim>) -> usize {
    create_collision_map(&claims)
        .iter()
        .fold(0, |acc, (_, count)| if count >= &2 { acc + 1 } else { acc })
}

fn create_collision_map(claims: &Vec<Claim>) -> HashMap<usize, usize> {
    claims
        .iter()
        //ugly
        .fold(HashMap::<usize, usize>::new(), |mut acc, claim| {
            for x in claim.margin_left..(claim.margin_left + claim.width) {
                for y in claim.margin_right..(claim.margin_right + claim.height) {
                    *acc.entry(x + y * 1000).or_insert(0) += 1;
                }
            }
            acc
        })
}
//PART 2
fn get_claim_no_collisions(claim_strings: Vec<String>) -> Option<usize> {
    let claims = claim_strings
        .iter()
        .map(|claim| parse_claim(claim))
        .collect();

    let map = create_collision_map(&claims);
    for claim in claims.iter() {
        if !map.has_collision(claim) {
            return Some(claim.id.clone());
        }
    }

    None
}

impl Collision for HashMap<usize, usize> {
    //ugly
    fn has_collision(&self, claim: &Claim) -> bool {
        for x in claim.margin_left..(claim.margin_left + claim.width) {
            for y in claim.margin_right..(claim.margin_right + claim.height) {
                if self.get(&(x + y * 1000)).unwrap_or(&0) > &1 {
                    return true;
                }
            }
        }
        return false;
    }
}

fn parse_claim(claim: &str) -> Claim {
    let claim_no_hash = &claim[1..];

    let split: Vec<&str> = claim_no_hash.split(' ').collect();

    let id = split[0];
    let (margin_left, margin_right) =
        parse_pair(drop_last(&split[2]), ',').expect("error parsing pair");
    let (width, height) = parse_pair(split[3], 'x').expect("error parsing pair");

    Claim {
        id: usize::from_str(id).expect("error parsing string to number"),
        margin_left: margin_left,
        margin_right: margin_right,
        width: width,
        height: height,
    }
}

fn drop_last(item: &str) -> &str {
    &item[0..item.len() - 1]
}

fn parse_pair(s: &str, seperator: char) -> Option<(usize, usize)> {
    match s.find(seperator) {
        None => None,
        Some(index) => match (
            usize::from_str(&s[..index]),
            usize::from_str(&s[index + 1..]),
        ) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

trait Collision {
    fn has_collision(&self, claim: &Claim) -> bool;
}

#[derive(Eq, PartialEq, Debug)]
struct Claim {
    id: usize,
    margin_left: usize,
    margin_right: usize,
    width: usize,
    height: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn parse_claim_test() {
        let input = "#1 @ 1,3: 4x4";
        let expected = Claim {
            id: 1,
            margin_left: 1,
            margin_right: 3,
            width: 4,
            height: 4,
        };
        assert_eq!(expected, parse_claim(&input));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let filename = "input/input-day-3.txt";

        println!("In file {}", filename);
        let file = File::open(filename).expect("file not found");

        let reader = BufReader::new(file);
        let claims: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        b.iter(|| {
            get_claim_no_collisions(claims.to_owned());
        });
    }
}
