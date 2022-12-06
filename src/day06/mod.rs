use color_eyre::Result;

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
        Ok(get_index(input, 14))
    }
}

fn get_index(input: &Vec<u8>, window_size: usize) -> usize {
    // Fill the set with the first w - 1 items
    // The XOR will remove an item if it is in there twice.
    let mut set = input
        .into_iter()
        .take(window_size - 1)
        .fold(0_u64, |a, &c| a ^ (c - b'a') as u64);
    input
        .windows(window_size)
        .enumerate()
        .find_map(|(i, slice)| {
            // Add the w_th item
            set ^= 1 << (slice.last().unwrap() - b'a');
            if set.count_ones() == window_size.try_into().unwrap() {
                return Some(i + window_size);
            }
            // And when you remove it here, it will turn on if there was an even number of them
            set ^= 1 << (slice.first().unwrap() - b'a');
            None
        })
        .unwrap()
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
