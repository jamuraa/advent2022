use color_eyre::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}

fn main() -> Result<()> {
    let file = BufReader::new(File::open("day3.txt")?);

    let mut sum = 0;

    let mut iter = file.lines();

    loop {
        let first = match iter.next() {
            Some(first) => first?,
            None => break,
        };
        let second = iter.next().unwrap()?;
        let third = iter.next().unwrap()?;

        let first_set: HashSet<char> = first.chars().collect();
        let second_set: HashSet<char> = second.chars().collect();
        let third_set: HashSet<char> = third.chars().collect();

        let firstsecond: HashSet<char> = first_set.intersection(&second_set).copied().collect();
        let intersection = firstsecond.intersection(&third_set).next().unwrap();

        let priority = priority(*intersection);
        sum += priority;

        println!("{first} and {second} and {third} share {intersection:?} (pri {priority})");
    }
    println!("sum: {sum}");

    Ok(())
}
