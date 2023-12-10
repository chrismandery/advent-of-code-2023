use anyhow::{anyhow, Result};
use array2d::Array2D;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

type PipeField = Vec<Dir>;
type Position = (usize, usize);
type Field = Array2D<PipeField>;

fn calc_steps_to_farthest_point(field: &Field, start_pos: (usize, usize)) -> Result<usize> {
    // Determine any valid direction from the start field
    let mut cur_pos = start_pos;
    let mut next_dir = Dir::Down; // TODO

    // Start in any direction from start field and keep going until we reach the start field again
    let mut num_steps = 0;

    while num_steps == 0 || cur_pos != start_pos {
        num_steps += 1;

        // Determine next field and next direction
        // (Note: No explicit error handling for pipes running in the void or leaving the field here, the program will crash in that case)
        match next_dir {
            Dir::Up => {
                cur_pos = (cur_pos.0 - 1, cur_pos.1);
                next_dir = *field
                    .get(cur_pos.0, cur_pos.1)
                    .unwrap()
                    .iter()
                    .find(|dir| **dir != Dir::Down)
                    .unwrap();
            }
            Dir::Down => {
                cur_pos = (cur_pos.0 + 1, cur_pos.1);
                next_dir = *field
                    .get(cur_pos.0, cur_pos.1)
                    .unwrap()
                    .iter()
                    .find(|dir| **dir != Dir::Up)
                    .unwrap();
            }
            Dir::Left => {
                cur_pos = (cur_pos.0, cur_pos.1 - 1);
                next_dir = *field
                    .get(cur_pos.0, cur_pos.1)
                    .unwrap()
                    .iter()
                    .find(|dir| **dir != Dir::Right)
                    .unwrap();
            }
            Dir::Right => {
                cur_pos = (cur_pos.0, cur_pos.1 + 1);
                next_dir = *field
                    .get(cur_pos.0, cur_pos.1)
                    .unwrap()
                    .iter()
                    .find(|dir| **dir != Dir::Left)
                    .unwrap();
            }
        }
    }

    Ok(num_steps / 2)
}

fn main() -> Result<()> {
    let (field, start_pos) = read_input_file("../inputs/day10_input.txt")?;

    println!(
        "Number of steps to point farthest away in the loop: {}",
        calc_steps_to_farthest_point(&field, start_pos)?
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<(Field, Position)> {
    let input = read_to_string(input_path)?;

    // Read pipe directions for each field
    let field_vec: Vec<Vec<PipeField>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '|' => {
                        vec![Dir::Up, Dir::Down]
                    }
                    '-' => vec![Dir::Left, Dir::Right],
                    'L' => vec![Dir::Up, Dir::Right],
                    'J' => vec![Dir::Up, Dir::Left],
                    '7' => vec![Dir::Down, Dir::Left],
                    'F' => vec![Dir::Down, Dir::Right],
                    '.' => vec![],
                    'S' => {
                        vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right]
                    }
                    _ => panic!("Unknown character in input!"),
                })
                .collect()
        })
        .collect();
    let field = Array2D::from_rows(&field_vec).unwrap();

    // Find start position on field
    for row in 0..field.num_rows() {
        for column in 00..field.num_columns() {
            if *field.get(row, column).unwrap() == vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                return Ok((field, (row, column)));
            }
        }
    }

    Err(anyhow!("No start position found!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let (field, start_pos) = read_input_file("../inputs/day10_example.txt").unwrap();
        assert_eq!(calc_steps_to_farthest_point(&field, start_pos).unwrap(), 8);
    }
}
