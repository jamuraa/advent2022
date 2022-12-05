use color_eyre::eyre::ContextCompat;
use color_eyre::Result;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Stacks {
    /// Crates in each stack, starting from the bottom to the top
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn new(stacks: usize) -> Self {
        Stacks {
            stacks: (0..stacks).map(|_| Vec::new()).collect(),
        }
    }

    fn add_to_stack(&mut self, num: usize, c: char) {
        self.stacks.get_mut(num - 1).unwrap().push(c);
    }

    fn move_crate(&mut self, times: usize, from: usize, to: usize) {
        for _ in 0..times {
            let c = self.stacks.get_mut(from - 1).unwrap().pop().unwrap();
            self.stacks.get_mut(to - 1).unwrap().push(c);
        }
    }

    fn move_crate_stack(&mut self, num: usize, from: usize, to: usize) {
        let cratestack_len = self.stacks.get(from - 1).unwrap().len();
        let mut cratestack = self
            .stacks
            .get_mut(from - 1)
            .unwrap()
            .split_off(cratestack_len - num);
        assert_eq!(num, cratestack.len());
        self.stacks.get_mut(to - 1).unwrap().append(&mut cratestack);
    }

    fn tops(&self) -> String {
        self.stacks
            .iter()
            .enumerate()
            .map(|(n, stack)| {
                stack
                    .as_slice()
                    .last()
                    .context(format!("stack {n} is empty"))
                    .unwrap()
            })
            .collect()
    }

    fn get_crate(&self, from: usize, num: usize) -> Option<&char> {
        self.stacks.get(from).map(|c| c.get(num)).flatten()
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_stacks = self.stacks.len();
        let tallest = self.stacks.iter().max_by_key(|x| x.len()).unwrap().len();
        for height in (0..tallest).rev() {
            for cr_idx in 0..num_stacks {
                match self.get_crate(cr_idx, height) {
                    Some(c) => write!(f, "[{c}] ")?,
                    None => write!(f, "    ")?,
                }
            }
            writeln!(f)?;
        }
        for cr_idx in 1..=num_stacks {
            write!(f, " {cr_idx}  ")?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let file = File::open("day5.txt").map(BufReader::new)?;

    let mut iter = file.lines();

    // Get the crates into the stacks
    let mut crates_lines_upsidedown: Vec<String> = Vec::new();
    while let Some(line) = iter.next() {
        let line = line?;
        if line.get(1..=1).unwrap() == "1" {
            break;
        }
        crates_lines_upsidedown.push(line);
    }

    crates_lines_upsidedown.reverse();
    let num_stacks = crates_lines_upsidedown.get(0).unwrap().split(' ').count();

    let mut stacks = Stacks::new(num_stacks);

    for line in crates_lines_upsidedown {
        for num in 0..num_stacks {
            let index = 4 * num + 1;
            if let Some(c) = line.get(index..=index) {
                if c == " " {
                    continue;
                }
                println!("Adding {c} to stack {}", num + 1);
                stacks.add_to_stack(num + 1, c.chars().next().unwrap());
            }
        }
    }

    println!("Iniital Stacks:");
    println!("{stacks}");

    let _blank_line = iter.next();

    let mut stacks_copy = stacks.clone();

    while let Some(move_line) = iter.next() {
        let move_line = move_line?;
        let mut tokens = move_line.split(' ');
        let _move = tokens.next();
        let count: usize = tokens.next().unwrap().parse()?;
        let _from = tokens.next();
        let fr: usize = tokens.next().unwrap().parse()?;
        let _to = tokens.next();
        let to: usize = tokens.next().unwrap().parse()?;
        stacks_copy.move_crate(count, fr, to);
        stacks.move_crate_stack(count, fr, to);
    }

    println!("Stack tops part one: {}", stacks_copy.tops());
    println!("Stack tops part two: {}", stacks.tops());

    Ok(())
}
