use crate::shared::*;
use std::collections::HashSet;

pub struct Day10;

impl Solution for Day10 {
    fn part1(&self) -> Result<String> {
        part1("inputs/day10.txt").map(|v| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        part2("inputs/day10.txt").map(|v| v.to_string())
    }
}

fn part1(path: &str) -> Result<usize> {
    let cg = CharGrid::from_file(path)?;

    let mut sum = 0;
    for trailhead in cg.find_all_pos('0') {
        sum += find_peaks(&cg, trailhead);
    }

    Ok(sum)
}

#[test]
fn test_part1() {
    let result = part1("inputs/day10_example.txt").unwrap();

    assert_eq!(36, result);
}

fn find_peaks(cg: &CharGrid, origin: Position) -> usize {
    let mut searched = HashSet::new();
    let mut frontier = vec![origin];

    let mut found_peaks = 0;

    while !frontier.is_empty() {
        let loc = frontier.pop().unwrap();
        debug_assert!(!searched.contains(&loc));

        searched.insert(loc);

        let this = cg.at(loc.x, loc.y);

        match this {
            '9' => found_peaks += 1,
            '0'..='8' => {
                for candidate in [loc + UP, loc + DOWN, loc + LEFT, loc + RIGHT] {
                    if cg.in_bounds(candidate)
                        && !searched.contains(&candidate)
                        && !frontier.contains(&candidate)
                        && cg.at(candidate.x, candidate.y) as u32 == this as u32 + 1
                    {
                        frontier.push(candidate);
                    }
                }
            }
            _ => panic!("unexpected character in map: {}", this),
        }
    }

    found_peaks
}

#[test]
fn test_find_peaks() {
    let cg = CharGrid::from_file("inputs/day10_example.txt").unwrap();

    assert_eq!(5, find_peaks(&cg, Position { x: 2, y: 0 }))
}

fn rate_trailhead(cg: &CharGrid, loc: Position) -> usize {
    let this = cg.at(loc.x, loc.y);

    if this == '9' {
        1
    } else {
        let mut sum = 0;

        for candidate in [loc + UP, loc + DOWN, loc + LEFT, loc + RIGHT] {
            if cg.in_bounds(candidate) && cg.at(candidate.x, candidate.y) as u32 == this as u32 + 1
            {
                sum += rate_trailhead(&cg, candidate);
            }
        }

        sum
    }
}

#[test]
fn test_rate_trailhead() {
    let cg = CharGrid::from_file("inputs/day10_example.txt").unwrap();

    assert_eq!(20, rate_trailhead(&cg, Position { x: 2, y: 0 }));
}

fn part2(path: &str) -> Result<usize> {
    let cg = CharGrid::from_file(path)?;

    let mut sum = 0;
    for trailhead in cg.find_all_pos('0') {
        sum += rate_trailhead(&cg, trailhead);
    }

    Ok(sum)
}

#[test]
fn test_part2() {
    let result = part2("inputs/day10_example.txt").unwrap();

    assert_eq!(81, result);
}
