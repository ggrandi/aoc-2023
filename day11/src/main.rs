use shared::dprintln;

const INPUT: &str = include_str!("../input");

fn main() {
    let mut rows = INPUT.lines().map(|_| 2).collect::<Vec<_>>();
    let mut columns = INPUT
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|_| 2)
        .collect::<Vec<_>>();
    let mut galaxies = vec![];

    for (i, line) in INPUT.lines().enumerate() {
        for (j, _) in line.chars().enumerate().filter(|&(_, c)| c == '#') {
            rows[i] = 1;
            columns[j] = 1;
            galaxies.push((i, j));
        }
    }

    let mut sum = 0;

    for (g1, g2) in galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g1)| galaxies[i + 1..].iter().map(|g2| (*g1, *g2)))
    {
        let i = ordered(g1.0, g2.0);
        let j = ordered(g1.1, g2.1);

        let i_distance: u64 = rows[i.0..i.1].iter().sum();
        let j_distance: u64 = columns[j.0..j.1].iter().sum();

        let num = j_distance + i_distance;
        dprintln!(
            "{g1:?}, {g2:?}: {}|{:?}|{:?}",
            num,
            &rows[i.0..i.1],
            &columns[j.0..j.1]
        );

        sum += num;
    }

    println!("part1: {sum}");

    // part 2
    let distance = 1_000_000;
    columns
        .iter_mut()
        .filter(|i| **i == 2)
        .for_each(|i| *i = distance);

    rows.iter_mut()
        .filter(|i| **i == 2)
        .for_each(|i| *i = distance);

    let mut sum = 0;

    for (g1, g2) in galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g1)| galaxies[i + 1..].iter().map(|g2| (*g1, *g2)))
    {
        let i = ordered(g1.0, g2.0);
        let j = ordered(g1.1, g2.1);

        let i_distance: u64 = rows[i.0..i.1].iter().sum();
        let j_distance: u64 = columns[j.0..j.1].iter().sum();

        let num = j_distance + i_distance;
        dprintln!(
            "{g1:?}, {g2:?}: {}|{:?}|{:?}",
            num,
            &rows[i.0..i.1],
            &columns[j.0..j.1]
        );

        sum += num;
    }

    println!("part2: {sum}");
}

fn ordered<T: PartialOrd<T>>(a: T, b: T) -> (T, T) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
