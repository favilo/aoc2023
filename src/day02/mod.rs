use std::{cmp::Ordering, str::FromStr};

use anyhow::Result;

use crate::Runner;

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

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err("Wrong letter"),
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

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().split(' ').take(2).collect();
        Ok(Self(Hand::from_str(&s[0])?, Hand::from_str(&s[1])?))
    }
}

impl Runner for Day {
    type Input = Vec<Round>;

    fn day() -> usize {
        2
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .lines()
            .map(Round::from_str)
            .map(Result::unwrap)
            .collect())
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
