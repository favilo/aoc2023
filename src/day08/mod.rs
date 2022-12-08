use std::iter::once;

use color_eyre::Result;
use itertools::Itertools;
use ndarray::{s, Array2};
use nom::{
    character::complete::multispace0, combinator::all_consuming, error::VerboseError, multi::many1,
    sequence::preceded, Finish,
};

use crate::{parsers::single_digit_line, Runner};

pub struct Day;

impl Runner for Day {
    type Input<'input> = Array2<usize>;

    fn day() -> usize {
        8
    }

    fn get_input<'input>(input: &'input str) -> Result<Self::Input<'input>> {
        let v = all_consuming::<_, _, VerboseError<_>, _>(many1(preceded(
            multispace0,
            single_digit_line,
        )))(input.as_bytes())
        .finish()
        .unwrap()
        .1;
        let height = v.len();
        let width = v[0].len();
        let v = v.into_iter().flatten().collect::<Vec<_>>();
        let v = Array2::from_shape_vec((height, width), v)?;
        Ok(v)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .indexed_iter()
            .filter(|(idx, e)| is_visible(input, *idx, **e))
            .count())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .indexed_iter()
            .map(|(idx, e)| scenic_score(input, idx, *e))
            .max()
            .unwrap())
    }
}

fn is_visible(array: &Array2<usize>, idx: (usize, usize), e: usize) -> bool {
    let (row, col) = idx;
    [
        s![..row, col],
        s![row + 1.., col],
        s![row, ..col],
        s![row, col + 1..],
    ]
    .into_iter()
    .any(|i| array.slice(i).iter().all(|v| *v < e))
}

fn scenic_score(array: &Array2<usize>, idx: (usize, usize), e: usize) -> usize {
    let (row, col) = idx;
    [
        s![..row;-1, col],
        s![row + 1.., col],
        s![row, ..col; -1],
        s![row, col + 1..; 1],
    ]
    .into_iter()
    .map(|idx| array.slice(idx))
    .map(|s| {
        let mut count = s.iter().take_while(|v| **v < e).count();
        // If we finish early, count the tree that we see last
        if count < s.len() {
            count += 1
        }
        count
    })
    .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::sample_case;

    sample_case! {
        sample1 =>
            input = "\
                30373
                25512
                65332
                33549
                35390";
            part1 = 21;
            part2 = 8;
    }
}
