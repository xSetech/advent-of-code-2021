// AoC 2021 - Day template

use std::fs;
use std::path::Path;


/// Transform the puzzle input into data structures shared between puzzle parts.
mod input {

    /// Original puzzle input, deserialized.
    pub struct Parsed {

    }

    /// Shared with both puzzle parts optimized for generating a solution.
    pub struct Optimized {

    }

    /// Deserializes the original puzzle input.
    pub fn parse(source: &str) -> Parsed {
        return Parsed {};
    }

    /// Generate the optimal shared input for both parts.
    pub fn optimize(parsed: &Parsed) -> Optimized {
        return Optimized {};
    }

}


mod part1 {

    use super::input;

    pub fn solution(input: &input::Optimized) -> u32 {
        0
    }

}


mod part2 {

    use super::input;

    pub fn solution(input: &input::Optimized) -> u32 {
        0
    }

}


pub fn main(input_path: &Path) {

    let input_source: String = fs::read_to_string(input_path).expect("failed to read input");
    let input_parsed = input::parse(&input_source);
    let input_optimized = input::optimize(&input_parsed);
    let part1_output = part1::solution(&input_optimized);
    println!("Part 1 answer: {}", part1_output);
    let part2_output = part2::solution(&input_optimized);
    println!("Part 2 answer: {}", part2_output);

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1_example() {

    }

}
