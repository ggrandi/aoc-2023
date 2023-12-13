use std::fmt::Display;

use shared::dprintln;

const INUPUT: &str = include_str!("../input");

fn main() {
    let patterns = INUPUT
        .split("\n\n")
        .map(|p| {
            let vcount = p.lines().next().unwrap().chars().count();

            let (pattern, hcount) = p.lines().map(|l| l.chars().map(|c| c == '#')).fold(
                (vec![], 0),
                |(mut total, mut count), next| {
                    next.for_each(|v| total.push(v));
                    count += 1;

                    (total, count)
                },
            );

            ReflectionPattern::new(pattern, hcount, vcount)
        })
        .collect::<Vec<_>>();

    let mut columns_left_1 = 0;
    let mut rows_above_1 = 0;

    let mut columns_left_2 = 0;
    let mut rows_above_2 = 0;

    'patterns: for (i, mut pattern) in patterns.into_iter().enumerate() {
        let line = pattern.find_line(None).unwrap();

        match line {
            Line::Vertical(c) => columns_left_1 += c,
            Line::Horizontal(r) => rows_above_1 += r,
        }

        for smudge in 0..pattern.pattern.len() {
            pattern.flip(smudge);

            // dprintln!("{pattern}");

            let line = pattern.find_line(Some(line));

            if let Some(v) = line {
                match v {
                    Line::Vertical(c) => columns_left_2 += c,
                    Line::Horizontal(r) => rows_above_2 += r,
                }

                dprintln!("sm: {smudge}");
                continue 'patterns;
            };

            pattern.flip(smudge);
        }

        println!("{i}");

        unreachable!();
    }

    println!("part1: {}", rows_above_1 * 100 + columns_left_1);
    println!("part2: {}", rows_above_2 * 100 + columns_left_2);
}

#[derive(Debug)]
struct ReflectionPattern {
    pattern: Vec<bool>,
    hcount: usize,
    vcount: usize,
}

#[derive(Debug, Clone, Copy)]
enum Line {
    Vertical(usize),
    Horizontal(usize),
}

impl ReflectionPattern {
    fn new(pattern: Vec<bool>, hcount: usize, vcount: usize) -> Self {
        Self {
            pattern,
            hcount,
            vcount,
        }
    }

    fn flip(&mut self, smudge: usize) {
        self.pattern[smudge] = !self.pattern[smudge];
    }

    fn match_vertical(&self, j1: usize, j2: usize) -> bool {
        debug_assert!(j1 < j2);

        j2 >= self.vcount
            || (0..self.hcount)
                .map(|i| i * self.vcount)
                .all(|i| self.pattern[i + j1] == self.pattern[i + j2])
    }

    fn match_horizontal(&self, i1: usize, i2: usize) -> bool {
        debug_assert!(i1 < i2);

        i2 >= self.hcount
            || (0..self.vcount)
                .all(|j| self.pattern[i1 * self.vcount + j] == self.pattern[i2 * self.vcount + j])
    }

    fn find_line(&self, not: Option<Line>) -> Option<Line> {
        self.find_vertical_line(not)
            .or_else(|| self.find_horizontal_line(not))
    }

    fn find_vertical_line(&self, not: Option<Line>) -> Option<Line> {
        'outer: for i in 0..self.vcount - 1 {
            for curr in 0..=i {
                if !self.match_vertical(i - curr, i + curr + 1) {
                    continue 'outer;
                }
            }

            let line = i + 1;

            if matches!(not, Some(Line::Vertical(l)) if l == line) {
                continue;
            }

            return Some(Line::Vertical(line));
        }

        None
    }

    fn find_horizontal_line(&self, not: Option<Line>) -> Option<Line> {
        'outer: for j in 0..self.hcount - 1 {
            for curr in 0..=j {
                if !self.match_horizontal(j - curr, j + curr + 1) {
                    continue 'outer;
                }
            }

            let line = j + 1;

            if matches!(not, Some(Line::Horizontal(l)) if l == line) {
                continue;
            }

            return Some(Line::Horizontal(line));
        }

        None
    }
}

impl Display for ReflectionPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &c) in self.pattern.iter().enumerate() {
            if i % self.vcount == self.vcount - 1 {
                writeln!(f, "{}", if c { '#' } else { '.' })?
            } else {
                write!(f, "{}", if c { '#' } else { '.' })?
            }
        }

        Ok(())
    }
}
