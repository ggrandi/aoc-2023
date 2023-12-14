use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::{anyhow, bail, Error, Result};
use shared::dprintln;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let mut inp: Grid = INPUT.parse()?;

    dprintln!("{inp}");

    inp.move_north();

    dprintln!("{inp}");

    let load = inp.north_load();

    println!("part1: {load}");

    let mut prev_occurences = HashMap::<_, Vec<_>>::new();

    let mut loads = vec![];

    for i in 0usize..200 {
        inp.cycle();

        let load = inp.north_load();

        loads.push(load);

        let prev_times = prev_occurences
            .entry(load)
            .and_modify(|prev_times| prev_times.push(i))
            .or_insert(vec![i]);

        println!("{i: >3}: {} {:?}", load, prev_times)
    }

    let mut stdio = std::io::stdin()
        .lines()
        .map(|l| l?.parse::<usize>().map_err(|e| anyhow!(e)));

    println!("enter the first line it cycled at > ");

    let first = stdio.next().unwrap()?;

    println!("enter the next line it cycled at > ");
    let next = stdio.next().unwrap()?;

    let cycle_length = next - first;

    for (i, load) in (first..).zip(loads[first..].iter()) {
        let index = ((i - first) % cycle_length) + first;

        assert_eq!(loads[index], *load, "you picked the numbers wrong");
    }

    let index = ((1000000000 - 1 - first) % cycle_length) + first;

    println!("part2: {}", loads[index]);

    Ok(())
}
#[derive(Debug, Clone, Copy)]
enum Rock {
    Cube,
    Sphere,
    None,
}

#[derive(Debug)]
struct Grid {
    rocks: Vec<Rock>,
    row_len: usize,
    col_height: usize,
}

impl Grid {
    fn get_column(&self, j: usize) -> impl DoubleEndedIterator<Item = (usize, Rock)> + '_ {
        (0..self.col_height).map(move |i| (i, self.rocks[i * self.row_len + j]))
    }

    fn get_row(&self, i: usize) -> impl DoubleEndedIterator<Item = (usize, Rock)> + '_ {
        (0..self.col_height).map(move |j| (j, self.rocks[i * self.row_len + j]))
    }

    fn cycle(&mut self) {
        self.move_north();
        self.move_west();
        self.move_south();
        self.move_east();
    }

    fn move_north(&mut self) {
        for col in 0..self.row_len {
            let mut prev_free_spot = 0;
            for (i, rock) in self.get_column(col).collect::<Vec<_>>() {
                match rock {
                    Rock::Cube => prev_free_spot = i + 1,
                    Rock::Sphere => {
                        if i != prev_free_spot {
                            self.rocks[prev_free_spot * self.row_len + col] = Rock::Sphere;
                            self.rocks[i * self.row_len + col] = Rock::None;
                        }

                        prev_free_spot += 1;
                    }
                    _ => (),
                }
            }
        }
    }

    fn move_east(&mut self) {
        for row in 0..self.col_height {
            let mut prev_free_spot = self.row_len - 1;
            for (j, rock) in self.get_row(row).rev().collect::<Vec<_>>() {
                match rock {
                    Rock::Cube => prev_free_spot = j - 1,
                    Rock::Sphere => {
                        if j != prev_free_spot {
                            self.rocks[row * self.row_len + prev_free_spot] = Rock::Sphere;
                            self.rocks[row * self.row_len + j] = Rock::None;
                        }

                        prev_free_spot -= 1;
                    }
                    _ => (),
                }
            }
        }
    }

    fn move_south(&mut self) {
        for col in 0..self.row_len {
            let mut prev_free_spot = self.col_height - 1;
            for (i, rock) in self.get_column(col).rev().collect::<Vec<_>>() {
                match rock {
                    Rock::Cube => prev_free_spot = i - 1,
                    Rock::Sphere => {
                        if i != prev_free_spot {
                            self.rocks[prev_free_spot * self.row_len + col] = Rock::Sphere;
                            self.rocks[i * self.row_len + col] = Rock::None;
                        }

                        prev_free_spot -= 1;
                    }
                    _ => (),
                }
            }
        }
    }

    fn move_west(&mut self) {
        for row in 0..self.col_height {
            let mut prev_free_spot = 0;
            for (j, rock) in self.get_row(row).collect::<Vec<_>>() {
                match rock {
                    Rock::Cube => prev_free_spot = j + 1,
                    Rock::Sphere => {
                        if j != prev_free_spot {
                            self.rocks[row * self.row_len + prev_free_spot] = Rock::Sphere;
                            self.rocks[row * self.row_len + j] = Rock::None;
                        }

                        prev_free_spot += 1;
                    }
                    _ => (),
                }
            }
        }
    }

    fn north_load(&self) -> usize {
        (0..self.row_len)
            .flat_map(|col| {
                self.get_column(col).filter_map(|(i, rock)| {
                    if let Rock::Sphere = rock {
                        Some(self.col_height - i)
                    } else {
                        None
                    }
                })
            })
            .sum()
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row_len = s
            .lines()
            .next()
            .ok_or(anyhow!("no first line"))?
            .chars()
            .count();

        let col_len = s.lines().count();

        let rocks = s
            .lines()
            .flat_map(|l| {
                l.chars().map(|c| match c {
                    '#' => Ok(Rock::Cube),
                    'O' => Ok(Rock::Sphere),
                    '.' => Ok(Rock::None),
                    _ => bail!("not a rock"),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            rocks,
            row_len,
            col_height: col_len,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, rock) in self.rocks.iter().enumerate() {
            if i % self.row_len == self.row_len - 1 {
                writeln!(f, "{} ", rock)?;
            } else {
                write!(f, "{} ", rock)?;
            }
        }

        Ok(())
    }
}

impl From<&Rock> for char {
    fn from(value: &Rock) -> Self {
        match value {
            Rock::Cube => '#',
            Rock::Sphere => 'O',
            Rock::None => '.',
        }
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl FromIterator<Rock> for String {
    fn from_iter<T: IntoIterator<Item = Rock>>(iter: T) -> Self {
        iter.into_iter()
            .map(|r| char::from(&r))
            .fold(String::new(), |mut s, c| {
                s.push(c);
                s.push(' ');

                s
            })
    }
}
