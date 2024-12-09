use crate::shared::*;

pub struct Day4;

impl Solution for Day4 {
    fn part1(&self) -> Result<String> {
        find_xmas("inputs/day4.txt").map(|v| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        find_x_mas("inputs/day4.txt").map(|v| v.to_string())
    }
}

fn find_xmas(path: &str) -> Result<usize> {
    let mut ct: usize = 0;
    let cg = CharGrid::from_file(path)?;
    let needle = vec!['X', 'M', 'A', 'S'];

    fn count_matches_in_str(dir_name: &str, hay: &[char], needle: &[char]) -> usize {
        let d = hay.windows(needle.len()).filter(|b| *b == needle).count();
        println!("{dir_name} : {:?} = {}", hay, d);
        d
    }

    // We take four passes generating strings, and search both directions of those strings.

    // Right, starting from all leftmost positions.
    for y in 0..=cg.y_max() {
        let mut word = cg.line_direction(0, y, RIGHT);
        ct += count_matches_in_str("right        ", &word, &needle);

        word.reverse();
        ct += count_matches_in_str("left         ", &word, &needle);
    }

    // Down, starting from all topmost positions.
    for x in 0..=cg.x_max() {
        let mut word = cg.line_direction(x, 0, DOWN);
        ct += count_matches_in_str("down         ", &word, &needle);

        word.reverse();
        ct += count_matches_in_str("up           ", &word, &needle);
    }

    // Down-Right, starting from all leftmost positions, and all topmost positions.
    let mut pos = Vec::new();
    for x in 0..=cg.x_max() {
        pos.push((x, 0));
    }
    for y in 1..=cg.y_max() {
        // Already got (0, 0).
        pos.push((0, y));
    }

    for (x, y) in pos.into_iter() {
        let mut word = cg.line_direction(x, y, DOWN + RIGHT);
        ct += count_matches_in_str("down-right   ", &word, &needle);

        word.reverse();
        ct += count_matches_in_str("up-left      ", &word, &needle);
    }

    // Down-Left, starting from all rightmost positions, and all topmost positions.
    let mut pos = Vec::new();
    for y in 0..=cg.y_max() {
        pos.push((cg.x_max(), y));
    }
    for x in 0..=cg.x_max() - 1 {
        // Already have (x_max, 0)
        pos.push((x, 0));
    }

    for (x, y) in pos.into_iter() {
        let mut word = cg.line_direction(x, y, DOWN + LEFT);
        ct += count_matches_in_str("down-left    ", &word, &needle);

        word.reverse();
        ct += count_matches_in_str("up-right     ", &word, &needle);
    }

    Ok(ct)
}

fn find_x_mas(path: &str) -> Result<usize> {
    let cg = CharGrid::from_file(path)?;

    fn is_x_mas(view: &CharGridView) -> bool {
        if view.at(1, 1) != 'A' {
            return false;
        }

        let top_left = view.at(0, 0);
        let top_right = view.at(2, 0);
        let bot_left = view.at(0, 2);
        let bot_right = view.at(2, 2);

        (top_left == 'M' || top_left == 'S')
            && (top_right == 'M' || top_right == 'S')
            && (bot_left == 'M' || bot_left == 'S')
            && (bot_right == 'M' || bot_right == 'S')
            && bot_right != top_left
            && bot_left != top_right
    }

    Ok(cg.windows(3, 3).filter(is_x_mas).count())
}

#[test]
fn test_day1() {
    let ct = find_xmas("inputs/day4_example.txt").expect("find_xmas failed");

    assert_eq!(18, ct);
}

#[test]
fn test_day1_part2() {
    let ct = find_x_mas("inputs/day4_example.txt").expect("find_x_mas failed");

    assert_eq!(9, ct);
}
