use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
struct ScratchCard {
    winning_numbers: Vec<u8>,
    numbers_we_have: Vec<u8>,
}

fn calc_points(sc: &ScratchCard) -> u32 {
    let win_count = calc_win_count(sc);

    if win_count > 0 {
        u32::pow(2, win_count - 1)
    } else {
        0
    }
}

fn calc_total_card_count(all_scs: &[ScratchCard]) -> u32 {
    let n = all_scs.len();
    let mut card_count = vec![1; n];

    for i in 0..n {
        let multiplier = card_count[i];
        let win_count = calc_win_count(&all_scs[i]);

        let mut j = i + 1;
        while j < n && j <= i + win_count as usize {
            card_count[j] += multiplier;
            j += 1;
        }
    }

    card_count.iter().sum()
}

fn calc_win_count(sc: &ScratchCard) -> u32 {
    let winning_set: HashSet<u8> = sc.winning_numbers.iter().cloned().collect();
    sc.numbers_we_have
        .iter()
        .filter(|n| winning_set.contains(n))
        .count() as u32
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day4_input.txt")?;

    println!(
        "Sum of all points (first star): {}",
        input.iter().map(calc_points).sum::<u32>()
    );

    println!(
        "Total number of cards (second star): {}",
        calc_total_card_count(&input)
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<ScratchCard>> {
    let r = Regex::new(r"^Card +\d+: ([\d ]+) \| ([\d ]+)$").unwrap();

    let input = read_to_string(input_path)?;
    let parsed: Vec<_> = input
        .lines()
        .map(|l| {
            let cap = r.captures(l).expect("Could not parse line!");

            ScratchCard {
                winning_numbers: cap
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|num_s| num_s.parse().unwrap())
                    .collect(),
                numbers_we_have: cap
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|num_s| num_s.parse().unwrap())
                    .collect(),
            }
        })
        .collect();

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = read_input_file("../inputs/day4_example.txt").unwrap();
        assert_eq!(input.iter().map(calc_points).sum::<u32>(), 13);
    }

    #[test]
    fn example_second_star() {
        let input = read_input_file("../inputs/day4_example.txt").unwrap();
        assert_eq!(calc_total_card_count(&input), 30);
    }
}
