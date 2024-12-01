use crate::shared::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day1 {}

impl Solution for Day1 {
    fn part1(&self) -> Result<String> {
        let lists = Lists::load("inputs/day1.txt")?.into_sorted();

        Ok(list_distance(&lists)?.to_string())
    }

    fn part2(&self) -> Result<String> {
        let lists = Lists::load("inputs/day1.txt")?;

        Ok(calculate_similarity_score(&lists)?.to_string())
    }
}

fn list_distance(lists: &SortedLists) -> Result<i32> {
    if lists.left.len() != lists.right.len() {
        return Err(Error::new(&format!(
            "expected list lengths to be equal, left={}, right={}",
            lists.left.len(),
            lists.right.len()
        )));
    }

    let mut sum = 0;
    for i in 0..lists.left.len() {
        let distance = (lists.right[i] - lists.left[i]).abs();
        sum += distance;
    }

    Ok(sum)
}

fn calculate_similarity_score(lists: &Lists) -> Result<i32> {
    let mut occurance_map: HashMap<i32, usize> = HashMap::new();
    for num in &lists.right {
        if let Some(count) = occurance_map.get(&num) {
            occurance_map.insert(*num, count + 1);
        } else {
            occurance_map.insert(*num, 1);
        }
    }

    let mut sum = 0;
    for v in &lists.left {
        sum += *v * occurance_map.get(v).unwrap_or(&(0usize)).to_owned() as i32
    }

    Ok(sum)
}

struct Lists {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Lists {
    fn load(path: &str) -> Result<Lists> {
        let mut left = Vec::new();
        let mut right = Vec::new();

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            let line = line?;

            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(Error::new(&format!(
                    "expected two fields in line, found {} on line {}",
                    parts.len(),
                    i
                )));
            }

            left.push(parts[0].parse::<i32>()?);
            right.push(parts[1].parse::<i32>()?);
        }

        Ok(Lists { left, right })
    }

    fn into_sorted(mut self) -> SortedLists {
        self.left.sort();
        self.right.sort();

        SortedLists {
            left: self.left,
            right: self.right,
        }
    }
}

struct SortedLists {
    left: Vec<i32>,
    right: Vec<i32>,
}

#[test]
fn test_day1_part1() {
    let r = list_distance(
        &Lists::load("inputs/day1_example.txt")
            .expect("failed to load example lists")
            .into_sorted(),
    )
    .expect("failed to calculate distance");
    assert_eq!(11, r);
}

#[test]
fn test_day1_part2() {
    let lists = Lists::load("inputs/day1_example.txt").expect("failed to load example lists");

    let score = calculate_similarity_score(&lists).expect("failed to calculate similarity score");

    assert_eq!(31, score)
}
