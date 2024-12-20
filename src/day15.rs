use crate::shared::*;
use std::fs::read_to_string;

pub struct Day15;

impl Solution for Day15 {
    fn part1(&self) -> Result<String> {
        solve("inputs/day15.txt", false).map(|v| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        solve("inputs/day15.txt", true).map(|v| v.to_string())
    }
}

fn solve(path: &str, expand: bool) -> Result<usize> {
    let contents = read_to_string(path)?;
    let docs = split_docs(contents);
    debug_assert!(docs.len() == 2);

    let mut cg = CharGrid::from_str(&docs[0])?;
    if expand {
        cg = expand_map(cg);
    }

    let (x, y) = cg.find_one('@')?;
    let mut pos = Position::at(x, y);

    for command in docs[1].bytes() {
        if command == '\n' as u8 {
            continue;
        }

        let dir = match command as char {
            '^' => UP,
            '<' => LEFT,
            '>' => RIGHT,
            'v' => DOWN,
            v => panic!("unexpected command '{}'", v),
        };

        if cg_move(&mut cg, pos, dir) {
            pos = pos + dir;
        }
    }

    let count_target = match expand {
        true => '[',
        false => 'O',
    };

    Ok(cg
        .find_all_pos(count_target)
        .iter()
        .map(|p| (p.y * 100 + p.x) as usize)
        .sum())
}

#[test]
fn test_part1() {
    assert_eq!(10092, solve("inputs/day15_example.txt", false).unwrap());
}

#[test]
fn test_part2() {
    assert_eq!(9021, solve("inputs/day15_example.txt", true).unwrap())
}

fn cg_move(cg: &mut CharGrid, pos: Position, dir: Direction) -> bool {
    debug_assert!(cg.at_pos(pos) == '@');
    fn is_movable(cg: &CharGrid, pos: Position, dir: Direction) -> bool {
        match cg.at_pos(pos + dir) {
            '#' => false,
            '.' => true,
            'O' => is_movable(cg, pos + dir, dir),
            '[' | ']' if dir == LEFT || dir == RIGHT => is_movable(cg, pos + dir, dir),
            '[' => is_movable(cg, pos + dir, dir) && is_movable(cg, (pos + RIGHT) + dir, dir),
            ']' => is_movable(cg, pos + dir, dir) && is_movable(cg, (pos + LEFT) + dir, dir),
            v => panic!("unexpected character '{}'", v),
        }
    }

    if !is_movable(cg, pos, dir) {
        return false;
    }

    fn do_move(cg: &mut CharGrid, pos: Position, dir: Direction) {
        let target = cg.at_pos(pos + dir);

        if target != '.' {
            match target {
                '#' => panic!("cannot move walls!"),
                'O' => do_move(cg, pos + dir, dir),
                '[' | ']' if dir == LEFT || dir == RIGHT => do_move(cg, pos + dir, dir),
                '[' => {
                    do_move(cg, pos + dir, dir);
                    do_move(cg, (pos + RIGHT) + dir, dir)
                }
                ']' => {
                    do_move(cg, pos + dir, dir);
                    do_move(cg, (pos + LEFT) + dir, dir);
                }
                _ => panic!("unexpected target '{}'", target),
            }
        }

        let current = cg.at_pos(pos);
        cg.set_pos(pos + dir, current);
        cg.set_pos(pos, '.');
    }

    do_move(cg, pos, dir);
    true
}

#[test]
fn test_part2_move() {
    let mut cg = CharGrid::from_str(
        "##############\n\
         ##......##..##\n\
         ##..........##\n\
         ##...[][]...##\n\
         ##....[]....##\n\
         ##.....@....##\n\
         ##############\n",
    )
    .unwrap();

    assert_eq!(true, cg_move(&mut cg, Position::at(7, 5), UP));

    assert_eq!(
        cg.draw(),
        "##############\n\
         ##......##..##\n\
         ##...[][]...##\n\
         ##....[]....##\n\
         ##.....@....##\n\
         ##..........##\n\
         ##############\n\n",
    );
}

fn expand_map(cg: CharGrid) -> CharGrid {
    let mut target = CharGrid::new(cg.width() * 2, cg.height());

    for y in 0..cg.height() {
        for x in 0..cg.width() {
            let chars = match cg.at(x, y) {
                '#' => ('#', '#'),
                '.' => ('.', '.'),
                'O' => ('[', ']'),
                '@' => ('@', '.'),
                v => panic!("unexpected char '{}'", v),
            };

            target.set(x * 2, y, chars.0);
            target.set(x * 2 + 1, y, chars.1);
        }
    }

    target
}

#[test]
fn test_expand_map() {
    let cg = CharGrid::from_str(
        "#######\n\
         #...#.#\n\
         #.....#\n\
         #..OO@#\n\
         #..O..#\n\
         #.....#\n\
         #######\n",
    )
    .unwrap();

    let expanded = expand_map(cg);

    assert_eq!(
        expanded.draw(),
        "##############\n\
         ##......##..##\n\
         ##..........##\n\
         ##....[][]@.##\n\
         ##....[]....##\n\
         ##..........##\n\
         ##############\n\n",
    );
}
