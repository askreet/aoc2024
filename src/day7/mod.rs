use crate::shared::*;
use std::fs::read_to_string;
use std::num::ParseIntError;

pub struct Day7;

impl Solution for Day7 {
    fn part1(&self) -> Result<String> {
        part1("inputs/day7.txt", &[Op::Add, Op::Mul]).map(|v| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        part1("inputs/day7.txt", &[Op::Add, Op::Mul, Op::Concat]).map(|v| v.to_string())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Op {
    Add,
    Mul,
    Concat,
}

fn is_solvable(solution: u64, terms: &[u64], ops: &[Op]) -> bool {
    fn calculate(terms: &[u64], ops: &[Op]) -> u64 {
        debug_assert!(terms.len() == ops.len() + 1);
        debug_assert!(terms.len() > 1);

        let mut result = terms[0];
        let steps = terms[1..terms.len()].iter().zip(ops);

        for (v, op) in steps {
            match op {
                Op::Add => result += *v,
                Op::Mul => result *= *v,
                Op::Concat => {
                    result = format!("{}{}", result, v)
                        .parse::<u64>()
                        .expect("failed to parse concatination")
                }
            }
        }

        result
    }

    for ops in op_permutations(terms.len() - 1, ops) {
        if calculate(terms, &ops) == solution {
            return true;
        }
    }

    false
}

#[test]
fn test_is_solvable() {
    let ops = &[Op::Add, Op::Mul];

    assert_eq!(true, is_solvable(190, &[10, 19], ops));
    assert_eq!(true, is_solvable(3267, &[81, 40, 27], ops));
    assert_eq!(false, is_solvable(83, &[17, 5], ops));
    assert_eq!(false, is_solvable(156, &[15, 6], ops));
    assert_eq!(false, is_solvable(7290, &[6, 8, 6, 15], ops));
    assert_eq!(false, is_solvable(161011, &[16, 10, 13], ops));
    assert_eq!(false, is_solvable(192, &[17, 8, 14,], ops));
    assert_eq!(false, is_solvable(21037, &[9, 7, 18, 13], ops));
    assert_eq!(true, is_solvable(292, &[11, 6, 16, 20], ops));
}

fn op_permutations(n: usize, ops: &[Op]) -> Vec<Vec<Op>> {
    fn add_op(v: &Vec<Op>, op: Op) -> Vec<Op> {
        let mut v = v.clone();
        v.push(op);
        v
    }

    match n {
        0 => panic!("invalid op permutations value"),
        1 => ops.iter().map(|op| vec![*op]).collect(),
        n => op_permutations(n - 1, ops)
            .into_iter()
            .flat_map(|vs| {
                ops.iter()
                    .map(|op| add_op(&vs, *op))
                    .collect::<Vec<Vec<Op>>>()
            })
            .collect(),
    }
}

fn part1(path: &str, ops: &[Op]) -> Result<u64> {
    let contents = read_to_string(path)?;

    let mut sum = 0;
    for line in contents.lines() {
        let parts = line.split(": ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(Error::new("unexpected number of parts in input line"));
        }

        let solution = parts[0].parse::<u64>()?;
        let terms = parts[1]
            .split_whitespace()
            .map(|v| v.parse::<u64>())
            .collect::<std::result::Result<Vec<_>, ParseIntError>>()?;

        if is_solvable(solution, &terms, ops) {
            sum += solution;
        }
    }

    Ok(sum)
}

#[test]
fn test_part1() {
    assert_eq!(
        3749,
        part1("inputs/day7_example.txt", &[Op::Add, Op::Mul]).unwrap()
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        11387,
        part1("inputs/day7_example.txt", &[Op::Add, Op::Mul, Op::Concat]).unwrap()
    );
}
