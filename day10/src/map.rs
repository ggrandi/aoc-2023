use anyhow::{anyhow, bail, Result};
use shared::dprintln;

#[derive(Debug)]
pub struct Map {
    data: Vec<Vec<Pipes>>,
    start: (usize, usize),
}

impl Map {
    fn find_start(data: &[Vec<Pipes>]) -> Result<(usize, usize)> {
        data.iter()
            .enumerate()
            .find_map(|(i, r)| {
                r.iter().enumerate().find_map(|(j, &p)| {
                    if p == Pipes::Start {
                        Some((i, j))
                    } else {
                        None
                    }
                })
            })
            .ok_or(anyhow!("couldn't find the start"))
    }

    pub fn get_start(&self) -> &(usize, usize) {
        &self.start
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&Pipes> {
        self.data.get(i).and_then(|l| l.get(j))
    }

    pub fn visit_from_bottom(&self, (i, j): (usize, usize)) -> Option<((usize, usize), From)> {
        if let Some(p) = self.get(i, j) {
            match p {
                Pipes::Vertical => Some(((i - 1, j), From::Bottom)),
                Pipes::BottomLeft => Some(((i, j - 1), From::Right)),
                Pipes::BottomRight => Some(((i, j + 1), From::Left)),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn visit_from_top(&self, (i, j): (usize, usize)) -> Option<((usize, usize), From)> {
        if let Some(p) = self.get(i, j) {
            match p {
                Pipes::Vertical => Some(((i + 1, j), From::Top)),
                Pipes::TopLeft => Some(((i, j - 1), From::Right)),
                Pipes::TopRight => Some(((i, j + 1), From::Left)),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn visit_from_left(&self, (i, j): (usize, usize)) -> Option<((usize, usize), From)> {
        if let Some(p) = self.get(i, j) {
            match p {
                Pipes::Horizontal => Some(((i, j + 1), From::Left)),
                Pipes::TopLeft => Some(((i - 1, j), From::Bottom)),
                Pipes::BottomLeft => Some(((i + 1, j), From::Top)),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn visit_from_right(&self, (i, j): (usize, usize)) -> Option<((usize, usize), From)> {
        if let Some(p) = self.get(i, j) {
            match p {
                Pipes::Horizontal => Some(((i, j - 1), From::Right)),
                Pipes::TopRight => Some(((i - 1, j), From::Bottom)),
                Pipes::BottomRight => Some(((i + 1, j), From::Top)),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn fence_points(&self) -> MapToPoints<'_> {
        let next_direction = if self
            .visit_from_bottom((self.start.0 - 1, self.start.1))
            .is_some()
        {
            From::Top
        } else if self
            .visit_from_top((self.start.0 + 1, self.start.1))
            .is_some()
        {
            From::Bottom
        } else {
            From::Left
        };

        MapToPoints {
            map: self,
            curr: self.start,
            next_direction: Some(next_direction),
        }
    }
}

impl FromIterator<Vec<Pipes>> for Map {
    fn from_iter<T: IntoIterator<Item = Vec<Pipes>>>(iter: T) -> Self {
        let data = iter.into_iter().collect::<Vec<_>>();
        let start = Map::find_start(&data).unwrap();
        Map { data, start }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum From {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pipes {
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    None,
    Start,
}

impl TryFrom<char> for Pipes {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipes::Vertical),
            '-' => Ok(Pipes::Horizontal),
            'L' => Ok(Pipes::TopRight),
            'J' => Ok(Pipes::TopLeft),
            '7' => Ok(Pipes::BottomLeft),
            'F' => Ok(Pipes::BottomRight),
            '.' => Ok(Pipes::None),
            'S' => Ok(Pipes::Start),
            _ => bail!("couldn't find pipe"),
        }
    }
}

pub struct MapToPoints<'a> {
    map: &'a Map,
    curr: (usize, usize),
    next_direction: Option<From>,
}

impl<'a> Iterator for MapToPoints<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let prev_direction = self.next_direction?;

        let next = match prev_direction {
            From::Left => (
                self.curr.0,
                self.curr.1
                    - self.map.data[self.curr.0][..self.curr.1]
                        .iter()
                        .rev()
                        .take_while(|&&p| p == Pipes::Horizontal)
                        .count()
                    - 1,
            ),
            From::Right => (
                self.curr.0,
                self.curr.1
                    + self.map.data[self.curr.0][self.curr.1 + 1..]
                        .iter()
                        .take_while(|&&p| p == Pipes::Horizontal)
                        .count()
                    + 1,
            ),
            From::Top => (
                self.curr.0
                    - self.map.data[..self.curr.0]
                        .iter()
                        .rev()
                        .map(|l| l[self.curr.1])
                        .take_while(|&p| p == Pipes::Vertical)
                        .count()
                    - 1,
                self.curr.1,
            ),
            From::Bottom => {
                let num = self.map.data[self.curr.0 + 1..]
                    .iter()
                    .map(|l| l[self.curr.1])
                    .take_while(|&p| p == Pipes::Vertical)
                    .count();

                dprintln!("{num}");
                (self.curr.0 + num + 1, self.curr.1)
            }
        };

        self.next_direction = match self.map.get(next.0, next.1).unwrap() {
            Pipes::TopLeft => match prev_direction {
                From::Bottom => Some(From::Left),
                From::Right => Some(From::Top),
                _ => unreachable!(),
            },
            Pipes::TopRight => match prev_direction {
                From::Bottom => Some(From::Right),
                From::Left => Some(From::Top),
                _ => unreachable!(),
            },
            Pipes::BottomLeft => match prev_direction {
                From::Top => Some(From::Left),
                From::Right => Some(From::Bottom),
                _ => unreachable!(),
            },
            Pipes::BottomRight => match prev_direction {
                From::Top => Some(From::Right),
                From::Left => Some(From::Bottom),
                _ => unreachable!(),
            },
            Pipes::Start => None,
            Pipes::None | Pipes::Vertical | Pipes::Horizontal => unreachable!(),
        };

        self.curr = next;

        Some(self.curr)
    }
}
