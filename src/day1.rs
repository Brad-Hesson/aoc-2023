use crate::AocDay;

pub struct Day1;

impl AocDay for Day1 {
    fn input(&self) -> &'static str {
        include_str!("data/day1.txt")
    }

    fn part1(&self, input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let digits = line
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<_>>();
                digits.first().unwrap() * 10 + digits.last().unwrap()
            })
            .sum::<u32>() as usize
    }

    fn part2(&self, input: &str) -> usize {
        let mut sum = 0;
        for full_line in input.lines() {
            let mut line = full_line;
            let first = 'outer: loop {
                if let Some(n) = line.chars().next().unwrap().to_digit(10) {
                    break n;
                }
                for (n, spelled) in NUMS.iter().enumerate() {
                    if line.starts_with(spelled) {
                        break 'outer n as u32 + 1;
                    }
                }
                line = line.split_at(1).1;
            };
            line = full_line;
            let last = 'outer: loop {
                if let Some(n) = line.chars().last().unwrap().to_digit(10) {
                    break n;
                }
                for (n, spelled) in NUMS.iter().enumerate() {
                    if line.ends_with(spelled) {
                        break 'outer n as u32 + 1;
                    }
                }
                line = line.split_at(line.len() - 1).0;
            };
            let num = first * 10 + last;
            sum += num;
        }
        sum as usize
    }
}

#[test]
fn part1() {
    assert_eq!(Day1.part1(TEST_INPUT_1), 142);
}

#[test]
fn part2() {
    assert_eq!(Day1.part2(TEST_INPUT_2), 281);
}
const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[cfg(test)]
const TEST_INPUT_1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

#[cfg(test)]
const TEST_INPUT_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
