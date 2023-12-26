use itertools::Itertools;
use range_set_blaze::RangeSetBlaze;
use winnow::{
    ascii::{digit1, multispace0, multispace1, space0, space1},
    combinator::{preceded, separated},
    prelude::*,
    token::take_till,
};

use crate::AocDay;

pub struct Day5;
impl AocDay for Day5 {
    fn input(&self) -> &'static str {
        include_str!("data/day5.txt")
    }

    fn part1(&self, mut input: &str) -> usize {
        let (seeds, mappers) = input_p(&mut input).unwrap();
        seeds
            .into_iter()
            .map(|seed| {
                mappers
                    .iter()
                    .fold(seed, |seed, mapper| mapper.map_id(seed))
            })
            .min()
            .unwrap()
    }

    fn part2(&self, mut input: &str) -> usize {
        let (seeds, mappers) = input_p(&mut input).unwrap();
        let seed_ranges = seeds
            .into_iter()
            .batching(|iter| {
                let start = iter.next()?;
                let end = start + iter.next()? - 1;
                Some(start..=end)
            })
            .collect::<RangeSetBlaze<usize>>();
        let location_ranges = mappers
            .iter()
            .fold(seed_ranges, |ranges, mapper| mapper.map_rangeset(ranges));
        location_ranges.first().unwrap()
    }
}

#[test]
fn part1() {
    assert_eq!(Day5.part1(TEST_INPUT), 35)
}
#[test]
fn part2() {
    assert_eq!(Day5.part2(TEST_INPUT), 46)
}

// ------------- Structs --------------

#[derive(Debug)]
struct IdMapper<'i> {
    _name: &'i str,
    ranges: Vec<MapRange>,
}
impl<'i> IdMapper<'i> {
    fn map_id(&self, id: usize) -> usize {
        self.ranges
            .iter()
            .find_map(|range| range.map_id(id))
            .unwrap_or(id)
    }
    fn map_rangeset(&self, mut original: RangeSetBlaze<usize>) -> RangeSetBlaze<usize> {
        let mut new = RangeSetBlaze::new();
        for range in &self.ranges {
            new |= range.extract_mapped_range_from(&mut original);
        }
        new |= original;
        new
    }
}
#[derive(Debug)]
struct MapRange {
    dst: usize,
    src: usize,
    len: usize,
}
impl MapRange {
    fn map_id(&self, id: usize) -> Option<usize> {
        (self.src..=(self.src + self.len - 1))
            .contains(&id)
            .then(|| id - self.src + self.dst)
    }
    fn extract_mapped_range_from(&self, range: &mut RangeSetBlaze<usize>) -> RangeSetBlaze<usize> {
        let src = RangeSetBlaze::from_iter([self.src..=(self.src + self.len - 1)]);
        let extracted = &*range & &src;
        *range = &*range - src;
        extracted
            .into_ranges()
            .map(|range| {
                (range.start() - self.src + self.dst)..=(range.end() - self.src + self.dst)
            })
            .collect()
    }
}

// --------------- Parsing -----------------

fn input_p<'i>(input: &mut &'i str) -> PResult<(Vec<usize>, Vec<IdMapper<'i>>)> {
    (
        seed_list_p,
        multispace0,
        separated(0.., idmapper_p, multispace1),
    )
        .map(|(seeds, _, mappers)| (seeds, mappers))
        .parse_next(input)
}

fn idmapper_p<'i>(input: &mut &'i str) -> PResult<IdMapper<'i>> {
    (
        take_till(0.., ' '),
        (space1, "map:", multispace0),
        separated(1.., maprange_p, multispace1),
    )
        .map(|(_name, _, ranges)| IdMapper { ranges, _name })
        .parse_next(input)
}

fn maprange_p(input: &mut &str) -> PResult<MapRange> {
    separated(3, digit1.parse_to::<usize>(), space1)
        .map(|ns: Vec<_>| MapRange {
            dst: ns[0],
            src: ns[1],
            len: ns[2],
        })
        .parse_next(input)
}

fn seed_list_p(input: &mut &str) -> PResult<Vec<usize>> {
    preceded(
        ("seeds:", space0),
        separated(1.., digit1.parse_to::<usize>(), space1),
    )
    .parse_next(input)
}

#[cfg(test)]
const TEST_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
