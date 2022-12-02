use anyhow::{anyhow, Error};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Scissors,
    Paper,
    Rock,
}

impl FromStr for Shape {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(anyhow!("Shapes are only one letter, not {}", s.len()));
        }
        match s.chars().next().unwrap() {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => Err(anyhow!("Invalid shape string")),
        }
    }
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

impl GameResult {
    fn score(&self) -> u64 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loss => 0,
        }
    }
}

impl FromStr for GameResult {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(anyhow!("Results are only one letter, not {}", s.len()));
        }
        match s.chars().next().unwrap() {
            'X' => Ok(GameResult::Loss),
            'Y' => Ok(GameResult::Draw),
            'Z' => Ok(GameResult::Win),
            _ => Err(anyhow!("Invalid result string")),
        }
    }
}

impl Shape {
    /// Returns the shape you throw to get the result given when playing against this shape.
    fn for_result(&self, result: GameResult) -> Self {
        use GameResult::*;
        use Shape::*;
        match (self, result) {
            (x, Draw) => *x,
            (Scissors, Win) => Rock,
            (Rock, Win) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Loss) => Paper,
            (Rock, Loss) => Scissors,
            (Paper, Loss) => Rock,
        }
    }

    fn value(&self) -> u64 {
        match self {
            Shape::Scissors => 3,
            Shape::Paper => 2,
            Shape::Rock => 1,
        }
    }

    fn versus(&self, opp: &Self) -> GameResult {
        use GameResult::*;
        use Shape::*;
        match (self, opp) {
            (Scissors, Paper) => Win,
            (Paper, Rock) => Win,
            (Rock, Scissors) => Win,
            (x, y) if x == y => Draw,
            _ => Loss,
        }
    }

    /// Returns the score you receive if you play against opp
    fn score(&self, opp: &Self) -> u64 {
        self.value() + self.versus(opp).score()
    }
}

fn main() -> Result<(), Error> {
    let file = BufReader::new(File::open("day2.txt")?);

    let mut total_score = 0;
    for line in file.lines() {
        let line = line?;
        let mut game: Vec<_> = line.split(' ').collect();
        let my_result = game.pop().unwrap().parse::<GameResult>()?;
        let their_throw = game.pop().unwrap().parse::<Shape>()?;
        let my_throw = their_throw.for_result(my_result);

        let score = my_throw.score(&their_throw);
        println!("Played {my_throw:?} vs {their_throw:?} and got {score} points");
        total_score += score;
    }

    println!("Total score: {total_score}");

    Ok(())
}
