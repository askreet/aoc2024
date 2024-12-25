mod shared;

use shared::*;
use std::collections::HashMap;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err(Error::new("usage: aoc2024 <day#> <part#>"));
    }

    let day = args[1].parse::<u32>()?;
    let part = args[2].parse::<u32>()?;

    let mut days: HashMap<u32, Box<dyn Solution>> = HashMap::new();
    days.insert(1, Box::new(day1::Day1));
    days.insert(2, Box::new(day2::Day2));
    days.insert(3, Box::new(day3::Day3));
    days.insert(4, Box::new(day4::Day4));
    days.insert(5, Box::new(day5::Day5));
    days.insert(6, Box::new(day6::Day6));
    days.insert(7, Box::new(day7::Day7));
    days.insert(8, Box::new(day8::Day8));
    days.insert(9, Box::new(day9::Day9));
    days.insert(10, Box::new(day10::Day10));
    days.insert(11, Box::new(day11::Day11));
    days.insert(12, Box::new(day12::Day12));
    days.insert(13, Box::new(day13::Day13));
    days.insert(14, Box::new(day14::Day14));
    days.insert(15, Box::new(day15::Day15));
    days.insert(16, Box::new(day16::Day16));
    days.insert(17, Box::new(day17::Day17));
    days.insert(18, Box::new(day18::Day18));
    days.insert(19, Box::new(day19::Day19));
    days.insert(20, Box::new(day20::Day20));

    if let Some(solution) = days.get(&day) {
        let result = if part == 1 {
            solution.part1()
        } else if part == 2 {
            solution.part2()
        } else {
            return Err(Error::new("invalid part number"));
        };

        match result {
            Ok(v) => println!("result: {}", v),
            Err(e) => println!("error: {}", e),
        }
    } else {
        return Err(Error::new(&format!("day {day} not found")));
    }

    Ok(())
}
