use crate::shared::*;
use std::collections::{HashMap, HashSet};

pub struct Day12;

impl Solution for Day12 {
    fn part1(&self) -> Result<String> {
        let cg = CharGrid::from_file("inputs/day12.txt")?;

        Ok(part1(&cg).to_string())
    }

    fn part2(&self) -> Result<String> {
        let cg = CharGrid::from_file("inputs/day12.txt")?;

        Ok(part2(&cg).to_string())
    }
}

type Region = HashSet<Position>;

fn part1(cg: &CharGrid) -> usize {
    let mut visited: HashSet<Position> = HashSet::new();

    let mut sum = 0;
    for y in 0..cg.height() {
        for x in 0..cg.width() {
            let pos = Position { x, y };
            if visited.contains(&pos) {
                continue;
            }

            let region = find_region(&cg, pos);
            let area = region.len();
            let perimeter = calc_perimeter(&region);
            sum += area * perimeter;

            for pos in region {
                visited.insert(pos);
            }
        }
    }

    sum
}

fn part2(cg: &CharGrid) -> usize {
    let mut visited: HashSet<Position> = HashSet::new();

    let mut sum = 0;
    for y in 0..cg.height() {
        for x in 0..cg.width() {
            let pos = Position { x, y };
            if visited.contains(&pos) {
                continue;
            }

            let region = find_region(&cg, pos);
            let area = region.len();
            let sides = calc_sides(&region);
            sum += area * sides;

            let plant_pos = region.iter().take(1).collect::<Vec<_>>()[0];
            let plant = cg.at(plant_pos.x, plant_pos.y);
            println!(
                "found region of {} plants with price {} * {} = {}",
                plant,
                area,
                sides,
                area * sides
            );

            for pos in region {
                visited.insert(pos);
            }
        }
    }

    sum
}

fn find_region(cg: &CharGrid, pos: Position) -> Region {
    let mut region = HashSet::new();
    region.insert(pos);

    let crop = cg.at(pos.x, pos.y);

    let mut search = vec![pos];

    while !search.is_empty() {
        let this = search.pop().unwrap();
        region.insert(this);

        for adj in [this + LEFT, this + UP, this + DOWN, this + RIGHT] {
            if cg.in_bounds(adj)
                && cg.at(adj.x, adj.y) == crop
                && !search.contains(&adj)
                && !region.contains(&adj)
            {
                search.push(adj);
            }
        }
    }

    region
}

fn calc_perimeter(region: &Region) -> usize {
    let mut sum = 0;

    for pos in region {
        for adj in [pos + LEFT, pos + UP, pos + DOWN, pos + RIGHT] {
            if !region.contains(&adj) {
                sum += 1;
            }
        }
    }

    sum
}

fn calc_sides(region: &Region) -> usize {
    let mut sides = 0;

    fn group_by_with_no_adjacent_direction(
        region: &Region,
        key: fn(&Position) -> i32,
        dir: Direction,
    ) -> HashMap<i32, Vec<Position>> {
        let mut grouped: HashMap<i32, Vec<Position>> = HashMap::new();

        for pos in region.iter().filter(|p| !region.contains(&(*p + dir))) {
            grouped.entry(key(pos)).or_insert(vec![]).push(*pos);
        }

        println!("dir={:?}, grouped={:?}", dir, grouped);

        grouped
    }

    fn count_unique_vertical_segments(mut positions: Vec<Position>) -> usize {
        positions.sort_by_key(|p| p.y);

        let gaps = positions
            .windows(2)
            .filter(|ps| ps[0].y + 1 != ps[1].y)
            .count();

        println!("gaps={}", gaps);

        1 + gaps
    }

    fn count_unique_horiz_segments(mut positions: Vec<Position>) -> usize {
        positions.sort_by_key(|p| p.x);

        let gaps = positions
            .windows(2)
            .filter(|ps| ps[0].x + 1 != ps[1].x)
            .count();

        println!("gaps={}", gaps);

        1 + gaps
    }

    for (_, positions) in group_by_with_no_adjacent_direction(&region, |p| p.y, UP) {
        sides += count_unique_horiz_segments(positions);
    }

    for (_, positions) in group_by_with_no_adjacent_direction(&region, |p| p.y, DOWN) {
        sides += count_unique_horiz_segments(positions);
    }

    for (_, positions) in group_by_with_no_adjacent_direction(&region, |p| p.x, LEFT) {
        sides += count_unique_vertical_segments(positions);
    }

    for (_, positions) in group_by_with_no_adjacent_direction(&region, |p| p.x, RIGHT) {
        sides += count_unique_vertical_segments(positions);
    }

    sides
}

#[test]
fn test_calc_sides() {
    let cg = CharGrid::from_str(
        "EEEEE\n\
         E....\n\
         EEEEE\n\
         E....\n\
         EEEEE",
    )
    .unwrap();

    let region = find_region(&cg, Position { x: 0, y: 0 });
    assert_eq!(12, calc_sides(&region));
}

#[test]
fn test_calc_sides_R_example() {
    let cg = CharGrid::from_file("inputs/day12_example.txt").unwrap();

    let region = find_region(&cg, Position { x: 0, y: 0 });
    assert_eq!(12, region.len());
    assert_eq!(10, calc_sides(&region));
}

#[test]
fn test_part1() {
    let cg = CharGrid::from_file("inputs/day12_example.txt").unwrap();

    assert_eq!(1930, part1(&cg));
}

#[test]
fn test_part2() {
    let cg = CharGrid::from_file("inputs/day12_example.txt").unwrap();

    assert_eq!(1206, part2(&cg));
}
