use color_eyre::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, multispace1, newline},
    combinator::{map, opt},
    error::VerboseError,
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use heapless::String;

use crate::{parsers::number, Runner};

pub struct Day;

#[derive(Debug, Clone)]
pub struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

// TODO: Let's see about making these heapless::Vec
#[derive(Debug, Clone, Default)]
pub struct Stacks {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

fn container(input: &str) -> IResult<&str, Option<char>, VerboseError<&str>> {
    let (input, c): (&str, Option<char>) = terminated(
        alt((
            map(tag("   "), |_| None),                         // Either it's 3 spaces
            delimited(tag("["), map(anychar, Some), tag("]")), // Or it's `[<letter>]`
        )),
        opt(char(' ')), // Might have a single space after it, unless it's the end of the line
    )(input)?;
    Ok((input, c))
}

fn container_line(input: &str) -> IResult<&str, Vec<Option<char>>, VerboseError<&str>> {
    terminated(many1(container), newline)(input)
}

fn index_line(input: &str) -> IResult<&str, usize, VerboseError<&str>> {
    let (input, v) = many1(terminated(
        delimited(char(' '), number, opt(char(' '))),
        alt((char(' '), newline)),
    ))(input)?;
    Ok((input, *v.last().unwrap()))
}

fn instruction(input: &str) -> IResult<&str, Instruction, VerboseError<&str>> {
    let (input, (_, num, _, from, _, to, _)) = tuple((
        tag("move "),
        number,
        tag(" from "),
        number,
        tag(" to "),
        number,
        opt(newline),
    ))(input)?;
    Ok((
        input,
        Instruction {
            num,
            from: from - 1,
            to: to - 1,
        },
    ))
}

fn stacks_section(input: &str) -> IResult<&str, Vec<Vec<char>>, VerboseError<&str>> {
    let (input, container_lines) = many1(container_line)(input)?;
    let (input, num_containers) = index_line(input)?;
    let (input, _) = multispace1(input)?;

    let mut containers = vec![Vec::default(); num_containers]; // create a vector for each container

    container_lines.iter().for_each(|line| {
        line.iter()
            .enumerate()
            .for_each(|(i, c)| c.iter().for_each(|&c| containers[i].insert(0, c)))
    });
    Ok((input, containers))
}

fn stacks<'a>(input: &'a str) -> IResult<&'a str, Stacks, VerboseError<&'a str>> {
    let (input, stacks) = stacks_section(input)?; // Containers section

    let (input, instructions) = many1(instruction)(input)?; // Instruction section
    Ok((
        input,
        Stacks {
            stacks,
            instructions,
        },
    ))
}

impl Runner<String<9>, String<9>> for Day {
    type Input = Stacks;

    fn day() -> usize {
        5
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let input = input.to_owned();
        let (input, stacks) = stacks(&input).unwrap();
        debug_assert_eq!(input, "");
        Ok(stacks)
    }

    fn part1(input: &Self::Input) -> Result<String<9>> {
        let mut stacks = input.stacks.clone();
        input
            .instructions
            .iter()
            .for_each(|Instruction { num, from, to }| {
                (0..*num).for_each(|_| {
                    let t = stacks[*from].pop().unwrap(); // We'll just panic if we have an empty list
                    stacks[*to].push(t);
                });
            });
        Ok(stacks.iter().map(|l| l.last().unwrap()).collect())
    }

    fn part2(input: &Self::Input) -> Result<String<9>> {
        let mut stacks = input.stacks.clone();
        input
            .instructions
            .iter()
            .for_each(|Instruction { num, from, to }| {
                let mut to_stack = std::mem::replace(&mut stacks[*to], Vec::default());
                let new_len = stacks[*from].len() - num;
                let mut drained = stacks[*from].split_off(new_len);
                to_stack.append(&mut drained);
                let _ = std::mem::replace(&mut stacks[*to], to_stack);
            });
        Ok(stacks.iter().map(|l| l.last().unwrap()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!("CMZ", &Day::part1(&input)?);
        assert_eq!("MCD", &Day::part2(&input)?);
        Ok(())
    }
}
