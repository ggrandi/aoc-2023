use std::str::FromStr;

use anyhow::{anyhow, Error, Ok, Result};
use shared::dprintln;

pub fn part1(input: &str) -> Result<()> {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let mut seeds = seeds
        .split_at(7)
        .1
        .split_whitespace()
        .map(FromStr::from_str)
        .map(|n| n.map_err(Into::into))
        .collect::<Result<Vec<i64>>>()?;

    let maps = maps
        .split("\n\n")
        .map(|m| {
            m.lines()
                .skip(1)
                .map(FromStr::from_str)
                .collect::<Result<Vec<MapEntry>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    for map in maps.iter() {
        dprintln!("{seeds:?}");
        for seed in seeds.iter_mut() {
            for entry in map {
                if let Some(new_seed) = entry.transform(*seed) {
                    *seed = new_seed;
                    break;
                }
            }
        }
        dprintln!("{seeds:?}\n");
    }

    println!(
        "part1: {}",
        seeds.into_iter().min().ok_or(anyhow!("no min"))?
    );

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct MapEntry(i64, i64, i64);

impl FromStr for MapEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace().map(FromStr::from_str);

        Ok(MapEntry(
            s.next().ok_or(anyhow!("missing value"))??,
            s.next().ok_or(anyhow!("missing value"))??,
            s.next().ok_or(anyhow!("missing value"))??,
        ))
    }
}

impl MapEntry {
    pub fn transform(&self, v: i64) -> Option<i64> {
        if self.1 <= v && v < self.1 + self.2 {
            Some(v + self.0 - self.1)
        } else {
            None
        }
    }
}
