use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input/input-day-2.txt";

    println!("In file {}", filename);
    let file = File::open(filename).expect("file not found");

    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap()).collect();

    println!("{}", calculate_checksum(lines));
}

fn calculate_checksum(box_ids: Vec<String>) -> u32 {
    let (doublets, triplets) = box_ids.iter().fold((0, 0), |acc, item| {
        let (total_doublet_count, total_triplet_count) = acc;
        let (has_doublet, has_triplet) = check_box_id(item);
        (
            total_doublet_count + has_doublet,
            total_triplet_count + has_triplet,
        )
    });

    doublets * triplets
}

fn check_box_id(box_id: &String) -> (u32, u32) {
    let mut map = HashMap::<char, u32>::new();

    for item in box_id.chars() {
        let new_value = match map.get(&item) {
            Some(x) => x + 1,
            None => 1,
        };
        map.insert(item, new_value);
    }

    map.iter().fold((0, 0), |acc, (_, count)| {
        let (has_doublet, has_triplet) = acc;
        match count {
            2 => (1, has_triplet),
            3 => (has_doublet, 1),
            _ => acc,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_checksum_test() {
        let input = vec![
            "abcdef".to_string(),
            "bababc".to_string(),
            "abbcde".to_string(),
            "abcccd".to_string(),
            "aabcdd".to_string(),
            "abcdee".to_string(),
            "ababab".to_string(),
        ];
        assert_eq!(12, calculate_checksum(input));
    }
}
