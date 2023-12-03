use std::collections::HashSet;

use shared::{char_to_usize, dprintln};
//
// const INPUT: &str = "
// ..111
// .*...
// 2..44";
//
const INPUT: &str = include_str!("../input");

fn main() {
    let inp = INPUT
        .trim()
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();

    part1(&inp);
    part2(&inp);
}

fn part1(inp: &[Vec<char>]) {
    let mut num = 0;
    let mut symbol_adjacent = false;

    let mut total = 0;

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            if inp[i][j].is_ascii_digit() {
                // if at start of number
                if num == 0
                // and in previous column
                    && j != 0
                    && (i != 0 && is_symbol(inp, i - 1, j - 1)
                        || is_symbol(inp, i, j - 1)
                        || is_symbol(inp, i + 1, j - 1))
                // or if not already found and in middle columns (this will also happen in the
                // first column)
                    || !symbol_adjacent
                        && (i != 0 && is_symbol(inp, i - 1, j) || is_symbol(inp, i + 1, j))
                {
                    symbol_adjacent = true
                }

                dprintln!("d: {}", inp[i][j]);

                num *= 10;
                num += char_to_usize(inp[i][j])
                //if it was the end of a number and adjacent to a symbol
            } else if num != 0 {
                if symbol_adjacent
                    // if there is a symbol in the next column
                    || i != 0 && is_symbol(inp, i - 1, j)
                    || is_symbol(inp, i, j)
                    || is_symbol(inp, i + 1, j)
                {
                    dprintln!("{num}");
                    total += num;
                    symbol_adjacent = false;
                }
                num = 0;
            }
        }
    }

    println!("part1: {total}");
}

fn part2(inp: &[Vec<char>]) {
    let mut total = 0;

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            if inp[i][j] == '*' {
                let mut nums = HashSet::new();

                // check top right
                let skip_next = if let Some((n, skip)) = find_number(inp, i - 1, j - 1) {
                    nums.insert(n);

                    skip
                } else {
                    0
                };

                if skip_next == 0 {
                    if let Some((n, _)) = find_number(inp, i - 1, j) {
                        nums.insert(n);
                    }

                    if let Some((n, _)) = find_number(inp, i, j - 1) {
                        nums.insert(n);
                    }
                }

                if skip_next <= 1 {
                    if let Some((n, _)) = find_number(inp, i - 1, j + 1) {
                        nums.insert(n);
                    }

                    if let Some((n, _)) = find_number(inp, i, j + 1) {
                        nums.insert(n);
                    }
                }

                if let Some((n, _)) = find_number(inp, i + 1, j - 1) {
                    nums.insert(n);
                }

                if let Some((n, _)) = find_number(inp, i + 1, j) {
                    nums.insert(n);
                }

                if let Some((n, _)) = find_number(inp, i + 1, j + 1) {
                    nums.insert(n);
                }

                dprintln!("[{i}][{j}]: {:?}", nums);

                assert!(nums.len() <= 2);

                if nums.len() >= 2 {
                    total += nums.iter().product::<usize>();
                }
            }
        }
    }

    println!("part2: {total}");
}

fn find_number(inp: &[Vec<char>], i: usize, j: usize) -> Option<(usize, usize)> {
    if inp
        .get(i)
        .and_then(|l| l.get(j))
        .map_or(true, |c| !c.is_ascii_digit())
    {
        return None;
    }

    let mut num = char_to_usize(inp[i][j]);
    let mut multiple = 1;

    for n in inp[i][0..j].iter().rev() {
        if !n.is_ascii_digit() {
            break;
        }

        multiple *= 10;
        num += char_to_usize(*n) * multiple;
    }

    let mut skip_right = 1;

    for n in inp[i][(j + 1)..].iter() {
        if !n.is_ascii_digit() {
            break;
        }

        skip_right += 1;

        num *= 10;
        num += char_to_usize(*n);
    }

    Some((num, skip_right))
}

fn is_symbol(inp: &[Vec<char>], i: usize, j: usize) -> bool {
    let res = inp
        .get(i)
        .and_then(|l| l.get(j))
        .map_or(false, |&c| c != '.' && !c.is_ascii_digit());

    dprintln!("({i}, {j}): {res}");

    res
}
