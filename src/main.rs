use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let filename = "input/input-day-1-part-2.txt";

    println!("In file {}", filename);
    let file = File::open(filename).expect("file not found");

    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap()).collect();

    println!("{}", calc_repeated_frequency(lines));
}

fn calc_repeated_frequency(changes: Vec<String>) -> i32 {
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

fn calc_frequency(changes: Vec<String>) -> i32 {
    changes.iter().fold(0, calc_next_frequency)
}

fn calc_next_frequency(acc: i32, item: &String) -> i32 {
    let operator = &item[0..1];
    let value = &item[1..item.len()];
    let number = i32::from_str(value).expect("error parsing string to number");
    match operator {
        "+" => acc + number,
        "-" => acc - number,
        _ => acc,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_frequency_test() {
        let input = vec!["+1".to_string(), "+1".to_string(), "+1".to_string()];
        assert_eq!(3, calc_frequency(input));

        let input = vec!["+1".to_string(), "+1".to_string(), "-2".to_string()];
        assert_eq!(0, calc_frequency(input));

        let input = vec!["-1".to_string(), "-2".to_string(), "-3".to_string()];
        assert_eq!(-6, calc_frequency(input));
    }

    #[test]
    fn calc_repeated_frequency_test() {
        let input = vec!["+1".to_string(), "-1".to_string()];
        assert_eq!(0, calc_repeated_frequency(input));

        let input = vec![
            "+3".to_string(),
            "+3".to_string(),
            "+4".to_string(),
            "-2".to_string(),
            "-4".to_string(),
        ];
        assert_eq!(10, calc_repeated_frequency(input));

        let input = vec![
            "-6".to_string(),
            "+3".to_string(),
            "+8".to_string(),
            "+5".to_string(),
            "-6".to_string(),
        ];
        assert_eq!(5, calc_repeated_frequency(input));

        let input = vec![
            "+7".to_string(),
            "+7".to_string(),
            "-2".to_string(),
            "-7".to_string(),
            "-4".to_string(),
        ];
        assert_eq!(14, calc_repeated_frequency(input));
    }
}
