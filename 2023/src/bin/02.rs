use std::cmp::max;

use aoc23::parse_input;

fn main() {
    // let input = parse_input("02.test");
    let input = parse_input("02.a");
    // part1(input);
    part2(input);
}

fn part1(input: String) {
    let mut sum_of_possible_game_ids = 0;

    const RED_MAX: usize = 12;
    const GREEN_MAX: usize = 13;
    const BLUE_MAX: usize = 14;

    for line in input.lines() {
        let (game_id, game_records) = line.split_once(':').unwrap();

        let game_id: usize = game_id
            .strip_prefix("Game ")
            .expect("Game keyword missing ")
            .parse()
            .unwrap();

        let mut game_red_max = 0;
        let mut game_green_max = 0;
        let mut game_blue_max = 0;

        for play in game_records.split(';') {
            let balls = play.split(',').map(|play| play.trim());

            for ball in balls {
                let (count, color) = ball.split_once(' ').unwrap();

                let count: usize = count.parse().unwrap();
                match color {
                    "red" => {
                        game_red_max = max(game_red_max, count);
                    }
                    "green" => {
                        game_green_max = max(game_green_max, count);
                    }
                    "blue" => {
                        game_blue_max = max(game_blue_max, count);
                    }
                    _ => unreachable!(),
                }
            }
        }

        if game_red_max <= RED_MAX && game_green_max <= GREEN_MAX && game_blue_max <= BLUE_MAX {
            dbg!("{game_id} is possible!");
            sum_of_possible_game_ids += game_id;
        }
    }

    println!("part 1 - {}", sum_of_possible_game_ids);
}

fn part2(input: String) {
    let mut sum_of_game_powers = 0;

    for line in input.lines() {
        let (_, game_records) = line.split_once(':').unwrap();

        let mut game_red_required = 0;
        let mut game_green_required = 0;
        let mut game_blue_required = 0;

        for play in game_records.split(';') {
            let balls = play.split(',').map(|play| play.trim());

            for ball in balls {
                let (count, color) = ball.split_once(' ').unwrap();

                let count: usize = count.parse().unwrap();
                match color {
                    "red" => {
                        game_red_required = max(game_red_required, count);
                    }
                    "green" => {
                        game_green_required = max(game_green_required, count);
                    }
                    "blue" => {
                        game_blue_required = max(game_blue_required, count);
                    }
                    _ => unreachable!(),
                }
            }
        }

        let game_power = game_red_required * game_green_required * game_blue_required;

        dbg!(game_power);
        sum_of_game_powers += game_power;
    }

    println!("part 2 - {}", sum_of_game_powers);
}

macro_rules! test {
    ($l:literal) => {
        println!("testing - {}", $l)
    };
}

#[test]
fn testing() {
    test!("hellow!")
}
