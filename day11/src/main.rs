use anyhow::Result;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
struct Input {
    galaxy_pos: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
}

fn calc_distance(
    input: &Input,
    pos1: (usize, usize),
    pos2: (usize, usize),
    expansion_factor: usize,
) -> usize {
    let row_min = pos1.0.min(pos2.0);
    let row_max = pos1.0.max(pos2.0);
    let column_min = pos1.1.min(pos2.1);
    let column_max = pos1.1.max(pos2.1);

    let dist_rows = row_max - row_min
        + (expansion_factor - 1)
            * input
                .empty_rows
                .iter()
                .filter(|n| **n > row_min && **n < row_max)
                .count();

    let dist_columns = column_max - column_min
        + (expansion_factor - 1)
            * input
                .empty_columns
                .iter()
                .filter(|n| **n > column_min && **n < column_max)
                .count();

    dist_rows + dist_columns
}

fn calc_distance_all_pairs(input: &Input, expansion_factor: usize) -> usize {
    let mut dist_sum = 0;

    for (i, pos1) in input.galaxy_pos.iter().enumerate() {
        for pos2 in input.galaxy_pos[(i + 1)..].iter() {
            dist_sum += calc_distance(input, *pos1, *pos2, expansion_factor);
        }
    }

    dist_sum
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day11_input.txt")?;

    println!(
        "Sum of distances between all pairs with expansion factor 1 (first star): {}",
        calc_distance_all_pairs(&input, 1)
    );

    println!(
        "Sum of distances between all pairs with expansion factor 1000000 (second star): {}",
        calc_distance_all_pairs(&input, 1000000)
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Input> {
    let input = read_to_string(input_path)?;
    let mut galaxy_pos = vec![];

    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c == '#' {
                galaxy_pos.push((row, column));
            }
        }
    }

    let max_row = *galaxy_pos.iter().map(|(row, _)| row).max().unwrap();
    let max_column = *galaxy_pos.iter().map(|(_, column)| column).max().unwrap();

    let input = Input {
        empty_rows: (0..max_row)
            .filter(|n| !galaxy_pos.iter().any(|(row, _)| row == n))
            .collect(),
        empty_columns: (0..max_column)
            .filter(|n| !galaxy_pos.iter().any(|(_, column)| column == n))
            .collect(),
        galaxy_pos,
    };

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = read_input_file("../inputs/day11_example.txt").unwrap();
        assert_eq!(calc_distance_all_pairs(&input, 2), 374);
    }

    #[test]
    fn examples_second_star() {
        let input = read_input_file("../inputs/day11_example.txt").unwrap();
        assert_eq!(calc_distance_all_pairs(&input, 10), 1030);
        assert_eq!(calc_distance_all_pairs(&input, 100), 8410);
    }
}
