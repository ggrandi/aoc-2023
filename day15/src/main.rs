const INPUT: &str = include_str!("../input");

fn main() {
    let inp = INPUT.trim().split(',');

    println!(
        "part1: {}",
        inp.clone().map(ReindeerHash::hash).sum::<usize>()
    );

    let inp = inp.map(|ins| {
        if ins.contains('-') {
            (&ins[..ins.len() - 1], None)
        } else {
            (
                &ins[..ins.len() - 2],
                Some(ins[ins.len() - 1..].parse::<usize>().unwrap()),
            )
        }
    });

    let mut map = vec![Vec::<(&str, usize)>::new(); 256];

    for (key, ins) in inp {
        let map_idx = key.hash();
        let position = map[map_idx].iter().position(|v| v.0 == key);

        match ins {
            // =
            Some(label) => match position {
                Some(p) => map[map_idx][p].1 = label,
                None => map[map_idx].push((key, label)),
            },
            // -
            None => {
                if let Some(p) = position {
                    map[map_idx].remove(p);
                }
            }
        }
    }

    let part2 = map
        .into_iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.into_iter()
                .enumerate()
                .map(move |(j, (_, p))| (i + 1) * (j + 1) * p)
        })
        .sum::<usize>();

    println!("part2: {part2}");
}

trait ReindeerHash {
    fn hash(&self) -> usize;
}

impl ReindeerHash for str {
    fn hash(&self) -> usize {
        (self.chars().fold(0u8, |acc, c| acc * 17 + c as u8) * 17) as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::ReindeerHash;

    #[test]
    fn reindeerhash() {
        assert_eq!("HASH".hash(), 52);
    }
}
