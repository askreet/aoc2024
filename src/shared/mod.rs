mod char_grid;
pub use char_grid::*;

use std::fmt::Formatter;
use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, Error>;

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
        return Error {
            msg: format!("failed to parse int: {}", value.to_string()),
        };
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        return Error {
            msg: format!("i/o error: {}", value.to_string()),
        };
    }
}

pub trait Solution {
    fn part1(&self) -> Result<String>;
    fn part2(&self) -> Result<String>;
}

pub struct Direction(i8, i8);
pub const LEFT: Direction = Direction(-1, 0);
pub const RIGHT: Direction = Direction(1, 0);
pub const UP: Direction = Direction(0, -1);
pub const DOWN: Direction = Direction(0, 1);

impl std::ops::Add for Direction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(rhs.0 + self.0, rhs.1 + self.1)
    }
}