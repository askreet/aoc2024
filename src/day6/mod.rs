use crate::shared::*;

pub struct Day6;

impl Solution for Day6 {
    fn part1(&self) -> Result<String> {
        part1_walk("inputs/day6.txt").map(|(v, _)| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        part2("inputs/day6.txt").map(|v| v.to_string())
    }
}

fn part1_walk(path: &str) -> Result<(usize, CharGrid)> {
    let mut cg = CharGrid::from_file(path)?;

    let (x, y) = cg.find_one('^')?;
    let mut pos = Position { x, y };
    let mut direction = UP;

    loop {
        cg.set(pos.x, pos.y, 'X');
        let next = pos.add(direction);

        if !cg.in_bounds(next) {
            return Ok((cg.count('X'), cg));
        }

        match cg.at(next.x, next.y) {
            '#' => direction = direction.clockwise(),
            '.' | 'X' => pos = next,
            c => return Err(Error::new(&format!("unexpected char: '{}'", c))),
        }
    }
}

fn part2(path: &str) -> Result<usize> {
    let original = CharGrid::from_file(path)?;
    let (start_x, start_y) = original.find_one('^')?;
    let (_, walked) = part1_walk(path)?;

    let mut ct = 0;
    for pos in walked.find_all_pos('X') {
        if start_x == pos.x && start_y == pos.y {
            continue;
        }

        let mut hypothetical = original.clone();
        hypothetical.set(pos.x, pos.y, '#');

        if WalkResult::InfiniteLoop == part2_walk(&mut hypothetical)? {
            ct += 1;
        }
    }

    Ok(ct)
}

#[derive(Debug, PartialEq, Eq)]
enum WalkResult {
    ExitMap,
    InfiniteLoop,
}
fn part2_walk(cg: &mut CharGrid) -> Result<WalkResult> {
    let (x, y) = cg.find_one('^')?;
    let mut pos = Position { x, y };
    let mut direction = UP;

    loop {
        let next = pos.add(direction);

        if !cg.in_bounds(next) {
            return Ok(WalkResult::ExitMap);
        }

        match cg.at(next.x, next.y) {
            '#' => direction = direction.clockwise(),
            '.' | '1'..'6' => {
                match cg.at(pos.x, pos.y) {
                    '^' | '.' => cg.set(pos.x, pos.y, '1'),
                    '1' => cg.set(pos.x, pos.y, '2'),
                    '2' => cg.set(pos.x, pos.y, '3'),
                    '3' => cg.set(pos.x, pos.y, '4'),
                    '4' => cg.set(pos.x, pos.y, '5'),
                    '5' => cg.set(pos.x, pos.y, '6'),
                    c => panic!(
                        "unexpected char at position(x={}, y={}): '{}'",
                        pos.x, pos.y, c
                    ),
                }

                pos = next
            }
            '6' => return Ok(WalkResult::InfiniteLoop),
            c => return Err(Error::new(&format!("unexpected char: '{}'", c))),
        }
    }
}

#[test]
fn test_part1_example() {
    let (ct, _) = part1_walk("inputs/day6_example.txt").unwrap();

    assert_eq!(41, ct);
}

#[test]
fn test_part2_example() {
    let ct = part2("inputs/day6_example.txt").unwrap();

    assert_eq!(6, ct);
}
