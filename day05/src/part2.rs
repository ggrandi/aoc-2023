use std::{collections::VecDeque, mem::swap, ops::Range, str::FromStr};

use anyhow::{anyhow, Error, Ok, Result};
use shared::dprintln;

pub fn part2(input: &str) -> Result<()> {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds = seeds
        .split_at(7)
        .1
        .split_whitespace()
        .map(FromStr::from_str)
        .map(|n| n.map_err(Into::into))
        .collect::<Result<Vec<i64>>>()?
        .chunks(2)
        .map(|a| a[0]..a[0] + a[1])
        .collect::<VecDeque<_>>();

    let maps = maps
        .split("\n\n")
        .map(|m| {
            m.lines()
                .skip(1)
                .map(FromStr::from_str)
                .collect::<Result<Vec<MapEntry>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    let mut front = seeds;
    let mut back = VecDeque::with_capacity(front.len());

    for map in maps.iter() {
        // if let Some(map) = maps.first() {
        dprintln!("{front:?}|{back:?}");
        dprintln!("{map:?}");

        'outer: while let Some(mut next) = front.pop_front() {
            dprintln!("{next:?}");
            for entry in map {
                dprintln!("{entry:?}");
                match next.clone().intersect(&entry.range) {
                    Intersection::None(_) => {
                        dprintln!("None");
                    }
                    Intersection::Once(not_intersected, intersected) => {
                        next = not_intersected;
                        back.push_back(shift(intersected, entry.delta));

                        dprintln!("Once: {next:?}|{front:?}|{back:?}");
                    }
                    Intersection::Twice(n_start, intersected, n_end) => {
                        next = n_start;
                        front.push_back(n_end);
                        back.push_back(shift(intersected, entry.delta));

                        dprintln!("Twice: {next:?}|{front:?}|{back:?}");
                    }
                    Intersection::Fully(intersected) => {
                        dprintln!("Fully");
                        back.push_back(shift(intersected, entry.delta));
                        continue 'outer;
                    }
                }
            }

            if !next.is_empty() {
                back.push_back(next);
            }
        }

        dprintln!("{front:?}|{back:?}\n");

        swap(&mut front, &mut back);
    }

    println!(
        "part2: {}",
        front
            .into_iter()
            .map(|r| r.start)
            .min()
            .ok_or(anyhow!("couldn't find min"))?
    );

    Ok(())
}

fn shift(r: Range<i64>, delta: i64) -> Range<i64> {
    r.start + delta..r.end + delta
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Intersection<T>
where
    T: Sized + PartialEq,
{
    None(T),
    Fully(T),
    /// not interected, interected
    Once(T, T),
    Twice(T, T, T),
}

trait Intersect
where
    Self: Sized + PartialEq,
{
    fn intersect(self, other: &Self) -> Intersection<Self>;
}

impl<T: PartialOrd + Copy> Intersect for Range<T> {
    fn intersect(self, other: &Self) -> Intersection<Self> {
        if other.fully_contains(&self) {
            Intersection::Fully(self)
        } else if self.start < other.start && other.start < self.end && self.contains(&other.end) {
            Intersection::Twice(
                self.start..other.start,
                (*other).clone(),
                other.end..self.end,
            )
        } else if self.start < other.start && other.start < self.end {
            Intersection::Once(self.start..other.start, other.start..self.end)
        } else if self.start < other.end && other.end <= self.end {
            Intersection::Once(other.end..self.end, self.start..other.end)
        } else {
            Intersection::None(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::part2::FullyContains;

    use super::{Intersect, Intersection};

    #[test]
    fn intersect() {
        assert_eq!(Intersection::None(0..10), (0..10).intersect(&(20..100)));
        assert_eq!(Intersection::Once(0..4, 4..10), (0..10).intersect(&(4..10)));
        assert_eq!(
            Intersection::Once(4..10, 0..4),
            (0..10).intersect(&(-10..4))
        );
        assert_eq!(
            Intersection::Twice(0..4, 4..10, 10..100),
            (0..100).intersect(&(4..10))
        );

        assert!((0..35).fully_contains(&(0..7)));

        assert_eq!(Intersection::Fully(10..20), (10..20).intersect(&(0..100)));
        assert_eq!(Intersection::Fully(14..15), (14..15).intersect(&(0..15)));
        assert_eq!(Intersection::Fully(0..15), (0..15).intersect(&(0..100)));
        assert_eq!(Intersection::Once(7..35, 0..7), (0..35).intersect(&(0..7)));
        assert_eq!(Intersection::None(7..35), (7..35).intersect(&(0..7)));
    }
}

trait FullyContains {
    fn fully_contains(&self, other: &Self) -> bool;
}

impl<T: PartialOrd> FullyContains for Range<T> {
    fn fully_contains(&self, other: &Self) -> bool {
        self.contains(&other.start) && self.start < other.end && other.end <= self.end
    }
}

#[derive(Debug, Clone)]
struct MapEntry {
    range: Range<i64>,
    delta: i64,
}

impl FromStr for MapEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace().map(FromStr::from_str);

        let dest = s.next().ok_or(anyhow!("missing value"))??;
        let src = s.next().ok_or(anyhow!("missing value"))??;
        let len = s.next().ok_or(anyhow!("missing value"))??;

        Ok(MapEntry {
            range: src..src + len,
            delta: dest - src,
        })
    }
}

impl MapEntry {
    pub fn transform(&self, v: i64) -> Option<i64> {
        todo!();
    }
}
