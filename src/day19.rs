use crate::shared::*;
use std::fs::read_to_string;

pub struct Day19;

impl Solution for Day19 {
    fn part1(&self) -> Result<String> {
        part1("inputs/day19.txt").map(|v| v.to_string())
    }

    fn part2(&self) -> Result<String> {
        part2("inputs/day19.txt").map(|v| v.to_string())
    }
}

type Towels = Vec<String>;
macro_rules! towels {
    ($($item:expr),* $(,)?) => {
        vec![$($item.to_string()),*]
    };
}

fn possible_towel_paths(goal: &str, towels: &Towels) -> usize {
    let mut arrivals = vec![0; goal.len() + 1];
    arrivals[0] = 1;

    for i in 0..goal.len() {
        for towel in towels {
            if goal[i..].starts_with(towel) {
                if i + towel.len() < arrivals.len() {
                    arrivals[i + towel.len()] += arrivals[i];
                }
            }
        }
    }

    arrivals[arrivals.len() - 1]
}

fn part1(path: &str) -> Result<usize> {
    let contents = read_to_string(path)?;
    let docs = split_docs(contents);
    debug_assert!(docs.len() == 2);

    let towels: Towels = docs[0].split(", ").map(|v| v.trim().to_owned()).collect();

    let mut count = 0;

    for goal in docs[1].lines() {
        if possible_towel_paths(goal, &towels) > 0 {
            count += 1;
        }
    }

    Ok(count)
}

#[test]
fn test_part1() {
    assert_eq!(6, part1("inputs/day19_example.txt").unwrap());
}

fn part2(path: &str) -> Result<usize> {
    let contents = read_to_string(path)?;
    let docs = split_docs(contents);
    debug_assert!(docs.len() == 2);

    let towels: Towels = docs[0].split(", ").map(|v| v.trim().to_owned()).collect();

    let mut count = 0;

    for goal in docs[1].lines() {
        count += possible_towel_paths(goal, &towels);
    }

    Ok(count)
}

#[test]
fn test_part2() {
    assert_eq!(16, part2("inputs/day19_example.txt").unwrap());
}

#[test]
fn test_slow() {
    let goal = "bgrurgbbwrbwgbggrruuguwwugbgwurbrbgbgrguuuurrwrwwgggr";
    let towels = towels![
        "uug", "wwg", "uuu", "grburwrb", "uubrw", "bw", "wwbb", "bugg", "wb", "uwub", "ubb",
        "ubruug", "bgr", "gbw", "ruw", "brw", "ubu", "bwgb", "bwbbgg", "rbu", "rbww", "uwwgubg",
        "urggw", "urgg", "brubwurr", "wrrbw", "wggbbr", "rbbrwg", "uubb", "rbuguu", "rwugwr",
        "ubugw", "buwuuuu", "uuru", "wbbrb", "gbg", "ug", "wwr", "grbwgw", "ugu", "brwwu", "www",
        "bgwguuub", "gur", "ugbgubrg", "ururgu", "ruru", "uuug", "urw", "uwg", "buwuguwr", "ugrw",
        "rugrrr", "wwgr", "gbbug", "wrbb", "gwrgw", "bgw", "wbuur", "wurbrr", "rg", "bbubgur",
        "bwb", "uwwggg", "rugwwg", "rggwu", "wugwbggg", "rwrw", "rrgrw", "urwg", "rgw", "urg",
        "gbuu", "gb", "uwbwbr", "ruu", "bugrwbu", "bwuu", "wug", "uwrugrw", "uuww", "br", "ubwub",
        "grbwu", "brruur", "wuubr", "bbr", "wrrubu", "rwuugbg", "grbwbg", "guu", "brg", "wgwub",
        "gwr", "wgu", "ggbrbb", "bwub", "wgg", "wbu", "wwb", "bwrgu", "uwgr", "grgw", "uurgbg",
        "wrwbbg", "ubub", "bb", "ruuw", "uwrbugu", "gwrb", "bwr", "rguwuw", "rrbr", "rubbgr",
        "wrggu", "gbwrbb", "uugguug", "gbbb", "rwugg", "wwbubr", "rrurgbb", "bwrwugr", "ugg",
        "brbuubb", "bwbwrug", "uwr", "wuggwu", "wuwww", "bwbgrwbw", "bubur", "ugrb", "grub",
        "grgggb", "uwru", "wgr", "gwwrg", "wubu", "bgu", "bgrg", "wuw", "guwu", "rbuwb", "uuwgr",
        "bggrurg", "wrurbb", "rgrgrbg", "wbwr", "uuwwub", "ggwr", "bbwg", "rbg", "wur", "urgbg",
        "wbr", "uwwwrug", "rurw", "wwbgwub", "urugwbw", "g", "uubg", "ur", "bwur", "uurwruub",
        "uw", "uguuur", "rub", "wgb", "wrrwuu", "uwug", "rww", "wrrwuw", "wbw", "brr", "bwrugg",
        "ubgbbrr", "gwg", "uggbwb", "uugbb", "uguwubuw", "wguwug", "bwg", "ggb", "wgbur", "urr",
        "bbuwg", "burur", "grrb", "gubru", "ubrrugr", "ubg", "rbwgu", "uwgbw", "wgubw", "gwgrrw",
        "rrr", "wbwu", "ru", "guwgg", "rgr", "brurg", "rr", "gw", "wurbwgw", "rrw", "ubbr", "wgub",
        "wrrwu", "gwguwrbw", "uurb", "rrg", "bbgwu", "uwwb", "uggugw", "rubbrgur", "gwu", "wrgwwg",
        "gubbg", "wggw", "uuwr", "ggru", "wuu", "bbu", "bruwu", "rgbrrwwr", "wuurgbgg", "ubugwgg",
        "ubgr", "rguwu", "ruug", "bwruubg", "grgwrw", "bgrw", "uwrwg", "gbbg", "guuuu", "bgwr",
        "wwwuw", "rgrw", "rwr", "wrurgr", "rw", "wwu", "wbbwwrb", "rgbwr", "bgbwg", "wr", "wgugr",
        "grw", "gru", "wurubug", "wgbu", "uuuwuug", "rbgw", "wuwur", "bbb", "ugwr", "ubuwub",
        "rbwb", "grwgrbg", "bg", "bwww", "guub", "ugw", "uub", "ugbw", "rwub", "bggwuuu", "bgwwg",
        "uru", "uggbrw", "bggwg", "gubgwubu", "bguggu", "uwuuub", "wwwb", "ugb", "rbb", "uwwg",
        "bbw", "bbruwr", "rur", "buw", "wbbr", "ubrgugrb", "grurg", "ggugr", "gugwr", "ubuwu",
        "rggrg", "wbrb", "bwrr", "wbg", "ww", "gwub", "grgubu", "brubu", "gbwg", "brb", "rurug",
        "buwu", "gbbbwb", "bbgbw", "rug", "uwrb", "gggw", "ubrgbu", "bru", "gww", "bur",
        "gbggrrwr", "uuwuu", "bgbgw", "wgbr", "grr", "gwgwrbr", "gbb", "gbr", "bbbugr", "uuw",
        "wrg", "gub", "gg", "rubgggg", "rugg", "urur", "rwbu", "bwu", "bwbuw", "rgbgb", "bwgr",
        "rgwuw", "wbruw", "ubr", "wrb", "rbrbb", "bu", "uwu", "gbwwrr", "buuguru", "uggrw",
        "wbuurur", "wgrw", "wrw", "wgbggrru", "uubburwb", "bgbr", "rbr", "gug", "wrr", "uuubwgr",
        "buu", "uu", "wububwg", "ugwruu", "rru", "wgrrbgg", "gguw", "ruub", "rgu", "ggw", "rrwu",
        "rwggr", "rwwbwu", "w", "uuguw", "wwgbu", "brggw", "rbwgru", "guw", "wub", "gwuggur",
        "grrw", "wg", "ubbguu", "wgggb", "uwb", "uwwu", "wrub", "uwwgw", "rubwbrr", "wgbrg", "bgb",
        "bbwb", "brgb", "rwu", "uur", "bbruurrw", "u", "gbu", "urb", "wuwbggg", "rwwrbr", "gwb",
        "grgrburw", "bwrg", "wbb", "wuurw", "bugbuug", "uggrb", "gubbgbb", "bww", "bbbw", "gbwwr",
        "wruw", "rb", "gguww", "ugww", "wgw", "rgrwwg", "gwbwr", "brrw", "bbrubu", "rgb", "rbwu",
        "rrb", "gwugug", "rbgb", "bgwg", "gu", "wwugguw", "rgg", "wbugrr", "ubw", "rbrru", "grb",
        "wrurrw", "ggg", "wbwg", "rruu", "ugr", "bgg", "ub", "ggu", "bbbr", "rbw", "bug", "gggwbu",
        "bbuu", "urrb", "gruww", "b", "uww", "brbuw", "urbubg", "gugwg", "bub", "grg"
    ];

    assert_eq!(0, possible_towel_paths(goal, &towels));
}
