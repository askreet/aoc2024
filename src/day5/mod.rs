use crate::shared::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

pub struct Day5;

impl Solution for Day5 {
    fn part1(&self) -> Result<String> {
        score_part1("inputs/day5.txt").map(|v| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        score_part2("inputs/day5.txt").map(|v| v.to_string())
    }
}

#[derive(Debug)]
struct Rule {
    before: u8,
    after: u8,
}

impl Rule {
    fn from_str(input: &str) -> Result<Rule> {
        let items: Vec<_> = input.trim().split("|").collect();
        if items.len() != 2 {
            return Err(Error::new("rule does not contain two items"));
        }

        Ok(Rule {
            before: items[0].parse::<u8>()?,
            after: items[1].parse::<u8>()?,
        })
    }
}

struct Rules {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Update {
    ns: Vec<u8>,

    // An occurrence map of the position of each page number in the list, used to validate rules.
    pos: [Option<u8>; 100],
}

impl Update {
    fn from_str(input: &str) -> Result<Update> {
        let mut result = Update {
            ns: input
                .trim()
                .split(",")
                .map(|v| v.parse::<u8>())
                .collect::<std::result::Result<Vec<u8>, ParseIntError>>()?,
            pos: [None; 100],
        };
        if result.ns.len() % 2 != 1 {
            return Err(Error::new("page list contains even numbers of items"));
        }

        result.rebuild_pos();

        Ok(result)
    }

    fn rebuild_pos(&mut self) {
        self.pos = [None; 100];

        for (idx, n) in self.ns.iter().enumerate() {
            debug_assert!(idx <= 255);
            debug_assert!(*n <= 99);

            self.pos[*n as usize] = Some(idx as u8);
        }
    }

    fn midpoint(&self) -> u8 {
        self.ns[self.ns.len() / 2]
    }

    fn check_rules(&self, rules: &Rules) -> Result<()> {
        for rule in &rules.rules {
            if !self.check_rule(rule) {
                return Err(Error::new(&format!(
                    "rule {:?} violated in update {:?}",
                    rule, self
                )));
            }
        }

        Ok(())
    }

    fn check_rule(&self, rule: &Rule) -> bool {
        let before = self.pos[rule.before as usize];
        let after = self.pos[rule.after as usize];

        if let (Some(before), Some(after)) = (before, after) {
            if before > after {
                return false;
            }
        }

        true
    }

    fn correct_for(&mut self, rules: &Rules) {
        loop {
            let mut n_updates = 0;
            for rule in &rules.rules {
                if !self.check_rule(rule) {
                    let before_idx = self.pos[rule.before as usize].unwrap() as usize;
                    let after_idx = self.pos[rule.after as usize].unwrap() as usize;

                    // move before to before after
                    let temp = self.ns[before_idx];
                    for i in (after_idx..before_idx).rev() {
                        self.ns[i + 1] = self.ns[i]; // shift right
                    }
                    self.ns[after_idx] = temp;

                    self.rebuild_pos();
                    n_updates += 1;
                }
            }

            if n_updates == 0 {
                return;
            }
        }
    }
}

fn parse_file(path: &str) -> Result<(Rules, Vec<Update>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut rules = Rules { rules: Vec::new() };
    let mut updates = Vec::new();

    enum Mode {
        Rules,
        Updates,
    }
    let mut mode = Mode::Rules;
    for line in reader.lines() {
        let line = line?;

        if line.trim() == "" {
            mode = Mode::Updates;
            continue;
        }

        match mode {
            Mode::Rules => rules.rules.push(Rule::from_str(&line)?),
            Mode::Updates => updates.push(Update::from_str(&line)?),
        }
    }

    Ok((rules, updates))
}

fn score_part1(path: &str) -> Result<usize> {
    let (rules, updates) = parse_file(path)?;

    let mut sum: usize = 0;
    for update in updates {
        if update.check_rules(&rules).is_ok() {
            sum += update.midpoint() as usize;
        }
    }

    Ok(sum)
}

fn score_part2(path: &str) -> Result<usize> {
    let (rules, updates) = parse_file(path)?;

    let mut failed_updates: Vec<_> = updates
        .into_iter()
        .filter(|u| u.check_rules(&rules).is_err())
        .collect();

    let mut sum: usize = 0;
    for update in &mut failed_updates {
        update.correct_for(&rules);
        sum += update.midpoint() as usize;
    }

    Ok(sum)
}

#[test]
fn test_part1() {
    assert_eq!(143, score_part1("inputs/day5_example.txt").unwrap());
}

#[test]
fn test_correct_update() {
    let (rules, _) = parse_file("inputs/day5_example.txt").unwrap();

    let mut update = Update::from_str("75,97,47,61,53").unwrap();
    update.correct_for(&rules);

    assert_eq!(vec![97, 75, 47, 61, 53], update.ns);
}

#[test]
fn test_part2() {
    assert_eq!(123, score_part2("inputs/day5_example.txt").unwrap());
}
