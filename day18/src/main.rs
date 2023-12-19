use anyhow::{anyhow, Result};
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

struct Input {
    direction: char,
    step_size: usize,
}

fn calc_total_area(input: &[Input]) -> usize {
    let vertices = calc_vertex_positions(input);
    let vertex_count = vertices.len();

    // Calculate inner area (doubled here) using Shoelace formula (see https://en.wikipedia.org/wiki/Shoelace_formula)
    let mut area2 = 0;
    for i in 0..vertex_count {
        let i_next = if i != vertex_count - 1 { i + 1 } else { 0 };
        let i_prev = if i != 0 { i - 1 } else { vertex_count - 1 };
        area2 += vertices[i].0 * (vertices[i_next].1 - vertices[i_prev].1);
    }

    // Divide by two to get actual area
    let mut area = area2.abs() / 2;

    // Add exterior tiles: This area has already been counted half by the Shoelace formula, so we are adding the other half here
    area += input.iter().map(|instr| instr.step_size).sum::<usize>() as isize / 2;

    // Add one for the final exterior tile (starting position)
    area as usize + 1
}

fn calc_vertex_positions(input: &[Input]) -> Vec<(isize, isize)> {
    let mut vertices = vec![];
    let mut cur_pos = (0, 0);
    for instr in input {
        match instr.direction {
            'U' => {
                cur_pos.0 -= instr.step_size as isize;
            }
            'D' => {
                cur_pos.0 += instr.step_size as isize;
            }
            'L' => {
                cur_pos.1 -= instr.step_size as isize;
            }
            'R' => {
                cur_pos.1 += instr.step_size as isize;
            }
            _ => {
                panic!("Unknown direction!");
            }
        }

        vertices.push(cur_pos);
    }

    assert!(cur_pos == (0, 0)); // Last pos must be (0, 0) again

    vertices
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day18_input.txt", false)?;
    println!(
        "Interior area when ignoring the colors (first star): {}",
        calc_total_area(&input)
    );

    let input = read_input_file("../inputs/day18_input.txt", true)?;
    println!(
        "Interior area when using the color information (second star): {}",
        calc_total_area(&input)
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P, parse_colors: bool) -> Result<Vec<Input>> {
    let regex = Regex::new(r"^([UDLR]) (\d+) \(#([0-9a-z]{6})\)$").unwrap();

    let input = read_to_string(input_path)?;
    let res: Vec<Result<Input>> = input
        .lines()
        .map(|l| {
            if let Some(cap) = regex.captures(l) {
                if parse_colors {
                    let color = cap.get(3).unwrap().as_str();

                    Ok(Input {
                        direction: match color.chars().last().unwrap() {
                            '0' => 'R',
                            '1' => 'D',
                            '2' => 'L',
                            _ => 'U',
                        },
                        step_size: usize::from_str_radix(&color[0..5], 16).unwrap(),
                    })
                } else {
                    Ok(Input {
                        direction: cap.get(1).unwrap().as_str().chars().next().unwrap(),
                        step_size: cap.get(2).unwrap().as_str().parse().unwrap(),
                    })
                }
            } else {
                Err(anyhow!("Could not parse line with regex: {}", l))
            }
        })
        .collect();

    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = read_input_file("../inputs/day18_example.txt", false).unwrap();
        assert_eq!(calc_total_area(&input), 62);
    }

    #[test]
    fn example_second_star() {
        let input = read_input_file("../inputs/day18_example.txt", true).unwrap();
        assert_eq!(calc_total_area(&input), 952408144115);
    }
}
