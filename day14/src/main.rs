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
        _ => {}
    }

    changed
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
}
