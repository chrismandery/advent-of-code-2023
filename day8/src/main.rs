use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

struct Input {
    directions: Vec<bool>,
    network_map: HashMap<String, (String, String)>,
}

fn count_steps(input: &Input) -> Result<usize> {
    let mut step_count = 0;
    let mut cur_pos = "AAA";
    let mut cur_dir = input.directions.iter().cycle();

    while cur_pos != "ZZZ" {
        let map = input
            .network_map
            .get(cur_pos)
            .with_context(|| format!("Position {} not mapped in input!", cur_pos))?;

        cur_pos = if *cur_dir.next().unwrap() {
            &map.1
        } else {
            &map.0
        };

        step_count += 1;
    }

    Ok(step_count)
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day8_input.txt")?;

    println!("Number of steps to reach ZZZ: {}", count_steps(&input)?);

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Input> {
    let input = read_to_string(input_path)?;
    let mut lines = input.lines();

    // Read directions
    let dir_line = lines
        .next()
        .context("Could not read line with directions!")?;
    let directions = dir_line
        .chars()
        .filter_map(|c| match c {
            'L' => Some(false),
            'R' => Some(true),
            _ => None,
        })
        .collect();

    // Skip empty line
    lines.next();

    // Read mappings in network map
    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    let mut network_map = HashMap::new();

    for line in lines {
        let cap = re
            .captures(line)
            .with_context(|| format!("Could not parse: {}", line))?;
        network_map.insert(
            cap.get(1).unwrap().as_str().to_owned(),
            (
                cap.get(2).unwrap().as_str().to_owned(),
                cap.get(3).unwrap().as_str().to_owned(),
            ),
        );
    }

    Ok(Input {
        directions,
        network_map,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = read_input_file("../inputs/day8_example1.txt").unwrap();
        assert_eq!(count_steps(&input).unwrap(), 2);
    }

    #[test]
    fn example2() {
        let input = read_input_file("../inputs/day8_example2.txt").unwrap();
        assert_eq!(count_steps(&input).unwrap(), 6);
    }
}
