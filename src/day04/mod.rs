use std::ops::RangeInclusive;

use color_eyre::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map,
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::{utils::parse_int, Runner};

pub struct Day;

#[derive(Debug)]
pub struct Pair(RangeInclusive<usize>, RangeInclusive<usize>);

fn inside(first: &RangeInclusive<usize>, second: &RangeInclusive<usize>) -> bool {
    (first.start() <= second.start() && first.end() >= second.end())
        || (second.start() <= first.start() && second.end() >= first.end())
}

fn overlap(first: &RangeInclusive<usize>, second: &RangeInclusive<usize>) -> bool {
    inside(first, second)
        || (first.start() <= second.start() && first.end() >= second.start())
        || (first.start() <= second.end() && first.end() >= second.end())
}

fn range(input: &[u8]) -> IResult<&[u8], RangeInclusive<usize>> {
    let (input, (first, second)) =
        separated_pair(map(digit1, parse_int), tag("-"), map(digit1, parse_int))(input)?;
    Ok((input, first..=second))
}

fn range_pair(input: &[u8]) -> IResult<&[u8], Pair> {
    let (input, (first, second)) = separated_pair(range, tag(","), range)(input)?;
    Ok((input, Pair(first, second)))
}

fn pair_vec(input: &[u8]) -> IResult<&[u8], Vec<Pair>> {
    many1(delimited(multispace0, range_pair, multispace0))(input)
}

impl Runner for Day {
    type Input = Vec<Pair>;

    fn day() -> usize {
        4
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(pair_vec(input.as_bytes()).unwrap().1)
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(input.iter().filter(|Pair(f, s)| inside(f, s)).count())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        Ok(input.iter().filter(|Pair(f, s)| overlap(f, s)).count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "\
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(2, Day::part1(&input)?);
        assert_eq!(4, Day::part2(&input)?);
        Ok(())
    }
}
