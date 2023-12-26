use std::{cmp::Ordering, collections::HashMap};

use crate::AocDay;

use itertools::Itertools;
use winnow::{ascii::space0, prelude::*, token::take};

pub struct Day8;
impl AocDay for Day8 {
    fn input(&self) -> &'static str {
        include_str!("data/day8.txt")
    }

    fn part1(&self, input: &str) -> usize {
        let (lrs, nodemap) = input_p(input);
        let mut node = "AAA";
        for (i, lr) in lrs.chars().into_iter().cycle().enumerate() {
            if node == "ZZZ" {
                return i;
            }
            node = nodemap.next(node, lr);
        }
        0
    }

    fn part2(&self, input: &str) -> usize {
        let (lrs, nodemap) = input_p(input);
        let nodes = nodemap
            .hashmap
            .keys()
            .copied()
            .filter(|k| k.ends_with('A'))
            .collect_vec();
        let mut cycles = Vec::new();
        for mut node in nodes {
            let mut seen = HashMap::<(&str, usize), usize>::new();
            for (i, (lri, lr)) in lrs.chars().into_iter().enumerate().cycle().enumerate() {
                if let Some(n) = seen.get(&(node, lri)) {
                    cycles.push(i - n);
                    break;
                }
                seen.insert((node, lri), i);
                node = nodemap.next(node, lr);
            }
        }
        cycles.into_iter().fold(1, lcm)
    }
}

#[test]
fn part1() {
    assert_eq!(Day8.part1(TEST_INPUT_1), 6)
}

#[test]
fn part2() {
    assert_eq!(Day8.part2(TEST_INPUT_2), 6)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    match a.cmp(&b) {
        Ordering::Less => gcd(a, b - a),
        Ordering::Equal => return a,
        Ordering::Greater => gcd(a - b, b),
    }
}

#[derive(Debug)]
struct NodeMap<'i> {
    hashmap: HashMap<&'i str, (&'i str, &'i str)>,
}
impl<'i> NodeMap<'i> {
    fn next(&self, current: &str, lr: char) -> &'i str {
        let (left, right) = self.hashmap.get(current).unwrap();
        match lr {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        }
    }
}
impl<'i> FromIterator<(&'i str, (&'i str, &'i str))> for NodeMap<'i> {
    fn from_iter<T: IntoIterator<Item = (&'i str, (&'i str, &'i str))>>(iter: T) -> Self {
        Self {
            hashmap: iter.into_iter().collect(),
        }
    }
}

fn input_p<'i>(input: &'i str) -> (&str, NodeMap<'i>) {
    let mut lines = input.lines();
    let lrs = lines.next().unwrap();
    let nodemap = lines
        .filter(|line| !line.is_empty())
        .map(|mut line| node_p(&mut line).unwrap())
        .collect();
    (lrs, nodemap)
}

fn node_p<'i>(input: &mut &'i str) -> PResult<(&'i str, (&'i str, &'i str))> {
    let src = element_p.parse_next(input)?;
    let _ = (space0, '=', space0, '(', space0).parse_next(input)?;
    let dst_l = element_p.parse_next(input)?;
    let _ = (space0, ',', space0).parse_next(input)?;
    let dst_r = element_p.parse_next(input)?;
    let _ = (space0, ')').parse_next(input)?;
    Ok((src, (dst_l, dst_r)))
}

fn element_p<'i>(input: &mut &'i str) -> PResult<&'i str> {
    take(3usize).parse_next(input)
}

#[cfg(test)]
const TEST_INPUT_1: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

#[cfg(test)]
const TEST_INPUT_2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
