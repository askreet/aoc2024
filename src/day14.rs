use crate::shared::*;
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

pub struct Day14;

impl Solution for Day14 {
    fn part1(&self) -> Result<String> {
        part1("inputs/day14.txt", Dimensions::of(101, 103)).map(|v| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        part2("inputs/day14.txt", Dimensions::of(101, 103)).map(|v| v.to_string())
    }
}

struct Robot {
    pos: Position,
    vel: Direction,
}

struct Robots {
    robots: Vec<Robot>,
    bounds: Dimensions,
}

impl Robots {
    fn from_str(str: &str, bounds: Dimensions) -> Result<Robots> {
        let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")?;

        let mut robots = Vec::new();

        for line in str.lines() {
            match regex.captures(line) {
                None => return Err(Error::new(&format!("invalid input in robots: {}", line))),

                Some(c) => robots.push(Robot {
                    pos: Position::at(
                        c.get(1).unwrap().as_str().parse::<i32>()?,
                        c.get(2).unwrap().as_str().parse::<i32>()?,
                    ),
                    vel: Direction::of(
                        c.get(3).unwrap().as_str().parse::<i8>()?,
                        c.get(4).unwrap().as_str().parse::<i8>()?,
                    ),
                }),
            }
        }

        if bounds.w % 2 != 1 || bounds.h % 2 != 1 {
            return Err(Error::new("invalid bounds must be odd numbers"));
        }

        Ok(Robots { robots, bounds })
    }

    fn advance_p1(&mut self) {
        let bounds = (Position::at(0, 0), self.bounds);

        for robot in &mut self.robots {
            robot.pos = robot.pos.wrapping_add_direction(robot.vel, bounds);
        }
    }

    fn safety_factor(&mut self) -> usize {
        let mid_x = self.bounds.w / 2;
        let mid_y = self.bounds.h / 2;

        let mut quadrant_count = [0, 0, 0, 0];

        for robot in &self.robots {
            if robot.pos.x < mid_x && robot.pos.y < mid_y {
                quadrant_count[0] += 1;
            } else if robot.pos.x < mid_x && robot.pos.y > mid_y {
                quadrant_count[2] += 1;
            } else if robot.pos.x > mid_x && robot.pos.y < mid_y {
                quadrant_count[1] += 1;
            } else if robot.pos.x > mid_x && robot.pos.y > mid_y {
                quadrant_count[3] += 1;
            }
        }

        self.draw();
        println!("quadrant_count = {:?}", quadrant_count);

        quadrant_count[0] * quadrant_count[1] * quadrant_count[2] * quadrant_count[3]
    }

    fn draw(&self) {
        let mut cg = CharGrid::new(self.bounds.w, self.bounds.h);

        let mid_x = self.bounds.w / 2;
        let mid_y = self.bounds.h / 2;

        for y in 0..cg.height() {
            for x in 0..cg.width() {
                if x == mid_x || y == mid_y {
                    cg.set(x, y, ' ')
                } else {
                    cg.set(x, y, '.')
                }
            }
        }

        for robot in &self.robots {
            let c = match cg.at(robot.pos.x, robot.pos.y) {
                ' ' => ' ',
                '.' => '1',
                '1' => '2',
                '2' => '3',
                '3' => '4',
                '4' => '5',
                _ => 'n',
            };

            cg.set(robot.pos.x, robot.pos.y, c)
        }

        println!("{}", cg.draw())
    }

    fn all_unique_positions(&self) -> bool {
        let mut hs = HashSet::new();

        for robot in &self.robots {
            if hs.contains(&robot.pos) {
                return false;
            } else {
                hs.insert(robot.pos);
            }
        }

        true
    }
}

fn part1(path: &str, bounds: Dimensions) -> Result<usize> {
    let mut robots = Robots::from_str(&read_to_string(path)?, bounds)?;

    for _ in 0..100 {
        robots.advance_p1();
    }

    Ok(robots.safety_factor())
}

fn part2(path: &str, bounds: Dimensions) -> Result<usize> {
    let mut robots = Robots::from_str(&read_to_string(path)?, bounds)?;

    for i in 0..10000 {
        robots.advance_p1();
        if robots.all_unique_positions() {
            robots.draw();
            println!("unique at {} iters", i);
            return Ok(i + 1);
        }
    }

    Err(Error::new("no stable state found after 10000 iters"))
}

#[test]
fn test_part1() {
    assert_eq!(
        12,
        part1("inputs/day14_example.txt", Dimensions::of(11, 7)).unwrap()
    )
}
