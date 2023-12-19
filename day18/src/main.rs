use std::collections::BinaryHeap;

use anyhow::Result;
use shared::dprintln;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let part1 = handle_input(|l| {
        let mut c = l.split_whitespace();

        (
            c.next().unwrap().chars().next().unwrap(),
            c.next().unwrap().parse::<i64>().unwrap(),
        )
    })?;

    println!("part1: {part1}");

    let part2 = handle_input(|l| {
        let mut n = l.split_whitespace().rev();

        let i = n.next().unwrap();
        let i = &i[2..i.len() - 1].split_at(5);

        let d = match i.1 {
            "0" => 'R',
            "1" => 'D',
            "2" => 'L',
            "3" => 'U',
            _ => unreachable!(),
        };

        (d, i64::from_str_radix(i.0, 16).unwrap())
    })?;

    println!("part2: {part2}");

    Ok(())
}

fn handle_input(get_dir_length: impl Fn(&str) -> (char, i64)) -> Result<usize> {
    let mut horizontal_lines = BinaryHeap::new();
    let mut lines = vec![];

    let mut points = INPUT
        .lines()
        .map(get_dir_length)
        .scan((0, 0), |curr, (d, l)| {
            match d {
                'R' => curr.1 += l,
                'L' => curr.1 -= l,
                'U' => curr.0 -= l,
                'D' => curr.0 += l,
                _ => panic!(),
            }

            Some((*curr, l))
        })
        .chain([((0, 0), 0)]);

    let mut sum = 0;

    let first = points.next().unwrap();
    let mut prev = first.0;

    let mut min = (0, 0);
    let mut max = (0, 0);

    for (next, l) in points.chain([first]) {
        sum += l;
        if min.0 > next.0 {
            min.0 = next.0;
        }
        if min.1 > next.1 {
            min.1 = next.1;
        }
        if max.0 < next.0 {
            max.0 = next.0;
        }
        if max.1 < next.1 {
            max.1 = next.1;
        }

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

    // println!("{min:?} {max:?}");
    // for i in min.0..=max.0 {
    //     for j in min.1..=max.1 {
    //         if lines.iter().any(|l| l.contains_i(i) && l.contains_j(j)) {
    //             print!("#")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //
    //     println!()
    // }

    let mut sum = sum as usize;

    println!("sum: {sum}");

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

    for w in horizontal_lines.windows(2) {
        let prev_line = w[0];
        let line = w[1];
        // no horizontal lines part
        if line - prev_line != 1 {
            let rlines = lines
                .iter()
                .filter(|l| l.contains_i(prev_line + 1))
                .collect::<Vec<_>>();

            let to_check = rlines
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
                .count();

            // gets all the possible positions
            let num = (line - prev_line - 1) as usize * to_check;

            dprintln!("{prev_line}-{line}: {to_check} * {}", line - prev_line - 1);

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
            .count();

        dprintln!("{line}: {num}");

        sum += num;
    }

    Ok(sum)
}

#[derive(Debug, Clone, Copy)]
enum Line {
    Horizontal { i: i64, j1: i64, j2: i64 },
    Vertical { i1: i64, i2: i64, j: i64 },
}

impl Line {
    pub fn from_points(p1: (i64, i64), p2: (i64, i64)) -> Option<Self> {
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
            assert_eq!(p1.1, p2.1);
            Some(Line::Vertical {
                j: p1.1,
                i1: p1.0.min(p2.0),
                i2: p1.0.max(p2.0),
            })
        }
    }

    pub fn contains_i(&self, i_val: i64) -> bool {
        match self {
            Line::Horizontal { i, j1: _, j2: _ } => *i == i_val,
            Line::Vertical { i1, i2, j: _ } => (*i1..=*i2).contains(&i_val),
        }
    }

    pub fn contains_j(&self, j_val: i64) -> bool {
        match self {
            Line::Horizontal { i: _, j1, j2 } => (*j1..=*j2).contains(&j_val),
            Line::Vertical { i1: _, i2: _, j } => j_val == *j,
        }
    }
}
