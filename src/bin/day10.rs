use std::str::FromStr;

use color_eyre::{eyre::eyre, Result};

enum Inst {
    Addx { x: i64 },
    Noop,
}

impl FromStr for Inst {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            Ok(Self::Noop)
        } else if s.starts_with("addx") {
            let (_, num) = s.split_once(' ').unwrap();
            Ok(Self::Addx { x: num.parse()? })
        } else {
            Err(eyre!("Couldn't recognize instruction {s}"))
        }
    }
}

struct Cpu {
    reg: i64,
    cycles: usize,
    signal_strengths: Vec<i64>,
    crt_rows: Vec<Vec<char>>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            reg: 1,
            cycles: 0,
            signal_strengths: Vec::new(),
            crt_rows: vec![Vec::new()],
        }
    }

    fn add_pixel(&mut self, c: bool) {
        if self.crt_rows.last().unwrap().len() == 40 {
            self.crt_rows.push(Vec::new());
        }
        if c {
            self.crt_rows.last_mut().unwrap().push('#');
        } else {
            self.crt_rows.last_mut().unwrap().push('.');
        }
    }

    fn is_pixel_showing(&self, cycles: usize) -> bool {
        let current_horiz = (cycles % 40) as i64;
        current_horiz == self.reg || current_horiz == self.reg - 1 || current_horiz == self.reg + 1
    }

    fn apply(&mut self, inst: &Inst) {
        match inst {
            Inst::Addx { x } => {
                // two cycles
                self.add_pixel(self.is_pixel_showing(self.cycles));
                self.add_pixel(self.is_pixel_showing(self.cycles + 1));
                if ((self.cycles + 21) % 40) == 0 {
                    // Signal strength halfway through this cycle.
                    self.signal_strengths
                        .push((self.cycles as i64 + 1) * self.reg);
                } else if ((self.cycles + 22) % 40) == 0 {
                    // Signal strength at the second cycle of this, but _during_, so the reg is
                    // still not updated.
                    self.signal_strengths
                        .push((self.cycles as i64 + 2) * self.reg);
                }
                self.cycles += 2;
                self.reg += x;
            }
            Inst::Noop => {
                self.add_pixel(self.is_pixel_showing(self.cycles));
                if ((self.cycles + 21) % 40) == 0 {
                    // Signal strength at the end of the noop.
                    self.signal_strengths
                        .push((self.cycles as i64 + 1) * self.reg);
                }
                self.cycles += 1;
            }
        }
    }

    fn sum_signal_strengths(&self) -> i64 {
        self.signal_strengths.iter().sum()
    }

    fn print_crt(&self) {
        for v in &self.crt_rows {
            println!("{}", v.iter().collect::<String>());
        }
    }
}

fn part1(insts: &Vec<Inst>) -> i64 {
    let mut cpu = Cpu::new();

    for inst in insts {
        cpu.apply(&inst);
    }

    println!("Signal strength sums is {}", cpu.sum_signal_strengths());
    cpu.sum_signal_strengths()
}

fn part2(insts: &Vec<Inst>) {
    let mut cpu = Cpu::new();
    for inst in insts {
        cpu.apply(&inst);
    }

    cpu.print_crt();
}

fn parse_instrs(s: &str) -> Result<Vec<Inst>> {
    s.lines().map(Inst::from_str).collect()
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let instrs = parse_instrs(include_str!("../../day10.txt"))?;

    part1(&instrs);
    part2(&instrs);

    Ok(())
}

#[cfg(test)]
mod test {}
