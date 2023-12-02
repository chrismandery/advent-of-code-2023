use anyhow::Result;
use std::fs::read_to_string;
use std::path::Path;

fn calc_calibration_value(input: &str, parse_words: bool) -> u32 {
    // Simple (inefficient) hack to recognize those words as a digit in the correct part of the line
    let input = if parse_words {
        input
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "th3ee")
            .replace("four", "f4ur")
            .replace("five", "f5ve")
            .replace("six", "s6x")
            .replace("seven", "se7en")
            .replace("eight", "ei8ht")
            .replace("nine", "n9ne")
    } else {
        input.to_string()
    };

    let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    let first_digit = digits.chars().next().expect("No digit found?");
    let last_digit = digits.chars().next_back().expect("No digit found?");
    let number_str = format!("{}{}", first_digit, last_digit);
    number_str
        .parse()
        .expect("Could not parse combination of two digits as number")
}

fn main() -> Result<()> {
    let calibrations_values = process_input_file("../inputs/day1_input.txt", false)?;
    println!(
        "Sum of calibration values is (first star): {}",
        calibrations_values.iter().sum::<u32>()
    );

    let calibrations_values = process_input_file("../inputs/day1_input.txt", true)?;
    println!(
        "Sum of calibration values is (second star): {}",
        calibrations_values.iter().sum::<u32>()
    );

    Ok(())
}

fn process_input_file<P: AsRef<Path>>(input_path: P, parse_words: bool) -> Result<Vec<u32>> {
    let input = read_to_string(input_path)?;
    let res = input
        .lines()
        .map(|l| calc_calibration_value(l, parse_words))
        .collect();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let calibrations_values = process_input_file("../inputs/day1_example1.txt", false).unwrap();
        assert_eq!(calibrations_values, vec!(12, 38, 15, 77));
    }

    #[test]
    fn example_second_star() {
        let calibrations_values = process_input_file("../inputs/day1_example2.txt", true).unwrap();
        assert_eq!(calibrations_values, vec!(29, 83, 13, 24, 42, 14, 76));
    }
}
