use anyhow::Result;

mod part1;
mod part2;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    part1::part1(INPUT)?;

    part2::part2(INPUT)
}
