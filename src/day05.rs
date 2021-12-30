// AoC 2021 - Day 5

use std::fmt;
use std::fs;
use std::path::Path;


#[derive(Eq, PartialEq)]
pub struct Point {
    x: u32,
    y: u32,
}


#[derive(Eq, PartialEq)]
pub struct Line (Point, Point);


impl Line {

    /// Shared implementation of "fmt" for traits fmt::Display and fmt::Debug
    ///
    /// Example:
    /// ```
    /// let line = Line (
    ///     Point {x: 0, y: 1},
    ///     Point {x: 2, y: 3},
    /// );
    /// let repr = format!("{}", line);
    /// assert_eq!(
    ///     repr,
    ///     "Line<(0, 1), (2, 3)>",
    /// );
    /// ```
    pub fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = format!(
            "Line<({x1}, {y1}), ({x2}, {y2})>",
            x1=self.0.x, y1=self.0.y,
            x2=self.1.x, y2=self.1.y,
        );
        return write!(f, "{}", repr);
    }

}


impl fmt::Debug for Line {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.fmt(f);
    }

}


impl fmt::Display for Line {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.fmt(f);
    }

}



type Axis<'line> = Vec<&'line Line>;


/// Transform the puzzle input into data structures shared between puzzle parts.
mod input {

    use super::{Axis, Point, Line};

    /// Original puzzle input, deserialized.
    pub type Parsed = Vec<Line>;

    /// Shared with both puzzle parts optimized for generating a solution.
    pub struct Optimized {

    }

    /// Deserializes the original puzzle input.
    ///
    /// Example:
    /// ```
    /// let source = "711,893 -> 711,29";
    /// let parsed = parse(source);
    /// assert_eq!(
    ///     parsed,
    ///     vec![
    ///         Line (
    ///             Point {x: 711, y: 893},
    ///             Point {x: 711, y: 29},
    ///         )
    ///     ]
    /// );
    /// ```
    ///
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
                Line (
                    Point {x: tuples[0][0], y: tuples[0][1]},
                    Point {x: tuples[1][0], y: tuples[1][1]},
                )
            }
        ).collect();

        return parsed;
    }

    /// Generate the optimal shared input for both day parts.
    pub fn optimize(parsed: &Parsed) -> Optimized {

        let (mut horizontals, mut verticals): (Axis, Axis) = parsed.iter()

            // Filter only horizontal and vertical lines.
            // Panic on diagonals, as a runtime sanity check.
            .filter(
                |line| {
                    let Point {x: x1, y: y1} = line.0;
                    let Point {x: x2, y: y2} = line.1;
                    if (x1 == x2) || (y1 == y2) {
                        true
                    } else {
                        panic!("diagonal line encountered: {}", line);
                    }
                }
            )

            // Group lines by whether they're horizontal or vertical
            .partition(
                |line| {
                    let Point {x: x1, y: _} = line.0;
                    let Point {x: x2, y: _} = line.1;
                    x1 == x2
                }
            );

        // Sort the horizontal lines by their y value, ascending
        horizontals.sort_by(|a, b| { (a.0).y.cmp(&(b.0).y) } );

        // Sort the vertical lines by their x value.
        verticals.sort_by(|a, b| { (a.0).x.cmp(&(b.0).x) } );

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
                Line(
                    Point {x: 0, y: 0},
                    Point {x: 0, y: 0},
                ),
                Line(
                    Point {x: 1, y: 2},
                    Point {x: 3, y: 4},
                ),
                Line(
                    Point {x: 98, y: 76},
                    Point {x: 54, y: 32},
                ),
            ]
        );

}

    #[test]
    fn test_part1_example() {

    }

}
