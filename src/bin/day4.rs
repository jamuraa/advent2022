use color_eyre::{eyre::eyre, Report, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct CleaningRange {
    first: u64,
    last: u64,
}

impl CleaningRange {
    fn contains(&self, other: &Self) -> bool {
        self.first <= other.first && self.last >= other.last
    }

    fn overlaps(&self, other: &Self) -> bool {
        (other.last >= self.last && other.first <= self.last)
            || (other.first <= self.first && other.last >= self.first)
    }
}

impl FromStr for CleaningRange {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, last) = s
            .split_once('-')
            .ok_or(eyre!("Missing a '-' in this {s}"))?;
        Ok(Self {
            first: first.parse()?,
            last: last.parse()?,
        })
    }
}

fn main() -> Result<()> {
    let file = BufReader::new(File::open("day4.txt")?);

    let mut completely_overlaps = 0;
    let mut partially_overlaps = 0;

    let mut iter = file.lines();

    while let Some(line) = iter.next() {
        let line = line?;
        let (first, second) = line.split_once(',').unwrap();
        let f_range = CleaningRange::from_str(first)?;
        let s_range = CleaningRange::from_str(second)?;
        if f_range.contains(&s_range) || s_range.contains(&f_range) {
            println!("{f_range:?} completely overlaps {s_range:?}");
            completely_overlaps += 1;
            partially_overlaps += 1;
        } else if f_range.overlaps(&s_range) || s_range.overlaps(&f_range) {
            println!("{f_range:?} partially overlaps {s_range:?}");
            partially_overlaps += 1;
        }
    }

    println!("Completely overlaps: {completely_overlaps}");
    println!("Partially overlaps: {partially_overlaps}");

    Ok(())
}
