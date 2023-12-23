use anyhow::Result;
use array2d::Array2D;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Position = (usize, usize);
type Field = Array2D<char>;

type JunctionMap = HashMap<Position, Vec<(Position, usize)>>;

/// Returns the position of the next junction (more than one neighbor) and the distance, assuming that the given start position is already
/// one steps from the junction before.
fn find_next_junction(
    f: &Field,
    mut cur_pos: Position,
    mut last_dir: Direction,
) -> (Position, usize) {
    let mut steps = 1;

    loop {
        // Abort if we reached the start or end position
        if cur_pos.0 == 0 || cur_pos.0 == f.num_rows() - 1 {
            break;
        }

        // Check neighbors
        let mut neighbors = Vec::new();
        if cur_pos.0 > 0 && *f.get(cur_pos.0 - 1, cur_pos.1).unwrap() != '#' {
            neighbors.push(Direction::Up);
        }
        if *f.get(cur_pos.0 + 1, cur_pos.1).unwrap() != '#' {
            neighbors.push(Direction::Down);
        }
        if *f.get(cur_pos.0, cur_pos.1 - 1).unwrap() != '#' {
            neighbors.push(Direction::Left);
        }
        if *f.get(cur_pos.0, cur_pos.1 + 1).unwrap() != '#' {
            neighbors.push(Direction::Right);
        }

        assert!(neighbors.len() >= 2);
        match last_dir {
            Direction::Up => {
                neighbors.retain(|d| *d != Direction::Down);
            }
            Direction::Down => {
                neighbors.retain(|d| *d != Direction::Up);
            }
            Direction::Left => {
                neighbors.retain(|d| *d != Direction::Right);
            }
            Direction::Right => {
                neighbors.retain(|d| *d != Direction::Left);
            }
        }

        // Abort if more than one neighbor available (= we reached a junction)
        if neighbors.len() > 1 {
            break;
        }

        // Move forward
        steps += 1;
        last_dir = *neighbors.first().unwrap();
        cur_pos = match last_dir {
            Direction::Up => (cur_pos.0 - 1, cur_pos.1),
            Direction::Down => (cur_pos.0 + 1, cur_pos.1),
            Direction::Left => (cur_pos.0, cur_pos.1 - 1),
            Direction::Right => (cur_pos.0, cur_pos.1 + 1),
        };
    }

    (cur_pos, steps)
}

/// Optimized version for the second part of the puzzle (ignoring slopes), which pre-calculates the paths between junctions in the maze
/// before running the BFS.
fn get_max_length_optimized(f: &Field) -> usize {
    println!("Building graph of junctions from input...");

    let mut jm = HashMap::new();
    let mut junctions_to_add = vec![(0, 1)];

    while let Some(junction_pos) = junctions_to_add.pop() {
        let mut next = vec![];

        if junction_pos.0 != 0 && *f.get(junction_pos.0 - 1, junction_pos.1).unwrap() != '#' {
            next.push(find_next_junction(
                f,
                (junction_pos.0 - 1, junction_pos.1),
                Direction::Up,
            ));
        }

        if junction_pos.0 != f.num_rows() - 1
            && *f.get(junction_pos.0 + 1, junction_pos.1).unwrap() != '#'
        {
            next.push(find_next_junction(
                f,
                (junction_pos.0 + 1, junction_pos.1),
                Direction::Down,
            ));
        }

        if *f.get(junction_pos.0, junction_pos.1 - 1).unwrap() != '#' {
            next.push(find_next_junction(
                f,
                (junction_pos.0, junction_pos.1 - 1),
                Direction::Left,
            ));
        }

        if *f.get(junction_pos.0, junction_pos.1 + 1).unwrap() != '#' {
            next.push(find_next_junction(
                f,
                (junction_pos.0, junction_pos.1 + 1),
                Direction::Right,
            ));
        }

        for (pos, _) in next.iter() {
            if !jm.contains_key(pos) {
                junctions_to_add.push(*pos);
            }
        }

        jm.insert(junction_pos, next);
    }

    println!(
        "Build graph with a total number of {} nodes (junctions).",
        jm.len()
    );
    println!("Running BFS...");

    let (path, length) =
        get_max_length_optimized_bfs(&jm, &[(0, 1)], (f.num_rows() - 1, f.num_columns() - 2), 0)
            .expect("No path found?!");
    println!("Path between junctions: {:?}", path);

    length
}

fn get_max_length_optimized_bfs(
    jm: &JunctionMap,
    cur_path: &[Position],
    end_pos: Position,
    cur_steps: usize,
) -> Option<(Vec<Position>, usize)> {
    let cur_pos = *cur_path.last().unwrap();

    if cur_pos == end_pos {
        Some((cur_path.to_vec(), cur_steps))
    } else {
        jm.get(&cur_pos)
            .unwrap()
            .iter()
            .filter_map(|(next_pos, steps)| {
                if cur_path.contains(next_pos) {
                    None
                } else {
                    // Create copy with next position added
                    let mut new_path = cur_path.to_vec();
                    new_path.push(*next_pos);

                    get_max_length_optimized_bfs(jm, &new_path, end_pos, cur_steps + steps)
                }
            })
            .max_by_key(|(_, steps)| *steps)
    }
}

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
        "Length of the longest hike ignoring slopes (second star): {}",
        get_max_length_optimized(&f)
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
    fn example_second_star_brute_force() {
        let f = read_input_file("../inputs/day23_example.txt").unwrap();
        assert_eq!(
            get_max_length_path(&f, (1, 1), &Vec::new(), true).len(),
            154
        );
    }

    #[test]
    fn example_second_star_optimized() {
        let f = read_input_file("../inputs/day23_example.txt").unwrap();
        assert_eq!(get_max_length_optimized(&f), 154);
    }
}
