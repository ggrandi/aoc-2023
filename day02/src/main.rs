// const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

use std::cmp::max;

const INPUT: &str = include_str!("../input");

enum Balls {
    Red,
    Green,
    Blue,
}

fn main() {
    println!(
        "part1: {}",
        INPUT
            .lines()
            .map(|l| {
                let (id, l) = l.split_once(": ").unwrap();
                (
                    id[5..].parse::<usize>().unwrap(),
                    l.split("; ")
                        .map(|l| {
                            l.split(", ")
                                .map(|n| {
                                    let (n, col) = n.split_once(' ').unwrap();

                                    (
                                        n.parse::<usize>().unwrap(),
                                        match col {
                                            "red" => Balls::Red,
                                            "green" => Balls::Green,
                                            "blue" => Balls::Blue,
                                            _ => panic!("unknown color"),
                                        },
                                    )
                                })
                                .fold((0, 0, 0), |acc, n| match n.1 {
                                    Balls::Red => (acc.0 + n.0, acc.1, acc.2),
                                    Balls::Green => (acc.0, acc.1 + n.0, acc.2),
                                    Balls::Blue => (acc.0, acc.1, acc.2 + n.0),
                                })
                        })
                        .all(|(r, g, b)| r <= 12 && g <= 13 && b <= 14),
                )
            })
            .filter_map(|(id, possible)| if possible { Some(id) } else { None })
            .sum::<usize>()
    );

    println!(
        "part2: {}",
        INPUT
            .lines()
            .map(|l| {
                l.split_once(": ")
                    .unwrap()
                    .1
                    .split("; ")
                    .map(|l| {
                        l.split(", ")
                            .map(|n| {
                                let (n, col) = n.split_once(' ').unwrap();

                                (
                                    n.parse::<usize>().unwrap(),
                                    match col {
                                        "red" => Balls::Red,
                                        "green" => Balls::Green,
                                        "blue" => Balls::Blue,
                                        _ => panic!("unknown color"),
                                    },
                                )
                            })
                            .fold((0, 0, 0), |acc, n| match n.1 {
                                Balls::Red => (acc.0 + n.0, acc.1, acc.2),
                                Balls::Green => (acc.0, acc.1 + n.0, acc.2),
                                Balls::Blue => (acc.0, acc.1, acc.2 + n.0),
                            })
                    })
                    .fold((0, 0, 0), |acc, n| {
                        (max(acc.0, n.0), max(acc.1, n.1), max(acc.2, n.2))
                    })
            })
            .map(|(r, g, b)| r * g * b)
            .sum::<usize>()
    );
}
