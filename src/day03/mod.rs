use std::fmt::{Debug, Write};

use color_eyre::{eyre::eyre, Result};
use heapless::{FnvIndexSet as HashSet, Vec};

use crate::Runner;

pub struct Day;

#[derive(Clone, Copy, PartialEq, Ord, PartialOrd, Hash, Eq, hash32_derive::Hash32)]
pub struct Priority(u8);

impl Debug for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.0 {
            c @ 1..=26 => b'a' + c - 1,
            c @ 27..=52 => b'A' + c - 27,
            c => panic!("Bad data {c}"),
        } as char;
        f.write_char(c)
        // f.debug_tuple("Priority").field(&c).finish()
    }
}

impl Priority {
    fn from_char(c: u8) -> Result<Self> {
        Ok(Self(match c {
            c @ b'a'..=b'z' => c - b'a' + 1,
            c @ b'A'..=b'Z' => c - b'A' + 27,
            c => Err(eyre!("Not letter {c}"))?,
        }))
    }
}

impl Runner for Day {
    type Input = Vec<Vec<Priority, 48>, 300>;

    fn day() -> usize {
        3
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .lines()
            .map(|line| {
                line.trim()
                    .bytes()
                    .map(Priority::from_char)
                    .collect::<Result<_, _>>()
                    .unwrap()
            })
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(input
            .into_iter()
            .map(|pack| pack.split_at(pack.len() / 2))
            .map(|t| {
                assert_eq!(t.0.len(), t.1.len());
                t
            })
            .map(|(left, right)| -> usize {
                let left = HashSet::<_, 32>::from_iter(left.iter().copied());
                let right = HashSet::<_, 32>::from_iter(right.iter().copied());
                let mut intersection = left.intersection(&right).copied();
                intersection.next().unwrap().0 as usize
            })
            .sum())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        let answer = input
            .chunks_exact(3)
            .map(|l| {
                let [first, second, last] = l else {panic!("Bad number of packs")};
                let first = HashSet::<_, 32>::from_iter(first.iter().copied());
                let second = HashSet::<_, 32>::from_iter(second.iter().copied());
                let last = HashSet::<_, 32>::from_iter(last.iter().copied());
                let first = first
                    .intersection(&second)
                    .copied()
                    .collect::<HashSet<Priority, 32>>();
                let mut intersection = first.intersection(&last);
                intersection.next().unwrap().0 as usize
            })
            .sum::<usize>();
        // todo!();
        Ok(answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "\
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw";

        let input = Day::get_input(input)?;
        println!("{:#?}", input);
        assert_eq!(157, Day::part1(&input)?);
        assert_eq!(70, Day::part2(&input)?);
        Ok(())
    }
}
