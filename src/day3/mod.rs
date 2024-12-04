use crate::shared::*;
use std::fs::read_to_string;

pub struct Day3 {}

impl Solution for Day3 {
    fn part1(&self) -> Result<String> {
        compute("inputs/day3.txt").map(|v| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        compute_part2("inputs/day3.txt").map(|v| v.to_string())
    }
}

fn compute(path: &str) -> Result<i32> {
    let r = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let content = read_to_string(path)?;

    let mut sum = 0;
    for (_, [lh, rh]) in r.captures_iter(&content).map(|c| c.extract()) {
        sum += lh.parse::<i32>()? * rh.parse::<i32>()?;
    }

    Ok(sum)
}

fn compute_part2(path: &str) -> Result<i32> {
    let r = regex::Regex::new(r"(mul|do|don't)\(((\d{1,3}),(\d{1,3}))?\)").unwrap();
    let content = read_to_string(path)?;

    let mut sum = 0;
    let mut enabled = true;
    for captures in r.captures_iter(&content) {
        println!("captures={:?}", captures);
        match captures.get(1).unwrap().as_str() {
            "mul" if enabled => {
                let lh = captures.get(3).unwrap().as_str();
                let rh = captures.get(4).unwrap().as_str();

                sum += lh.parse::<i32>()? * rh.parse::<i32>()?;
            }
            "mul" if !enabled => {}
            "do" => enabled = true,
            "don't" => enabled = false,
            v => panic!("unexpected func {}", v),
        }
    }

    Ok(sum)
}

#[test]
fn test_part1() {
    assert_eq!(
        161,
        compute("inputs/day3_example.txt").expect("compute failed")
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        48,
        compute_part2("inputs/day3_example2.txt").expect("compute failed")
    )
}
