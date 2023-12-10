use std::collections::BinaryHeap;

use anyhow::{Ok, Result};
use shared::dprintln;

use crate::map::Pipes;

pub fn part2(map: &crate::map::Map) -> Result<()> {
    let mut horizontal_lines = BinaryHeap::new();
    let mut lines = vec![];

    dprintln!("{:?}", map.fence_points().collect::<Vec<_>>());

    let mut points = map.fence_points();

    let first = points.next().unwrap();
    let mut prev = first;

    for next in points.chain([first]) {
        dprintln!("p|{prev:?}|{next:?}");
        if prev.0 == next.0 {
            // if not already in the heap
            if horizontal_lines.iter().all(|&l| l != next.0) {
                horizontal_lines.push(next.0)
            }
        }

        if let Some(line) = Line::from_points(prev, next) {
            lines.push(line);
        };

        prev = next;
    }

    lines.sort_unstable_by_key(|l| match l {
        Line::Horizontal { i: _, j1, j2: _ } => *j1,
        Line::Vertical { i1: _, i2: _, j } => *j,
    });

    #[cfg(debug_assertions)]
    for line in lines.iter() {
        println!("{line:?}")
    }

    let horizontal_lines = horizontal_lines.into_sorted_vec();
    dprintln!("{horizontal_lines:?}");

    let mut sum = 0;

    for w in horizontal_lines.windows(2) {
        let prev_line = w[0];
        let line = w[1];
        // no horizontal lines part
        if line - prev_line != 1 {
            let rlines = lines
                .iter()
                .filter(|l| l.contains_i(prev_line + 1))
                .collect::<Vec<_>>();

            let to_check: Vec<_> = rlines
                .into_iter()
                .scan((false, 0), |state, l| match l {
                    Line::Horizontal { i: _, j1: _, j2: _ } => unreachable!(),
                    Line::Vertical { i1, i2: _, j } => {
                        let ret = if state.0 {
                            Some(state.1..*j)
                        } else {
                            Some(0..0)
                        };

                        if line != *i1 {
                            state.0 = !state.0;
                        }
                        state.1 = *j + 1;

                        ret
                    }
                })
                .flatten()
                .collect();

            // gets all the possible positions
            let cross = (prev_line + 1..line).flat_map(|i| to_check.iter().map(move |&j| (i, j)));

            let num = cross.filter_map(|(i, j)| map.get(i, j)).count();

            dprintln!("{prev_line}-{line}: {num}");

            sum += num;
        }

        // where the line is
        let rlines = lines
            .iter()
            .filter(|l| l.contains_i(line))
            .collect::<Vec<_>>();

        let num = rlines
            .into_iter()
            .scan((false, 0), |state, l| match l {
                Line::Horizontal { i: _, j1, j2 } => {
                    let og = state.1;
                    state.1 = *j2 + 1;

                    Some(og..*j1)
                }
                Line::Vertical { i1, i2: _, j } => {
                    let ret = if state.0 {
                        Some(state.1..*j)
                    } else {
                        Some(0..0)
                    };

                    if line != *i1 {
                        state.0 = !state.0;
                    }
                    state.1 = *j + 1;

                    ret
                }
            })
            .flatten()
            .filter_map(|j| map.get(line, j))
            .count();

        dprintln!("{line}: {num}");

        sum += num;
    }

    println!("part2: {sum}");

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Line {
    Horizontal { i: usize, j1: usize, j2: usize },
    Vertical { i1: usize, i2: usize, j: usize },
}

impl Line {
    pub fn from_points(p1: (usize, usize), p2: (usize, usize)) -> Option<Self> {
        if p1.0 == p2.0 {
            let j2 = p1.1.max(p2.1);
            let j1 = p1.1.min(p2.1);
            if j2 - j1 <= 1 {
                None
            } else {
                Some(Line::Horizontal {
                    i: p1.0,
                    j1: j1 + 1,
                    j2: j2 - 1,
                })
            }
        } else {
            Some(Line::Vertical {
                j: p1.1,
                i1: p1.0.min(p2.0),
                i2: p1.0.max(p2.0),
            })
        }
    }

    pub fn contains_i(&self, i_val: usize) -> bool {
        match self {
            Line::Horizontal { i, j1: _, j2: _ } => *i == i_val,
            Line::Vertical { i1, i2, j: _ } => (*i1..=*i2).contains(&i_val),
        }
    }
}
