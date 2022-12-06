use color_eyre::Result;
use std::collections::HashSet;

const UNIQUE_CHARS: usize = 14;

fn main() -> Result<()> {
    let mut iter = include_str!("../../day6.txt").lines();

    // Get the crates into the stacks
    while let Some(line) = iter.next() {
        let mut end_idx = UNIQUE_CHARS;
        let mut startofmessage: &str = line.get((end_idx - UNIQUE_CHARS)..end_idx).unwrap();
        while startofmessage.chars().collect::<HashSet<_>>().len() != UNIQUE_CHARS {
            end_idx += 1;
            startofmessage = line.get((end_idx - UNIQUE_CHARS)..end_idx).unwrap();
        }

        println!("Start at {end_idx}");
    }

    Ok(())
}
