use std::ops::{Add, Sub};

use crate::AocDay;

use itertools::Itertools;
use winnow::{
    ascii::{dec_int, space1},
    combinator::separated,
    prelude::*,
};

pub struct Day9;
impl AocDay for Day9 {
    fn input(&self) -> &'static str {
        include_str!("data/day9.txt")
    }

    fn part1(&self, input: &str) -> usize {
        input_p(input)
            .into_iter()
            .map(|seq| Extrapol(seq.as_slice()).next())
            .sum::<i64>() as usize
    }

    fn part2(&self, input: &str) -> usize {
        input_p(input)
            .into_iter()
            .map(|seq| seq.into_iter().rev().collect_vec())
            .map(|seq| Extrapol(seq.as_slice()).next())
            .sum::<i64>() as usize
    }
}

#[test]
fn part1() {
    assert_eq!(Day9.part1(TEST_INPUT), 114)
}

#[test]
fn part2() {
    assert_eq!(Day9.part2(TEST_INPUT), 2)
}

#[repr(transparent)]
struct Extrapol<'a, T>(pub &'a [T]);
impl<'a, T: Copy + PartialEq + Add<Output = T> + Sub<Output = T>> Extrapol<'a, T> {
    pub fn next(&self) -> T {
        let mut vecs = vec![self.0.to_vec()];
        while !vecs.last().unwrap().iter().all_equal() {
            let new = vecs
                .last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(a, b)| *b - *a)
                .collect_vec();
            vecs.push(new)
        }
        for i in (1..vecs.len()).rev() {
            let a = *vecs[i].last().unwrap();
            let b = *vecs[i - 1].last().unwrap();
            vecs[i - 1].push(b + a);
        }
        *vecs.first().unwrap().last().unwrap()
    }
}

fn input_p(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .into_iter()
        .map(|mut line| line_p(&mut line).unwrap())
        .collect()
}

fn line_p(input: &mut &str) -> PResult<Vec<i64>> {
    separated(1.., dec_int::<_, i64, _>, space1).parse_next(input)
}

#[cfg(test)]
const TEST_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
