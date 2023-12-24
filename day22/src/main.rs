use regex::Regex;
use std::collections::HashSet;
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

/// Determines how many other bricks would start to fall in the chain reaction triggered off by removing a brick. We calculate this by
/// extending a list of unstable (= removed) bricks, iteratively adding bricks until the list does not change anymore.
fn chain_reaction_size(brick: &Brick, all_bricks: &[Brick]) -> usize {
    let mut unstable_bricks = HashSet::new();
    unstable_bricks.insert(brick.id);

    loop {
        let mut stable = true;

        // Search for bricks that are supported only by unstable bricks (and neither in unstable_bricks already nor on the floor)
        for brick in all_bricks {
            if !unstable_bricks.contains(&brick.id)
                && brick.z_range.0 != 1
                && brick
                    .supported_by
                    .iter()
                    .all(|id| unstable_bricks.contains(id))
            {
                unstable_bricks.insert(brick.id);
                stable = false;
            }
        }

        if stable {
            return unstable_bricks.len() - 1; // Originating brick is not counted
        }
    }
}

/// Checks whether this brick can be removed, i.e., no other brick is supported only by this brick. (equal to chain_reaction_size() == 1
/// for the first part of the puzzle).
fn check_brick_can_be_removed(brick: &Brick, all_bricks: &[Brick]) -> bool {
    !all_bricks.iter().any(|b| b.supported_by == [brick.id])
}

/// Lets the given brick fall down and returns it final resting position.
fn get_brick_resting_position(brick: &Brick, all_bricks: &[Brick]) -> Brick {
    let mut cur = brick.clone();

    loop {
        // Check if brick has reached the floor and cannot drop further (supported_by is empty in that case)
        if cur.z_range.0 == 1 {
            return cur;
        }

        // Drop brick by one Z unit
        let mut dropped = cur.clone();
        dropped.z_range.0 -= 1;
        dropped.z_range.1 -= 1;

        // Abort if dropped brick collides with any of the other bricks (except itself)
        let supported_by: Vec<_> = all_bricks
            .iter()
            .filter(|b| **b != *brick && dropped.intersects_with(b))
            .map(|b| b.id)
            .collect();
        if !supported_by.is_empty() {
            cur.supported_by = supported_by;
            return cur;
        }

        cur = dropped;
    }
}

/// Lets all bricks fall down to their final resting position. Since the bricks are processed by rising lower Z coordinate, the result is
/// a stable configuration where every brick is supported and the supported_by attributes of the bricks are set.
fn get_stable_state(bricks: &mut Vec<Brick>) {
    // Sort bricks by lower Z coordinate
    bricks.sort_unstable_by_key(|b| b.z_range.0);

    // For each brick, let it fall down (checking for collisions only with bricks that are below it in the input)
    for i in 0..bricks.len() {
        bricks[i] = get_brick_resting_position(&bricks[i], &bricks[0..i]);
    }
}

fn main() {
    let mut bricks = read_input_file("../inputs/day22_input.txt");
    get_stable_state(&mut bricks);

    println!(
        "Bricks that can be safely removed (first star): {}",
        bricks
            .iter()
            .filter(|b| check_brick_can_be_removed(b, &bricks))
            .count()
    );

    println!(
        "Sum of all bricks falling in chain reactions (second star): {}",
        bricks
            .iter()
            .map(|b| chain_reaction_size(b, &bricks))
            .sum::<usize>()
    );
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
        let mut bricks = read_input_file("../inputs/day22_example.txt");
        get_stable_state(&mut bricks);
        assert_eq!(
            bricks
                .iter()
                .filter(|b| check_brick_can_be_removed(b, &bricks))
                .count(),
            5
        );
    }

    #[test]
    fn example_second_star() {
        let mut bricks = read_input_file("../inputs/day22_example.txt");
        get_stable_state(&mut bricks);
        assert_eq!(
            bricks
                .iter()
                .map(|b| chain_reaction_size(b, &bricks))
                .sum::<usize>(),
            7
        );
    }
}
