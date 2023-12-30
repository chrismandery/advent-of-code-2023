use anyhow::{anyhow, Result};
use array2d::Array2D;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

type Field = Array2D<char>;

// Check https://github.com/samoylenkodmitry/AdventOfCode_2023/blob/main/src/day21.rs

/// Implement a simple BFS. Note that this function already implements the wrap-around logic necessary for the second part of the puzzle.
fn calc_reachable_fields(field: &Field, start_pos: (isize, isize), steps: usize) -> usize {
    let mut cur_pos = HashSet::new();
    cur_pos.insert(start_pos);

    for _ in 0..steps {
        let mut new_pos = HashSet::new();

        for (row, col) in cur_pos {
            new_pos.insert((row - 1, col));
            new_pos.insert((row, col - 1));
            new_pos.insert((row + 1, col));
            new_pos.insert((row, col + 1));
        }

        new_pos.retain(|(row, col)| {
            *field
                .get(
                    row.rem_euclid(field.num_rows() as isize) as usize,
                    col.rem_euclid(field.num_columns() as isize) as usize,
                )
                .unwrap()
                != '#'
        });
        cur_pos = new_pos;
    }

    cur_pos.len()
}

/// Solve second part of the puzzle (not completely generic, some assumptions about the characteristics of the input and field size used
/// here, see comments below).
fn calc_reachable_fields_second_star(field: &Field, start_pos: (isize, isize)) -> usize {
    let total_steps = 26501365; // = 202300 * 131 + 65
    let total_tiles = total_steps / 131;

    // Determine reachable fields after 65, 196 and 327 steps to fit quadratic formula
    // (see discussion at https://www.reddit.com/r/adventofcode/comments/18nevo3/2023_day_21_solutions/)
    let reachable_65_steps = calc_reachable_fields(field, start_pos, 65);
    let reachable_196_steps = calc_reachable_fields(field, start_pos, 196);
    let reachable_327_steps = calc_reachable_fields(field, start_pos, 327);

    // With the values for 65, 196 and and 327 steps known, we can now fit to the quadratic formula k * 131 + 65 (with k=0, k=1 and k=2)
    let s_65_196 = reachable_196_steps - reachable_65_steps;
    let s_196_327 = reachable_327_steps - reachable_196_steps;

    // Calculate value of quadratic function for x = total_steps
    reachable_65_steps
        + s_65_196 * total_tiles
        + (total_tiles * (total_tiles - 1) / 2) * (s_196_327 - s_65_196)
}

fn main() -> Result<()> {
    let (field, start_pos) = read_input_file("../inputs/day21_input.txt")?;

    println!(
        "Reachable fields after 64 steps (first star): {}",
        calc_reachable_fields(&field, start_pos, 64)
    );

    println!(
        "Reachable fields after 26501365 steps (second star): {}",
        calc_reachable_fields_second_star(&field, start_pos)
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<(Field, (isize, isize))> {
    let input = read_to_string(input_path)?;

    let field_vec: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let field = Array2D::from_rows(&field_vec).unwrap();

    // Find start position on field
    for row in 0..field.num_rows() {
        for column in 00..field.num_columns() {
            if *field.get(row, column).unwrap() == 'S' {
                return Ok((field, (row as isize, column as isize)));
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
        let (field, start_pos) = read_input_file("../inputs/day21_example.txt").unwrap();
        assert_eq!(calc_reachable_fields(&field, start_pos, 6), 16);
    }
}
