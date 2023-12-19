use anyhow::Result;
use array2d::Array2D;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::read_to_string;
use std::path::Path;

type Position = (usize, usize);
type Field = Array2D<u8>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MapPath {
    positions: Vec<Position>,
    directions: Vec<Direction>,
    cost: usize,
    heuristics_to_goal: usize,
}

impl Ord for MapPath {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristics_to_goal).cmp(&(self.cost + self.heuristics_to_goal))
    }
}

impl PartialOrd for MapPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Calculates the optimal path under the given constraints using the A* algorithm. We are using the Manhattan distance to the end position
/// as heuristics for the A* algorithm (and assume cost of only 1 for each field). Since the BinaryHeap in Rust's standard library is a max
/// heap, the Ord implementation for MapPath above is inverted. Note that the returned path does not include the start position.
fn calculate_optimal_path(f: &Field) -> MapPath {
    let rows0 = f.num_rows() - 1;
    let columns0 = f.num_columns() - 1;

    let mut open_paths = BinaryHeap::new();

    let initial_path = MapPath {
        positions: vec![],
        directions: vec![],
        cost: 0,
        heuristics_to_goal: 2 * (rows0 + columns0),
    };
    open_paths.push(initial_path);

    while let Some(cur_path) = open_paths.pop() {
        let last_pos = cur_path.positions.last().copied().unwrap_or((0, 0));
        let last_dir = cur_path
            .directions
            .last()
            .copied()
            .unwrap_or(Direction::Right);

        /* println!(
            "Cur path @ {}/{} ({} total positions), cost of {}, heuristic is {}, cost+heuristics is {}",
            last_pos.0,
            last_pos.1,
            cur_path.positions.len(),
            cur_path.cost,
            cur_path.heuristics_to_goal,
            cur_path.cost + cur_path.heuristics_to_goal
        ); */

        if last_pos == (rows0, columns0) {
            return cur_path;
        }

        // TODO: Check last three elements and determine possibly forbidden direction

        // Go up
        if last_pos.0 != 0 && last_dir != Direction::Down {
            let new_pos = (last_pos.0 - 1, last_pos.1);

            let mut p = cur_path.clone();
            p.positions.push(new_pos);
            p.directions.push(Direction::Up);
            p.cost += *f.get(new_pos.0, new_pos.1).unwrap() as usize;
            p.heuristics_to_goal = 2 * ((rows0 - new_pos.0) + (columns0 - new_pos.1));

            open_paths.push(p);
        }

        // Go down
        if last_pos.0 != rows0 && last_dir != Direction::Up {
            let new_pos = (last_pos.0 + 1, last_pos.1);

            let mut p = cur_path.clone();
            p.positions.push(new_pos);
            p.directions.push(Direction::Down);
            p.cost += *f.get(new_pos.0, new_pos.1).unwrap() as usize;
            p.heuristics_to_goal = 2 * ((rows0 - new_pos.0) + (columns0 - new_pos.1));

            open_paths.push(p);
        }

        // Go left
        if last_pos.1 != 0 && last_dir != Direction::Right {
            let new_pos = (last_pos.0, last_pos.1 - 1);

            let mut p = cur_path.clone();
            p.positions.push(new_pos);
            p.directions.push(Direction::Left);
            p.cost += *f.get(new_pos.0, new_pos.1).unwrap() as usize;
            p.heuristics_to_goal = 2 * ((rows0 - new_pos.0) + (columns0 - new_pos.1));

            open_paths.push(p);
        }

        // Go right
        if last_pos.1 != columns0 && last_dir != Direction::Left {
            let new_pos = (last_pos.0, last_pos.1 + 1);

            let mut p = cur_path.clone();
            p.positions.push(new_pos);
            p.directions.push(Direction::Right);
            p.cost += *f.get(new_pos.0, new_pos.1).unwrap() as usize;
            p.heuristics_to_goal = 2 * ((rows0 - new_pos.0) + (columns0 - new_pos.1));

            open_paths.push(p);
        }
    }

    panic!("No solution found - this should never happen!");
}

fn main() -> Result<()> {
    let f = read_input_file("../inputs/day17_input.txt")?;

    let path = calculate_optimal_path(&f);
    println!("Heat loss on optimal path (first star): {}", path.cost);

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Field> {
    let input = read_to_string(input_path)?;
    let rows: Vec<Vec<u8>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|d| d as u8)
                        .expect("Could not parse digit!")
                })
                .collect()
        })
        .collect();
    Ok(Field::from_rows(&rows).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let f = read_input_file("../inputs/day17_example.txt").unwrap();
        assert_eq!(calculate_optimal_path(&f).cost, 102);
    }
}
