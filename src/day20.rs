use crate::shared::*;

pub struct Day20;

impl Solution for Day20 {
    fn part1(&self) -> Result<String> {
        part1("inputs/day20.txt", 100).map(|v| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        todo!()
    }
}

fn part1(path: &str, cheat_floor: usize) -> Result<usize> {
    let mut mg = MetaGrid::<Option<usize>>::from_file(path)?;

    let mut pos = mg.find_one('S')?;
    let mut move_cost = 0;

    let mut path = Vec::new();

    // Trace the 'correct' path.
    loop {
        mg.set_meta(pos, Some(move_cost));
        path.push((pos, move_cost));

        if mg.at(pos).0 == 'E' {
            break;
        }

        let next = mg
            .adjacent_orthogonal(pos)
            .into_iter()
            .filter(|(_, c, m)| (*c == '.' || *c == 'E') && m.is_none())
            .collect::<Vec<_>>();

        pos = next[0].0;
        move_cost += 1;
    }

    let mut found_cheats = 0;

    for (p, cost) in path {
        // assumption: only straight lines are viable cheats
        for dir in [UP, DOWN, LEFT, RIGHT] {
            let cheat_start = p + dir;
            let cheat_end = p + dir + dir;

            if let ('#', _) = mg.at(cheat_start) {
                if mg.in_bounds(cheat_end) {
                    if let (_, Some(other_cost)) = mg.at(cheat_end) {
                        if *other_cost > (cost + 2) && *other_cost - (cost + 2) >= cheat_floor {
                            found_cheats += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(found_cheats)
}

#[test]
fn test_part1() {
    assert_eq!(5, part1("inputs/day20_example.txt", 20).unwrap());
    assert_eq!(44, part1("inputs/day20_example.txt", 1).unwrap());
}
