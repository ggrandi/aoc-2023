use std::{collections::HashMap, ops::ControlFlow};

use anyhow::{bail, Result};
use shared::{dprintln, LeastCommonMultiple};
const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let mut lines = INPUT.lines();

    let instructions = lines.next().unwrap().chars().map(|l| l == 'L').cycle();

    let map = lines
        .skip(1)
        .map(|l| (&l[0..3], (&l[7..10], &l[12..15])))
        .collect::<HashMap<_, _>>();

    let res = instructions
        .clone()
        .try_fold(("AAA", 0), |(curr, count), left| {
            if curr == "ZZZ" {
                ControlFlow::Break(count)
            } else if left {
                ControlFlow::Continue((map.get(curr).unwrap().0, count + 1))
            } else {
                ControlFlow::Continue((map.get(curr).unwrap().1, count + 1))
            }
        });

    match res {
        ControlFlow::Continue(_) => bail!("couldn't find ZZZ"),
        ControlFlow::Break(c) => println!("part1: {c}"),
    }

    let cycles = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|&start| {
            let instructions = instructions.clone();

            let mut ends = instructions
                .scan(start, |curr, left| {
                    *curr = if left {
                        map.get(*curr).unwrap().0
                    } else {
                        map.get(*curr).unwrap().1
                    };

                    Some(*curr)
                })
                .enumerate()
                .filter(|(_, c)| c.ends_with('Z'))
                .take(2)
                .map(|(i, _)| i);

            let first = ends.next().unwrap();
            let last = ends.next().unwrap();

            last - first
        })
        .collect::<Vec<_>>();

    dprintln!("{cycles:?}");

    println!("part2: {}", cycles.iter().fold(1, |acc, n| acc.lcm(n)));

    Ok(())
}
