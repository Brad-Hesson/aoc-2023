mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

mod linespan;

const DAYS: &[&dyn AocDay] = &[
    &day1::Day1,
    &day2::Day2,
    &day3::Day3,
    &day4::Day4,
    &day5::Day5,
    &day6::Day6,
    &day7::Day7,
    &day8::Day8,
    &day9::Day9,
    &day10::Day10,
];

fn main() {
    let mut args = std::env::args();
    let _program = args.next().unwrap();
    let command = args.next().unwrap();
    match &command[..3] {
        "all" => {
            for n in 0..DAYS.len() {
                run_day(n + 1);
            }
        }
        "day" => {
            let day = command[3..].parse::<usize>().unwrap();
            run_day(day);
        }
        _ => {}
    }
}

fn run_day(n: usize) {
    let day = DAYS[n - 1];
    let input = day.input();
    println!("----------[Day {n}]----------");
    let result = day.part1(input);
    println!("Part 1: {result}");
    let result = day.part2(input);
    println!("Part 2: {result}");
    println!();
}

trait AocDay {
    fn input(&self) -> &'static str;
    fn part1(&self, input: &str) -> usize;
    fn part2(&self, input: &str) -> usize;
}
