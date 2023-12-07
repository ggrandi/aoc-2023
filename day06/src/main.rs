use core::f64;

use anyhow::{Ok, Result};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let mut iter = INPUT.lines().map(|l| {
        l.split_at(10)
            .1
            .split_whitespace()
            .filter_map(|n| n.parse::<f64>().ok())
    });

    let part1 = iter
        .next()
        .unwrap()
        .zip(iter.next().unwrap())
        .map(end_points)
        .map(num_viable)
        .product::<f64>();

    println!("part1: {part1}");

    let mut iter = INPUT.lines().filter_map(|l| {
        l.split_at(10)
            .1
            .split_whitespace()
            .collect::<String>()
            .parse::<f64>()
            .ok()
    });

    let part2 = num_viable(end_points((iter.next().unwrap(), iter.next().unwrap())));

    println!("part2: {part2}");

    Ok(())
}

/// time: $t$
/// distance: $d$
///
/// $$
/// n = \frac{t}{2} \pm \sqrt{\frac{t}{2} - {d}}
/// $$
fn end_points((time, distance): (f64, f64)) -> (f64, f64) {
    let m = time / 2.;
    let delta = f64::sqrt(m * m - distance);

    (m - delta, m + delta)
}

fn num_viable(roots: (f64, f64)) -> f64 {
    f64::ceil(roots.1 - 1.) - f64::floor(roots.0 + 1.) + 1.
}
