use crate::shared::*;
use nalgebra::{Matrix2, Vector2, QR};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::read_to_string;

pub struct Day13;

impl Solution for Day13 {
    fn part1(&self) -> Result<String> {
        part1("inputs/day13.txt").map(|v| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        part2("inputs/day13.txt").map(|v| v.to_string())
    }
}

#[derive(Copy, Clone, Debug)]
struct Scenario {
    a_x: usize,
    a_y: usize,

    b_x: usize,
    b_y: usize,

    prize_x: usize,
    prize_y: usize,
}

impl Scenario {
    fn from_str(str: &str) -> Result<Scenario> {
        let rx = regex::Regex::new(
            r"(?m)Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
            .unwrap();
        match rx.captures(str) {
            None => Err(Error::new("failed to parse")),
            Some(c) => Ok(Scenario {
                a_x: c.get(1).unwrap().as_str().parse::<usize>()?,
                a_y: c.get(2).unwrap().as_str().parse::<usize>()?,
                b_x: c.get(3).unwrap().as_str().parse::<usize>()?,
                b_y: c.get(4).unwrap().as_str().parse::<usize>()?,
                prize_x: c.get(5).unwrap().as_str().parse::<usize>()?,
                prize_y: c.get(6).unwrap().as_str().parse::<usize>()?,
            }),
        }
    }

    fn solved_by(&self, p: &Plan) -> bool {
        self.a_x * p.a_count + self.b_x * p.b_count == self.prize_x
            && self.a_y * p.a_count + self.b_y * p.b_count == self.prize_y
    }

    fn exceeded_by(&self, p: &Plan) -> bool {
        self.a_x * p.a_count + self.b_x * p.b_count > self.prize_x
            || self.a_y * p.a_count + self.b_y * p.b_count > self.prize_y
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialEq, Eq)]
struct Plan {
    a_count: usize,
    b_count: usize,
}

impl Plan {
    fn of(a: usize, b: usize) -> Self {
        Plan {
            a_count: a,
            b_count: b,
        }
    }

    fn cost(&self) -> usize {
        (self.a_count * 3) + self.b_count
    }
}

impl PartialOrd for Plan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost().cmp(&other.cost()))
    }
}

fn solve(scenario: Scenario) -> Option<Plan> {
    let mut searched = HashSet::new();
    let mut frontier = vec![(0, 0)];

    let mut cheapest: Option<Plan> = None;

    let mut iter_ct = 0;
    while !frontier.is_empty() {
        iter_ct += 1;

        let (a, b) = frontier.pop().unwrap();
        searched.insert((a, b));
        let attempt = Plan::of(a, b);

        if scenario.solved_by(&attempt) {
            if cheapest.is_none() {
                cheapest = Some(attempt);
            } else if attempt.cost() < cheapest.unwrap().cost() {
                cheapest = Some(attempt);
            }

            continue;
        }

        if scenario.exceeded_by(&attempt) {
            continue;
        }

        for (a, b) in [(a + 1, b), (a, b + 1)] {
            if a <= 100 && b <= 100 && !searched.contains(&(a, b)) {
                frontier.push((a, b));
            }
        }
    }

    println!("found cheapest={:?} after {} iterations", cheapest, iter_ct);

    cheapest
}

fn solve_with_math(s: Scenario) -> Option<Plan> {
    #[rustfmt::skip]
    let terms = Matrix2::<f64>::new(s.a_x as f64, s.b_x as f64, s.a_y as f64, s.b_y as f64);

    let c = QR::new(terms);

    let solution = c.solve(&Vector2::new(s.prize_x as f64, s.prize_y as f64));
    if solution.is_some() {
        println!("{}", &solution.unwrap());
    }

    match solution {
        Some(solution) => {
            let a = solution[0].round();
            let b = solution[1].round();

            println!("a={}, b={}", a, b);
            // determine if our solution is made of integers by confirming the solution with
            // rounded results
            let plan = Plan {
                a_count: a as usize,
                b_count: b as usize,
            };
            match s.solved_by(&plan) {
                true => Some(plan),
                false => None,
            }
        }
        None => None,
    }
}

#[test]
fn test_solve() {
    let scenario = Scenario {
        a_x: 94,
        a_y: 34,
        b_x: 22,
        b_y: 67,
        prize_x: 8400,
        prize_y: 5400,
    };

    assert_eq!(
        Some(Plan {
            a_count: 80,
            b_count: 40
        }),
        solve(scenario)
    );
    assert_eq!(
        Some(Plan {
            a_count: 80,
            b_count: 40
        }),
        solve_with_math(scenario)
    );
}

#[test]
fn test_unsolvable() {
    let scenario = Scenario {
        a_x: 26,
        a_y: 66,
        b_x: 67,
        b_y: 21,
        prize_x: 12748,
        prize_y: 12176,
    };

    assert_eq!(None, solve(scenario));
    assert_eq!(None, solve_with_math(scenario));
}

fn part1(path: &str) -> Result<usize> {
    let mut sum = 0;

    for doc in split_docs(read_to_string(path)?) {
        let scenario = Scenario::from_str(&doc)?;
        dbg!(&scenario);

        if let Some(plan) = solve(scenario) {
            sum += plan.cost();
        }
    }

    Ok(sum)
}

fn part2(path: &str) -> Result<usize> {
    let mut sum = 0;

    for doc in split_docs(read_to_string(path)?) {
        let mut scenario = Scenario::from_str(&doc)?;

        scenario.prize_x += 10000000000000;
        scenario.prize_y += 10000000000000;

        if let Some(plan) = solve_with_math(scenario) {
            sum += plan.cost();
        }
    }

    Ok(sum)
}

#[test]
fn test_part1() {
    assert_eq!(Ok(480), part1("inputs/day13_example.txt"));
}
