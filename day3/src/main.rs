use anyhow::Result;
use array2d::Array2D;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<()> {
    let field = read_input_file("../inputs/day3_input.txt")?;

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Array2D<char>> {
    let input = read_to_string(input_path)?;
    let rows: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    Ok(Array2D::from_rows(&rows))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let field = read_input_file("../inputs/day3_example.txt").unwrap();
    }
}
