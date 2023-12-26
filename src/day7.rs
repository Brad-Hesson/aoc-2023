use crate::AocDay;

use itertools::Itertools;
use winnow::{
    ascii::{digit1, space1},
    combinator::repeat,
    prelude::*,
    token::any,
};

pub struct Day7;
impl AocDay for Day7 {
    fn input(&self) -> &'static str {
        include_str!("data/day7.txt")
    }

    fn part1(&self, input: &str) -> usize {
        input_p::<'S'>(input)
            .into_iter()
            .sorted()
            .enumerate()
            .map(|(rank, hand)| (rank + 1) * hand.bid)
            .sum()
    }

    fn part2(&self, input: &str) -> usize {
        input_p::<'J'>(input)
            .into_iter()
            .sorted()
            .enumerate()
            .map(|(rank, hand)| (rank + 1) * hand.bid)
            .sum()
    }
}

#[test]
fn part1() {
    assert_eq!(Day7.part1(TEST_INPUT), 6440)
}

#[test]
fn part2() {
    assert_eq!(Day7.part2(TEST_INPUT), 5905)
}

#[derive(Debug, Eq, Ord)]
struct Hand<const D: DeckType> {
    cards: [Card<D>; 5],
    bid: usize,
}
impl<const D: DeckType> Hand<D> {
    fn hand_type(&self) -> HandType {
        let mut groups = self
            .cards
            .iter()
            .sorted()
            .group_by(|c| *c)
            .into_iter()
            .map(|group| group.1.count())
            .collect_vec();
        let num_jokers = self
            .cards
            .contains(&Card::Joker)
            .then(|| groups.remove(0))
            .unwrap_or_default();
        groups.sort();
        if groups.is_empty() {
            groups.push(0);
        }
        *groups.last_mut().unwrap() += num_jokers;
        match groups.as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::Pair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}
impl<const D: DeckType> PartialEq for Hand<D> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl<const D: DeckType> PartialOrd for Hand<D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type().partial_cmp(&other.hand_type()) {
            Some(core::cmp::Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            ord => ord,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type DeckType = char;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card<const D: DeckType> {
    Joker,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N10,
    J,
    Q,
    K,
    A,
}
impl<const D: DeckType> TryFrom<char> for Card<D> {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::N2),
            '3' => Ok(Card::N3),
            '4' => Ok(Card::N4),
            '5' => Ok(Card::N5),
            '6' => Ok(Card::N6),
            '7' => Ok(Card::N7),
            '8' => Ok(Card::N8),
            '9' => Ok(Card::N9),
            'T' => Ok(Card::N10),
            'J' => Ok(match D {
                'S' => Card::J,
                'J' => Card::Joker,
                _ => unimplemented!(),
            }),
            'Q' => Ok(Card::Q),
            'K' => Ok(Card::K),
            'A' => Ok(Card::A),
            _ => Err(()),
        }
    }
}

fn input_p<const D: DeckType>(input: &str) -> Vec<Hand<D>> {
    input
        .lines()
        .map(|mut line| hand_p(&mut line).unwrap())
        .collect()
}

fn hand_p<const D: DeckType>(input: &mut &str) -> PResult<Hand<D>> {
    let cards: [Card<D>; 5] = repeat::<_, _, Vec<_>, _, _>(5, card_p)
        .parse_next(input)?
        .try_into()
        .unwrap();
    let _ = space1.parse_next(input)?;
    let bid = digit1.parse_to::<usize>().parse_next(input)?;
    Ok(Hand { cards, bid })
}

fn card_p<const D: DeckType>(input: &mut &str) -> PResult<Card<D>> {
    any.verify_map(|c: char| c.try_into().ok())
        .parse_next(input)
}

#[cfg(test)]
const TEST_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
