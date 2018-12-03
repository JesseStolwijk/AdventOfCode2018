use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let filename = "input/input-day-3.txt";

    println!("In file {}", filename);
    let file = File::open(filename).expect("file not found");

    let reader = BufReader::new(file);
    let claims: Vec<Claim> = reader
        .lines()
        .map(|line| parse_claim(&line.unwrap()))
        .collect();

    println!("{}", count_overlap(claims));
}

fn count_overlap(claims: Vec<Claim>) -> usize {
    let mut map = HashMap::<usize, usize>::new();

    for claim in claims.iter() {
        for x in claim.margin_left..(claim.margin_left + claim.width) {
            for y in claim.margin_right..(claim.margin_right + claim.height) {
                *map.entry(x + y * 1000).or_insert(0) += 1;
            }
        }
    }

    map.iter()
        .fold(0, |acc, (_, count)| if count >= &2 { acc + 1 } else { acc })
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
}
