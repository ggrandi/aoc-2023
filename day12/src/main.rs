use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, bail, Error, Result};
use shared::dprintln;

const INPUT: &str = include_str!("../input");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Springs {
    Y,
    N,
    M,
}

fn main() -> Result<()> {
    let cases = INPUT
        .lines()
        .map(|l| -> Result<_> {
            let (springs, nums) = l.split_once(' ').ok_or(anyhow!("couldn't split"))?;

            let chains = nums
                .split(',')
                .map(FromStr::from_str)
                .collect::<Result<Vec<u32>, _>>()?;

            let num_total: u32 = chains.iter().sum();

            let (unknowns, springs, num_known) = springs
                .chars()
                .map(|c| match c {
                    '#' => Ok(Springs::Y),
                    '.' => Ok(Springs::N),
                    '?' => Ok(Springs::M),
                    _ => bail!("the character {c} is not a valid spring"),
                })
                .try_fold(
                    (vec![], vec![], 0),
                    |(mut unknowns, mut springs, mut num_known), n| -> Result<_> {
                        let n = n.map_err::<Error, _>(Into::into)?;

                        match n {
                            Springs::M => unknowns.push(springs.len()),
                            Springs::Y => num_known += 1,
                            _ => (),
                        }

                        springs.push(n);

                        Ok((unknowns, springs, num_known))
                    },
                )?;

            Ok((unknowns, num_total, num_known, springs, chains))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut sum1 = 0;
    let mut sum2 = 0;

    for (unknowns, num_total, num_known, mut springs, chains) in cases {
        sum1 += SpringSolver::new(springs.clone(), chains.clone()).solve((0, 0, 0));

        springs.push(Springs::M);
        let mut springs = springs.repeat(5);
        springs.pop();

        dprintln!(
            "{:?}|{:?}|{}|{}",
            springs
                .iter()
                .map(|s| match s {
                    Springs::Y => '#',
                    Springs::N => '.',
                    Springs::M => '?',
                })
                .collect::<String>(),
            chains.repeat(5),
            unknowns.len() * 5 + 4,
            num_total * 5 - num_known * 5,
        );
        sum2 += SpringSolver::new(springs, chains.repeat(5)).solve((0, 0, 0));
    }

    println!("part1: {sum1}");
    println!("part1: {sum2}");

    Ok(())
}

struct SpringSolver {
    springs: Vec<Springs>,
    chains: Vec<u32>,
    map: HashMap<(usize, usize, u32), usize>,
}

impl SpringSolver {
    fn new(springs: Vec<Springs>, chains: Vec<u32>) -> Self {
        Self {
            springs,
            chains,
            map: HashMap::new(),
        }
    }

    fn solve(&mut self, key: (usize, usize, u32)) -> usize {
        if let Some(v) = self.map.get(&key) {
            return *v;
        }

        let (curr_idx, chain_idx, curr_chain) = key;

        if curr_idx == self.springs.len() {
            if chain_idx == self.chains.len() && curr_chain == 0
                || chain_idx == self.chains.len() - 1 && curr_chain == *self.chains.last().unwrap()
            {
                return 1;
            } else {
                return 0;
            }
        }

        let mut ans = 0;

        let next = self.springs[curr_idx];

        let states = match next {
            Springs::Y => vec![true].into_iter(),
            Springs::N => vec![false].into_iter(),
            Springs::M => vec![true, false].into_iter(),
        };

        for is_block in states {
            if is_block {
                ans += self.solve((curr_idx + 1, chain_idx, curr_chain + 1))
            } else if curr_chain != 0 {
                if chain_idx < self.chains.len() && curr_chain == self.chains[chain_idx] {
                    ans += self.solve((curr_idx + 1, chain_idx + 1, 0))
                }
            } else {
                ans += self.solve((curr_idx + 1, chain_idx, 0))
            }
        }

        self.map.insert(key, ans);

        ans
    }
}
