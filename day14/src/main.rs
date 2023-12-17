use anyhow::Result;
use array2d::Array2D;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone, PartialEq)]
enum State {
    Empty,
    MovableRock,
    SolidRock,
}

type Field = Array2D<State>;

fn calc_total_load(f: &Field) -> usize {
    let row_count = f.num_rows();
    let mut res = 0;

    for row in 0..row_count {
        for col in 0..f.num_columns() {
            if *f.get(row, col).unwrap() == State::MovableRock {
                res += row_count - row;
            }
        }
    }

    res
}

fn main() -> Result<()> {
    let mut f = read_input_file("../inputs/day14_input.txt")?;
    while rock_slide(&mut f, 0) {}
    println!(
        "Total load after sliding north (first star): {}",
        calc_total_load(&f)
    );

    let mut f = read_input_file("../inputs/day14_input.txt")?;
    rock_slide_cycle_n_times(&mut f, 1000000000);
    println!(
        "Total load after running 1 billion slide cycles (second star): {}",
        calc_total_load(&f)
    );

    Ok(())
}

fn rock_slide(f: &mut Field, dir: u8) -> bool {
    let mut changed = false;

    match dir {
        0 => {
            for row in 1..f.num_rows() {
                for col in 0..f.num_columns() {
                    if *f.get(row, col).unwrap() == State::MovableRock
                        && *f.get(row - 1, col).unwrap() == State::Empty
                    {
                        f.set(row - 1, col, State::MovableRock).unwrap();
                        f.set(row, col, State::Empty).unwrap();
                        changed = true;
                    }
                }
            }
        }
        1 => {
            for col in 1..f.num_columns() {
                for row in 0..f.num_rows() {
                    if *f.get(row, col).unwrap() == State::MovableRock
                        && *f.get(row, col - 1).unwrap() == State::Empty
                    {
                        f.set(row, col - 1, State::MovableRock).unwrap();
                        f.set(row, col, State::Empty).unwrap();
                        changed = true;
                    }
                }
            }
        }
        2 => {
            for row in 0..(f.num_rows() - 1) {
                for col in 0..f.num_columns() {
                    if *f.get(row, col).unwrap() == State::MovableRock
                        && *f.get(row + 1, col).unwrap() == State::Empty
                    {
                        f.set(row + 1, col, State::MovableRock).unwrap();
                        f.set(row, col, State::Empty).unwrap();
                        changed = true;
                    }
                }
            }
        }
        3 => {
            for col in 0..(f.num_columns() - 1) {
                for row in 0..f.num_rows() {
                    if *f.get(row, col).unwrap() == State::MovableRock
                        && *f.get(row, col + 1).unwrap() == State::Empty
                    {
                        f.set(row, col + 1, State::MovableRock).unwrap();
                        f.set(row, col, State::Empty).unwrap();
                        changed = true;
                    }
                }
            }
        }
        _ => {}
    }

    changed
}

fn rock_slide_cycle_n_times(f: &mut Field, n: usize) {
    // Store field state after each cycle to determine the cycle length
    let mut field_history = vec![f.clone()];
    let mut cur_cycle = 0;

    while cur_cycle < n {
        while rock_slide(f, 0) {}
        while rock_slide(f, 1) {}
        while rock_slide(f, 2) {}
        while rock_slide(f, 3) {}

        cur_cycle += 1;

        if let Some(last_occurrence) = field_history.iter().position(|x| x == f) {
            let cycle_length = cur_cycle - last_occurrence;
            println!(
                "Found cycle: Field after cycle {} is same as after cycle {} (cycle length {}).",
                cur_cycle, last_occurrence, cycle_length
            );

            cur_cycle += ((n - cur_cycle) / cycle_length) * cycle_length;
            println!("Skipping to cycle {}.", cur_cycle);
            field_history.clear(); // Avoid running this code again in the same run
        }

        field_history.push(f.clone());
    }
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Field> {
    let input = read_to_string(input_path)?;

    let rows: Vec<Vec<State>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'O' => State::MovableRock,
                    '#' => State::SolidRock,
                    _ => State::Empty,
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
        let mut f = read_input_file("../inputs/day14_example.txt").unwrap();
        while rock_slide(&mut f, 0) {}
        assert_eq!(calc_total_load(&f), 136);
    }

    #[test]
    fn example_second_star() {
        let mut f = read_input_file("../inputs/day14_example.txt").unwrap();
        rock_slide_cycle_n_times(&mut f, 1000000000);
        assert_eq!(calc_total_load(&f), 64);
    }
}
