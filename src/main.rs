use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, PartialEq, Eq)]
struct Elf {
    // Calorie counts of this elf's inventory
    item_calories: Vec<usize>,
    total_calories: usize,
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_calories.cmp(&other.total_calories)
    }
}

impl Elf {
    fn add_item(&mut self, calories: usize) {
        self.item_calories.push(calories);
        self.total_calories += calories;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut elves = Vec::new();

    let file = BufReader::new(File::open("day1.txt")?);

    let mut new_elf = Elf::default();
    for line in file.lines() {
        let line = line?;
        if line.is_empty() {
            let done_elf = std::mem::replace(&mut new_elf, Elf::default());
            elves.push(done_elf);
        } else {
            new_elf.add_item(line.parse()?);
        }
    }
    // Add the last elf if for some reason we didn't end with an empty line
    if !new_elf.item_calories.is_empty() {
        elves.push(new_elf);
    }

    elves.as_mut_slice().sort();
    elves.as_mut_slice().reverse();
    let _rest = elves.split_off(5);

    let mut total_for_top_3 = 0;
    for (i, elf) in elves.iter().enumerate() {
        println!(
            "Elf #{i}: Carrying {} items for {} calories",
            elf.item_calories.len(),
            elf.total_calories
        );
        if i < 3 {
            total_for_top_3 += elf.total_calories;
        }
    }

    println!("Top three elves are carrying {total_for_top_3} calories");

    Ok(())
}
