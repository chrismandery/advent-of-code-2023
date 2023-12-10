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

#[derive(Clone, PartialEq)]
enum UpscaledTile {
    Inside,
    Outside,
    Pipe,
}

type PipeField = Vec<Dir>;
type Position = (usize, usize);
type Field = Array2D<PipeField>;
type UpscaledField = Array2D<UpscaledTile>;

fn calc_enclosed_tiles(field: &Field, start_pos: (usize, usize)) -> usize {
    // Initialize x3 upscaled field for flood fill algorithm
    let mut uf: UpscaledField = Array2D::filled_with(
        UpscaledTile::Inside,
        field.num_rows() * 3,
        field.num_columns() * 3,
    );

    // Initialize x3 upscaled field with correct pipe segments from main loop
    let main_loop = calc_steps_to_farthest_point(field, start_pos);
    for (row, column) in main_loop {
        let dirs = field.get(row, column).unwrap();
        if dirs.is_empty() {
            continue;
        }

        uf.set(row * 3 + 1, column * 3 + 1, UpscaledTile::Pipe)
            .unwrap();

        if dirs.contains(&Dir::Up) {
            uf.set(row * 3, column * 3 + 1, UpscaledTile::Pipe).unwrap();
        }

        if dirs.contains(&Dir::Down) {
            uf.set(row * 3 + 2, column * 3 + 1, UpscaledTile::Pipe)
                .unwrap();
        }

        if dirs.contains(&Dir::Left) {
            uf.set(row * 3 + 1, column * 3, UpscaledTile::Pipe).unwrap();
        }

        if dirs.contains(&Dir::Right) {
            uf.set(row * 3 + 1, column * 3 + 2, UpscaledTile::Pipe)
                .unwrap();
        }
    }

    // Run flood fill starting at (0, 0) field (assuming this is outside)
    flood_fill(&mut uf, 0, 0);

    // Count pipe fields that are inside (= consist of 9 upscaled inside tiles)
    let mut inside_fields = 0;
    for row in 0..field.num_rows() {
        for column in 00..field.num_columns() {
            if *uf.get(row * 3, column * 3).unwrap() == UpscaledTile::Inside
                && *uf.get(row * 3, column * 3 + 1).unwrap() == UpscaledTile::Inside
                && *uf.get(row * 3, column * 3 + 2).unwrap() == UpscaledTile::Inside
                && *uf.get(row * 3 + 1, column * 3).unwrap() == UpscaledTile::Inside
                && *uf.get(row * 3 + 1, column * 3 + 1).unwrap() == UpscaledTile::Inside
                && *uf.get(row * 3 + 1, column * 3 + 2).unwrap() == UpscaledTile::Inside
                && *uf.get(row * 3 + 2, column * 3).unwrap() == UpscaledTile::Inside
                && *uf.get(row * 3 + 2, column * 3 + 1).unwrap() == UpscaledTile::Inside
                && *uf.get(row * 3 + 2, column * 3 + 2).unwrap() == UpscaledTile::Inside
            {
                inside_fields += 1;
            }
        }
    }

    inside_fields
}

fn calc_steps_to_farthest_point(field: &Field, start_pos: Position) -> Vec<Position> {
    // Determine any valid direction from the start field
    let mut all_pos = vec![start_pos];
    let mut next_dir = Dir::Down; // Guessed - for a generic solution, determine this by looking at the start's neighbors

    // Start in any direction from start field and keep going until we reach the start field again
    while all_pos.len() == 1 || *all_pos.last().unwrap() != start_pos {
        let cur_pos = all_pos.last().unwrap();

        // Determine next field and next direction
        // (Note: No explicit error handling for pipes running in the void or leaving the field here, the program will crash in that case)
        let next_pos = match next_dir {
            Dir::Up => {
                next_dir = *field
                    .get(cur_pos.0 - 1, cur_pos.1)
                    .unwrap()
                    .iter()
                    .find(|dir| **dir != Dir::Down)
                    .unwrap();
                (cur_pos.0 - 1, cur_pos.1)
            }
            Dir::Down => {
                next_dir = *field
                    .get(cur_pos.0 + 1, cur_pos.1)
                    .unwrap()
                    .iter()
                    .find(|dir| **dir != Dir::Up)
                    .unwrap();
                (cur_pos.0 + 1, cur_pos.1)
            }
            Dir::Left => {
                next_dir = *field
                    .get(cur_pos.0, cur_pos.1 - 1)
                    .unwrap()
                    .iter()
                    .find(|dir| **dir != Dir::Right)
                    .unwrap();
                (cur_pos.0, cur_pos.1 - 1)
            }
            Dir::Right => {
                next_dir = *field
                    .get(cur_pos.0, cur_pos.1 + 1)
                    .unwrap()
                    .iter()
                    .find(|dir| **dir != Dir::Left)
                    .unwrap();
                (cur_pos.0, cur_pos.1 + 1)
            }
        };

        all_pos.push(next_pos);
    }

    all_pos
}

fn flood_fill(f: &mut UpscaledField, row: usize, column: usize) {
    if *f.get(row, column).unwrap() != UpscaledTile::Inside {
        return;
    }

    f.set(row, column, UpscaledTile::Outside).unwrap();

    if row != 0 {
        flood_fill(f, row - 1, column);
    }
    if column != 0 {
        flood_fill(f, row, column - 1);
    }
    if row != f.num_rows() - 1 {
        flood_fill(f, row + 1, column);
    }
    if column != f.num_columns() - 1 {
        flood_fill(f, row, column + 1);
    }
}

fn main() -> Result<()> {
    let (field, start_pos) = read_input_file("../inputs/day10_input.txt")?;

    println!(
        "Number of steps to point farthest away in the loop: {}",
        calc_steps_to_farthest_point(&field, start_pos).len() / 2
    );

    println!(
        "Number of tiles enclosed in the loop: {}",
        calc_enclosed_tiles(&field, start_pos)
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
    fn test_first_star_example() {
        let (field, start_pos) = read_input_file("../inputs/day10_example1.txt").unwrap();
        assert_eq!(calc_steps_to_farthest_point(&field, start_pos).len() / 2, 8);
    }

    #[test]
    fn test_second_star_example1() {
        let (field, start_pos) = read_input_file("../inputs/day10_example2.txt").unwrap();
        assert_eq!(calc_enclosed_tiles(&field, start_pos), 8);
    }

    #[test]
    fn test_second_star_example2() {
        let (field, start_pos) = read_input_file("../inputs/day10_example3.txt").unwrap();
        assert_eq!(calc_enclosed_tiles(&field, start_pos), 10);
    }
}
