use std::{num::ParseIntError, str::FromStr};

use anyhow::{anyhow, Result};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let (part2, part1) = INPUT
        .lines()
        .map(|l| {
            let mut nums = l
                .split_whitespace()
                .map(FromStr::from_str)
                .collect::<Result<Vec<i64>, ParseIntError>>()
                .map_err(|e| anyhow!(e))?;

            let next = gen_next_number(&nums, 0);
            nums.reverse();
            let prev = gen_next_number(&nums, 0);

            Ok((prev, next))
        })
        .try_fold::<_, _, Result<_>>((0, 0), |(a, b), n| match n {
            Ok((c, d)) => Ok((a + c, b + d)),
            e @ Err(_) => e,
        })?;

    println!("part1: {part1}\npart2: {part2}");

    Ok(())
}

fn gen_next_number(seq: &[i64], acc: i64) -> i64 {
    let last = seq.last().unwrap();

    let deltas = seq.windows(2).map(|a| a[1] - a[0]).collect::<Vec<_>>();

    if deltas.iter().all(|d| *d == 0) {
        return acc + last;
    }

    gen_next_number(&deltas, acc + last)
}
