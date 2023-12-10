use std::collections::{HashSet, VecDeque};

use anyhow::Result;
use shared::dprintln;

use crate::map::{From, Map};

pub fn part1(map: &Map, start: &(usize, usize)) -> Result<()> {
    // part1
    let mut visited = HashSet::new();
    visited.insert(*start);

    let mut queue = VecDeque::new();

    if let Some(v) = map.visit_from_bottom((start.0 - 1, start.1)) {
        queue.push_back((v, 2));
    }

    if let Some(v) = map.visit_from_top((start.0 + 1, start.1)) {
        queue.push_back((v, 2));
    }

    if let Some(v) = map.visit_from_right((start.0, start.1 - 1)) {
        queue.push_back((v, 2));
    }

    if let Some(v) = map.visit_from_left((start.0, start.1 + 1)) {
        queue.push_back((v, 2));
    }

    dprintln!("{queue:?}");
    dprintln!("{start:?}");

    let mut max = 0;

    while let Some(((p, f), d)) = queue.pop_front() {
        dprintln!("{p:?}|{f:?}|{d}");
        if visited.contains(&p) {
            continue;
        }

        visited.insert(p);

        let next = match f {
            From::Left => map.visit_from_left(p),
            From::Right => map.visit_from_right(p),
            From::Top => map.visit_from_top(p),
            From::Bottom => map.visit_from_bottom(p),
        };

        if let Some(v) = next {
            if d > max {
                max = d;
            }

            queue.push_back((v, d + 1))
        }
    }

    println!("part1: {max}");

    Ok(())
}
