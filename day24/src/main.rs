use anyhow::{anyhow, ensure, Result};
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;
use z3::ast::{Ast, Int};

#[derive(Debug)]
struct Hailstone {
    pos: [i64; 3],
    vel: [i64; 3],
}

/// Checks for a 2D hailstone collision (ignoring the Z component).
fn check_hailstone_collision_2d(hs1: &Hailstone, hs2: &Hailstone, test_area: (i64, i64)) -> bool {
    let dx = hs2.pos[0] - hs1.pos[0];
    let dy = hs2.pos[1] - hs1.pos[1];
    let cp = (hs2.vel[0] * hs1.vel[1] - hs2.vel[1] * hs1.vel[0]) as f64;

    if cp <= 0.0000001 {
        return false;
    }

    let u = (dy * hs2.vel[0] - dx * hs2.vel[1]) as f64 / cp;
    let v = (dy * hs1.vel[0] - dx * hs1.vel[1]) as f64 / cp;

    if u < 0.0 || v < 0.0 {
        return false;
    }

    let pos_col = (
        hs1.pos[0] as f64 + u * hs1.vel[0] as f64,
        hs1.pos[1] as f64 + u * hs1.vel[1] as f64,
    );

    pos_col.0 >= test_area.0 as f64
        && pos_col.0 <= test_area.1 as f64
        && pos_col.1 >= test_area.0 as f64
        && pos_col.1 <= test_area.1 as f64
}

fn count_hailstone_collisions_2d(hs: &[Hailstone], test_area: (i64, i64)) -> usize {
    let mut collisions = 0;
    for i in 0..hs.len() {
        for j in 0..hs.len() {
            if i != j && check_hailstone_collision_2d(&hs[i], &hs[j], test_area) {
                collisions += 1;
            }
        }
    }
    collisions
}

/// Find position and valocity of a hailstone that is intercepting all given hailstones in their paths.
/// We are solving this equation system using Z3 (3 equations for each hailstone i):
/// pos_interc + vel_interc * t_i = pos_i + vel_i * t_i  (with constraint t_i > 0)
fn find_intercepting_hailstone(all_hs: &[Hailstone]) -> Result<Hailstone> {
    // Setup Z3
    let context = z3::Context::new(&z3::Config::new());
    let solver = z3::Solver::new(&context);

    // Create constant zero and variables for intercepting hailstone
    let zero = Int::from_i64(&context, 0);
    let interc_pos_x = Int::new_const(&context, "interc_pos_x");
    let interc_pos_y = Int::new_const(&context, "interc_pos_y");
    let interc_pos_z = Int::new_const(&context, "interc_pos_z");
    let interc_vel_x = Int::new_const(&context, "interc_vel_x");
    let interc_vel_y = Int::new_const(&context, "interc_vel_y");
    let interc_vel_z = Int::new_const(&context, "interc_vel_z");

    // Add variables and constraints for all hailstones
    for (i, hs) in all_hs.iter().enumerate() {
        let hs_pos_x = Int::from_i64(&context, hs.pos[0]);
        let hs_pos_y = Int::from_i64(&context, hs.pos[1]);
        let hs_pos_z = Int::from_i64(&context, hs.pos[2]);
        let hs_vel_x = Int::from_i64(&context, hs.vel[0]);
        let hs_vel_y = Int::from_i64(&context, hs.vel[1]);
        let hs_vel_z = Int::from_i64(&context, hs.vel[2]);
        let t = Int::new_const(&context, format!("t_{}", i));

        solver.assert(&t.ge(&zero));
        solver.assert(&(&interc_pos_x + &interc_vel_x * &t)._eq(&(hs_pos_x + hs_vel_x * &t)));
        solver.assert(&(&interc_pos_y + &interc_vel_y * &t)._eq(&(hs_pos_y + hs_vel_y * &t)));
        solver.assert(&(&interc_pos_z + &interc_vel_z * &t)._eq(&(hs_pos_z + hs_vel_z * &t)));
    }

    // Check problem is solvable
    ensure!(
        solver.check() == z3::SatResult::Sat,
        "Assertions of problem are not satisfiable! (should not happen)"
    );

    // Get position and velocity of hailstone
    let model = solver
        .get_model()
        .ok_or(anyhow!("Could not get model from Z3!"))?;
    let interc_hs = Hailstone {
        pos: [
            model.eval(&interc_pos_x, true).unwrap().as_i64().unwrap(),
            model.eval(&interc_pos_y, true).unwrap().as_i64().unwrap(),
            model.eval(&interc_pos_z, true).unwrap().as_i64().unwrap(),
        ],
        vel: [
            model.eval(&interc_vel_x, true).unwrap().as_i64().unwrap(),
            model.eval(&interc_vel_y, true).unwrap().as_i64().unwrap(),
            model.eval(&interc_vel_z, true).unwrap().as_i64().unwrap(),
        ],
    };

    Ok(interc_hs)
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day24_input.txt")?;

    println!(
        "Number of colliding hailstones disregarding Z dimension (first star): {}",
        count_hailstone_collisions_2d(&input, (200000000000000, 400000000000000))
    );

    println!(
        "Sum of position coordinates for intercepting hailstone (second star): {}",
        find_intercepting_hailstone(&input)?.pos.iter().sum::<i64>()
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
                    pos: [
                        cap.get(1).unwrap().as_str().parse().unwrap(),
                        cap.get(2).unwrap().as_str().parse().unwrap(),
                        cap.get(3).unwrap().as_str().parse().unwrap(),
                    ],
                    vel: [
                        cap.get(4).unwrap().as_str().parse().unwrap(),
                        cap.get(5).unwrap().as_str().parse().unwrap(),
                        cap.get(6).unwrap().as_str().parse().unwrap(),
                    ],
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

    #[test]
    fn test_example_second_star() {
        let input = read_input_file("../inputs/day24_example.txt").unwrap();
        assert_eq!(
            find_intercepting_hailstone(&input)
                .unwrap()
                .pos
                .iter()
                .sum::<i64>(),
            47
        );
    }
}
