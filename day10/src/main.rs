use anyhow::Result;

mod map;
mod part1;
mod part2;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let map = INPUT
        .lines()
        .map(|l| l.chars().map(TryFrom::try_from).collect::<Result<_, _>>())
        .collect::<Result<map::Map, _>>()?;

    let start = map.get_start();

    part1::part1(&map, start)?;

    part2::part2(&map)?;

    Ok(())
}
