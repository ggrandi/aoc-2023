use std::{collections::HashMap, ops::Range, str::FromStr};

use anyhow::{anyhow, bail, Error, Result};
use shared::dprintln;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let (workflows, parts) = INPUT.split_once("\n\n").ok_or(anyhow!("couldn't split"))?;

    let workflows = workflows
        .lines()
        .map(TryFrom::try_from)
        .map(|e| e.map(|w: Workflow| (w.name, w)))
        .collect::<Result<HashMap<&str, Workflow>, _>>()?;

    let parts = parts
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<Vec<Part>, _>>()?;

    let mut sum = 0;

    for part in parts {
        dprintln!("part: {part:?}\n  in");
        let mut current_workflow = workflows.get("in").unwrap();
        let mut current_index = 0;
        let accepted = loop {
            let next = match &current_workflow.ops[current_index] {
                Op::Gt(c, num, next) => (part.get_category(*c) > *num).then_some(next),
                Op::Lt(c, num, next) => (part.get_category(*c) < *num).then_some(next),
                Op::Final(next) => Some(next),
            };

            match next {
                Some(Next::Rejected) => {
                    dprintln!("  R");
                    break false;
                }
                Some(Next::Accepted) => {
                    dprintln!("  A");
                    break true;
                }
                Some(Next::Workflow(n)) => {
                    dprintln!("  {n}");
                    current_workflow = workflows.get(n).unwrap();
                    current_index = 0;
                }
                None => {
                    current_index += 1;
                }
            }
        };

        if accepted {
            sum += part.x + part.m + part.a + part.s;
        }
    }

    println!("part1: {sum}");

    let mut stack = vec![(
        RangePart {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        },
        Next::Workflow("in"),
        0,
    )];

    let mut sum = 0;

    'outer: while let Some((mut part, workflow, mut current_index)) = stack.pop() {
        dprintln!("{} {part:?}", stack.len());
        let workflow = match workflow {
            Next::Workflow(w) => w,
            Next::Accepted => {
                sum += part.num_combinations();
                continue;
            }
            Next::Rejected => continue,
        };

        let mut current_workflow = workflows.get(workflow).unwrap();

        loop {
            dprintln!(
                "  {:?} {} {:?}",
                part,
                current_workflow.name,
                current_workflow.ops[current_index]
            );
            let next = match &current_workflow.ops[current_index] {
                Op::Gt(c, num, next) => match part.split_at_gt(*c, *num) {
                    Ok((o, p)) => {
                        stack.push((o, *next, 0));
                        part = p;
                        None
                    }
                    Err(p) => {
                        part = p;
                        None
                    }
                },
                Op::Lt(c, num, next) => match part.split_at_lt(*c, *num) {
                    Ok((o, p)) => {
                        stack.push((o, *next, 0));
                        part = p;
                        None
                    }
                    Err(p) => {
                        part = p;
                        None
                    }
                },
                Op::Final(next) => Some(next),
            };

            match next {
                Some(Next::Rejected) => {
                    continue 'outer;
                }
                Some(a @ Next::Accepted) => {
                    stack.push((part, *a, 0));
                    continue 'outer;
                }
                Some(Next::Workflow(n)) => {
                    dprintln!("  {n}");
                    current_workflow = workflows.get(n).unwrap();
                    current_index = 0;
                }
                None => {
                    current_index += 1;
                }
            }
        }
    }

    println!("part2: {sum}");

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl FromStr for Category {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => bail!("couldn't parse the category"),
        })
    }
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn get_category(&self, category: Category) -> u64 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }
}

#[derive(Debug, Clone)]
struct RangePart {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>,
}

impl RangePart {
    fn split_at_gt(self, category: Category, v: u64) -> Result<(RangePart, RangePart), RangePart> {
        match category {
            Category::X => match self.x.split_gt(v) {
                None => Err(self),
                Some((x, o)) => Ok((RangePart { x, ..self.clone() }, RangePart { x: o, ..self })),
            },
            Category::M => match self.m.split_gt(v) {
                None => Err(self),
                Some((m, o)) => Ok((RangePart { m, ..self.clone() }, RangePart { m: o, ..self })),
            },
            Category::A => match self.a.split_gt(v) {
                None => Err(self),
                Some((a, o)) => Ok((RangePart { a, ..self.clone() }, RangePart { a: o, ..self })),
            },
            Category::S => match self.s.split_gt(v) {
                None => Err(self),
                Some((s, o)) => Ok((RangePart { s, ..self.clone() }, RangePart { s: o, ..self })),
            },
        }
    }

    fn split_at_lt(self, category: Category, v: u64) -> Result<(RangePart, RangePart), RangePart> {
        match category {
            Category::X => match self.x.split_lt(v) {
                None => Err(self),
                Some((x, o)) => Ok((RangePart { x, ..self.clone() }, RangePart { x: o, ..self })),
            },
            Category::M => match self.m.split_lt(v) {
                None => Err(self),
                Some((m, o)) => Ok((RangePart { m, ..self.clone() }, RangePart { m: o, ..self })),
            },
            Category::A => match self.a.split_lt(v) {
                None => Err(self),
                Some((a, o)) => Ok((RangePart { a, ..self.clone() }, RangePart { a: o, ..self })),
            },
            Category::S => match self.s.split_lt(v) {
                None => Err(self),
                Some((s, o)) => Ok((RangePart { s, ..self.clone() }, RangePart { s: o, ..self })),
            },
        }
    }

    fn num_combinations(self) -> u64 {
        let Self {
            x: x_r,
            m: m_r,
            a: a_r,
            s: s_r,
        } = self;

        (x_r.end - x_r.start)
            * (m_r.end - m_r.start)
            * (a_r.end - a_r.start)
            * (s_r.end - s_r.start)

        // x_r.flat_map(|x| m_r.clone().map(move |m| x + m))
        //     .flat_map(|xm| a_r.clone().map(move |a| xm + a))
        //     .flat_map(|xma| s_r.clone().map(move |s| xma + s))
        //     .sum()
    }
}

impl FromStr for Part {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[3..];

        let (x, s) = s.split_once(",m=").ok_or(anyhow!("couldn't find m"))?;

        let (m, s) = s.split_once(",a=").ok_or(anyhow!("couldn't find a"))?;

        let (a, s) = s.split_once(",s=").ok_or(anyhow!("couldn't find s"))?;

        let s = &s[..s.len() - 1];

        Ok(Self {
            x: x.parse()?,
            m: m.parse()?,
            a: a.parse()?,
            s: s.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Next<'a> {
    Workflow(&'a str),
    Accepted,
    Rejected,
}

impl<'a> TryFrom<&'a str> for Next<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(match s {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            e => Self::Workflow(e),
        })
    }
}

#[derive(Debug)]
enum Op<'a> {
    Gt(Category, u64, Next<'a>),
    Lt(Category, u64, Next<'a>),
    Final(Next<'a>),
}

impl<'a> TryFrom<&'a str> for Op<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value.split_once(':') {
            Some((s, n)) => match (s.split_once('<'), s.split_once('>')) {
                (Some((c, w)), _) => Op::Lt(c.parse()?, w.parse()?, n.try_into()?),
                (_, Some((c, w))) => Op::Gt(c.parse()?, w.parse()?, n.try_into()?),
                (_, _) => bail!("couldn't parse op"),
            },
            None => Op::Final(value.try_into()?),
        })
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    ops: Vec<Op<'a>>,
    name: &'a str,
}

impl<'a> TryFrom<&'a str> for Workflow<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let s = s.split_once('{').ok_or(anyhow!("couldn't get the name"))?;

        let name = s.0;

        let ops = s.1[..s.1.len() - 1]
            .split(',')
            .map(TryFrom::try_from)
            .collect::<Result<_, _>>()?;

        Ok(Self { ops, name })
    }
}

trait RangeSplit
where
    Self: Sized,
{
    fn split_gt(&self, val: u64) -> Option<(Self, Self)>;
    fn split_lt(&self, val: u64) -> Option<(Self, Self)>;
}

impl RangeSplit for Range<u64> {
    fn split_gt(&self, val: u64) -> Option<(Self, Self)> {
        if self.contains(&val) {
            Some((val + 1..self.end, self.start..val + 1))
        } else {
            None
        }
    }

    fn split_lt(&self, val: u64) -> Option<(Self, Self)> {
        if self.contains(&val) {
            Some((self.start..val, val..self.end))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::RangeSplit;

    #[test]
    fn range_split() {
        assert_eq!(None, (1u64..4001).split_lt(4001));

        assert_eq!(Some((1..2000, 2000..4001)), (1u64..4001).split_lt(2000));

        assert_eq!(Some((1..1000, 1000..4001)), (1u64..4001).split_lt(1000));

        assert_eq!(None, (1u64..4001).split_gt(5000));

        assert_eq!(Some((11..4001, 1..11)), (1u64..4001).split_gt(10));
        assert_eq!(Some((24..4001, 1..24)), (1u64..4001).split_gt(23));
    }
}
