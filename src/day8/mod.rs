use crate::shared::*;

pub struct Day8;

impl Solution for Day8 {
    fn part1(&self) -> Result<String> {
        count_antinodes("inputs/day8.txt").map(|v| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        count_antinodes_p2("inputs/day8.txt").map(|v| v.to_string())
    }
}

fn count_antinodes(path: &str) -> Result<usize> {
    let map = CharGrid::from_file(path)?;
    let mut antinodes = map.clone();

    for antenna in map.uniq_chars() {
        if antenna == '.' {
            continue;
        }

        let locations = map.find_all_pos(antenna);
        if locations.len() == 1 {
            println!("found only one antenna of type '{}'", antenna);
            continue;
        }

        let mut mark_antinode = |p: Position| {
            if antinodes.in_bounds(p) {
                antinodes.set_pos(p, '#');
            }
        };

        for positions in combinations(2, &locations) {
            let p1 = positions[0];
            let p2 = positions[1];

            let offset = p1.delta(p2);

            mark_antinode(p1.add_pos(offset));
            mark_antinode(p2.sub_pos(offset));
        }
    }

    Ok(antinodes.count('#'))
}

fn count_antinodes_p2(path: &str) -> Result<usize> {
    let map = CharGrid::from_file(path)?;
    let mut antinodes = map.clone();

    for antenna in map.uniq_chars() {
        if antenna == '.' {
            continue;
        }

        let locations = map.find_all_pos(antenna);
        if locations.len() == 1 {
            println!("found only one antenna of type '{}'", antenna);
            continue;
        }

        for positions in combinations(2, &locations) {
            let offset = positions[0].delta(positions[1]);

            antinodes.set_pos(positions[0], '#');
            let mut target = positions[0].sub_pos(offset);
            while antinodes.in_bounds(target) {
                antinodes.set_pos(target, '#');
                target = target.sub_pos(offset);
            }

            let mut target = positions[0].add_pos(offset);
            while antinodes.in_bounds(target) {
                antinodes.set_pos(target, '#');
                target = target.add_pos(offset);
            }
        }
    }

    Ok(antinodes.count('#'))
}

#[test]
fn test_part1() {
    assert_eq!(14, count_antinodes("inputs/day8_example.txt").unwrap());
}

#[test]
fn test_part2() {
    assert_eq!(34, count_antinodes_p2("inputs/day8_example.txt").unwrap());
}
