use std::fmt::{Debug, Write};

use byte_set::ByteSet;
use color_eyre::{eyre::eyre, Result};
use heapless::Vec;

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
    fn from_ascii(c: u8) -> Result<Self> {
        Ok(Self(match c {
            c @ b'a'..=b'z' => c - b'a' + 1,
            c @ b'A'..=b'Z' => c - b'A' + 27,
            c => Err(eyre!("Not letter {c}"))?,
        }))
    }

    fn to_inner(&self) -> u8 {
        self.0
    }
}

impl Runner for Day {
    type Input = Vec<[ByteSet; 2], 300>;

    fn day() -> usize {
        3
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .lines()
            .map(|line| {
                let (first, second) = line.trim().split_at(line.len() / 2);
                [first, second].map(|line| {
                    line.bytes()
                        .map(Priority::from_ascii)
                        .map(Result::unwrap)
                        .map(|p| p.to_inner())
                        .collect::<ByteSet>()
                })
            })
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(input
            .into_iter()
            .map(|[left, right]| -> usize {
                let intersection = left.intersection(*right);
                intersection.into_iter().next().unwrap() as usize
            })
            .sum())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        let answer = input
            .chunks_exact(3)
            .map(|l| {
                let [first, second, last] = l else {panic!("Bad number of packs")};
                let first = first[0].union(first[1]);
                let second = second[0].union(second[1]);
                let last = last[0].union(last[1]);
                let intersect = first.intersection(second);
                intersect.intersection(last).into_iter().next().unwrap() as usize
            })
            .sum::<usize>();
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
