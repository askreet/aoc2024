use crate::shared::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct Day16;

impl Solution for Day16 {
    fn part1(&self) -> Result<String> {
        let cg = CharGrid::from_file("inputs/day16.txt")?;

        part1(&cg).map(|v| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        let cg = CharGrid::from_file("inputs/day16.txt")?;

        part2(&cg).map(|v| v.to_string())
    }
}

#[derive(Debug, Clone)]
struct Path {
    pos: Position,
    dir: Direction,
    visited: Vec<Position>,
    cost: usize,
    distance: usize,
}

impl Path {
    fn start(pos: Position) -> Path {
        Path {
            pos,
            dir: RIGHT,
            visited: vec![pos],
            cost: 0,
            distance: 0,
        }
    }

    fn forward(&mut self) {
        self.pos = self.pos + self.dir;
        self.cost += 1;
        self.distance += 1;
        self.visited.push(self.pos);
    }

    fn clockwise(&mut self) {
        self.dir = self.dir.clockwise();
        self.cost += 1000;
    }

    fn anticlockwise(&mut self) {
        self.dir = self.dir.anticlockwise();
        self.cost += 1000;
    }
}

impl PartialEq<Self> for Path {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Path {}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.distance.cmp(&other.distance))
    }
}

struct FastestPath {
    best_at: HashMap<(Position, Direction), usize>,
}

impl FastestPath {
    fn new() -> Self {
        FastestPath {
            best_at: HashMap::new(),
        }
    }

    fn should_visit(&mut self, path: &Path) -> bool {
        let key = (path.pos, path.dir);

        match self.best_at.get(&key) {
            Some(v) if *v < path.cost => false,
            _ => {
                self.best_at.insert(key, path.cost);
                true
            }
        }
    }
}

fn solve(cg: &CharGrid) -> Result<Vec<Path>> {
    let start = cg.find_one_pos('S')?;

    let mut active: BinaryHeap<Path> = BinaryHeap::from([Path::start(start)]);
    let mut best_at = FastestPath::new();
    let mut best: usize = usize::MAX;
    let mut complete: Vec<Path> = Vec::new();

    let mut iters = 0;
    let mut completed = 0;
    let mut dismissed = 0;

    while let Some(mut path) = active.pop() {
        iters += 1;

        if iters % 10_000 == 0 {
            println!(
                "[{}] completed={} dismissed={} best={} active={} this.distance={}",
                iters,
                completed,
                dismissed,
                best,
                active.len() + 1, //the one we're processing
                path.distance
            );
        }
        if path.cost >= best {
            dismissed += 1;
            continue;
        }

        let can_visit =
            |dir| cg.at_pos(path.pos + dir) == '.' && !path.visited.contains(&(path.pos + dir));

        if can_visit(path.dir) {
            let mut path = path.clone();
            path.forward();

            if best_at.should_visit(&path) {
                active.push(path);
            } else {
                dismissed += 1;
            }
        }

        if can_visit(path.dir.clockwise()) {
            let mut path = path.clone();
            path.clockwise();
            path.forward();

            if best_at.should_visit(&path) {
                active.push(path);
            } else {
                dismissed += 1;
            }
        }

        if can_visit(path.dir.anticlockwise()) {
            let mut path = path.clone();
            path.anticlockwise();
            path.forward();

            if best_at.should_visit(&path) {
                active.push(path);
            } else {
                dismissed += 1;
            }
        }

        if cg.at_pos(path.pos + path.dir) == 'E' {
            let mut path = path.clone();
            path.forward();
            if path.cost < best {
                println!("found new best cost = {}", path.cost);
                best = path.cost;
            }
            completed += 1;
            complete.push(path);
        }
    }

    Ok(complete.into_iter().filter(|p| p.cost == best).collect())
}

fn part1(cg: &CharGrid) -> Result<usize> {
    let paths = solve(&cg)?;
    if paths.len() == 0 {
        return Err(Error::new("no paths to end found"));
    }

    Ok(paths[0].cost)
}

fn part2(cg: &CharGrid) -> Result<usize> {
    let paths = solve(&cg)?;
    if paths.len() == 0 {
        return Err(Error::new("no paths to end found"));
    }

    let mut uniq_positions = HashSet::new();
    for path in paths {
        for pos in path.visited {
            uniq_positions.insert(pos);
        }
    }

    Ok(uniq_positions.len())
}

#[test]
fn test_example1() {
    let cg = CharGrid::from_file("inputs/day16_example1.txt").unwrap();

    assert_eq!(7036, part1(&cg).unwrap());
    assert_eq!(45, part2(&cg).unwrap());
}

#[test]
fn test_example2() {
    let cg = CharGrid::from_file("inputs/day16_example2.txt").unwrap();

    assert_eq!(11048, part1(&cg).unwrap());
    assert_eq!(64, part2(&cg).unwrap());
}
