// AoC 2021 - Day 5

use std::fs;
use std::path::Path;


/// Transform the puzzle input into data structures shared between puzzle parts.
mod input {

    pub type Coord = (u32, u32);
    pub type Line = (Coord, Coord);  // A -> B

    /// Original puzzle input, deserialized.
    pub type Parsed = Vec<((u32, u32), (u32, u32))>;

    /// Shared with both puzzle parts optimized for generating a solution.
    pub struct Optimized {

    }

    /// Deserializes the original puzzle input.
    pub fn parse(source: &str) -> Parsed {

        // Cast each line into a sequence of integers.
        let intermediate: Vec<Vec<Vec<u32>>> = source.lines().map(
            |line| {
                line
                .split(" -> ")
                .map(
                    |pair| {
                        pair
                        .split(',')
                        .map(
                            |n| {
                                n.parse().unwrap()
                            }
                        ).collect::<Vec<u32>>()     // (u32, u32)
                    }
                ).collect::<Vec<Vec<u32>>>()        // ((u32, u32), (u32, u32))
            }
        ).collect();                                // Vec<...>

        // Convert the inner sequences into a fixed size tuple.
        let parsed: Parsed = intermediate.iter().map(
            |tuples| {
                (
                    (tuples[0][0], tuples[0][1]),
                    (tuples[1][0], tuples[1][1]),
                )
            }
        ).collect();

        return parsed;
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
    use super::super::utils::remove_multiline_whitespace;

    #[test]
    fn test_input_parsing() {

        let input_source = remove_multiline_whitespace("
            0,0 -> 0,0
            1,2 -> 3,4
            98,76 -> 54,32
        ");
        let input_parsed = input::parse(&input_source);
        assert_eq!(
            input_parsed,
            vec![
                ((0, 0), (0, 0)),
                ((1, 2), (3, 4)),
                ((98,76), (54,32)),
            ]
        );

    }

    #[test]
    fn test_part1_example() {

    }

}
