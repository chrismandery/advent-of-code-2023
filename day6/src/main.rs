use anyhow::{Context, Result};
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

fn calc_number_of_ways_to_win(time: u64, record: u64) -> usize {
    // Equation to solve: (x is waiting time)
    // x * (time - x) = distance
    // <=> -x^2 + x * time = distance
    // <=> -x^2 + time * x - distance = 0
    // Can be solved using quadratic formula... but anyways, brute force will also work...
    let mut ways_to_win = 0;
    for waiting_time in 1..time {
        let dist = waiting_time * (time - waiting_time);
        if dist > record {
            ways_to_win += 1;
        }
    }

    ways_to_win
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day6_input.txt", false)?;

    println!(
        "Product of numbers (first star): {}",
        input
            .into_iter()
            .map(|(time, record)| calc_number_of_ways_to_win(time, record))
            .product::<usize>()
    );

    let input = read_input_file("../inputs/day6_input.txt", true)?;
    assert!(input.len() == 1);
    let (time, record) = input.first().unwrap();

    println!(
        "Ways to win (second star): {}",
        calc_number_of_ways_to_win(*time, *record)
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(
    input_path: P,
    ignore_white_space: bool,
) -> Result<Vec<(u64, u64)>> {
    let regex_time = Regex::new(r"^Time: ([\d ]+)$").unwrap();
    let regex_distance = Regex::new(r"^Distance: ([\d ]+)$").unwrap();

    let input = read_to_string(input_path)?;
    let mut lines = input.lines();

    let cap = regex_time
        .captures(lines.next().unwrap())
        .context("Could not parse line with times!")?;
    let times_str = cap.get(1).unwrap().as_str();

    let cap = regex_distance
        .captures(lines.next().unwrap())
        .context("Could not parse line with distances!")?;
    let distances_str = cap.get(1).unwrap().as_str();

    if ignore_white_space {
        Ok(vec![(
            times_str
                .replace(' ', "")
                .parse::<u64>()
                .expect("Could not parse time as number!"),
            distances_str
                .replace(' ', "")
                .parse::<u64>()
                .expect("Could not parse distance as number!"),
        )])
    } else {
        let times = times_str
            .split_whitespace()
            .map(|s| s.parse::<u64>().expect("Could not parse time as number!"));
        let distances = distances_str.split_whitespace().map(|s| {
            s.parse::<u64>()
                .expect("Could not parse distance as number!")
        });
        Ok(times.zip(distances).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = read_input_file("../inputs/day6_example.txt", false).unwrap();

        assert_eq!(
            input
                .into_iter()
                .map(|(time, record)| calc_number_of_ways_to_win(time, record))
                .product::<usize>(),
            288
        );
    }

    #[test]
    fn example_second_star() {
        let input = read_input_file("../inputs/day6_example.txt", true).unwrap();
        assert!(input.len() == 1);
        let (time, record) = input.first().unwrap();

        assert_eq!(calc_number_of_ways_to_win(*time, *record), 71503)
    }
}
