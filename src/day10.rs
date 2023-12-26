use crate::AocDay;

use itertools::Itertools;
use num_enum::TryFromPrimitive;
use winnow::{
    ascii::{multispace1, newline},
    combinator::{repeat, separated},
    prelude::*,
    token::any,
};

pub struct Day10;
impl AocDay for Day10 {
    fn input(&self) -> &'static str {
        include_str!("data/day10.txt")
    }

    fn part1(&self, mut input: &str) -> usize {
        let grid = grid_p(&mut input).unwrap();
        let mut coord = grid
            .iter_with_coords()
            .find_map(|(coord, tile)| (tile == Tile::Start).then_some(coord))
            .unwrap();
        let mut dir = [Dir::N, Dir::S, Dir::E, Dir::W]
            .into_iter()
            .find_map(|start_dir| {
                let mut coord = coord;
                let mut dir = start_dir;
                coord.walk(dir).ok()?;
                dir.rotate(grid.get(coord)?).ok()?;
                Some(start_dir)
            })
            .unwrap();
        let mut num = 0;
        loop {
            coord.walk(dir).unwrap();
            num += 1;
            let tile = grid.get(coord).unwrap();
            if tile == Tile::Start {
                break;
            }
            dir.rotate(tile).unwrap();
        }
        num / 2
    }

    fn part2(&self, mut input: &str) -> usize {
        0
    }
}

#[test]
fn part1() {
    assert_eq!(Day10.part1(TEST_INPUT), 8)
}

#[test]
fn part2() {
    assert_eq!(Day10.part2(TEST_INPUT), 0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord(usize, usize);
impl Coord {
    fn walk(&mut self, dir: Dir) -> Result<(), ()> {
        *self = match dir {
            Dir::N => Coord(self.0, self.1.checked_sub(1).ok_or(())?),
            Dir::S => Coord(self.0, self.1 + 1),
            Dir::E => Coord(self.0 + 1, self.1),
            Dir::W => Coord(self.0.checked_sub(1).ok_or(())?, self.1),
        };
        Ok(())
    }
}

struct Grid {
    grid: Vec<Vec<Tile>>,
}
impl Grid {
    fn get(&self, coord: Coord) -> Option<Tile> {
        self.grid
            .get(coord.1)
            .and_then(|line| line.get(coord.0).copied())
    }
    fn get_mut(&mut self, coord: Coord) -> Option<&mut Tile> {
        self.grid
            .get_mut(coord.1)
            .and_then(|line| line.get_mut(coord.0))
    }
    fn iter_with_coords<'g>(&'g self) -> impl Iterator<Item = (Coord, Tile)> + 'g {
        self.grid.iter().enumerate().flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, tile)| (Coord(x, y), *tile))
        })
    }
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Tile> {
        self.grid.iter_mut().flat_map(|line| line.iter_mut())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
enum Tile {
    NS = 0b1100,
    EW = 0b0011,
    NE = 0b1010,
    NW = 0b1001,
    SW = 0b0101,
    SE = 0b0110,
    Ground = 0b0000,
    Start = 0b1111,
    Swap,
    Ignore,
    Countable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
enum Dir {
    N = 0b1000,
    S = 0b0100,
    E = 0b0010,
    W = 0b0001,
}
impl Dir {
    fn reverse(&mut self) {
        let odds = (*self as u8) & 0b1010;
        let evens = (*self as u8) & 0b0101;
        *self = (odds >> 1 | evens << 1).try_into().unwrap()
    }
    fn rotate(&mut self, tile: Tile) -> Result<(), ()> {
        self.reverse();
        *self = ((tile as u8) & !(*self as u8)).try_into().map_err(|_| ())?;
        Ok(())
    }
}

fn grid_p(input: &mut &str) -> PResult<Grid> {
    separated(1.., repeat::<_, _, Vec<_>, _, _>(1.., tile_p), multispace1)
        .map(|grid| Grid { grid })
        .parse_next(input)
}

fn tile_p(input: &mut &str) -> PResult<Tile> {
    any.verify_map(|c: char| match c {
        '|' => Some(Tile::NS),
        '-' => Some(Tile::EW),
        'L' => Some(Tile::NE),
        'J' => Some(Tile::NW),
        '7' => Some(Tile::SW),
        'F' => Some(Tile::SE),
        '.' => Some(Tile::Ground),
        'S' => Some(Tile::Start),
        _ => None,
    })
    .parse_next(input)
}

#[cfg(test)]
const TEST_INPUT: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
