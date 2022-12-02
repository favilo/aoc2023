use anyhow::Result;
use itertools::Itertools;

use crate::Runner;

pub struct Day;

impl Runner<i32, i32> for Day {
    type Input = Vec<Vec<i32>>;

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
                    Some(group.map(|i| i.parse::<i32>().unwrap()).collect())
                } else {
                    None
                }
            })
            .collect();
        Ok(nums)
    }

    fn part1(input: &Self::Input) -> Result<i32> {
        let total = input.iter().map(|v| v.iter().sum()).max().unwrap();
        Ok(total)
    }

    fn part2(input: &Self::Input) -> Result<i32> {
        let total = input.iter().map(|v| v.iter().sum::<i32>()).sorted();
        Ok(total.into_iter().rev().take(3).sum())
    }
}
