use anyhow::Result;
use array2d::Array2D;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

type Field = Array2D<char>;
type Position = (usize, usize);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Tracks a beam from any possible start position on an edge tile and returns the maximum number of tiles being energized.
fn find_max_energized(f: &Field) -> usize {
    let mut max_tiles_energized = 0;

    let last_row = f.num_rows() - 1;
    let last_col = f.num_columns() - 1;

    for col in 0..=last_col {
        max_tiles_energized = max_tiles_energized
            .max(track_beam(f, (0, col), Direction::Down, &mut HashSet::new()).len());
        max_tiles_energized = max_tiles_energized
            .max(track_beam(f, (last_row, col), Direction::Up, &mut HashSet::new()).len());
    }

    for row in 0..=last_row {
        max_tiles_energized = max_tiles_energized
            .max(track_beam(f, (row, 0), Direction::Right, &mut HashSet::new()).len());
        max_tiles_energized = max_tiles_energized
            .max(track_beam(f, (row, last_col), Direction::Left, &mut HashSet::new()).len());
    }

    max_tiles_energized
}

fn main() -> Result<()> {
    let f = read_input_file("../inputs/day16_input.txt")?;

    println!(
        "Energized tiles from top left start position (first star): {}",
        track_beam(&f, (0, 0), Direction::Right, &mut HashSet::new()).len()
    );

    println!(
        "Maximum energized tiles from any edge tile start position: {}",
        find_max_energized(&f)
    );

    Ok(())
}

/// Tracks a beam and returns all visited positions as (row, column) from the given start position and direction. Keep track of which
/// combinations of start position and direction have already been visited to avoid cycles.
fn track_beam(
    f: &Field,
    mut cur_pos: Position,
    mut dir: Direction,
    visited: &mut HashSet<(Position, Direction)>,
) -> HashSet<Position> {
    if visited.contains(&(cur_pos, dir)) {
        return HashSet::new();
    }

    visited.insert((cur_pos, dir));

    let mut tiles = HashSet::new();

    loop {
        tiles.insert(cur_pos);
        let cur_tile = f.get(cur_pos.0, cur_pos.1).unwrap();

        // Ugly hard-coded and redundant case distinction, could be handled by a look-up table
        match dir {
            Direction::Up => match cur_tile {
                '/' => {
                    if cur_pos.1 != f.num_columns() - 1 {
                        cur_pos.1 += 1;
                        dir = Direction::Right;
                    } else {
                        break;
                    }
                }
                '\\' => {
                    if cur_pos.1 != 0 {
                        cur_pos.1 -= 1;
                        dir = Direction::Left;
                    } else {
                        break;
                    }
                }
                '-' => {
                    if cur_pos.1 != 0 {
                        tiles.extend(track_beam(
                            f,
                            (cur_pos.0, cur_pos.1 - 1),
                            Direction::Left,
                            visited,
                        ));
                    }
                    if cur_pos.1 != f.num_columns() - 1 {
                        tiles.extend(track_beam(
                            f,
                            (cur_pos.0, cur_pos.1 + 1),
                            Direction::Right,
                            visited,
                        ));
                    }
                    break;
                }
                _ => {
                    if cur_pos.0 != 0 {
                        cur_pos.0 -= 1;
                    } else {
                        break;
                    }
                }
            },
            Direction::Down => match cur_tile {
                '/' => {
                    if cur_pos.1 != 0 {
                        cur_pos.1 -= 1;
                        dir = Direction::Left;
                    } else {
                        break;
                    }
                }
                '\\' => {
                    if cur_pos.1 != f.num_columns() - 1 {
                        cur_pos.1 += 1;
                        dir = Direction::Right;
                    } else {
                        break;
                    }
                }
                '-' => {
                    if cur_pos.1 != 0 {
                        tiles.extend(track_beam(
                            f,
                            (cur_pos.0, cur_pos.1 - 1),
                            Direction::Left,
                            visited,
                        ));
                    }
                    if cur_pos.1 != f.num_columns() - 1 {
                        tiles.extend(track_beam(
                            f,
                            (cur_pos.0, cur_pos.1 + 1),
                            Direction::Right,
                            visited,
                        ));
                    }
                    break;
                }
                _ => {
                    if cur_pos.0 != f.num_rows() - 1 {
                        cur_pos.0 += 1;
                    } else {
                        break;
                    }
                }
            },
            Direction::Left => match cur_tile {
                '/' => {
                    if cur_pos.0 != f.num_rows() - 1 {
                        cur_pos.0 += 1;
                        dir = Direction::Down;
                    } else {
                        break;
                    }
                }
                '\\' => {
                    if cur_pos.0 != 0 {
                        cur_pos.0 -= 1;
                        dir = Direction::Up;
                    } else {
                        break;
                    }
                }
                '|' => {
                    if cur_pos.0 != 0 {
                        tiles.extend(track_beam(
                            f,
                            (cur_pos.0 - 1, cur_pos.1),
                            Direction::Up,
                            visited,
                        ));
                    }
                    if cur_pos.0 != f.num_rows() - 1 {
                        tiles.extend(track_beam(
                            f,
                            (cur_pos.0 + 1, cur_pos.1),
                            Direction::Down,
                            visited,
                        ));
                    }
                    break;
                }
                _ => {
                    if cur_pos.1 != 0 {
                        cur_pos.1 -= 1;
                    } else {
                        break;
                    }
                }
            },
            Direction::Right => match cur_tile {
                '/' => {
                    if cur_pos.0 != 0 {
                        cur_pos.0 -= 1;
                        dir = Direction::Up;
                    } else {
                        break;
                    }
                }
                '\\' => {
                    if cur_pos.0 != f.num_rows() - 1 {
                        cur_pos.0 += 1;
                        dir = Direction::Down;
                    } else {
                        break;
                    }
                }
                '|' => {
                    if cur_pos.0 != 0 {
                        tiles.extend(track_beam(
                            f,
                            (cur_pos.0 - 1, cur_pos.1),
                            Direction::Up,
                            visited,
                        ));
                    }
                    if cur_pos.0 != f.num_rows() - 1 {
                        tiles.extend(track_beam(
                            f,
                            (cur_pos.0 + 1, cur_pos.1),
                            Direction::Down,
                            visited,
                        ));
                    }
                    break;
                }
                _ => {
                    if cur_pos.1 != f.num_columns() - 1 {
                        cur_pos.1 += 1;
                    } else {
                        break;
                    }
                }
            },
        }
    }

    tiles
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Field> {
    let input = read_to_string(input_path)?;
    let rows: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    Ok(Field::from_rows(&rows).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let f = read_input_file("../inputs/day16_example.txt").unwrap();
        assert_eq!(
            track_beam(&f, (0, 0), Direction::Right, &mut HashSet::new()).len(),
            46
        );
    }

    #[test]
    fn example_second_star() {
        let f = read_input_file("../inputs/day16_example.txt").unwrap();
        assert_eq!(find_max_energized(&f), 51);
    }
}
