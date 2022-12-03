use color_eyre::Result;
use heapless::{
    binary_heap::{Max, Min},
    BinaryHeap,
};
use itertools::Itertools;

use crate::{utils::parse_int, Runner};

pub struct Day;

// Do something silly to see if I can get this faster
#[derive(Default, Clone, Debug)]
pub struct Top3(BinaryHeap<usize, Max, 3>);

impl FromIterator<usize> for Top3 {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut this = BinaryHeap::<usize, Min, 4>::new();
        iter.into_iter().for_each(|i| {
            this.push(i).unwrap();
            if this.len() > 3 {
                this.pop();
            }
        });
        let mut new = BinaryHeap::<_, Max, 3>::new();
        (0..3).for_each(|_| {
            new.push(this.pop().unwrap_or(0)).unwrap();
        });
        Self(new)
    }
}

impl<'a> IntoIterator for &'a Top3 {
    type Item = &'a usize;
    type IntoIter = std::slice::Iter<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Runner for Day {
    type Input = Top3;

    fn day() -> usize {
        1
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let nums = input
            .lines()
            .map(&str::trim)
            .batching(|it| {
                Some(
                    it.take_while(|l| !l.is_empty())
                        .map(str::as_bytes)
                        .map(parse_int)
                        .sum(),
                )
            })
            .take_while(|&s| s > 0)
            .collect();
        Ok(nums)
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(*input.into_iter().next().unwrap())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        Ok(input.into_iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "\
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(24000, Day::part1(&input)?);
        assert_eq!(45000, Day::part2(&input)?);
        Ok(())
    }
}
