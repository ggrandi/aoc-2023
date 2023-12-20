use std::collections::{HashMap, VecDeque};

use anyhow::{bail, Error, Result};
use shared::{dprintln, LeastCommonMultiple};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let mut modules: Modules = INPUT.try_into()?;

    let (sums, _) = (0..1000).fold(((0, 0), None), |((a, b), _), n| {
        let ((c, d), is_rx_pulsed) = modules.press_button();

        ((a + c, b + d), is_rx_pulsed.then_some(n))
    });

    println!("part1: {}", sums.0 * sums.1);

    #[cfg(debug_assertions)]
    {
        println!("flowchart LR\n  button --> broadcaster");
        for (n, (ptype, conn)) in modules.modules.iter() {
            match ptype {
                Pulser::Broadcast => println!("  {n}[{n}]\n  style {n} fill:"),
                Pulser::FlipFlop(_) => println!("  {n}([{n}])"),
                Pulser::Conjunction(_) => println!("  {n}{{{{{n}}}}}"),
            }

            for c in conn {
                println!("  {n} --> {c}");
            }
        }
    }

    let t = modules.rx_on();

    println!("part2: {t} ");

    Ok(())
}

#[derive(Debug, Clone)]
enum Pulser<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
}

#[derive(Debug)]
struct Modules<'a> {
    modules: HashMap<&'a str, (Pulser<'a>, Vec<&'a str>)>,
}

impl<'a> Modules<'a> {
    pub fn press_button(&mut self) -> ((usize, usize), bool) {
        let mut queue = VecDeque::new();

        queue.push_back(("broadcaster", "button", false));

        let mut sum_high = 0;
        let mut sum_low = 0;

        let mut is_rx_pulsed = false;

        while let Some((module, from, high)) = queue.pop_front() {
            dprintln!(
                "  {from} -{}-> {module}",
                if high { "high" } else { "low-" }
            );

            if module == "rx" && !high {
                is_rx_pulsed = true;
            }

            *(if high { &mut sum_high } else { &mut sum_low }) += 1;

            let (ptype, conn) = match self.modules.get_mut(module) {
                Some(c) => c,
                None => continue,
            };

            let pulse = match ptype {
                Pulser::Broadcast => Some(high),
                Pulser::FlipFlop(r) => {
                    if !high {
                        *r = !*r;
                        let pulse = *r;

                        Some(pulse)
                    } else {
                        None
                    }
                }
                Pulser::Conjunction(inps) => {
                    *inps.get_mut(from).unwrap() = high;

                    Some(!inps.iter().all(|(_, s)| *s))
                }
            };

            if let Some(pulse) = pulse {
                for c in conn {
                    queue.push_back((c, module, pulse));
                }
            }
        }

        ((sum_high, sum_low), is_rx_pulsed)
    }

    pub fn rx_on(&self) -> usize {
        self.modules
            .get("broadcaster")
            .unwrap()
            .1
            .iter()
            .map(|c| {
                let chain_last = self
                    .modules
                    .get(c)
                    .unwrap()
                    .1
                    .iter()
                    .find(|c| matches!(self.modules.get(*c), Some((Pulser::Conjunction(_), _))))
                    .unwrap();

                dprintln!("{c}|{chain_last}");

                let chain_len = self.calc_chain(c, chain_last, 0, 0);

                dprintln!("{chain_len}|{chain_len:b}");
                chain_len
            })
            .fold(1, |acc, n| acc.lcm(&n))
    }

    pub fn calc_chain(&self, curr: &str, chain_last: &str, acc: usize, len: usize) -> usize {
        let conn = &self.modules.get(curr).unwrap().1[..];

        let c = conn.iter().find(|c| **c != chain_last);

        dprintln!("  {curr}|{}|{c:?}", conn.len());

        match c {
            Some(next) => self.calc_chain(
                next,
                chain_last,
                acc + (if conn.len() != 1 { 1 << len } else { 0 }),
                len + 1,
            ),
            None => acc + (1 << len),
        }
    }
}

impl<'a> TryFrom<&'a str> for Modules<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut modules = value
            .lines()
            .map(|l| match l.split_once(" -> ") {
                Some((n, c)) => {
                    let (btype, name) = if n == "broadcaster" {
                        (Pulser::Broadcast, n)
                    } else {
                        let (p, name) = n.split_at(1);

                        (
                            match p {
                                "%" => Pulser::FlipFlop(false),
                                "&" => Pulser::Conjunction(HashMap::new()),
                                _ => bail!("invalid pulser"),
                            },
                            name,
                        )
                    };

                    Ok((name, (btype, c.split(", ").collect())))
                }
                None => bail!("invalid module"),
            })
            .collect::<Result<HashMap<_, (_, Vec<_>)>, _>>()?;

        let copy = modules.clone();

        modules.iter_mut().for_each(|(k, (ptype, _))| {
            if let Pulser::Conjunction(m) = ptype {
                copy.iter()
                    .filter(|(_, (_, c))| c.iter().any(|c| c == k))
                    .for_each(|(k, (_, _))| {
                        m.insert(k, false);
                    })
            }
        });

        Ok(Self { modules })
    }
}
