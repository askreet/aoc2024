mod shared;

use std::collections::HashMap;
use shared::*;

mod day1;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err(Error::new("usage: aoc2024 <day#> <part#>"));
    }

    let day = args[1].parse::<u32>()?;
    let part = args[2].parse::<u32>()?;

    let mut days: HashMap<u32, Box<dyn Solution>> = HashMap::new();
    days.insert(1, Box::new(day1::Day1 {}));

    if let Some(solution) = days.get(&day) {
        let result = if part == 1 {
            solution.part1()
        } else if part == 2 {
            solution.part2()
        } else {
            return Err(Error::new("invalid part number"))
        };

        match result {
            Ok(v) => println!("result: {}", v),
            Err(e) => println!("error: {}", e),
        }
    } else {
        return Err(Error::new(&format!("day {day} not found")))
    }

    Ok(())
}
