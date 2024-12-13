mod char_grid;
pub use char_grid::*;

use std::fmt::Formatter;
use std::num::ParseIntError;
use std::ops::Add;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq)]
pub struct Error {
    pub msg: String,
}

impl Error {
    pub fn new(string: &str) -> Error {
        Error {
            msg: string.to_owned(),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl std::error::Error for Error {}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error {
            msg: format!("failed to parse int: {}", value.to_string()),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            msg: format!("i/o error: {}", value.to_string()),
        }
    }
}

pub trait Solution {
    fn part1(&self) -> Result<String>;
    fn part2(&self) -> Result<String>;
}

#[derive(Debug, Clone, Copy)]
pub struct Direction(i8, i8);
pub const LEFT: Direction = Direction(-1, 0);
pub const RIGHT: Direction = Direction(1, 0);
pub const UP: Direction = Direction(0, -1);
pub const DOWN: Direction = Direction(0, 1);

impl Direction {
    pub fn clockwise(&self) -> Direction {
        match self {
            Direction(0, -1) => RIGHT,
            Direction(1, 0) => DOWN,
            Direction(0, 1) => LEFT,
            Direction(-1, 0) => UP,
            _ => panic!("cannot rotate non-cardinal Direction"),
        }
    }

    pub fn anticlockwise(&self) -> Direction {
        match self {
            Direction(0, -1) => LEFT,
            Direction(-1, 0) => DOWN,
            Direction(0, 1) => RIGHT,
            Direction(1, 0) => UP,
            _ => panic!("cannot rotate non-cardinal Direction"),
        }
    }
}

impl Add for Direction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(rhs.0 + self.0, rhs.1 + self.1)
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Self::Output {
            x: self.x + rhs.0 as i32,
            y: self.y + rhs.1 as i32,
        }
    }
}

impl Add<Direction> for &Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Self::Output {
        *self + rhs
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn add(&mut self, dir: Direction) -> Position {
        Position {
            x: self.x + dir.0 as i32,
            y: self.y + dir.1 as i32,
        }
    }

    pub fn add_pos(&self, pos: Position) -> Position {
        Position {
            x: self.x + pos.x,
            y: self.y + pos.y,
        }
    }
    pub fn sub_pos(&self, pos: Position) -> Position {
        Position {
            x: self.x - pos.x,
            y: self.y - pos.y,
        }
    }

    pub fn delta(&self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub fn permutations<T: Clone>(n: usize, items: &[T]) -> Vec<Vec<T>> {
    fn append<T: Clone>(v: &Vec<T>, op: T) -> Vec<T> {
        let mut v = v.clone();
        v.push(op);
        v
    }

    match n {
        0 => panic!("invalid size for permutations"),
        1 => items.iter().map(|value| vec![value.clone()]).collect(),
        n => permutations(n - 1, items)
            .into_iter()
            .flat_map(|vs| {
                items
                    .iter()
                    .map(|value| append(&vs, value.clone()))
                    .collect::<Vec<Vec<T>>>()
            })
            .collect(),
    }
}
#[test]
fn test_permuatations() {
    assert_eq!(
        vec![
            vec![1, 1, 1],
            vec![1, 1, 2],
            vec![1, 2, 1],
            vec![1, 2, 2],
            vec![2, 1, 1],
            vec![2, 1, 2],
            vec![2, 2, 1],
            vec![2, 2, 2],
        ],
        permutations(3, &[1, 2])
    );
}

pub fn combinations<T: Clone>(n: usize, items: &[T]) -> Vec<Vec<T>> {
    debug_assert!(n <= items.len());

    if n == 1 {
        items.iter().map(|v| vec![v.clone()]).collect()
    } else {
        let mut result = Vec::new();
        for head in 0..=items.len() - n {
            for tail in combinations(n - 1, &items[head + 1..]) {
                let mut combination = vec![items[head].clone()];
                tail.iter().for_each(|v| combination.push(v.clone()));
                result.push(combination);
            }
        }
        result
    }
}

#[test]
fn test_combinations() {
    assert_eq!(
        vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3]
        ],
        combinations(2, &[0, 1, 2, 3])
    );

    assert_eq!(
        vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 5],
            vec![1, 2, 4, 5],
            vec![1, 3, 4, 5],
            vec![2, 3, 4, 5]
        ],
        combinations(4, &[1, 2, 3, 4, 5])
    );
}

#[rustfmt::skip]
pub fn digits(n: usize) -> usize {
                                  if
                                n < 10                     { 1  }
    else if                    n  < 100                    { 2  }
    else if                   n   < 1000                   { 3  }
    else if                  n    < 10000                  { 4  }
    else if                 n     < 100000                 { 5  }
    else if                n      < 1000000                { 6  }
    else if               n       < 10000000               { 7  }
    else if              n        < 100000000              { 8  }
    else if             n         < 1000000000             { 9  }
    else if            n          < 10000000000            { 10 }
    else if           n           < 100000000000           { 11 }
    else if          n            < 1000000000000          { 12 }
    else if         n             < 10000000000000         { 13 }
    else if        n              < 100000000000000        { 14 }
    else if       n               < 1000000000000000       { 15 }
    else if      n                < 10000000000000000      { 16 }
    else if     n                 < 100000000000000000     { 17 }
    else if    n                  < 1000000000000000000    { 18 }
    else if   n                   < 10000000000000000000   { 19 }
    else                        { 20 }
}

pub fn split_docs(s: String) -> Vec<String> {
    let mut buf = String::new();
    let mut docs = Vec::new();

    for line in s.lines() {
        if line != "" {
            buf.push_str(line);
            buf.push('\n');
        } else {
            docs.push(buf.clone());
            buf.clear();
        }
    }
    if buf != "" {
        docs.push(buf);
    }

    docs
}
