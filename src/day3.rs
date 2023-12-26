use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use itertools::iproduct;
use winnow::{
    ascii::digit1,
    combinator::{repeat, repeat_till0},
    prelude::*,
    token::{any, none_of},
};

use crate::{
    linespan::{linespan, with_linespan, LineLocated, LineSpan},
    AocDay,
};

pub struct Day3;
impl AocDay for Day3 {
    fn input(&self) -> &'static str {
        include_str!("data/day3.txt")
    }

    fn part1(&self, input: &str) -> usize {
        let (lines, cols) = input.lines().map(str::len).enumerate().last().unwrap();
        let symbols = symbols_p
            .parse_next(&mut LineLocated::new(input))
            .unwrap()
            .into_iter()
            .map(|sym| (sym.span.start, sym.line))
            .collect::<HashSet<_>>();
        numlocs_p(&mut LineLocated::new(input))
            .unwrap()
            .into_iter()
            .filter(|nl| bounding_box(&nl.linespan, lines, cols).any(|xy| symbols.contains(&xy)))
            .map(|nl| nl.number)
            .sum()
    }

    fn part2(&self, input: &str) -> usize {
        let (lines, cols) = input.lines().map(str::len).enumerate().last().unwrap();
        let mut inters = HashMap::<(usize, usize), Vec<usize>>::new();
        numlocs_p(&mut LineLocated::new(input))
            .unwrap()
            .into_iter()
            .flat_map(|nl| bounding_box(&nl.linespan, lines, cols).map(move |xy| (xy, nl.number)))
            .for_each(|(xy, num)| inters.entry(xy).or_default().push(num));
        gears_p(&mut LineLocated::new(input))
            .unwrap()
            .into_iter()
            .map(|gear| (gear.span.start, gear.line))
            .filter_map(|xy| inters.get(&xy))
            .filter(|ns| ns.len() == 2)
            .map(|ns| ns[0] * ns[1])
            .sum()
    }
}

#[test]
fn part1() {
    assert_eq!(Day3.part1(TEST_INPUT), 4361);
}

#[test]
fn part2() {
    assert_eq!(Day3.part2(TEST_INPUT), 467835);
}

#[cfg(test)]
const TEST_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

fn bounding_box(
    LineSpan { line, span }: &LineSpan,
    num_lines: usize,
    num_cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let first_line = line.saturating_sub(1);
    let last_line = (line + 1).min(num_lines - 1);
    let first_col = span.start.saturating_sub(1);
    let last_col = span.end.min(num_cols - 1);
    iproduct!(first_col..=last_col, first_line..=last_line)
}

#[derive(Debug)]
struct NumLoc {
    number: usize,
    linespan: LineSpan,
}

fn numlocs_p(input: &mut LineLocated) -> PResult<Vec<NumLoc>> {
    repeat(
        0..,
        repeat_till0(any, numloc_p).map(|(_, b): (String, _)| b),
    )
    .parse_next(input)
}

fn numloc_p(input: &mut LineLocated) -> PResult<NumLoc> {
    with_linespan(digit1.parse_to::<usize>())
        .map(|(number, linespan)| NumLoc { number, linespan })
        .parse_next(input)
}

fn gears_p(input: &mut LineLocated) -> PResult<Vec<LineSpan>> {
    repeat(
        0..,
        repeat_till0(any, linespan('*')).map(|(_, b): (String, _)| b),
    )
    .parse_next(input)
}

fn symbols_p(input: &mut LineLocated) -> PResult<Vec<LineSpan>> {
    repeat(
        0..,
        repeat_till0(any, symbol_p).map(|(_, b): (String, LineSpan)| b),
    )
    .parse_next(input)
}

fn symbol_p(input: &mut LineLocated) -> PResult<LineSpan> {
    linespan(none_of(b"0123456789.")).parse_next(input)
}
