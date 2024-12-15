use crate::shared::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

pub struct Day11;

impl Solution for Day11 {
    fn part1(&self) -> Result<String> {
        let stones = read_stones("inputs/day11.txt")?;
        Ok(stoneify(stones, 25).count().to_string())
    }

    fn part2(&self) -> Result<String> {
        let stones = read_stones("inputs/day11.txt")?;
        Ok(stoneify(stones, 75).count().to_string())
    }
}

fn read_stones(path: &str) -> Result<Stones> {
    let values = read_to_string(path)?
        .split_whitespace()
        .map(|n| n.parse::<usize>())
        .collect::<std::result::Result<Vec<_>, _>>()?;

    let mut stones = Stones::new();
    for v in values {
        stones.add_many(v, 1);
    }

    Ok(stones)
}

struct Stones {
    ns: HashMap<usize, usize>,
}

impl Stones {
    fn new() -> Self {
        Stones { ns: HashMap::new() }
    }

    fn run_step(&mut self) -> Stones {
        let mut new_stones = Stones::new();

        for (stone, count) in &self.ns {
            let ndigits = digits(*stone);

            match stone {
                0 => new_stones.add_many(1, *count),
                n if is_even(ndigits) => {
                    let (left, right) = split_number(*n, ndigits);

                    new_stones.add_many(left, *count);
                    new_stones.add_many(right, *count);
                }
                n => new_stones.add_many(n * 2024, *count),
            }
        }

        new_stones
    }

    fn add_many(&mut self, n: usize, count: usize) {
        let current = self.ns.get(&n).unwrap_or(&0);

        self.ns.insert(n, current + count);
    }

    fn count(&self) -> usize {
        self.ns.values().sum()
    }

    fn uniq_count(&self) -> usize {
        self.ns.len()
    }
}

fn stoneify(mut stones: Stones, iters: usize) -> Stones {
    for i in 0..iters {
        let start = Instant::now();
        let start_count = stones.count();
        stones = stones.run_step();

        println!(
            "iteration {} complete in {}ms (-> {} stones [+ {}]) (seen {} unique numbers)",
            i,
            start.elapsed().as_millis(),
            stones.count(),
            stones.count() - start_count,
            stones.uniq_count()
        );
    }

    stones
}

fn is_even(n: usize) -> bool {
    n % 2 == 0
}

#[test]
fn test_part1() {
    let ns = read_stones("inputs/day11_example.txt").unwrap();
    let result = stoneify(ns, 25);

    assert_eq!(55312, result.count());
}

#[test]
fn test_0() {
    let mut s = Stones::new();
    s.add_many(0, 1);

    stoneify(s, 30);
}

fn split_number(n: usize, digits: usize) -> (usize, usize) {
    if digits % 2 != 0 {
        panic!("split_number called with uneven digits");
    }

    match digits {
        2 => (n / 10, n % 10),
        4 => (n / 100, n % 100),
        6 => (n / 1000, n % 1000),
        8 => (n / 10000, n % 10000),
        10 => (n / 100000, n % 100000),
        12 => (n / 1000000, n % 1000000),
        14 => (n / 10000000, n % 10000000),
        16 => (n / 100000000, n % 100000000),
        18 => (n / 1000000000, n % 1000000000),
        20 => (n / 10000000000, n % 10000000000),
        _ => panic!(),
    }
}

#[test]
fn test_split_number() {
    assert_eq!((1, 0), split_number(10, 2));
    assert_eq!((3, 8), split_number(38, 2));
    assert_eq!((1234, 5000), split_number(1234_5000, 8));
}
