use crate::shared::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day2 {}

impl Solution for Day2 {
    fn part1(&self) -> Result<String> {
        let rl = ReportList::load("inputs/day2.txt")?;

        Ok(rl.count_safe().to_string())
    }

    fn part2(&self) -> Result<String> {
        let rl = ReportList::load("inputs/day2.txt")?;

        Ok(rl.count_safe_with_problem_dampener().to_string())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Safety {
    Safe,
    Unsafe,
}

#[derive(Debug)]
struct Report {
    ns: Vec<i32>,
}

impl Report {
    fn dampened_candidates(&self) -> impl Iterator<Item = Report> {
        let mut candidates: Vec<Report> = Vec::new();

        for idx in 0..self.ns.len() {
            let mut candidate = Vec::new();
            candidate.extend_from_slice(&self.ns[0..idx]);
            candidate.extend_from_slice(&self.ns[idx + 1..self.ns.len()]);
            candidates.push(Report { ns: candidate });
        }

        candidates.into_iter()
    }
}

fn assess_report_safety(report: &Report) -> Safety {
    let deltas: Vec<_> = report.ns.windows(2).map(|vs| vs[0] - vs[1]).collect();

    let n_positive = deltas.clone().into_iter().filter(|v| *v > 0).count();

    let mut result = Safety::Safe;

    // We are either trending only downward, or only upward.
    if n_positive != 0 && n_positive != deltas.len() {
        result = Safety::Unsafe;
    }

    if deltas.iter().any(|v| *v > 3 || *v < -3 || *v == 0) {
        result = Safety::Unsafe;
    }

    println!(
        "{:?} (deltas={:?}, n_pos={}) = {:?}",
        report.ns, deltas, n_positive, result
    );

    result
}

struct ReportList {
    reports: Vec<Report>,
}

impl ReportList {
    fn load(path: &str) -> Result<Self> {
        let mut reports: Vec<Report> = Vec::new();

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;

            let mut ns: Vec<i32> = Vec::new();
            for str in line.split_whitespace() {
                ns.push(str.parse::<i32>()?);
            }

            reports.push(Report { ns });
        }

        Ok(ReportList { reports })
    }

    fn count_safe(&self) -> usize {
        let mut ct = 0;

        for r in &self.reports {
            if assess_report_safety(r) == Safety::Safe {
                ct += 1;
            }
        }

        ct
    }

    fn count_safe_with_problem_dampener(&self) -> usize {
        let mut ct = 0;

        for r in &self.reports {
            if assess_report_safety(r) == Safety::Safe {
                ct += 1;
            } else {
                if r.dampened_candidates()
                    .any(|c| assess_report_safety(&c) == Safety::Safe)
                {
                    ct += 1;
                }
            }
        }

        ct
    }
}

#[test]
fn test_part_1_assess_report_safety() {
    use Safety::*;

    let tests = vec![
        (Safe, vec![7, 6, 4, 2, 1]),
        (Unsafe, vec![1, 2, 7, 8, 9]),
        (Unsafe, vec![9, 7, 6, 2, 1]),
        (Unsafe, vec![1, 3, 2, 4, 5]),
        (Unsafe, vec![8, 6, 4, 4, 1]),
        (Safe, vec![1, 3, 6, 7, 9]),
    ];

    for (expected, input) in tests {
        assert_eq!(expected, assess_report_safety(&Report { ns: input }))
    }
}

#[test]
fn test_part_1() {
    let rl = ReportList::load("inputs/day2_example.txt").expect("failed to load reports");

    assert_eq!(2, rl.count_safe());
}

#[test]
fn test_part_2() {
    let rl = ReportList::load("inputs/day2_example.txt").expect("failed to load reports");

    assert_eq!(4, rl.count_safe_with_problem_dampener());
}
