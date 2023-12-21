use anyhow::Result;
use array2d::Array2D;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

type Position = (usize, usize);
type Field = Array2D<u8>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    position: Position,
    last_move_was_vertical: Option<bool>,
}

/// Calculates the optimal path under the given constraints using the Djikstra algorithm. Note that the returned path does not include the
/// start position.
fn calculate_optimal_path_cost(f: &Field, step_size_min: usize, step_size_max: usize) -> usize {
    let rows0 = f.num_rows() - 1;
    let columns0 = f.num_columns() - 1;

    let mut open_paths = HashMap::new();

    let initial_path = State {
        position: (0, 0),
        last_move_was_vertical: None,
    };
    open_paths.insert(initial_path, 0);

    loop {
        // Get path with minimal cost
        let (last_state, last_cost) = open_paths
            .iter()
            .min_by_key(|(_, cost)| *cost)
            .map(|(k, v)| (k.clone(), *v))
            .unwrap();
        open_paths.remove(&last_state);

        /* println!(
            "At {}/{} with cost {}",
            last_state.position.0, last_state.position.1, last_cost
        ); */

        if last_state.position == (rows0, columns0) {
            return last_cost;
        }

        if !last_state.last_move_was_vertical.unwrap_or(false) {
            // Go up
            let mut added_cost = 0;

            for i in 1..=step_size_max {
                if last_state.position.0 < i {
                    break;
                }

                let new_pos = (last_state.position.0 - i, last_state.position.1);
                added_cost += *f.get(new_pos.0, new_pos.1).unwrap() as usize;

                if i >= step_size_min {
                    let s = State {
                        position: new_pos,
                        last_move_was_vertical: Some(true),
                    };

                    let old_cost = *open_paths.get(&s).unwrap_or(&usize::MAX);
                    let new_cost = old_cost.min(last_cost + added_cost);
                    open_paths.insert(s, new_cost);
                }
            }

            // Go down
            let mut added_cost = 0;

            for i in 1..=step_size_max {
                if last_state.position.0 + i > rows0 {
                    break;
                }

                let new_pos = (last_state.position.0 + i, last_state.position.1);
                added_cost += *f.get(new_pos.0, new_pos.1).unwrap() as usize;

                if i >= step_size_min {
                    let s = State {
                        position: new_pos,
                        last_move_was_vertical: Some(true),
                    };

                    let old_cost = *open_paths.get(&s).unwrap_or(&usize::MAX);
                    let new_cost = old_cost.min(last_cost + added_cost);
                    open_paths.insert(s, new_cost);
                }
            }
        }

        if last_state.last_move_was_vertical.unwrap_or(true) {
            // Go left
            let mut added_cost = 0;

            for i in 1..=step_size_max {
                if last_state.position.1 < i {
                    break;
                }

                let new_pos = (last_state.position.0, last_state.position.1 - i);
                added_cost += *f.get(new_pos.0, new_pos.1).unwrap() as usize;

                if i >= step_size_min {
                    let s = State {
                        position: new_pos,
                        last_move_was_vertical: Some(false),
                    };

                    let old_cost = *open_paths.get(&s).unwrap_or(&usize::MAX);
                    let new_cost = old_cost.min(last_cost + added_cost);
                    open_paths.insert(s, new_cost);
                }
            }

            // Go right
            let mut added_cost = 0;

            for i in 1..=step_size_max {
                if last_state.position.1 + i > columns0 {
                    break;
                }

                let new_pos = (last_state.position.0, last_state.position.1 + i);
                added_cost += *f.get(new_pos.0, new_pos.1).unwrap() as usize;

                if i >= step_size_min {
                    let s = State {
                        position: new_pos,
                        last_move_was_vertical: Some(false),
                    };

                    let old_cost = *open_paths.get(&s).unwrap_or(&usize::MAX);
                    let new_cost = old_cost.min(last_cost + added_cost);
                    open_paths.insert(s, new_cost);
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let f = read_input_file("../inputs/day17_input.txt")?;

    println!(
        "Heat loss on optimal path first star: {}",
        calculate_optimal_path_cost(&f, 1, 3)
    );

    println!(
        "Heat loss on optimal path for second star: {}",
        calculate_optimal_path_cost(&f, 4, 10)
    );

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
        assert_eq!(calculate_optimal_path_cost(&f, 1, 3), 102);
    }

    #[test]
    fn example_second_star() {
        let f = read_input_file("../inputs/day17_example.txt").unwrap();
        assert_eq!(calculate_optimal_path_cost(&f, 4, 10), 94);
    }
}
