use color_eyre::Result;

use crate::Runner;

pub struct Day;

impl Runner for Day {
    type Input<'input> = &'input str;

    fn day() -> usize {
        6
    }

    fn get_input<'input>(input: &'input str) -> Result<Self::Input<'input>> {
        Ok(input)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(get_index(input.as_bytes(), 4))
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        Ok(get_index(input.as_bytes(), 14))
    }
}

fn get_index(input: &[u8], window_size: usize) -> usize {
    // Fill the set with the first w - 1 items
    // The XOR will remove an item if it is in there twice.
    let mut set = input
        .into_iter()
        .take(window_size - 1)
        .fold(0_u64, |a, &c| a ^ (1 << (c - b'a')));
    input
        .windows(window_size)
        .enumerate()
        .find_map(|(i, slice)| {
            // Add the w_th item
            set ^= 1 << (slice.last().unwrap() - b'a');
            if TryInto::<usize>::try_into(set.count_ones()).unwrap() == window_size {
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
    use crate::helpers::sample_case;

    use super::*;

    sample_case! {
        sample1 =>
            input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
            part1 = 7;
            part2 = 19;
    }

    sample_case! {
        sample2 =>
            input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
            part1 = 5;
            part2 = 23;
    }

    sample_case! {
        sample3 =>
            input = "nppdvjthqldpwncqszvftbrmjlhg";
            part1 = 6;
            part2 = 23;
    }
}
