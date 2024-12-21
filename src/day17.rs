use crate::shared::*;
use getch_rs::{Getch, Key};
use std::time::Instant;

pub struct Day17;

impl Solution for Day17 {
    fn part1(&self) -> Result<String> {
        let mut cpu = CPU::new_with_input();

        let output = cpu.execute();

        Ok(output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(","))
    }

    fn part2(&self) -> Result<String> {
        let mut cpu = CPU::new_with_input();

        let mut a = 0;
        let g = Getch::new();
        let mut magnitude = 1;
        getch_rs::disable_echo_input();

        loop {
            // there's some correlation between the octal format output and the target
            // number, probably enough to do a recursive approach starting from the end
            // of the desired list, but i've wasted enough time on this fake cpu.
            cpu.reset(a);
            let output = cpu.execute();

            println!();
            println!("[{a:o}o] desired={:?} len={}", cpu.ram, cpu.ram.len(),);
            println!(
                "[{a:o}o] output ={:?} len={} m={:o}o",
                output,
                output.len(),
                magnitude
            );

            let c = g.getch().map_err(|e| Error::new(&e.to_string()))?;
            match c {
                Key::Char('h') => a -= magnitude,
                Key::Char('j') if magnitude > 1 => magnitude /= 8,
                Key::Char('j') => {}
                Key::Char('k') => magnitude *= 8,
                Key::Char('l') => a += magnitude,

                _ => return Err(Error::new("aborted")),
            }
        }
    }
}

const OP_ADV: u64 = 0;

const OP_BXL: u64 = 1;
const OP_BST: u64 = 2;
const OP_JNZ: u64 = 3;
const OP_BXC: u64 = 4;
const OP_OUT: u64 = 5;
const OP_BDV: u64 = 6;
const OP_CDV: u64 = 7;
fn op_name(op: u64) -> &'static str {
    match op {
        OP_ADV => "OP_ADV",
        OP_BXL => "OP_BXL",
        OP_BST => "OP_BST",
        OP_JNZ => "OP_JNZ",
        OP_BXC => "OP_BXC",
        OP_OUT => "OP_OUT",
        OP_BDV => "OP_BDV",
        OP_CDV => "OP_CDV",
        _ => panic!("invalid opcode {}", op),
    }
}

struct CPU {
    pc: u64,

    ram: Vec<u64>,

    a: u64,
    b: u64,
    c: u64,

    quine_compare_mode: bool,
}

impl CPU {
    fn new(a: u64, b: u64, c: u64) -> Self {
        CPU {
            pc: 0,
            ram: Vec::new(),
            a,
            b,
            c,
            quine_compare_mode: false,
        }
    }

    fn new_with_input() -> Self {
        let mut cpu = CPU {
            pc: 0,
            ram: Vec::new(),
            a: 30118712,
            b: 0,
            c: 0,
            quine_compare_mode: false,
        };

        cpu.load_program(&[
            2, 4, // B = A % 8
            1, 3, // B = B ^ 3
            7, 5, // C = A / 2^B
            4, 2, // B = B ^ C
            0, 3, // A = A / 2^3 (8)
            1, 5, // B = B ^ 5
            5, 5, // OUT B
            3, 0, // IF (A != 0) GOTO 0
        ]);

        cpu
    }

    fn load_program(&mut self, prog: &[u64]) {
        assert!(prog.len() < u64::MAX as usize);

        self.ram.clear();
        self.ram.extend_from_slice(prog);
    }

    fn execute(&mut self) -> Vec<u64> {
        let mut output = Vec::with_capacity(self.ram.len());

        let mut iters = 0;

        while self.pc < self.ram.len() as u64 {
            iters += 1;

            let op = self.ram[self.pc as usize];
            let arg = self.ram[self.pc as usize + 1];

            self.pc += 2;

            match op {
                OP_ADV => self.a = self.a / 2_u64.pow(self.combo_value(arg).try_into().unwrap()),
                OP_BXL => self.b = self.b ^ arg,
                OP_BST => self.b = self.combo_value(arg) % 8,
                OP_JNZ => {
                    if self.a != 0 {
                        self.pc = arg;
                    }
                }
                OP_BXC => self.b = self.b ^ self.c,
                OP_OUT => {
                    output.push(self.combo_value(arg) % 8);
                    if self.quine_compare_mode {
                        if output[output.len() - 1] != self.ram[output.len() - 1] {
                            // we have failed to produce a quine already, abort.
                            return output;
                        }
                    }
                }
                OP_BDV => self.b = self.a / 2_u64.pow(self.combo_value(arg).try_into().unwrap()),
                OP_CDV => self.c = self.a / 2_u64.pow(self.combo_value(arg).try_into().unwrap()),
                op => panic!("invalid op={} at pc={}", op, self.pc),
            }
        }

        output
    }

    fn combo_value(&self, v: u64) -> u64 {
        match v {
            0..=3 => v,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("reference to reserved combo operand"),
            _ => panic!("invalid bit value for operand: {}", v),
        }
    }

    fn find_quine_range(&mut self) -> (u64, u64) {
        // idea: find low and high points in u64 space where we have the correct length to produce
        // a correctly sized program. this is based on analysis of how my input works.

        let low = self.binary_search_len_transition(0, u64::MAX, self.ram.len());
        let high = self.binary_search_len_transition(0, u64::MAX, self.ram.len() + 1);

        (low, high)
    }

    fn binary_search_len_transition(&mut self, low: u64, high: u64, to: usize) -> u64 {
        let midpoint: u64 = low + ((high - low) / 2); // almost certainly wrong
        let v1 = self.output_with_a_of(midpoint).len();
        let v2 = self.output_with_a_of(midpoint + 1).len();

        if v1 == to - 1 && v2 == to {
            midpoint
        } else if v1 < to {
            self.binary_search_len_transition(midpoint, high, to)
        } else {
            self.binary_search_len_transition(low, midpoint - 1, to)
        }
    }

    fn output_with_a_of(&mut self, a: u64) -> Vec<u64> {
        self.reset(a);
        self.execute()
    }

    fn find_quine(&mut self) -> u64 {
        let start = Instant::now();

        let (low, high) = self.find_quine_range();
        println!(
            "searching from {} to {} ({} values) for value",
            low,
            high,
            high - low
        );

        // self.quine_compare_mode = true;

        for a in low..=high {
            self.reset(a);

            let output = self.execute();
            if output == self.ram {
                return a;
            }

            if a % 100 == 0 {
                println!(
                    "[{a}] elapsed={}ms pct={:.2} output_len={}, output={:?}",
                    start.elapsed().as_millis(),
                    (a as f64 / u64::MAX as f64) * 100f64,
                    output.len(),
                    output
                )
            }
        }

        panic!("heat death of universe");
    }

    fn reset(&mut self, a: u64) {
        self.pc = 0;
        self.a = a;
        self.b = 0;
        self.c = 0;
    }
}

#[test]
fn test_part1() {
    let mut cpu = CPU::new(729, 0, 0);

    cpu.load_program(&[0, 1, 5, 4, 3, 0]);

    let output = cpu.execute();

    assert_eq!(vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0], output);
}

#[test]
fn test_part2() {
    let mut cpu = CPU::new(2024, 0, 0);

    cpu.load_program(&[0, 3, 5, 4, 3, 0]);

    let a = cpu.find_quine();

    assert_eq!(117440, a);
}
