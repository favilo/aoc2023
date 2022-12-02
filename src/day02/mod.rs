use std::cmp::Ordering;

use anyhow::Result;

use crate::{utils::trim_ascii_whitespace, Runner};

pub struct Day;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    Lose,
    Draw,
    Win,
}

impl Status {
    fn from_hand(h: Hand) -> Self {
        match h {
            Hand::Rock => Self::Lose,
            Hand::Paper => Self::Draw,
            Hand::Scissors => Self::Win,
        }
    }
}

impl Hand {
    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn beat(self) -> Self {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (a, b) if a == b => Some(Ordering::Equal),
            (Hand::Rock, Hand::Scissors)
            | (Hand::Paper, Hand::Rock)
            | (Hand::Scissors, Hand::Paper) => Some(Ordering::Greater),
            _ => Some(Ordering::Less),
        }
    }
}

impl From<&[u8]> for Hand {
    fn from(s: &[u8]) -> Self {
        match s[0] {
            b'A' | b'X' => Hand::Rock,
            b'B' | b'Y' => Hand::Paper,
            b'C' | b'Z' => Hand::Scissors,
            c => panic!("Wrong letter {c:?}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Round(Hand, Hand);

impl Round {
    fn score(&self) -> usize {
        let Self(other, mine) = self;
        let wins = match mine.partial_cmp(&other) {
            Some(Ordering::Greater) => 6,
            Some(Ordering::Equal) => 3,
            Some(Ordering::Less) => 0,
            None => unreachable!(),
        };
        wins + mine.score()
    }
}

impl From<&[u8]> for Round {
    fn from(s: &[u8]) -> Self {
        let mut s = trim_ascii_whitespace(s).split(u8::is_ascii_whitespace);
        Self(Hand::from(s.next().unwrap()), Hand::from(s.next().unwrap()))
    }
}

impl Runner for Day {
    type Input = Vec<Round>;

    fn day() -> usize {
        2
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input.lines().map(str::as_bytes).map(Round::from).collect())
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(input.iter().map(Round::score).sum())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        Ok(input
            .iter()
            .map(|r| (r.0, Status::from_hand(r.1)))
            .map(|(other, s)| {
                Round(
                    other,
                    match s {
                        Status::Lose => other.beat().beat(),
                        Status::Draw => other,
                        Status::Win => other.beat(),
                    },
                )
                .score()
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "\
            A Y
            B X
            C Z";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(15, Day::part1(&input)?);
        assert_eq!(12, Day::part2(&input)?);
        Ok(())
    }
}
