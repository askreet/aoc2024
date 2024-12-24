use crate::shared::*;
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

pub struct Day18;

impl Solution for Day18 {
    fn part1(&self) -> Result<String> {
        let bytes = Bytes::from_file("inputs/day18.txt")?;

        part1(
            Dimensions::of(71, 71),
            Position::at(70, 70),
            &bytes.first(1024),
        )
        .map(|v| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        let bytes = Bytes::from_file("inputs/day18.txt")?;

        let pos = part2(Dimensions::of(71, 71), Position::at(70, 70), &bytes)?;

        Ok(format!("{},{}", pos.x, pos.y))
    }
}

struct Bytes {
    positions: Vec<Position>,
}

impl Bytes {
    fn from_file(path: &str) -> Result<Bytes> {
        let contents = read_to_string(path)?;
        let mut positions = Vec::new();

        for line in contents.lines() {
            let parts = line.split(",").collect::<Vec<_>>();
            debug_assert!(parts.len() == 2);

            positions.push(Position::at(
                parts[0].parse::<i32>()?,
                parts[1].parse::<i32>()?,
            ));
        }

        Ok(Bytes {
            positions: positions,
        })
    }

    fn first(self, n: usize) -> Bytes {
        Bytes {
            positions: self.positions.into_iter().take(n).collect(),
        }
    }
}

fn shortest_path(cg: &CharGrid, start: Position, end: Position) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();
    explored.insert(start);
    queue.push_front((0, start));

    while !queue.is_empty() {
        let (dist, pos) = queue.pop_back().unwrap();
        if pos == end {
            return Some(dist);
        }

        for dir in [UP, DOWN, LEFT, RIGHT] {
            let target = pos + dir;
            if cg.in_bounds(target) && cg.at_pos(target) == '.' && !explored.contains(&target) {
                explored.insert(target);
                queue.push_front((dist + 1, target));
            }
        }
    }

    None
}

fn part1(dims: Dimensions, goal: Position, bytes: &Bytes) -> Result<usize> {
    let mut cg = CharGrid::new(dims.w, dims.h);
    cg.fill('.');

    for pos in &bytes.positions {
        cg.set_pos(*pos, '#');
    }

    match shortest_path(&cg, Position::at(0, 0), goal) {
        Some(v) => Ok(v),
        None => Err(Error::new("no valid path to goal")),
    }
}

fn part2(dims: Dimensions, goal: Position, bytes: &Bytes) -> Result<Position> {
    let mut cg = CharGrid::new(dims.w, dims.h);
    cg.fill('.');

    for pos in &bytes.positions {
        cg.set_pos(*pos, '#');

        if let None = shortest_path(&cg, Position::at(0, 0), goal) {
            return Ok(*pos);
        }
    }

    Err(Error::new("found path after consuming all bytes"))
}

#[test]
fn test_part1() {
    let bytes = Bytes::from_file("inputs/day18_example.txt").unwrap();

    assert_eq!(
        22,
        part1(Dimensions::of(7, 7), Position::at(6, 6), &bytes.first(12)).unwrap()
    );
}

#[test]
fn test_part2() {
    let bytes = Bytes::from_file("inputs/day18_example.txt").unwrap();

    assert_eq!(
        Position::at(6, 1),
        part2(Dimensions::of(7, 7), Position::at(6, 6), &bytes).unwrap()
    );
}
