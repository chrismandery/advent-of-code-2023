use anyhow::{anyhow, bail, Context, Result};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Default)]
struct GameDraw {
    drawn_red: usize,
    drawn_green: usize,
    drawn_blue: usize,
}

type GameResult = Vec<GameDraw>;

fn calc_possible_game_sum(
    games: &[(usize, GameResult)],
    num_red: usize,
    num_green: usize,
    num_blue: usize,
) -> usize {
    games
        .iter()
        .filter_map(|(game_num, gr)| {
            if check_game_possible(gr, num_red, num_green, num_blue) {
                Some(game_num)
            } else {
                None
            }
        })
        .sum()
}

fn check_game_possible(gr: &GameResult, num_red: usize, num_green: usize, num_blue: usize) -> bool {
    gr.iter().all(|draw| {
        (draw.drawn_red <= num_red)
            & (draw.drawn_green <= num_green)
            & (draw.drawn_blue <= num_blue)
    })
}

fn main() -> Result<()> {
    let games = parse_input_file("../inputs/day2_input.txt")?;
    println!(
        "Sum of the IDs of all possible games: {}",
        calc_possible_game_sum(&games, 12, 13, 14)
    );

    Ok(())
}

fn parse_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<(usize, GameResult)>> {
    let input = read_to_string(input_path)?;
    let res: Vec<_> = input.lines().map(parse_input_line).collect();
    res.into_iter().collect()
}

fn parse_input_line(line: &str) -> Result<(usize, GameResult)> {
    let (game_num_prefix, draws_str) = line
        .split_once(':')
        .ok_or_else(|| anyhow!("Could not find : char!"))?;

    if !game_num_prefix.starts_with("Game ") {
        bail!("Line did not start with \"Game\"");
    }
    let game_num = game_num_prefix[5..]
        .parse()
        .context("Could not parse game number!")?;

    let mut gr = vec![];
    for draw_str in draws_str.split(';') {
        let mut draw = GameDraw::default();

        for num_color_comb in draw_str.split(',') {
            let mut it = num_color_comb.trim().split(' ');
            let num_str = it.next().context("Could not extract number from draw!")?;
            let num: usize = num_str.parse().context("Could not parse draw number!")?;
            let color_str = it.next().context("Could not extract color from draw!")?;

            match color_str {
                "red" => {
                    draw.drawn_red = num;
                }
                "green" => {
                    draw.drawn_green = num;
                }
                "blue" => {
                    draw.drawn_blue = num;
                }
                _ => {
                    bail!("Unknown color!");
                }
            }
        }

        gr.push(draw);
    }

    Ok((game_num, gr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_star() {
        let games = parse_input_file("../inputs/day2_example.txt").unwrap();
        assert_eq!(calc_possible_game_sum(&games, 12, 13, 14), 8);
    }
}
