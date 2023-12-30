use anyhow::Result;
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
struct Hailstone {
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
}

/// Checks for a 2D hailstone collision (ignoring the Z component) by solving the following linear equation system:
/// pos1_x + t * vel1_x = pos2_x + t * vel2_x
/// pos1_y + t * vel1_y = pos2_y + t * vel2_y
/// First equation is equal two:
/// t * (vel1_x - vel2_x) = pos2_x - pos1_x
/// <=> t = (pos2_x - pos1_x) / (vel1_x - vel2_x)
fn check_hailstone_collision_2d(hs1: &Hailstone, hs2: &Hailstone, test_area: (i64, i64)) -> bool {
    // Check for parallel movement (would be division by zero in the code below)
    if hs1.vel.0 == hs2.vel.0 || hs1.vel.1 == hs2.vel.1 {
        return false;
    }

    dbg!(&hs1);
    dbg!(&hs2);

    let t1 = (hs2.pos.0 - hs1.pos.0) as f64 / (hs1.vel.0 - hs2.vel.0) as f64;
    let t2 = (hs2.pos.1 - hs1.pos.1) as f64 / (hs1.vel.1 - hs2.vel.1) as f64;
    dbg!(&t1);
    dbg!(&t2);

    if t1 == t2 {
        let pos_col = (
            hs1.pos.0 as f64 + t1 * hs1.vel.0 as f64,
            hs1.pos.1 as f64 + t2 * hs1.vel.1 as f64,
        );
        pos_col.0 >= test_area.0 as f64
            && pos_col.0 <= test_area.1 as f64
            && pos_col.1 >= test_area.0 as f64
            && pos_col.1 <= test_area.1 as f64
    } else {
        false
    }
}

fn count_hailstone_collisions_2d(hs: &[Hailstone], test_area: (i64, i64)) -> usize {
    let mut collisions = 0;
    for i in 1..hs.len() {
        for j in 0..i {
            if check_hailstone_collision_2d(&hs[i], &hs[j], test_area) {
                collisions += 1;
            }
            return collisions;
        }
    }
    collisions
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day24_input.txt")?;

    println!(
        "Number of colliding hailstones disregarding Z dimension (first star): {}",
        count_hailstone_collisions_2d(&input, (200000000000000, 400000000000000))
    );

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<Hailstone>> {
    let re = Regex::new(r"^(\d+), (\d+), (\d+) @ +([-\d]+), +([-\d]+), +([-\d]+)$").unwrap();

    let input = read_to_string(input_path).expect("Could not open file!");
    let res = input
        .lines()
        .filter_map(|l| {
            if let Some(cap) = re.captures(l) {
                let hs = Hailstone {
                    pos: (
                        cap.get(1).unwrap().as_str().parse().unwrap(),
                        cap.get(2).unwrap().as_str().parse().unwrap(),
                        cap.get(3).unwrap().as_str().parse().unwrap(),
                    ),
                    vel: (
                        cap.get(4).unwrap().as_str().parse().unwrap(),
                        cap.get(5).unwrap().as_str().parse().unwrap(),
                        cap.get(6).unwrap().as_str().parse().unwrap(),
                    ),
                };

                Some(hs)
            } else {
                println!("Could not parse input line: {}", l);
                None
            }
        })
        .collect();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_first_star() {
        let input = read_input_file("../inputs/day24_example.txt").unwrap();
        assert_eq!(count_hailstone_collisions_2d(&input, (7, 27)), 2);
    }
}
