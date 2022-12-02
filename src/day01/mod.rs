use std::collections::BinaryHeap;

use anyhow::Result;
use itertools::Itertools;

use crate::{utils::parse_int, Runner};

pub struct Day;

impl Runner for Day {
    type Input = BinaryHeap<usize>;

    fn day() -> usize {
        1
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let nums = input
            .lines()
            .map(&str::trim)
            .group_by(|i| !i.is_empty())
            .into_iter()
            .filter_map(|(k, group)| {
                if k {
                    Some(group.map(|i| parse_int(i.as_bytes())).sum())
                } else {
                    None
                }
            })
            .collect();
        Ok(nums)
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(*input.into_iter().next().unwrap())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        Ok(input.into_iter().take(3).sum())
    }
}
