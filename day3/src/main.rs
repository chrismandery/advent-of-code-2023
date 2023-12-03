use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

lazy_static! {
    // Compile regex only once
    static ref NUMBERS_REGEX: Regex = Regex::new("\\d+").unwrap();
}

/// Extracts the sum of all numbers that are adjacent to the given field, if this field contains a symbol.
fn calc_adjacent_number_sum(f: &[Vec<u8>], row: usize, col: usize) -> Option<u32> {
    let c = f[row][col];

    if c as char == '.' || c.is_ascii_digit() {
        None
    } else {
        let mut sum = get_adjacent_numbers_from_row(&f[row], col).iter().sum();

        if row != 0 {
            sum += get_adjacent_numbers_from_row(&f[row - 1], col)
                .iter()
                .sum::<u32>();
        }

        if row != f.len() - 1 {
            sum += get_adjacent_numbers_from_row(&f[row + 1], col)
                .iter()
                .sum::<u32>();
        }

        Some(sum)
    }
}

/// Extracts the product of two adjacent numbers if this field contains a * symbol (second part of the puzzle).
fn calc_adjacent_gear_ratio(f: &[Vec<u8>], row: usize, col: usize) -> Option<u32> {
    let c = f[row][col];

    if c as char == '*' {
        let mut numbers = vec![];

        numbers.append(&mut get_adjacent_numbers_from_row(&f[row], col));

        if row != 0 {
            numbers.append(&mut get_adjacent_numbers_from_row(&f[row - 1], col));
        }

        if row != f.len() - 1 {
            numbers.append(&mut get_adjacent_numbers_from_row(&f[row + 1], col));
        }

        if numbers.len() == 2 {
            return Some(numbers.first().unwrap() * numbers.get(1).unwrap());
        }
    }

    None
}

fn calc_field_result(f: &[Vec<u8>], compute_gear_ratio: bool) -> u32 {
    let mut sum = 0;

    for row in 0..f.len() {
        for col in 0..f[row].len() {
            let res = if compute_gear_ratio {
                calc_adjacent_gear_ratio(f, row, col)
            } else {
                calc_adjacent_number_sum(f, row, col)
            };

            if let Some(value) = res {
                sum += value;
            }
        }
    }

    sum
}

/// Returns every number in the row that is adjacent to the given column index
fn get_adjacent_numbers_from_row(row: &[u8], col_index: usize) -> Vec<u32> {
    let mut numbers = vec![];

    for m in NUMBERS_REGEX.find_iter(&String::from_utf8(row.to_owned()).unwrap()) {
        if m.start() <= col_index + 1 && m.end() >= col_index {
            numbers.push(m.as_str().parse().unwrap());
        }
    }

    numbers
}

fn main() -> Result<()> {
    let field = read_input_file("../inputs/day3_input.txt")?;

    println!(
        "Sum of all numbers that adjacent to symbols: {}",
        calc_field_result(&field, false)
    );

    println!(
        "Sum of all gear ratios: {}",
        calc_field_result(&field, true)
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<Vec<u8>>> {
    let input = read_to_string(input_path)?;
    let rows: Vec<_> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let field = read_input_file("../inputs/day3_example.txt").unwrap();
        assert_eq!(calc_field_result(&field, false), 4361);
    }

    #[test]
    fn example_second_star() {
        let field = read_input_file("../inputs/day3_example.txt").unwrap();
        assert_eq!(calc_field_result(&field, true), 467835);
    }
}
