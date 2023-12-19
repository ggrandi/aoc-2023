use std::{
    collections::{BinaryHeap, HashMap},
    ops::RangeBounds,
    str::FromStr,
};

const INPUT: &str = include_str!("../sample");

fn main() {
    let inp = INPUT
        .lines()
        .map(str::chars)
        .map(|c| c.filter_map(|c| c.to_digit(10)).collect())
        .collect::<Vec<Vec<_>>>();

    let target_i = inp.len() - 1;
    let target_j = inp[0].len() - 1;

    let mut visited = vec![vec![(u32::MAX, None); inp[0].len()]; inp.len()];

    let mut nodes = BinaryHeap::with_capacity(10_000);

    nodes.push(Node::new(0, 0, 0, 0, Dir::Right, None));
    nodes.push(Node::new(0, 0, 0, 0, Dir::Down, None));

    while let Some(node) = nodes.pop() {
        // if !(0..=target_i).contains(&node.i)
        //     || !(0..=target_j).contains(&node.j)
        //     || visited[node.i][node.j] <= node.heat_loss
        // {
        //     continue;
        // }

        let next_heat_loss = node.heat_loss + inp[node.i][node.j];

        visited[node.i][node.j] = (
            next_heat_loss,
            node.prev.map(|prev| (prev.0, prev.1, node.direction)),
        );

        if node.i == target_i && node.j == target_j {
            break;
        }

        let prev = Some((node.i, node.j));

        for (i, j, length, dir) in node.get_next() {
            if (0..=target_i).contains(&i)
                && (0..=target_j).contains(&j)
                && visited[i][j].0 >= next_heat_loss
            {
                nodes.push(Node::new(i, j, next_heat_loss, length, dir, prev));
            }
        }
    }

    let heat_loss = visited[target_i][target_j].0;

    for v in visited {
        for (d, p) in v {
            match p {
                Some((i, j, dir)) => print!("{d:3} {i:2},{j:2},{:5}|", format!("{dir:?}")),
                None => print!("{d:3} {:11}|", ' '),
            }
        }
        println!()
    }

    println!("part1 {heat_loss}");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    heat_loss: u32,
    i: usize,
    j: usize,
    length: usize,
    direction: Dir,
    prev: Option<(usize, usize)>,
}

impl Node {
    pub fn new(
        i: usize,
        j: usize,
        heat_loss: u32,
        length: usize,
        direction: Dir,
        prev: Option<(usize, usize)>,
    ) -> Self {
        Self {
            heat_loss,
            i,
            j,
            length,
            direction,
            prev,
        }
    }

    pub fn get_next(mut self) -> impl Iterator<Item = (usize, usize, usize, Dir)> {
        match self.direction {
            Dir::Left | Dir::Right => [
                Some((self.i - 1, self.j, 0, Dir::Up)),
                Some((self.i + 1, self.j, 0, Dir::Down)),
            ],
            Dir::Up | Dir::Down => [
                Some((self.i, self.j - 1, 0, Dir::Left)),
                Some((self.i, self.j + 1, 0, Dir::Right)),
            ],
        }
        .into_iter()
        .chain([(self.length < 3).then(|| {
            match self.direction {
                Dir::Left => self.j -= 1,
                Dir::Right => self.j += 1,
                Dir::Up => self.i -= 1,
                Dir::Down => self.i += 1,
            }

            (self.i, self.j, self.length + 1, self.direction)
        })])
        .flatten()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Dir, Node};

    #[test]
    fn get_next() {
        assert_eq!(
            vec![
                (9, 10, 1, Dir::Up),
                (11, 10, 1, Dir::Down),
                (10, 11, 2, Dir::Right),
            ],
            Node::new(10, 10, 10, 1, Dir::Right, None)
                .get_next()
                .collect::<Vec<_>>()
        );

        assert_eq!(
            vec![(9, 10, 1, Dir::Up), (11, 10, 1, Dir::Down),],
            Node::new(10, 10, 10, 3, Dir::Right, None)
                .get_next()
                .collect::<Vec<_>>()
        );
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}
