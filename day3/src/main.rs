use anyhow::Result;
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

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

fn calc_field_result(f: &[Vec<u8>]) -> u32 {
    let mut sum = 0;

    for row in 0..f.len() {
        for col in 0..f[row].len() {
            if let Some(value) = calc_adjacent_number_sum(f, row, col) {
                sum += value;
            }
        }
    }

    sum
}

/// Returns every number in the row that is adjacent to the given column index
fn get_adjacent_numbers_from_row(row: &[u8], col_index: usize) -> Vec<u32> {
    let r = Regex::new("\\d+").unwrap(); // regex recompiled every time, could be optimized
    let mut numbers = vec![];

    for m in r.find_iter(&String::from_utf8(row.to_owned()).unwrap()) {
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
        calc_field_result(&field)
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
        assert_eq!(calc_field_result(&field), 4361);
    }
}
