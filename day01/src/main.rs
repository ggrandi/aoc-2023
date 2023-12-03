use anyhow::{anyhow, Ok, Result};

use regex::Regex;
use shared::char_to_usize;

const INPUT: &str = include_str!("../input");
// const INPUT: &str = "two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
// ";

fn main() -> Result<()> {
    part1()?;

    part2()
}

fn part1() -> Result<()> {
    println!("part1");

    println!(
        "{}",
        INPUT
            .split_terminator('\n')
            .map(|line| {
                // println!("{line}");

                let first_match: usize =
                    char_to_usize(line.chars().find(char::is_ascii_digit).unwrap());

                let last: usize =
                    char_to_usize(line.chars().rev().find(char::is_ascii_digit).unwrap());

                first_match * 10 + last
            })
            .sum::<usize>()
    );

    Ok(())
}

fn part2() -> Result<()> {
    let first_num = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|zero|[0-9]).*?$")?;
    let last_num = Regex::new(r"^.*(one|two|three|four|five|six|seven|eight|nine|zero|[0-9])")?;

    println!("part2");

    println!(
        "{}",
        INPUT
            .split_terminator('\n')
            .map(|line| {
                let first = str_to_usize(
                    first_num
                        .captures(line)
                        .ok_or_else(|| anyhow!("no match for first num"))?
                        .get(1)
                        .unwrap()
                        .as_str(),
                );

                let last = str_to_usize(
                    last_num
                        .captures(line)
                        .ok_or_else(|| anyhow!("no match for first num"))?
                        .get(1)
                        .unwrap()
                        .as_str(),
                );

                // println!("{line}: {first} {last}");

                Ok(first * 10 + last)
            })
            .sum::<Result<usize>>()?
    );

    Ok(())
}

fn str_to_usize(s: &str) -> usize {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "zero" => 0,
        s => s.parse().unwrap(),
    }
}
