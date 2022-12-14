use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
};

use color_eyre::Result;
use itertools::{iterate, Itertools};
use nom::{
    bytes::complete::tag, character::complete::u64, combinator::map, error::VerboseError,
    multi::separated_list1, sequence::separated_pair, IResult,
};

use crate::Runner;

pub struct Day;

type ParseResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;
type Finished = bool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Falling,
    Stuck,
    Exited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NextStep {
    coord: Coord,
    status: Status,
}

// Lets do x-500 so we can center around the origin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(isize, isize);

impl Coord {
    fn next_step(self, map: &HashMap<Coord, Cell>) -> NextStep {
        let neighbor = self.neighbors().into_iter().find(|n| !map.contains_key(n));
        if let Some(coord) = neighbor {
            // TODO: Eventually check for exited better
            if coord.1 > 200 {
                NextStep {
                    coord,
                    status: Status::Exited,
                }
            } else {
                NextStep {
                    coord,
                    status: Status::Falling,
                }
            }
        } else {
            NextStep {
                coord: self,
                status: Status::Stuck,
            }
        }
    }

    fn neighbors(self) -> [Coord; 3] {
        [self + Coord(0, 1), self + Coord(-1, 1), self + Coord(1, 1)]
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cell {
    #[default]
    Rock,
    Sand,
}

fn rock_point(input: &str) -> ParseResult<Coord> {
    map(separated_pair(u64, tag(","), u64), |(c, r)| {
        Coord(c as isize - 500, r as isize)
    })(input)
}

fn rock_curve(input: &str) -> ParseResult<Vec<Coord>> {
    let (input, points) = separated_list1(tag(" -> "), rock_point)(input)?;
    let points = points
        .into_iter()
        .tuple_windows()
        .map(|(Coord(c0, r0), Coord(c1, r1))| {
            if c0 == c1 {
                // vertical
                (r0.min(r1)..=r0.max(r1))
                    .into_iter()
                    .map(|r| Coord(c0, r))
                    .collect_vec()
            } else if r0 == r1 {
                (c0.min(c1)..=c0.max(c1))
                    .into_iter()
                    .map(|c| Coord(c, r0))
                    .collect_vec()
            } else {
                unreachable!()
            }
        })
        .flatten()
        .collect();
    Ok((input, points))
}

fn parse_map(input: &str) -> Result<HashMap<Coord, Cell>> {
    Ok(input
        .lines()
        .map(|line| rock_curve(line).unwrap().1)
        .flatten()
        .map(|c| (c, Cell::Rock))
        .collect::<HashMap<_, _>>())
}

fn run_grain(map: &mut HashMap<Coord, Cell>) -> Finished {
    let start = Coord(0, 0);
    if map.contains_key(&start) {
        return true;
    }
    let end = iterate(
        NextStep {
            coord: start,
            status: Status::Falling,
        },
        |last_step| last_step.coord.next_step(map),
    )
    .find(|s| !matches!(s.status, Status::Falling))
    .unwrap();
    if end.status == Status::Exited {
        // println!("Exited at {end:?}");
        true
    } else if end.status == Status::Stuck {
        // println!("Stopped falling at {end:?}");
        map.insert(end.coord, Cell::Sand);
        false
    } else {
        unreachable!("Should never exit with Falling")
    }
}

impl Runner for Day {
    type Input<'input> = HashMap<Coord, Cell>;

    fn day() -> usize {
        14
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        parse_map(input)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut map = input.clone();
        Ok((1..)
            .map(|i| (i, run_grain(&mut map)))
            .take_while(|(_, finished)| !finished)
            .last()
            .unwrap()
            .0)
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let highest = input.keys().map(|c| c.1).max().unwrap();
        let mut map = input.clone();
        (-200..=200).for_each(|y| {
            map.insert(Coord(y, highest + 2), Cell::Rock);
        });
        Ok((1..)
            .map(|i| (i, run_grain(&mut map)))
            .take_while(|(_, finished)| !finished)
            .last()
            .unwrap()
            .0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                498,4 -> 498,6 -> 496,6\n\
                503,4 -> 502,4 -> 502,9 -> 494,9\n";
            part1 = 24;
            part2 = 93;
    }

    prod_case! {
        part1 = 1001;
        part2 = 27976;
    }
}
