use anyhow::{anyhow, Result};
use array2d::Array2D;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

type Field = Array2D<char>;

/// Implement a simple BFS
fn calc_reachable_fields(field: &Field, start_pos: (usize, usize), steps: usize) -> usize {
    let mut cur_pos = HashSet::new();
    cur_pos.insert(start_pos);

    for _ in 0..steps {
        let mut new_pos = HashSet::new();

        for (row, col) in cur_pos {
            if row != 0 {
                new_pos.insert((row - 1, col));
            }
            if col != 0 {
                new_pos.insert((row, col - 1));
            }
            if row != field.num_rows() - 1 {
                new_pos.insert((row + 1, col));
            }
            if col != field.num_columns() - 1 {
                new_pos.insert((row, col + 1));
            }
        }

        new_pos.retain(|(row, col)| *field.get(*row, *col).unwrap() != '#');
        cur_pos = new_pos;
    }

    cur_pos.len()
}

fn main() -> Result<()> {
    let (field, start_pos) = read_input_file("../inputs/day21_input.txt")?;

    println!(
        "Reachable fields (first star): {}",
        calc_reachable_fields(&field, start_pos, 64)
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<(Field, (usize, usize))> {
    let input = read_to_string(input_path)?;

    let field_vec: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let field = Array2D::from_rows(&field_vec).unwrap();

    // Find start position on field
    for row in 0..field.num_rows() {
        for column in 00..field.num_columns() {
            if *field.get(row, column).unwrap() == 'S' {
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
        let (field, start_pos) = read_input_file("../inputs/day21_example.txt").unwrap();
        assert_eq!(calc_reachable_fields(&field, start_pos, 6), 16);
    }
}
