use anyhow::Result;
use array2d::Array2D;
use std::fs::read_to_string;
use std::path::Path;

type Position = (usize, usize);
type Field = Array2D<char>;

/// Recursively calculates the longest available path from the given position to the goal position (max row, max col - 1), never visiting
/// any field twice. (Note: Could be optimized a lot using, e.g., a HashSet to keep track of visited positions.)
fn get_max_length_path(
    f: &Field,
    pos: Position,
    cur_path: &[Position],
    ignore_slopes: bool,
) -> Vec<Position> {
    // Create local copy with current position added
    let mut new_path = cur_path.to_vec();
    new_path.push(pos);

    // Goal reached?
    if pos == (f.num_rows() - 1, f.num_columns() - 2) {
        return new_path;
    }

    // Get current field (ignoring slopes if deesired)
    let mut cur_field = *f.get(pos.0, pos.1).unwrap();
    if ignore_slopes && cur_field != '#' {
        cur_field = '.';
    }

    let mut new_paths = vec![];

    // Move up
    if pos.0 != 1
        && !new_path.contains(&(pos.0 - 1, pos.1))
        && *f.get(pos.0 - 1, pos.1).unwrap() != '#'
        && (cur_field == '.' || cur_field == '^')
    {
        new_paths.push(get_max_length_path(
            f,
            (pos.0 - 1, pos.1),
            &new_path,
            ignore_slopes,
        ));
    }

    // Move down
    if !new_path.contains(&(pos.0 + 1, pos.1))
        && *f.get(pos.0 + 1, pos.1).unwrap() != '#'
        && (cur_field == '.' || cur_field == 'v')
    {
        new_paths.push(get_max_length_path(
            f,
            (pos.0 + 1, pos.1),
            &new_path,
            ignore_slopes,
        ));
    }

    // Move left
    if !new_path.contains(&(pos.0, pos.1 - 1))
        && *f.get(pos.0, pos.1 - 1).unwrap() != '#'
        && (cur_field == '.' || cur_field == '<')
    {
        new_paths.push(get_max_length_path(
            f,
            (pos.0, pos.1 - 1),
            &new_path,
            ignore_slopes,
        ));
    }

    // Move right
    if !new_path.contains(&(pos.0, pos.1 + 1))
        && *f.get(pos.0, pos.1 + 1).unwrap() != '#'
        && (cur_field == '.' || cur_field == '>')
    {
        new_paths.push(get_max_length_path(
            f,
            (pos.0, pos.1 + 1),
            &new_path,
            ignore_slopes,
        ));
    }

    // Return path with maximal length (or empty if their is no way to go from here)
    new_paths
        .into_iter()
        .max_by_key(|p| p.len())
        .unwrap_or(Vec::new())
}

fn main() -> Result<()> {
    let f = read_input_file("../inputs/day23_input.txt")?;

    println!(
        "Length of the longest hike respecting slopes (first star): {}",
        get_max_length_path(&f, (1, 1), &Vec::new(), false).len()
    );

    println!(
        "Length of the longest hike ignoring slopes (first star): {}",
        get_max_length_path(&f, (1, 1), &Vec::new(), true).len()
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Field> {
    let input = read_to_string(input_path)?;

    let field_vec: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let field = Array2D::from_rows(&field_vec).unwrap();

    Ok(field)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let f = read_input_file("../inputs/day23_example.txt").unwrap();
        assert_eq!(
            get_max_length_path(&f, (1, 1), &Vec::new(), false).len(),
            94
        );
    }

    #[test]
    fn example_second_star() {
        let f = read_input_file("../inputs/day23_example.txt").unwrap();
        assert_eq!(
            get_max_length_path(&f, (1, 1), &Vec::new(), true).len(),
            154
        );
    }
}
