use anyhow::{Ok, Result};
use shared::dprintln;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    part1()?;

    part2()
}

fn part2() -> Result<()> {
    let n = INPUT
        .lines()
        .map(|l| {
            let (winning, nums) = l.split_once(": ").unwrap().1.split_once("| ").unwrap();

            Ok((str_to_vec_nums(winning)?, str_to_vec_nums(nums)?))
        })
        .map(|n| n.unwrap())
        .map(|(winning, nums)| nums.into_iter().filter(|n| winning.contains(n)).count())
        .enumerate()
        .fold(
            INPUT.lines().map(|_| 1).collect::<Vec<usize>>(),
            |mut cards, (i, n)| {
                let num = cards[i];

                dprintln!("{cards:?}[{i}] = ({num}, {n})");

                cards[i + 1..=i + n].iter_mut().for_each(|n| *n += num);
                cards
            },
        )
        .into_iter()
        .sum::<usize>();

    println!("part2: {n:?}");

    Ok(())
}

fn part1() -> Result<()> {
    let n = INPUT
        .lines()
        .map(|l| {
            let (winning, nums) = l.split_once(": ").unwrap().1.split_once("| ").unwrap();

            Ok((str_to_vec_nums(winning)?, str_to_vec_nums(nums)?))
        })
        .map(|n| n.unwrap())
        // .collect::<Result<Vec<(_, _)>>>()?;
        .map(|(winning, nums)| nums.into_iter().filter(|n| winning.contains(n)).count())
        .filter(|n| *n != 0)
        .map(|n| 1 << (n - 1))
        .sum::<usize>();

    println!("part1: {n}");

    Ok(())
}

fn str_to_vec_nums(s: &str) -> Result<Vec<usize>> {
    s.split_whitespace()
        .map(|n| n.parse().map_err(Into::into))
        .collect()
}
