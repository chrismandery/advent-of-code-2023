use anyhow::Result;
use array2d::Array2D;
use std::fs::read_to_string;
use std::path::Path;

type Field = Array2D<bool>;

fn check_horizontal_reflection(f: &Field, required_differences: usize) -> Option<usize> {
    for top_row in 0..(f.num_rows() - 1) {
        let mut diffs = 0;

        for cur_check_top in 0..=top_row {
            let cur_check_bottom = top_row + 1 + (top_row - cur_check_top);
            if cur_check_bottom >= f.num_rows() {
                continue;
            }

            diffs += (0..f.num_columns())
                .filter(|column| f.get(cur_check_top, *column) != f.get(cur_check_bottom, *column))
                .count();
        }

        if diffs == required_differences {
            return Some(top_row + 1);
        }
    }

    None
}

fn check_vertical_reflection(f: &Field, required_differences: usize) -> Option<usize> {
    for left_col in 0..(f.num_columns() - 1) {
        let mut diffs = 0;

        for cur_check_left in 0..=left_col {
            let cur_check_right = left_col + 1 + (left_col - cur_check_left);
            if cur_check_right >= f.num_columns() {
                continue;
            }

            diffs += (0..f.num_rows())
                .filter(|row| f.get(*row, cur_check_left) != f.get(*row, cur_check_right))
                .count();
        }

        if diffs == required_differences {
            return Some(left_col + 1);
        }
    }

    None
}

fn get_input_answer(input: &[Field], required_differences: usize) -> usize {
    input
        .iter()
        .map(|f| {
            let mut res = 0;

            if let Some(n) = check_horizontal_reflection(f, required_differences) {
                res += n * 100;
            }
            if let Some(n) = check_vertical_reflection(f, required_differences) {
                res += n;
            }

            res
        })
        .sum()
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day13_input.txt")?;

    println!("Answer for first star: {}", get_input_answer(&input, 0));
    println!("Answer for second star: {}", get_input_answer(&input, 1));

    Ok(())
}

fn read_field(lines: &[&str]) -> Field {
    let rows: Vec<Vec<bool>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();
    Field::from_rows(&rows).unwrap()
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<Field>> {
    let input = read_to_string(input_path)?;
    let lines: Vec<_> = input.lines().collect();

    Ok(lines.split(|l| l.is_empty()).map(read_field).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let input = read_input_file("../inputs/day13_example.txt").unwrap();
        let mut it = input.iter();

        let f = it.next().unwrap();
        assert_eq!(check_horizontal_reflection(f, 0), None);
        assert_eq!(check_vertical_reflection(f, 0), Some(5));

        let f = it.next().unwrap();
        assert_eq!(check_horizontal_reflection(f, 0), Some(4));
        assert_eq!(check_vertical_reflection(f, 0), None);
    }

    #[test]
    fn example_second_star() {
        let input = read_input_file("../inputs/day13_example.txt").unwrap();
        let mut it = input.iter();

        let f = it.next().unwrap();
        assert_eq!(check_horizontal_reflection(f, 1), Some(3));
        assert_eq!(check_vertical_reflection(f, 1), None);

        let f = it.next().unwrap();
        assert_eq!(check_horizontal_reflection(f, 1), Some(1));
        assert_eq!(check_vertical_reflection(f, 1), None);
    }
}
