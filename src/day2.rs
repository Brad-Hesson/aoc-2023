use std::str::FromStr;

use winnow::{
    ascii::{alpha1, digit1, space0, space1},
    combinator::{delimited, separated, separated_pair},
    error::ContextError,
    prelude::*,
    stream::Accumulate,
};

use crate::AocDay;

pub struct Day2;

impl AocDay for Day2 {
    fn input(&self) -> &'static str {
        include_str!("data/day2.txt")
    }

    fn part1(&self, input: &str) -> usize {
        let mut sum = 0;
        for line in input.lines() {
            let game = line.parse::<Game>().unwrap();
            if game.pulls.iter().all(|pull| {
                pull.red <= MAX_CUBES.red
                    && pull.green <= MAX_CUBES.green
                    && pull.blue <= MAX_CUBES.blue
            }) {
                sum += game.id;
            }
        }
        sum
    }

    fn part2(&self, input: &str) -> usize {
        let mut sum = 0;
        for line in input.lines() {
            let game = line.parse::<Game>().unwrap();
            let mut min_set = CubeSet {
                red: 0,
                green: 0,
                blue: 0,
            };
            for pull in game.pulls {
                min_set.red = min_set.red.max(pull.red);
                min_set.green = min_set.green.max(pull.green);
                min_set.blue = min_set.blue.max(pull.blue);
            }
            sum += min_set.red * min_set.green * min_set.blue;
        }
        sum
    }
}

#[test]
fn part1() {
    assert_eq!(Day2.part1(TEST_INPUT), 8);
}
#[test]
fn part2() {
    assert_eq!(Day2.part2(TEST_INPUT), 2286);
}
const MAX_CUBES: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};

#[cfg(test)]
const TEST_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

#[derive(Debug)]
struct Game {
    id: usize,
    pulls: Vec<CubeSet>,
}

#[derive(Debug)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Game {
    type Err = winnow::error::ErrMode<ContextError>;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        game_p(&mut s)
    }
}

fn game_p(input: &mut &str) -> PResult<Game> {
    let pulls_p = separated(1.., delimited(space0, cube_set_p, space0), ';');
    let game_id_p = delimited("Game ", digit1.parse_to(), (':', space0));
    let (id, pulls) = (game_id_p, pulls_p).parse_next(input)?;
    Ok(Game { id, pulls })
}

fn cube_set_p(input: &mut &str) -> PResult<CubeSet> {
    let num_and_color_p = separated_pair(digit1.parse_to(), space1, alpha1);
    separated(1..=3, delimited(space0, num_and_color_p, space0), ',').parse_next(input)
}

impl Accumulate<(usize, &str)> for CubeSet {
    fn initial(_capacity: Option<usize>) -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn accumulate(&mut self, (num, color): (usize, &str)) {
        match color {
            "red" => self.red = num,
            "green" => self.green = num,
            "blue" => self.blue = num,
            _ => unreachable!(),
        }
    }
}
