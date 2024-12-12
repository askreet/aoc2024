use crate::shared::*;
use std::fs::read_to_string;

pub struct Day9;

impl Solution for Day9 {
    fn part1(&self) -> Result<String> {
        let mut disk = Disk::from_file("inputs/day9.txt")?;
        disk.fragment();

        Ok(disk.checksum().to_string())
    }

    fn part2(&self) -> Result<String> {
        let mut disk = Disk::from_file("inputs/day9.txt")?;
        disk.compactify();

        Ok(disk.checksum().to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FileID(u32);

#[derive(Debug)]
struct Disk {
    blocks: Vec<Option<FileID>>,
}

impl Disk {
    fn from_file(path: &str) -> Result<Disk> {
        Disk::from_map(&read_to_string(path)?)
    }

    fn from_map(map: &str) -> Result<Disk> {
        let mut blocks = Vec::new();

        let mut file_id = 0;
        for (idx, c) in map.chars().enumerate() {
            let len = match c.to_digit(10) {
                Some(d) => d,
                None => return Err(Error::new("encountered non-digit in Disk::from_map")),
            };

            if idx % 2 == 1 {
                for _ in 0..len {
                    blocks.push(None);
                }
            } else {
                for _ in 0..len {
                    blocks.push(Some(FileID(file_id)));
                }
                file_id += 1;
            }
        }

        Ok(Disk { blocks })
    }

    fn fragment(&mut self) {
        let mut free_block = 0;
        let mut move_block = self.blocks.len() - 1;

        loop {
            while self.blocks[free_block].is_some() {
                free_block += 1;
                if free_block == self.blocks.len() {
                    return;
                }
            }

            while self.blocks[move_block].is_none() {
                move_block -= 1;
                if move_block <= free_block {
                    return;
                }
            }

            self.blocks[free_block] = self.blocks[move_block];
            self.blocks[move_block] = None;
        }
    }

    fn compactify(&mut self) {
        let mut move_start = self.blocks.len() - 1;
        let mut move_len = 0;

        #[derive(Copy, Clone, Debug)]
        struct Gap {
            start: usize,
            len: usize,
        }
        let mut gaps: Vec<Gap> = Vec::new();

        let mut start = 0;
        while start < self.blocks.len() {
            if self.blocks[start].is_none() {
                let end = self.blocks[start..self.blocks.len()]
                    .iter()
                    .enumerate()
                    .find(|(_, b)| b.is_some());

                let len = match end {
                    None => return, // space continues to end of disk
                    Some((idx, _)) => idx,
                };

                gaps.push(Gap { start, len });
                start += len;
            } else {
                start += 1;
            }
        }

        loop {
            while self.blocks[move_start].is_none() {
                move_start -= 1;
            }

            move_len = 1;
            let file_id = self.blocks[move_start].unwrap();
            while self.blocks[move_start - 1] == Some(file_id) {
                if move_start == 1 {
                    return;
                }
                move_start -= 1;
                move_len += 1;
            }

            let mut gap = None;
            for (idx, g) in gaps.iter().enumerate() {
                if g.start > move_start {
                    break;
                }

                if g.len >= move_len {
                    gap = Some((idx, g));
                    break;
                }
            }

            match gap {
                None => {
                    // try next file
                    move_start -= 1;
                    continue;
                }

                Some((idx, Gap { start, len })) => {
                    #[cfg(test)]
                    let before = disk_string(&self);

                    for i in 0..move_len {
                        self.blocks[start + i] = self.blocks[move_start + i];
                        self.blocks[move_start + i] = None;
                    }

                    gaps[idx] = Gap {
                        start: start + move_len,
                        len: len - move_len, // this may be zero, that's ok.
                    };

                    #[cfg(test)]
                    println!("{} -> {}", before, disk_string(&self));
                }
            }
        }
    }

    fn checksum(&self) -> usize {
        let mut sum = 0;

        for (i, file_id) in self.blocks.iter().enumerate() {
            if let Some(FileID(id)) = file_id {
                sum += i * *id as usize;
            }
        }

        sum
    }
}

#[test]
fn test_from_map() {
    let disk = Disk::from_map("12345").unwrap();

    assert_eq!("0..111....22222", disk_string(&disk));
}

#[test]
fn test_part1() {
    let mut disk = Disk::from_file("inputs/day9_example.txt").unwrap();
    assert_eq!(
        "00...111...2...333.44.5555.6666.777.888899",
        disk_string(&disk)
    );

    disk.fragment();
    assert_eq!(
        "0099811188827773336446555566..............",
        disk_string(&disk)
    );

    assert_eq!(1928, disk.checksum());
}

#[test]
fn test_part2() {
    let mut disk = Disk::from_file("inputs/day9_example.txt").unwrap();

    assert_eq!(
        "00...111...2...333.44.5555.6666.777.888899",
        disk_string(&disk)
    );

    disk.compactify();
    assert_eq!(
        "00992111777.44.333....5555.6666.....8888..",
        disk_string(&disk)
    );

    assert_eq!(2858, disk.checksum());
}

#[cfg(test)]
fn disk_string(d: &Disk) -> String {
    let mut disk_string = String::new();

    for block in &d.blocks {
        match block {
            Some(FileID(id)) => disk_string.push_str(&format!("{}", id)),
            None => disk_string.push('.'),
        }
    }

    disk_string
}
