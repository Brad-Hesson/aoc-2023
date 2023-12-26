use crate::AocDay;

use itertools::izip;
use winnow::{
    ascii::{digit1, multispace0, space0, space1},
    combinator::{preceded, separated},
    prelude::*,
    token::take_till,
};

pub struct Day6;
impl AocDay for Day6 {
    fn input(&self) -> &'static str {
        include_str!("data/day6.txt")
    }

    fn part1(&self, mut input: &str) -> usize {
        let races = races_p(&mut input).unwrap();
        races.into_iter().map(|r| r.num_wins()).product()
    }

    fn part2(&self, mut input: &str) -> usize {
        let races = races_p(&mut input).unwrap();
        let mut time_s = String::new();
        let mut dist_s = String::new();
        for Race { time, dist } in races {
            time_s += &time.to_string();
            dist_s += &dist.to_string();
        }
        let race = Race {
            time: time_s.parse().unwrap(),
            dist: dist_s.parse().unwrap(),
        };
        race.num_wins()
    }
}

#[test]
fn part1() {
    assert_eq!(Day6.part1(TEST_INPUT), 288)
}

#[test]
fn part2() {
    assert_eq!(Day6.part2(TEST_INPUT), 71503)
}

#[derive(Debug)]
struct Race {
    time: usize,
    dist: usize,
}
impl Race {
    fn num_wins(&self) -> usize {
        let t = self.time as f64;
        let d = self.dist as f64;
        let desc = (t.powf(2.) - 4. * d).sqrt();
        let highest = ((t + desc) / 2.).ceil() as usize - 1;
        let lowest = ((t - desc) / 2.).floor() as usize + 1;
        highest - lowest + 1
    }
}

fn races_p(input: &mut &str) -> PResult<Vec<Race>> {
    let times = line_p(input)?;
    multispace0(input)?;
    let dists = line_p(input)?;
    Ok(izip!(times, dists)
        .into_iter()
        .map(|(time, dist)| Race { time, dist })
        .collect())
}

fn line_p(input: &mut &str) -> PResult<Vec<usize>> {
    preceded(
        (take_till(1.., ':'), ':', space0),
        separated(1.., digit1.parse_to::<usize>(), space1),
    )
    .parse_next(input)
}

#[cfg(test)]
const TEST_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;
