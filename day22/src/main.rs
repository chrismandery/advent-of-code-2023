use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone, Eq, PartialEq)]
struct Brick {
    id: usize,
    x_range: (usize, usize),
    y_range: (usize, usize),
    z_range: (usize, usize),
    supported_by: Vec<usize>,
}

impl Brick {
    fn intersects_with(&self, other: &Self) -> bool {
        for x in self.x_range.0..=self.x_range.1 {
            for y in self.y_range.0..=self.y_range.1 {
                for z in self.z_range.0..=self.z_range.1 {
                    if x >= other.x_range.0
                        && x <= other.x_range.1
                        && y >= other.y_range.0
                        && y <= other.y_range.1
                        && z >= other.z_range.0
                        && z <= other.z_range.1
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
}

/// Lets the given brick fall down and returns it final resting position.
fn get_brick_resting_position(brick: &Brick, all_bricks: &[Brick]) -> Brick {
    let mut cur = brick.clone();

    loop {
        // Check if brick has reached the floor and cannot drop further
        if cur.z_range.0 == 1 {
            break;
        }

        // Drop brick by one Z unit
        let mut dropped = cur.clone();
        dropped.z_range.0 -= 1;
        dropped.z_range.1 -= 1;

        // Abort if dropped brick collides with any of the other bricks (except itself)
        if all_bricks
            .iter()
            .any(|b| *b != *brick && dropped.intersects_with(b))
        {
            // TODO: Set supported_by
            break;
        }

        cur = dropped;
    }

    cur
}

/// Lets all bricks fall down to their final resting position. Since the bricks are processed by rising lower Z coordinate, the result is
/// a stable configuration where every brick is supported.
fn get_stable_state(bricks: &[Brick]) -> Vec<Brick> {
    todo!()
}

fn main() {
    let bricks = read_input_file("../inputs/day22_input.txt");
    let bricks = get_stable_state(&bricks);

    println!("Bricks that can be safely removed (first star): {}", 0);
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Vec<Brick> {
    let re = Regex::new(r"^(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)$").unwrap();
    let mut brick_id = 0;

    let input = read_to_string(input_path).expect("Could not open file!");
    input
        .lines()
        .map(|l| {
            let cap = re.captures(l).expect("Could not parse line!");
            brick_id += 1;

            Brick {
                id: brick_id,
                x_range: (
                    cap.get(1).unwrap().as_str().parse().unwrap(),
                    cap.get(4).unwrap().as_str().parse().unwrap(),
                ),
                y_range: (
                    cap.get(2).unwrap().as_str().parse().unwrap(),
                    cap.get(5).unwrap().as_str().parse().unwrap(),
                ),
                z_range: (
                    cap.get(3).unwrap().as_str().parse().unwrap(),
                    cap.get(6).unwrap().as_str().parse().unwrap(),
                ),
                supported_by: vec![],
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let bricks = read_input_file("../inputs/day22_example.txt");
        // TODO
    }
}
