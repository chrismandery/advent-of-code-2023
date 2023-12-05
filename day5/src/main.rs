use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;
struct Input {
    seeds: Vec<u64>,
    mappings: Vec<Vec<MappingRule>>,
}

struct MappingRule {
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

/// Stupid brute force solution, to be (maybe) optimized later.
fn brute_force_map_all_seed_ranges(input: &Input) -> u64 {
    input
        .seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|mut c| {
            let start = *c.next().unwrap();
            let end = start + c.next().unwrap() - 1;
            println!("Bruteforcing for values {}-{}...", start, end);
            (start..=end).map(|s| map_seed(input, s)).min().unwrap()
        })
        .min()
        .unwrap()
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day5_input.txt")?;

    println!(
        "Lowest location numbers for any seed (first star): {}",
        input
            .seeds
            .iter()
            .map(|s| map_seed(&input, *s))
            .min()
            .unwrap()
    );

    println!(
        "Lowest location numbers for all seed ranges (second star): {}",
        brute_force_map_all_seed_ranges(&input)
    );

    Ok(())
}

fn map_seed(input: &Input, seed: u64) -> u64 {
    let mut value = seed;

    for mapping_table in &input.mappings {
        for mapping_rule in mapping_table {
            if value >= mapping_rule.source_start
                && value < mapping_rule.source_start + mapping_rule.range_length
            {
                value = mapping_rule.destination_start + (value - mapping_rule.source_start);
                break;
            }
        }
    }

    value
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Input> {
    let seeds_regex = Regex::new(r"^seeds: ([\d ]+)$").unwrap();
    let mapping_rule_regex = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();
    let input = read_to_string(input_path)?;
    let mut lines = input.lines();

    // Read lines with seeds
    let cap = seeds_regex
        .captures(lines.next().unwrap())
        .expect("Could not parse seeds line!");
    let seeds: Vec<_> = cap
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Could not parse number!"))
        .collect();

    assert!(lines.next().unwrap() == "");

    // Read mappings until EOF
    let mut mappings = vec![];

    while lines.next().is_some_and(|s| s.ends_with("map:")) {
        let mut rules = vec![];

        loop {
            let line = lines.next().unwrap_or("");
            if line.is_empty() {
                break;
            }
            let cap = mapping_rule_regex
                .captures(line)
                .expect("Could not parse mapping rule line!");
            let rule = MappingRule {
                destination_start: cap
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Could not parse number!"),
                source_start: cap
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Could not parse number!"),
                range_length: cap
                    .get(3)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Could not parse number!"),
            };

            rules.push(rule);
        }

        mappings.push(rules);
    }

    let input = Input { seeds, mappings };

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = read_input_file("../inputs/day5_example.txt").unwrap();
        assert_eq!(
            input.seeds.iter().map(|s| map_seed(&input, *s)).min(),
            Some(35)
        );
    }

    #[test]
    fn example_second_star() {
        let input = read_input_file("../inputs/day5_example.txt").unwrap();
        assert_eq!(brute_force_map_all_seed_ranges(&input), 46);
    }
}
