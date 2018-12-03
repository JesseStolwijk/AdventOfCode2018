use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input(filename: &str) -> Vec<String> {
    println!("In file {}", filename);
    let file = File::open(filename).expect("file not found");

    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap()).collect()
}

pub fn part1() -> Option<String> {
    let input = get_input("input/input-day-2.txt");
    Some(calculate_checksum(&input).to_string())
}

pub fn part2() -> Option<String> {
    let input = get_input("input/input-day-2.txt");
    common_letters(&input)
}

// PART 1
fn calculate_checksum(box_ids: &Vec<String>) -> u32 {
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

// PART 2
fn common_letters(box_ids: &Vec<String>) -> Option<String> {
    for box_id in box_ids {
        let result = is_correct_box_id(&box_id, &box_ids);

        if result.is_some() {
            return result;
        }
    }

    None
}

fn is_correct_box_id(box_id: &String, box_ids: &Vec<String>) -> Option<String> {
    let box_id_chars: Vec<char> = box_id.chars().collect();

    for current_id in box_ids.iter() {
        let mut different_char_count = 0;
        for (index, c) in current_id.chars().enumerate() {
            let box_id_c = box_id_chars[index];
            if c != box_id_c {
                different_char_count += 1
            }
        }
        if different_char_count == 1 {
            let mutual_chars = inner_join(box_id, current_id);
            return Some(mutual_chars);
        }
    }

    None
}

fn inner_join(a: &str, b: &str) -> String {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    a_chars
        .iter()
        .enumerate()
        .fold(String::new(), |mut acc, (index, c)| {
            if c == &b_chars[index] {
                acc.push(c.clone());
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;
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
        assert_eq!(12, calculate_checksum(&input));
    }

    #[test]
    fn calc_checksum_test_part_2() {
        let input = vec![
            "abcde".to_string(),
            "fghij".to_string(),
            "klmno".to_string(),
            "pqrst".to_string(),
            "fguij".to_string(),
            "axcye".to_string(),
            "wvxyz".to_string(),
        ];
        assert_eq!(Some("fgij".to_string()), common_letters(&input));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| part2());
    }
}
