use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let filename = "input/input-day-1-part-2.txt";

    println!("In file {}", filename);
    let file = File::open(filename).expect("file not found");

    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| i32::from_str(&line.unwrap()).expect("error parsing string to number"))
        .collect();

    println!("{}", calc_repeated_frequency(lines));
}

// PART 1
fn calc_frequency(changes: Vec<i32>) -> i32 {
    changes.iter().sum()
}

// PART 2
fn calc_repeated_frequency(changes: Vec<i32>) -> i32 {
    let max_index = changes.len() - 1;

    let mut set = HashSet::<i32>::new();
    let mut frequency = 0;
    let mut index: usize = 0;

    set.insert(frequency);

    loop {
        frequency = calc_next_frequency(frequency, &changes[index]);

        if set.contains(&frequency) {
            break;
        }

        set.insert(frequency);

        if index == max_index {
            index = 0
        } else {
            index += 1
        }
    }

    frequency
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_frequency_test() {
        let input = vec![1, 1, 1];
        assert_eq!(3, calc_frequency(input));

        let input = vec![1, 1, -2];
        assert_eq!(0, calc_frequency(input));

        let input = vec![-1, -2, -3];
        assert_eq!(-6, calc_frequency(input));
    }

    #[test]
    fn calc_repeated_frequency_test() {
        let input = vec![1, -1];
        assert_eq!(0, calc_repeated_frequency(input));

        let input = vec![3, 3, 4, -2, -4];
        assert_eq!(10, calc_repeated_frequency(input));

        let input = vec![-6, 3, 8, 5, -6];
        assert_eq!(5, calc_repeated_frequency(input));

        let input = vec![7, 7, -2, -7, -4];
        assert_eq!(14, calc_repeated_frequency(input));
    }
}
