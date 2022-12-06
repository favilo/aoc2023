use byte_set::ByteSet;
use color_eyre::Result;
use multiset::HashMultiSet;

use crate::Runner;

pub struct Day;

impl Runner for Day {
    type Input = Vec<u8>;

    fn day() -> usize {
        6
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(Vec::from(input.as_bytes()))
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(get_index(input, 4))
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        Ok(get_index2(input, 14))
    }
}

fn get_index(input: &Vec<u8>, n: usize) -> usize {
    input
        .windows(n)
        .enumerate()
        .find(|(_, slice)| all_different(slice))
        .unwrap()
        .0
        + n
}

fn get_index2(input: &Vec<u8>, n: usize) -> usize {
    let mut set = input
        .into_iter()
        .take(n)
        .copied()
        .collect::<HashMultiSet<_>>();
    input
        .windows(n + 1)
        .enumerate()
        .find(|(_, slice)| {
            let found = set.distinct_elements().len() == slice.len() - 1;
            set.remove(&slice[0]);
            set.insert(*slice.last().unwrap());
            found
        })
        .unwrap()
        .0
        + n
}

fn all_different(slice: &[u8]) -> bool {
    let set = slice.into_iter().fold(ByteSet::new(), |mut a, t| {
        a.insert(*t);
        a
    });
    set.len() == slice.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(7, Day::part1(&input)?);
        assert_eq!(19, Day::part2(&input)?);
        Ok(())
    }

    #[test]
    fn sample2() -> Result<()> {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(5, Day::part1(&input)?);
        assert_eq!(23, Day::part2(&input)?);
        Ok(())
    }

    #[test]
    fn sample3() -> Result<()> {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(6, Day::part1(&input)?);
        assert_eq!(23, Day::part2(&input)?);
        Ok(())
    }
}
