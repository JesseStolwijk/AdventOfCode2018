use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

fn calc_frequency(diffs: Vec<String>) -> i32 {
    diffs.iter().fold(0, |acc: i32, item: &String| -> i32 {
        let operator = &item[0..1];
        let value = &item[1..item.len()];
        let number = i32::from_str(value).expect("error parsing string to number");
        match operator {
            "+" => acc + number,
            "-" => acc - number,
            _ => acc,
        }
    })
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
}
