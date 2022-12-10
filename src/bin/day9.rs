use std::{collections::HashSet, str::FromStr};

use color_eyre::{eyre::eyre, Result};

enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl FromStr for Direction {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .chars()
            .nth(0)
            .ok_or(eyre!("direction must be one char"))?
        {
            'R' => Ok(Self::Right),
            'U' => Ok(Self::Up),
            'L' => Ok(Self::Left),
            'D' => Ok(Self::Down),
            x => Err(eyre!("unrecognized direction {x}")),
        }
    }
}

struct Move {
    dir: Direction,
    steps: u8,
}

impl FromStr for Move {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, steps_str) = s.split_once(' ').ok_or(eyre!("Couldn't parse {s}"))?;
        Ok(Self {
            dir: dir_str.parse::<Direction>()?,
            steps: steps_str.parse::<u8>()?,
        })
    }
}

fn parse_moves(s: &str) -> Result<Vec<Move>> {
    s.lines().map(|l| l.parse::<Move>()).collect()
}

#[derive(Default, Debug)]
struct Board {
    knot_locations: Vec<(i32, i32)>,
}

fn pairs_mut_each<T: std::fmt::Debug>(vec: &mut Vec<T>, f: fn(&mut T, &mut T)) {
    for idx in 0..vec.len() - 1 {
        let sl = vec.as_mut_slice();
        let (front, back) = sl.split_at_mut(idx + 1);
        let mut front_mut = front.last_mut().unwrap();
        let mut back_mut = back.first_mut().unwrap();
        f(&mut front_mut, &mut back_mut);
    }
}

impl Board {
    fn new(length: usize) -> Self {
        Self {
            knot_locations: (0..length).map(|_| (0, 0)).collect(),
        }
    }

    fn tail_location(&self) -> (i32, i32) {
        *self.knot_locations.last().unwrap()
    }

    /// Applies the move given. Returns the path that the Tail has taken.
    fn apply(&mut self, m: &Move) -> Vec<(i32, i32)> {
        let mut tail_pos = vec![self.tail_location()];

        for _ in 0..m.steps {
            tail_pos.push(self.go_dir(&m.dir));
        }
        tail_pos
    }

    /// Makes the head go in a direction, and then returns the position of the Tail afterwards.
    fn go_dir(&mut self, d: &Direction) -> (i32, i32) {
        let loc = self.knot_locations.first_mut().unwrap();
        *loc = match d {
            Direction::Right => (loc.0, loc.1 + 1),
            Direction::Left => (loc.0, loc.1 - 1),
            Direction::Up => (loc.0 - 1, loc.1),
            Direction::Down => (loc.0 + 1, loc.1),
        };
        pairs_mut_each(&mut self.knot_locations, |head, tail| {
            match (head.0 - tail.0, head.1 - tail.1) {
                // T.H => .TH
                // or T..     ..H
                //    ..H  or T..
                (0, 2) | (1, 2) | (-1, 2) => *tail = (head.0, head.1 - 1),
                // H.T => HT
                // or H..     ..T
                //    ..T  or H..
                (0, -2) | (1, -2) | (-1, -2) => *tail = (head.0, head.1 + 1),
                // (same but vertically)
                (2, 0) | (2, 1) | (2, -1) => *tail = (head.0 - 1, head.1),
                (-2, 0) | (-2, 1) | (-2, -1) => *tail = (head.0 + 1, head.1),
                // Diagonally far away
                (-2, 2) => *tail = (head.0 + 1, head.1 - 1),
                (2, 2) => *tail = (head.0 - 1, head.1 - 1),
                (-2, -2) => *tail = (head.0 + 1, head.1 + 1),
                (2, -2) => *tail = (head.0 - 1, head.1 + 1),
                // Nothing to do, head is either covering or touxhing tail.
                (0, 0)
                | (1, 0)
                | (0, 1)
                | (1, 1)
                | (-1, 0)
                | (0, -1)
                | (-1, 1)
                | (1, -1)
                | (-1, -1) => {}
                _ => panic!(
                    "Somehow the head and tail got too far apart: head = {:?}, tail = {:?}",
                    head, tail
                ),
            }
        });
        *self.knot_locations.iter().last().unwrap()
    }
}

fn part1(moves: &Vec<Move>) -> usize {
    let mut visited_tail = HashSet::new();

    let mut board = Board::new(2);

    for m in moves {
        for loc in board.apply(&m) {
            visited_tail.insert(loc);
        }
    }
    visited_tail.len()
}

fn part2(moves: &Vec<Move>) -> usize {
    let mut visited_tail = HashSet::new();
    let mut board = Board::new(10);
    for m in moves {
        for loc in board.apply(&m) {
            visited_tail.insert(loc);
        }
    }
    visited_tail.len()
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let moves = parse_moves(include_str!("../../day9.txt"))?;

    let visited_locns = part1(&moves);
    let visited_locns_long = part2(&moves);

    println!("Tail visited {} locations", visited_locns);
    println!("Tail visited {} locations", visited_locns_long);

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    fn example_moves() -> Vec<Move> {
        let s = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        parse_moves(s).unwrap()
    }

    fn example_moves_longer() -> Vec<Move> {
        let s = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        parse_moves(s).unwrap()
    }

    #[test]
    fn example() {
        assert_eq!(13, part1(&example_moves()));
    }

    #[test]
    fn example_long() {
        assert_eq!(1, part2(&example_moves()));
    }

    #[test]
    fn rope_example() {
        assert_eq!(36, part2(&example_moves_longer()));
    }

    #[test]
    fn pairs_mut_works() {
        let mut v = vec![0, 1, 2, 3, 4, 5];
        pairs_mut_each(&mut v, |a, b| {
            *a = *a + 1;
            *b = *b + 1;
        });
        assert_eq!(1, v[0]);
        assert_eq!(3, v[1]);
        assert_eq!(4, v[2]);
        assert_eq!(5, v[3]);
        assert_eq!(6, v[4]);
        assert_eq!(6, v[5]);
    }
}
