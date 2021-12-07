// AoC 2021 - Day 3

use std::fs;
use std::path::Path;

pub struct Input {
    pub width: usize,
    pub data: Vec<u32>,
}


/// Return a bit mask representing the most common bit per column from rows of
/// equally size (give by Input.width) integers.
///
/// Example:
///
/// ```
/// let i = Input {
///     width: 3 as usize,
///     data: vec![
///         0b010
///         0b111
///         0b110
///     ],
/// }
/// assert_eq!(0b110, most_common_bit(&i, None));
/// ```
///
/// A "tie breaker" can be given if there's a tie between 0 or 1 for a column.
/// The value is either 0 or 1, whichever will win the tie by default. If there
/// is a tie without a tie breaker, the code will panic.
pub fn most_common_bit(input: &Input, tie_breaker: Option<u32>) -> u32 {

    let mut counters = vec![0; input.width];

    for int in input.data.iter() {
        for offset in 0..input.width {
            let mask = 1 << offset as u32;
            if (mask & int) == mask {
                counters[offset] += 1;
            } else {
                counters[offset] -= 1;
            }
        }
    }

    let value_parts = counters.iter().rev().map(
        |val| {
            if *val > 0 { 1 }
            else if *val < 0 { 0 }
            else {
                // tie!
                let tb = tie_breaker.expect("tie without a tie breaker");
                if tb == 1 { 1 } else { 0 }
            }
        }
    );

    let mut gamma_rate: u32 = 0;
    for part in value_parts {
        gamma_rate <<= 1;
        gamma_rate += part;
    }
    return gamma_rate;

}


mod part1 {

    use super::*;

    pub fn input(source: &str) -> Input {
        let lines: Vec<&str> = source.lines().collect();
        let width: usize = lines[0].len();
        let data: Vec<u32> = lines.iter().map(
            |line| {
                isize::from_str_radix(line, 2).expect("failed to cast to int") as u32
            }
        ).collect();
        return Input { width, data };
    }

    pub fn solution(input: &Input) -> u32 {

        let gamma_rate = most_common_bit(input, None);
        let mask = (1 << input.width) - 1;
        let epsilon_rate: u32 = !gamma_rate & mask;

        return gamma_rate * epsilon_rate;

    }

}


pub fn main(input_path: &Path) {

    let part1_input_source: String = fs::read_to_string(input_path).expect("failed to read part 1 input");
    let part1_input = part1::input(&part1_input_source);
    let part1_output = part1::solution(&part1_input);
    println!("Part 1 answer: {}", part1_output);

}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::utils;

    /// The example inputs and outputs given by AOC
    #[test]
    fn test_part1_example() {

        let example_input = "
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010
        ";
        let example_input_cleaned = utils::remove_multiline_whitespace(&example_input);
        let part1_input = part1::input(&example_input_cleaned);
        let part1_output = part1::solution(&part1_input);
        assert_eq!(part1_output, 198);

    }

}