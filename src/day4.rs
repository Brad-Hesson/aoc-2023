use winnow::{
    ascii::{digit1, space0, space1},
    combinator::separated,
    PResult, Parser,
};

use crate::AocDay;

pub struct Day4;
impl AocDay for Day4 {
    fn input(&self) -> &'static str {
        include_str!("data/day4.txt")
    }

    fn part1(&self, input: &str) -> usize {
        input
            .lines()
            .map(|line| 2usize.pow(card_wins(line) as u32) / 2)
            .sum()
    }

    fn part2(&self, input: &str) -> usize {
        let mut ns = input.lines().map(card_wins).rev().collect::<Vec<_>>();
        for i in 0..ns.len() {
            let wins = ns[i];
            ns[i] = 1 + (i - wins..i).map(|j| ns[j]).sum::<usize>();
        }
        ns.iter().sum()
    }
}

#[test]
fn part1() {
    assert_eq!(Day4.part1(TEST_INPUT), 13);
}
#[test]
fn part2() {
    assert_eq!(Day4.part2(TEST_INPUT), 30);
}

fn card_wins(mut line: &str) -> usize {
    let (c0, c1) = line_p(&mut line).unwrap();
    c1.into_iter().filter(|c| c0.contains(c)).count()
}

fn line_p(input: &mut &str) -> PResult<(Vec<usize>, Vec<usize>)> {
    (
        ("Card", space1, digit1, ':', space0),
        num_vec_p,
        (space0, '|', space0),
        num_vec_p,
    )
        .map(|(_, c0, _, c1)| (c0, c1))
        .parse_next(input)
}

fn num_vec_p(input: &mut &str) -> PResult<Vec<usize>> {
    separated(1.., digit1.parse_to::<usize>(), space1).parse_next(input)
}

#[cfg(test)]
const TEST_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
