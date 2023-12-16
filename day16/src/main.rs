use std::{collections::HashSet, fmt::Display, str::FromStr};

use anyhow::{anyhow, bail, Error, Result};
use shared::dprintln;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let mirrors: Mirrors = INPUT.parse()?;

    let part1 = mirrors.count_energized((0, usize::MAX), Dir::Right);

    println!("part1: {part1}");

    let mut max = part1;

    for (starting_position, starting_dir) in mirrors
        .get_column(0)
        .skip(1)
        .map(|(i, _)| ((i, usize::MAX), Dir::Right))
        .chain(
            mirrors
                .get_column(mirrors.row_len - 1)
                .map(|(i, _)| ((i, mirrors.row_len), Dir::Left)),
        )
        .chain(
            mirrors
                .get_row(0)
                .map(|(j, _)| ((usize::MAX, j), Dir::Down)),
        )
        .chain(
            mirrors
                .get_row(mirrors.col_height - 1)
                .map(|(j, _)| ((mirrors.col_height, j), Dir::Up)),
        )
    {
        let energized = mirrors.count_energized(starting_position, starting_dir);

        if energized > max {
            max = energized;
        }
    }

    println!("part2: {max}");

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Mirror {
    /// `.`
    None,
    ///  `/`
    ForwardDiagonal,
    ///  \
    BackwardsDiagonal,
    ///  `-`
    SplitterHorizontal,
    ///  `|`
    SplitterVertical,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug)]
pub struct Mirrors {
    mirrors: Vec<Mirror>,
    row_len: usize,
    col_height: usize,
}

impl Mirrors {
    fn get_column(&self, j: usize) -> impl DoubleEndedIterator<Item = (usize, Mirror)> + '_ {
        (0..self.col_height).map(move |i| (i, self.mirrors[i * self.row_len + j]))
    }

    fn get_row(&self, i: usize) -> impl DoubleEndedIterator<Item = (usize, Mirror)> + '_ {
        (0..self.col_height).map(move |j| (j, self.mirrors[i * self.row_len + j]))
    }

    fn count_energized(&self, starting_position: (usize, usize), starting_dir: Dir) -> usize {
        let mut energized = HashSet::new();

        let mut stack = vec![(starting_position, starting_dir)];

        let mut visited = HashSet::<((usize, usize), Dir)>::new();

        while let Some((pos, dir)) = stack.pop() {
            dprintln!("{pos:?} {dir:?}");
            let (new_energized, next_bounces) = self.next_bounce(pos, dir);

            for new in new_energized {
                energized.insert(new);
            }

            next_bounces.for_each(|b| {
                if !visited.contains(&b) {
                    visited.insert(b);
                    stack.push(b)
                }
            });
        }

        energized.len()
    }

    fn next_bounce(
        &self,
        (i, j): (usize, usize),
        direction: Dir,
    ) -> (
        Vec<(usize, usize)>,
        impl Iterator<Item = ((usize, usize), Dir)>,
    ) {
        let mut energized = vec![];
        match direction {
            Dir::Right => {
                for (j, mirror) in self.get_row(i).skip(j + 1) {
                    energized.push((i, j));

                    match mirror {
                        Mirror::None => {}
                        Mirror::ForwardDiagonal => {
                            return (energized, niter::one(((i, j), Dir::Up)))
                        }
                        Mirror::BackwardsDiagonal => {
                            return (energized, niter::one(((i, j), Dir::Down)));
                        }
                        Mirror::SplitterHorizontal => {}
                        Mirror::SplitterVertical => {
                            return (
                                energized,
                                niter::two(((i, j), Dir::Up), ((i, j), Dir::Down)),
                            )
                        }
                    }
                }
            }
            Dir::Left => {
                for (j, mirror) in self.get_row(i).rev().skip(self.row_len - j) {
                    energized.push((i, j));

                    match mirror {
                        Mirror::None => {}
                        Mirror::ForwardDiagonal => {
                            return (energized, niter::one(((i, j), Dir::Down)))
                        }
                        Mirror::BackwardsDiagonal => {
                            return (energized, niter::one(((i, j), Dir::Up)));
                        }
                        Mirror::SplitterHorizontal => {}
                        Mirror::SplitterVertical => {
                            return (
                                energized,
                                niter::two(((i, j), Dir::Up), ((i, j), Dir::Down)),
                            )
                        }
                    }
                }
            }
            Dir::Down => {
                for (i, mirror) in self.get_column(j).skip(i + 1) {
                    energized.push((i, j));

                    match mirror {
                        Mirror::None => {}
                        Mirror::ForwardDiagonal => {
                            return (energized, niter::one(((i, j), Dir::Left)))
                        }
                        Mirror::BackwardsDiagonal => {
                            return (energized, niter::one(((i, j), Dir::Right)));
                        }
                        Mirror::SplitterVertical => {}
                        Mirror::SplitterHorizontal => {
                            return (
                                energized,
                                niter::two(((i, j), Dir::Left), ((i, j), Dir::Right)),
                            )
                        }
                    }
                }
            }
            Dir::Up => {
                for (i, mirror) in self.get_column(j).rev().skip(self.col_height - i) {
                    energized.push((i, j));

                    match mirror {
                        Mirror::None => {}
                        Mirror::ForwardDiagonal => {
                            return (energized, niter::one(((i, j), Dir::Right)))
                        }
                        Mirror::BackwardsDiagonal => {
                            return (energized, niter::one(((i, j), Dir::Left)));
                        }
                        Mirror::SplitterVertical => {}
                        Mirror::SplitterHorizontal => {
                            return (
                                energized,
                                niter::two(((i, j), Dir::Left), ((i, j), Dir::Right)),
                            )
                        }
                    }
                }
            }
        };

        (energized, niter::zero())
    }
}

mod niter {
    use std::{array::IntoIter, iter::Flatten};

    type Iter<T> = Flatten<IntoIter<Option<T>, 2>>;

    pub fn zero<T>() -> Iter<T> {
        [None, None].into_iter().flatten()
    }

    pub fn one<T>(elem: T) -> Iter<T> {
        [Some(elem), None].into_iter().flatten()
    }

    pub fn two<T>(a: T, b: T) -> Iter<T> {
        [Some(a), Some(b)].into_iter().flatten()
    }
}

impl FromStr for Mirrors {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row_len = s
            .lines()
            .next()
            .ok_or(anyhow!("no first line"))?
            .chars()
            .count();

        let col_len = s.lines().count();

        let data = s
            .lines()
            .flat_map(|l| l.chars().map(TryFrom::try_from))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            mirrors: data,
            row_len,
            col_height: col_len,
        })
    }
}

impl Display for Mirrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, mirror) in self.mirrors.iter().enumerate() {
            if i % self.row_len == self.row_len - 1 {
                writeln!(f, "{}", mirror)?;
            } else {
                write!(f, "{}", mirror)?;
            }
        }

        Ok(())
    }
}

impl TryFrom<char> for Mirror {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Mirror::None),
            '/' => Ok(Mirror::ForwardDiagonal),
            '\\' => Ok(Mirror::BackwardsDiagonal),
            '-' => Ok(Mirror::SplitterHorizontal),
            '|' => Ok(Mirror::SplitterVertical),
            _ => bail!("not a mirror"),
        }
    }
}

impl Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mirror::None => write!(f, "."),
            Mirror::ForwardDiagonal => write!(f, "/"),
            Mirror::BackwardsDiagonal => write!(f, "\\"),
            Mirror::SplitterHorizontal => write!(f, "-"),
            Mirror::SplitterVertical => write!(f, "|"),
        }
    }
}
